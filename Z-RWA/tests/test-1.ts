import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddress,
  createMint,
  mintToChecked,
  getAccount,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { assert } from "chai";
import { TokenProgram } from "../target/types/token_program";
import { FundContract } from "../target/types/fund_contract";

var pdaSuperMaintainers,
  pdaSuperConfig,
  pdaSuperWhitelist,
  pdaSuperPartialFreeze,
  pdaSuperMintAccount = null;
const TOKEN_PROGRAM = new PublicKey(
  "33pHXYQbe41JJSA7oXor6h7JFY74eqH25xtSjBysmTYo",
);

// Helper for Robust Funding
const fundAccount = async (provider: anchor.AnchorProvider, recipient: anchor.web3.PublicKey, amountSol: number = 1) => {
  const balance = await provider.connection.getBalance(recipient);
  if (balance >= amountSol * anchor.web3.LAMPORTS_PER_SOL) return;

  try {
    // Try transferring from provider first
    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: recipient,
        lamports: amountSol * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await provider.sendAndConfirm(tx);
    console.log(`Transferred ${amountSol} SOL to ${recipient.toBase58()}`);
    return;
  } catch (e) {
    console.log(`Transfer failed for ${recipient.toBase58()}, trying airdrop...`);
  }

  // Fallback to Airdrop
  for (let i = 0; i < 3; i++) {
    try {
      const sig = await provider.connection.requestAirdrop(recipient, amountSol * anchor.web3.LAMPORTS_PER_SOL);
      await provider.connection.confirmTransaction(sig);
      console.log(`Airdropped ${amountSol} SOL to ${recipient.toBase58()}`);
      return;
    } catch {
      await new Promise(res => setTimeout(res, 2000));
    }
  }
  throw new Error(`Failed to fund ${recipient.toBase58()}`);
};

// Create test keypairs
const admin = anchor.web3.Keypair.generate();
const payer = anchor.web3.Keypair.generate();
const user1 = anchor.web3.Keypair.generate();
const user2 = anchor.web3.Keypair.generate();
const issuer = anchor.web3.Keypair.generate();
const transferAgent = anchor.web3.Keypair.generate();
const tokenizationAgent = anchor.web3.Keypair.generate();
const fundManager = anchor.web3.Keypair.generate();
const mintAuthority = anchor.web3.Keypair.generate();
const user1ATA = null; // Placeholder check

// Create constant amount fields
const MINT_AMOUNT = new BN(1000);
const BURN_AMOUNT = new BN(600);
const BURN_FROM_AMOUNT = new BN(200);
const DECIMALS = 6;

// Constant seeds
const TEST_TOKEN = "Test";
const TEST_1_TOKEN = "Test-1";
const MINT = Buffer.from("mint");
const MAINTAINERS = Buffer.from("maintainers");
const CONFIG = Buffer.from("config");
const PARTIAL_FREEZE = Buffer.from("partial_freeze");
const WHITELIST = Buffer.from("whitelist");
const STABLE_COIN = Buffer.from("stable_coin");
const TEST = Buffer.from(TEST_TOKEN);
const TEST_1 = Buffer.from(TEST_1_TOKEN);
const GLOBAL_CONFIG = Buffer.from("global_config");
const AGENT = Buffer.from("agent");

describe("token_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenProgram as Program<TokenProgram>;

  // Declare PDAs
  let pdaMaintainers,
    pdaConfig = null;

  const confirmTransaction = async (tx) => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });
  };

  const whitelistAccount = async (
    whitelistParams,
    pdaMaintainers,
    pdaConfig,
    pdaWhitelist,
  ) => {
    // Test whitelist account instruction
    let whitelist = await program.methods
      .whitelistUser(whitelistParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        whitelist: pdaWhitelist,
        authority: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(whitelist);
  };

  const partialFreeze = async (
    pdaMaintainers,
    pdaConfig,
    pdaPartialFreeze,
    user,
    token,
    amount,
  ) => {
    // Test partial freeze account instruction
    let freeze = await program.methods
      .partialFreezeAccount(token, amount)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        partialFreeze: pdaPartialFreeze,
        user,
        authority: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(freeze);
  };

  const partialUnfreeze = async (
    pdaMaintainers,
    pdaConfig,
    pdaPartialFreeze,
    token,
    amount,
  ) => {
    // Test partial unfreeze account instruction
    let unfreeze = await program.methods
      .partialUnfreezeAccount(token, amount)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        partialFreeze: pdaPartialFreeze,
        user: user1.publicKey,
        caller: admin.publicKey,
      })
      .signers([admin])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(unfreeze);
  };

  const freeze = async (
    pdaMaintainers,
    pdaConfig,
    mintAccount,
    token,
    user,
    signer,
  ) => {
    // Test freeze account
    let freeze = await program.methods
      .freezeUserAccount(token)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        mintAccount,
        user,
        caller: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(freeze);
  };

  const unfreeze = async (
    pdaMaintainers,
    pdaConfig,
    mintAccount,
    token,
    user,
    signer,
  ) => {
    // Test unfreeze account
    let unfreeze = await program.methods
      .unfreezeUserAccount(token)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        mintAccount,
        user,
        caller: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(unfreeze);
  };

  const createToken = async (
    createTokenParams,
    pdaConfig,
    mintAccount,
    pdaMaintainers,
  ) => {
    // Test create_token instruction
    let createToken = await program.methods
      .create(createTokenParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        mintAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        payer: admin.publicKey,
      })
      .signers([admin])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(createToken);
  };

  const mint = async (
    tokenParams,
    pdaMaintainers,
    mintAccount,
    pdaConfig,
    pdaWhitelist,
    user1ATA,
    signer,
  ) => {
    // Test mint_token instruction
    let mintToken = await program.methods
      .mintToken(tokenParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        whitelist: pdaWhitelist,
        mintAccount,
        tokenAccount: user1ATA,
        toAccount: user1ATA,
        authority: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(mintToken);
  };

  const dvp = async (
    tokenParams,
    pdaMaintainers,
    mintAccount,
    pdaPartialFreeze,
    pdaConfig,
    pdaWhitelist,
    user1ATA,
    signer,
  ) => {
    // Test delivery vs payment instruction
    let dvp = await program.methods
      .dvp(tokenParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        whitelist: pdaWhitelist,
        mintAccount,
        partialFreeze: pdaPartialFreeze,
        tokenAccount: user1ATA,
        toAccount: user1ATA,
        authority: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(dvp);
  };

  const burn = async (tokenParams, pdaMaintainers, mintAccount, user1ATA) => {
    // Test burn_token instruction
    let burnToken = await program.methods
      .burnToken(tokenParams)
      .accounts({
        maintainers: pdaMaintainers,
        mintAccount,
        from: user1ATA,
        authority: user1.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([user1])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(burnToken);
  };

  const burnFrom = async (
    tokenParams,
    pdaMaintainers,
    pdaConfig,
    pdaWhitelist,
    pdaPartialFreeze,
    mintAccount,
    user1ATA,
    signer,
  ) => {
    // Burn from user1 account by admin
    let burnToken = await program.methods
      .burnTokenFrom(tokenParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        whitelist: pdaWhitelist,
        partialFreeze: pdaPartialFreeze,
        mintAccount,
        from: user1ATA,
        tokenAccount: user1ATA,
        authority: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([signer])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(burnToken);
  };

  const transfer = async (
    transferParams,
    pdaMaintainers,
    pdaConfig,
    pdaWhitelist,
    pdaPartialFreeze,
    mintAccount,
    fromATA,
    toATA,
  ) => {
    // Test transfer token instruction
    let transferToken = await program.methods
      .transferTokens(transferParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        whitelist: pdaWhitelist,
        partialFreeze: pdaPartialFreeze,
        mintAccount,
        tokenAccount: fromATA,
        fromAccount: fromATA,
        toAccount: toATA,
        authority: user1.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user1])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(transferToken);
  };

  const forceTransfer = async (
    forceTransferParams,
    pdaMaintainers,
    pdaConfig,
    pdaWhitelistFrom,
    pdaWhitelistTo,
    pdaPartialFreeze,
    mintAccount,
    fromATA,
    toATA,
    signer,
  ) => {
    // Test force transfer token instruction
    let forceTransferToken = await program.methods
      .forceTransferTokens(forceTransferParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        fromWhitelist: pdaWhitelistFrom,
        toWhitelist: pdaWhitelistTo,
        partialFreeze: pdaPartialFreeze,
        mintAccount,
        tokenAccount: fromATA,
        fromAccount: fromATA,
        toAccount: toATA,
        authority: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .preInstructions([
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
      ])
      .rpc();

    await confirmTransaction(forceTransferToken);
  };

  it("Initialize test accounts", async () => {
    // Fund all accounts using robust helper
    await fundAccount(provider, admin.publicKey);
    await fundAccount(provider, payer.publicKey);
    await fundAccount(provider, user1.publicKey);
    await fundAccount(provider, user2.publicKey);
    await fundAccount(provider, mintAuthority.publicKey);
    await fundAccount(provider, issuer.publicKey);
    await fundAccount(provider, transferAgent.publicKey);
    await fundAccount(provider, tokenizationAgent.publicKey);
    await fundAccount(provider, fundManager.publicKey);
  });

  it("Initialize global account", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    pdaSuperMaintainers = pdaMaintainers;

    // Test initialize instruction
    try {
      let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
      console.log("Global Maintainers already initialized.");
    } catch (e) {
      let init = await program.methods
        .init()
        .accounts({
          maintainers: pdaMaintainers,
          authority: admin.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([admin])
        .preInstructions([
          anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({ units: 400000 })
        ])
        .rpc();

      await confirmTransaction(init);
    }

    let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
    assert.equal(maintainers.admin.toString(), admin.publicKey.toString());
    assert.isTrue(
      JSON.stringify(maintainers.subAdmins).includes(
        JSON.stringify(admin.publicKey),
      ),
    );
  });

  it("Test Create Token", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    pdaSuperConfig = pdaConfig;
    pdaSuperMintAccount = mintAccount;

    let tokenLimit = new BN(4000);
    let createTokenParams = {
      id: "unique",
      name: TEST_TOKEN,
      symbol: "tes",
      uri: "some/uri",
      tokenLimit,
      countryCodes: [91, 1],
      issuer: issuer.publicKey,
      transferAgent: transferAgent.publicKey,
      tokenizationAgent: tokenizationAgent.publicKey,
      holdingPeriod: new BN(0),
    };

    await createToken(
      createTokenParams,
      pdaConfig,
      mintAccount,
      pdaMaintainers,
    );

    // Check the configuration after transaction
    let config = await program.account.tokenConfiguration.fetch(pdaConfig);
    assert.equal(config.tokenLimit.toNumber(), tokenLimit.toNumber());
    assert.isTrue(JSON.stringify(config.countryCodes).includes("1"));
    assert.isTrue(JSON.stringify(config.countryCodes).includes("91"));
    assert.equal(config.issuer.toString(), issuer.publicKey.toString());
    assert.equal(
      config.transferAgent.toString(),
      transferAgent.publicKey.toString(),
    );
    assert.equal(
      config.tokenizationAgent.toString(),
      tokenizationAgent.publicKey.toString(),
    );
    assert.equal(
      Number(config.holdingPeriod),
      Number(createTokenParams.holdingPeriod),
    );

    // Creating another token
    createTokenParams = {
      id: "unique-1",
      name: TEST_1_TOKEN,
      symbol: "tes1",
      uri: "some/uri",
      tokenLimit: tokenLimit,
      countryCodes: [91, 1],
      issuer: issuer.publicKey,
      transferAgent: transferAgent.publicKey,
      tokenizationAgent: tokenizationAgent.publicKey,
      holdingPeriod: new BN(0),
    };

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST_1],
      program.programId,
    );

    [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST_1],
      program.programId,
    );

    await createToken(
      createTokenParams,
      pdaConfig,
      mintAccount,
      pdaMaintainers,
    );

    // Check the configuration after transaction
    config = await program.account.tokenConfiguration.fetch(pdaConfig);
    assert.equal(config.tokenLimit.toNumber(), tokenLimit.toNumber());
    assert.isTrue(JSON.stringify(config.countryCodes).includes("1"));
    assert.isTrue(JSON.stringify(config.countryCodes).includes("91"));
    assert.equal(config.issuer.toString(), issuer.publicKey.toString());
    assert.equal(
      config.transferAgent.toString(),
      transferAgent.publicKey.toString(),
    );
    assert.equal(
      config.tokenizationAgent.toString(),
      tokenizationAgent.publicKey.toString(),
    );
    assert.equal(
      Number(config.holdingPeriod),
      Number(createTokenParams.holdingPeriod),
    );
  });

  it("Test Mint Token", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
    };

    // Creating associated token for user1 for Test
    let user1ATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mintAccount,
      user1.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Whitelisting user1 for Test
    let whitelistParams = {
      token: TEST_TOKEN,
      user: user1.publicKey,
      code: 91,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    pdaSuperWhitelist = pdaWhitelist;

    await whitelistAccount(
      whitelistParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
    );

    await mint(
      tokenParams,
      pdaMaintainers,
      mintAccount,
      pdaConfig,
      pdaWhitelist,
      user1ATA.address,
      admin,
    );

    // Check balance after mint
    let supply = await provider.connection.getTokenSupply(mintAccount);
    assert.equal(Number(supply.value.amount), Number(MINT_AMOUNT));

    let user1Account = await getAccount(
      provider.connection,
      user1ATA.address,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.equal(Number(user1Account.amount), Number(MINT_AMOUNT));

    // Minting Token Test-1
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST_1],
      program.programId,
    );

    [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST_1],
      program.programId,
    );

    tokenParams = {
      name: TEST_1_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
    };

    // Whitelisting user1 for Test-1
    whitelistParams = {
      token: TEST_1_TOKEN,
      user: user1.publicKey,
      code: 91,
    };

    [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    await whitelistAccount(
      whitelistParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
    );

    // Creating associated token for user1 for Test-1
    user1ATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mintAccount,
      user1.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await mint(
      tokenParams,
      pdaMaintainers,
      mintAccount,
      pdaConfig,
      pdaWhitelist,
      user1ATA.address,
      admin,
    );

    // Check balance after mint
    supply = await provider.connection.getTokenSupply(mintAccount);
    assert.equal(Number(supply.value.amount), Number(MINT_AMOUNT));

    user1Account = await getAccount(
      provider.connection,
      user1ATA.address,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.equal(Number(user1Account.amount), Number(MINT_AMOUNT));

    let balance = await provider.connection.getBalance(user1.publicKey);
    // Here balance is divided by 10^6 to remove decimal values return by getBalance method
    assert.equal(balance / Math.pow(10, 6), Number(MINT_AMOUNT));
  });

  it("Test Burn Token", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_AMOUNT,
    };

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await burn(tokenParams, pdaMaintainers, mintAccount, user1ATA);

    // Check balance after mint
    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.equal(
      Number(user1Account.amount),
      Number(MINT_AMOUNT) - Number(BURN_AMOUNT),
    );
    let supply = await provider.connection.getTokenSupply(mintAccount);
    assert.equal(
      Number(supply.value.amount),
      Number(MINT_AMOUNT) - Number(BURN_AMOUNT),
    );

    // Burning Token Test-1
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST_1],
      program.programId,
    );

    [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST_1],
      program.programId,
    );

    tokenParams = {
      name: TEST_1_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_AMOUNT,
    };

    // Creating associated token for user1 and Test-1
    user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await burn(tokenParams, pdaMaintainers, mintAccount, user1ATA);

    // Check balance after mint
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.equal(
      Number(user1Account.amount),
      Number(MINT_AMOUNT) - Number(BURN_AMOUNT),
    );
    supply = await provider.connection.getTokenSupply(mintAccount);
    assert.equal(
      Number(supply.value.amount),
      Number(MINT_AMOUNT) - Number(BURN_AMOUNT),
    );
  });

  it("Test Whitelist Account", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let whitelistParams = {
      token: TEST_TOKEN,
      user: user1.publicKey,
      code: 91,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    await whitelistAccount(
      whitelistParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
    );

    let countryCode = (
      await program.account.whitelistedUser.fetch(pdaWhitelist)
    ).countryCode;
    assert.equal(countryCode, 91);
  });

  it("Test non whitelisted account", async () => {
    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    try {
      await program.account.whitelistedUser.fetch(pdaWhitelist);
    } catch (e) {
      console.log("Account is not whitelisted as PDA doesn't exist!");
    }
  });

  it("Test Partial Freeze", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    pdaSuperPartialFreeze = pdaPartialFreeze;

    let amount = new BN(100);

    await partialFreeze(
      pdaMaintainers,
      pdaConfig,
      pdaPartialFreeze,
      user1.publicKey,
      TEST_TOKEN,
      amount,
    );

    let partialFreezedAmount = (
      await program.account.partialFreeze.fetch(pdaPartialFreeze)
    ).amount;
    assert.equal(Number(partialFreezedAmount), Number(amount));

    let frozenTokens = (
      await program.account.tokenConfiguration.fetch(pdaConfig)
    ).frozenTokens;
    assert.equal(Number(frozenTokens), Number(amount));
  });

  it("Test Delivery Vs Payment", async () => {
    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
    };

    // Get associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1AccountBeforeDvp = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let partialFreezedAmountBeforeDvp = (
      await program.account.partialFreeze.fetch(pdaPartialFreeze)
    ).amount;

    let frozenTokensBeforeDvp = (
      await program.account.tokenConfiguration.fetch(pdaConfig)
    ).frozenTokens;

    await dvp(
      tokenParams,
      pdaMaintainers,
      mintAccount,
      pdaPartialFreeze,
      pdaConfig,
      pdaWhitelist,
      user1ATA,
      admin,
    );

    let user1AccountAfterDvp = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // User1 balance must be increased
    assert.equal(
      Number(user1AccountAfterDvp.amount),
      Number(user1AccountBeforeDvp.amount) + Number(tokenParams.amount),
    );

    let partialFreezedAmountAfterDvp = (
      await program.account.partialFreeze.fetch(pdaPartialFreeze)
    ).amount;

    // Frozen tokens of an account must be increased with the same value
    assert.equal(
      Number(partialFreezedAmountAfterDvp),
      Number(partialFreezedAmountBeforeDvp) + Number(tokenParams.amount),
    );

    let frozenTokensAfterDvp = (
      await program.account.tokenConfiguration.fetch(pdaConfig)
    ).frozenTokens;

    // Frozen tokens value must be increased with the same amount
    assert.equal(
      Number(frozenTokensAfterDvp),
      Number(frozenTokensBeforeDvp) + Number(tokenParams.amount),
    );
  });

  it("Test Partial Unfreeze", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let amount = new BN(100);

    let partialFreezedAmountBefore = (
      await program.account.partialFreeze.fetch(pdaPartialFreeze)
    ).amount;

    let frozenTokensBefore = (
      await program.account.tokenConfiguration.fetch(pdaConfig)
    ).frozenTokens;

    await partialUnfreeze(
      pdaMaintainers,
      pdaConfig,
      pdaPartialFreeze,
      TEST_TOKEN,
      amount,
    );

    let partialFreezedAmountAfter = (
      await program.account.partialFreeze.fetch(pdaPartialFreeze)
    ).amount;
    assert.equal(
      Number(partialFreezedAmountAfter),
      Number(partialFreezedAmountBefore) - Number(amount),
    );
    let frozenTokensAfter = (
      await program.account.tokenConfiguration.fetch(pdaConfig)
    ).frozenTokens;
    assert.equal(
      Number(frozenTokensAfter),
      Number(frozenTokensBefore) - Number(amount),
    );
  });

  it("Test account that is not partially frozen", async () => {
    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    try {
      await program.account.partialFreeze.fetch(pdaPartialFreeze);
    } catch (e) {
      console.log("Account is not partially freezed as PDA doesn't exist!");
    }
  });

  it("Test Burn Token From", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_FROM_AMOUNT,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check balance before burn from
    let user1AccountBeforeBurn = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check supply before burn
    let supplyBeforeBurn =
      await provider.connection.getTokenSupply(mintAccount);

    await burnFrom(
      tokenParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      admin,
    );

    // Check balance after burn from
    let user1AccountAfterBurn = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.equal(
      Number(user1AccountAfterBurn.amount),
      Number(user1AccountBeforeBurn.amount) - Number(tokenParams.amount),
    );

    // Check supply after burn
    let supplyAfterBurn = await provider.connection.getTokenSupply(mintAccount);

    assert.equal(
      Number(supplyAfterBurn.value.amount),
      Number(supplyBeforeBurn.value.amount) - Number(tokenParams.amount),
    );

    // Burning Token Test-1
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST_1],
      program.programId,
    );

    [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST_1],
      program.programId,
    );

    tokenParams = {
      name: TEST_1_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_FROM_AMOUNT,
    };

    [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test-1
    user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check balance before burn from
    user1AccountBeforeBurn = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check supply before burn
    supplyBeforeBurn = await provider.connection.getTokenSupply(mintAccount);

    await burnFrom(
      tokenParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      admin,
    );

    // Check balance after burn from
    user1AccountAfterBurn = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.equal(
      Number(user1AccountAfterBurn.amount),
      Number(user1AccountBeforeBurn.amount) - Number(tokenParams.amount),
    );

    // Check supply after burn
    supplyAfterBurn = await provider.connection.getTokenSupply(mintAccount);

    assert.equal(
      Number(supplyAfterBurn.value.amount),
      Number(supplyBeforeBurn.value.amount) - Number(tokenParams.amount),
    );
  });

  it("Test Burn Token From with Frozen Balance", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Burning Token Test-1
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST_1],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST_1],
      program.programId,
    );

    let tokenParams = {
      name: TEST_1_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_FROM_AMOUNT,
    };

    [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test-1
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let freezeAmount = new BN(150);

    await partialFreeze(
      pdaMaintainers,
      pdaConfig,
      pdaPartialFreeze,
      user1.publicKey,
      TEST_1_TOKEN,
      freezeAmount,
    );

    try {
      await burnFrom(
        tokenParams,
        pdaMaintainers,
        pdaConfig,
        pdaWhitelist,
        pdaPartialFreeze,
        mintAccount,
        user1ATA,
        admin,
      );
    } catch (err) {
      assert.equal(err.error.errorCode.code, "BalanceFrozen");
    }
  });

  it("Test Transfer Token", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let transferAmount = new BN(50);

    let transferParams = {
      token: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: transferAmount,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceBeforeTransfer = Number(user1Account.amount);

    let user2ATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mintAccount,
      user2.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let user2Account = await getAccount(
      provider.connection,
      user2ATA.address,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceBeforeTransfer = Number(user2Account.amount);

    await transfer(
      transferParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      user2ATA.address,
    );
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceAfterTransfer = Number(user1Account.amount);

    user2Account = await getAccount(
      provider.connection,
      user2ATA.address,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceAfterTransfer = Number(user2Account.amount);

    // Check balances after transfer
    assert.equal(
      user1BalanceAfterTransfer,
      user1BalanceBeforeTransfer - Number(transferAmount),
    );
    assert.equal(
      user2BalanceAfterTransfer,
      user2BalanceBeforeTransfer + Number(transferAmount),
    );
  });

  it("Test Transfer Token with Frozen Balance", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST_1],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST_1],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let transferAmount = new BN(50);

    let transferParams = {
      token: TEST_1_TOKEN,
      toAccount: user1.publicKey,
      amount: transferAmount,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user2ATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      mintAccount,
      user2.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    try {
      await transfer(
        transferParams,
        pdaMaintainers,
        pdaConfig,
        pdaWhitelist,
        pdaPartialFreeze,
        mintAccount,
        user1ATA,
        user2ATA.address,
      );
    } catch (err) {
      assert.equal(err.error.errorCode.code, "BalanceFrozen");
    }
  });

  it("Test Force Transfer Token", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let transferAmount = new BN(50);

    let forceTransferParams = {
      token: TEST_TOKEN,
      toAccount: user1.publicKey,
      fromAccount: user2.publicKey,
      amount: transferAmount,
    };

    let [pdaWhitelistUser1] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaWhitelistUser2] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user2.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user2.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceBeforeTransfer = Number(user1Account.amount);

    let user2ATA = await getAssociatedTokenAddress(
      mintAccount,
      user2.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user2Account = await getAccount(
      provider.connection,
      user2ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceBeforeTransfer = Number(user2Account.amount);

    // Add user2 in whitelist
    let fromWhitelistParams = {
      token: TEST_TOKEN,
      user: user2.publicKey,
      code: 91,
    };
    await whitelistAccount(
      fromWhitelistParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelistUser2,
    );

    // Add user1 in whitelist
    let toWhitelistParams = {
      token: TEST_TOKEN,
      user: user1.publicKey,
      code: 91,
    };
    await whitelistAccount(
      toWhitelistParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelistUser1,
    );

    await forceTransfer(
      forceTransferParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelistUser2,
      pdaWhitelistUser1,
      pdaPartialFreeze,
      mintAccount,
      user2ATA,
      user1ATA,
      admin,
    );

    // Check balances after Force Transfer
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceAfterTransfer = Number(user1Account.amount);

    user2Account = await getAccount(
      provider.connection,
      user2ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceAfterTransfer = Number(user2Account.amount);

    // Check balances after transfer
    assert.equal(
      user1BalanceAfterTransfer,
      user1BalanceBeforeTransfer + Number(transferAmount),
    );
    assert.equal(
      user2BalanceAfterTransfer,
      user2BalanceBeforeTransfer - Number(transferAmount),
    );
  });

  it("Test Force Transfer Token with Frozen Balance", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST_1],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST_1],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let transferAmount = new BN(50);

    let forceTransferParams = {
      token: TEST_1_TOKEN,
      toAccount: user1.publicKey,
      fromAccount: user2.publicKey,
      amount: transferAmount,
    };

    let [pdaWhitelistUser1] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaWhitelistUser2] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST_1, user2.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST_1, user2.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let frozenAmount = new BN(25);
    await partialFreeze(
      pdaMaintainers,
      pdaConfig,
      pdaPartialFreeze,
      user2.publicKey,
      TEST_1_TOKEN,
      frozenAmount,
    );

    let user2ATA = await getAssociatedTokenAddress(
      mintAccount,
      user2.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Add user2 in whitelist
    let fromWhitelistParams = {
      token: TEST_1_TOKEN,
      user: user2.publicKey,
      code: 91,
    };
    await whitelistAccount(
      fromWhitelistParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelistUser2,
    );

    // Add user1 in whitelist
    let toWhitelistParams = {
      token: TEST_1_TOKEN,
      user: user1.publicKey,
      code: 91,
    };
    await whitelistAccount(
      toWhitelistParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelistUser1,
    );

    try {
      await forceTransfer(
        forceTransferParams,
        pdaMaintainers,
        pdaConfig,
        pdaWhitelistUser2,
        pdaWhitelistUser1,
        pdaPartialFreeze,
        mintAccount,
        user2ATA,
        user1ATA,
        admin,
      );
    } catch (err) {
      assert.equal(err.error.errorCode.code, "BalanceFrozen");
    }
  });

  it("Test Mint Token by Issuer", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
    };

    // Creating associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let balanceBeforeMint = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;

    await mint(
      tokenParams,
      pdaMaintainers,
      mintAccount,
      pdaConfig,
      pdaWhitelist,
      user1ATA,
      issuer,
    );

    // Check balance after mint
    let balanceAfterMint = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;
    assert.equal(
      Number(balanceAfterMint),
      Number(balanceBeforeMint) + Number(MINT_AMOUNT),
    );
  });

  it("Test Mint Token by Tokenization Agent", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
    };

    // Creating associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let balanceBeforeMint = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;

    await mint(
      tokenParams,
      pdaMaintainers,
      mintAccount,
      pdaConfig,
      pdaWhitelist,
      user1ATA,
      tokenizationAgent,
    );

    // Check balance after mint
    let balanceAfterMint = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;
    assert.equal(
      Number(balanceAfterMint),
      Number(balanceBeforeMint) + Number(MINT_AMOUNT),
    );
  });

  it("Test Mint Token by other user", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
    };

    // Creating associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let balanceBeforeMint = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;

    // Ensure ATA exists so we fail on Authority check, not AccountNotInitialized
    await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer, // payer for rent
      mintAccount,
      user1.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    try {
      await mint(
        tokenParams,
        pdaMaintainers,
        mintAccount,
        pdaConfig,
        pdaWhitelist,
        user1ATA,
        transferAgent,
      );
    } catch (e) {
      assert.equal(e.error.errorCode.code, "Unauthorized");
    }
  });

  it("Test Burn Token From by Issuer", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_FROM_AMOUNT,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let balanceBeforeBurn = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;

    await burnFrom(
      tokenParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      issuer,
    );

    // Check balance after burn from
    let balanceAfterBurn = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;
    assert.equal(
      Number(balanceAfterBurn),
      Number(balanceBeforeBurn) - Number(BURN_FROM_AMOUNT),
    );
  });

  it("Test Burn Token From by Tokenization Agent", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_FROM_AMOUNT,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let balanceBeforeBurn = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;

    await burnFrom(
      tokenParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelist,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      tokenizationAgent,
    );

    // Check balance after burn from
    let balanceAfterBurn = (
      await getAccount(
        provider.connection,
        user1ATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount;
    assert.equal(
      Number(balanceAfterBurn),
      Number(balanceBeforeBurn) - Number(BURN_FROM_AMOUNT),
    );
  });

  it("Test Burn Token From by other user", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_FROM_AMOUNT,
    };

    let [pdaWhitelist] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    try {
      await burnFrom(
        tokenParams,
        pdaMaintainers,
        pdaConfig,
        pdaWhitelist,
        pdaPartialFreeze,
        mintAccount,
        user1ATA,
        transferAgent,
      );
    } catch (e) {
      assert.equal(e.error.errorCode.code, "Unauthorized");
    }
  });

  it("Test Force Transfer Token by Issuer", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let transferAmount = new BN(50);

    let forceTransferParams = {
      token: TEST_TOKEN,
      toAccount: user2.publicKey,
      fromAccount: user1.publicKey,
      amount: transferAmount,
    };

    let [pdaWhitelistUser1] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaWhitelistUser2] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user2.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceBeforeTransfer = Number(user1Account.amount);

    let user2ATA = await getAssociatedTokenAddress(
      mintAccount,
      user2.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user2Account = await getAccount(
      provider.connection,
      user2ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceBeforeTransfer = Number(user2Account.amount);

    await forceTransfer(
      forceTransferParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelistUser1,
      pdaWhitelistUser2,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      user2ATA,
      issuer,
    );
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceAfterTransfer = Number(user1Account.amount);

    user2Account = await getAccount(
      provider.connection,
      user2ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceAfterTransfer = Number(user2Account.amount);

    // Check balances after transfer
    assert.equal(
      user1BalanceAfterTransfer,
      user1BalanceBeforeTransfer - Number(transferAmount),
    );
    assert.equal(
      user2BalanceAfterTransfer,
      user2BalanceBeforeTransfer + Number(transferAmount),
    );
  });

  it("Test Force Transfer Token by Transfer Agent", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let transferAmount = new BN(50);

    let forceTransferParams = {
      token: TEST_TOKEN,
      toAccount: user2.publicKey,
      fromAccount: user1.publicKey,
      amount: transferAmount,
    };

    let [pdaWhitelistUser1] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaWhitelistUser2] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user2.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceBeforeTransfer = Number(user1Account.amount);

    let user2ATA = await getAssociatedTokenAddress(
      mintAccount,
      user2.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user2Account = await getAccount(
      provider.connection,
      user2ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceBeforeTransfer = Number(user2Account.amount);

    await forceTransfer(
      forceTransferParams,
      pdaMaintainers,
      pdaConfig,
      pdaWhitelistUser1,
      pdaWhitelistUser2,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      user2ATA,
      transferAgent,
    );
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user1BalanceAfterTransfer = Number(user1Account.amount);

    user2Account = await getAccount(
      provider.connection,
      user2ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    let user2BalanceAfterTransfer = Number(user2Account.amount);

    // Check balances after transfer
    assert.equal(
      user1BalanceAfterTransfer,
      user1BalanceBeforeTransfer - Number(transferAmount),
    );
    assert.equal(
      user2BalanceAfterTransfer,
      user2BalanceBeforeTransfer + Number(transferAmount),
    );
  });

  it("Test Force Transfer Token by other user", async () => {
    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let transferAmount = new BN(50);

    let forceTransferParams = {
      token: TEST_TOKEN,
      toAccount: user2.publicKey,
      fromAccount: user1.publicKey,
      amount: transferAmount,
    };

    let [pdaWhitelistUser1] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let [pdaWhitelistUser2] = anchor.web3.PublicKey.findProgramAddressSync(
      [WHITELIST, TEST, user2.publicKey.toBytes()],
      program.programId,
    );

    let [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user2ATA = await getAssociatedTokenAddress(
      mintAccount,
      user2.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    try {
      await forceTransfer(
        forceTransferParams,
        pdaMaintainers,
        pdaConfig,
        pdaWhitelistUser1,
        pdaWhitelistUser2,
        pdaPartialFreeze,
        mintAccount,
        user1ATA,
        user2ATA,
        tokenizationAgent,
      );
    } catch (e) {
      assert.equal(e.error.errorCode.code, "Unauthorized");
    }
  });

  it("Test Freeze Account", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check account is not frozen
    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isFalse(user1Account.isFrozen);

    await freeze(
      pdaMaintainers,
      pdaConfig,
      mintAccount,
      TEST_TOKEN,
      user1ATA,
      admin,
    );

    // Check account is frozen
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isTrue(user1Account.isFrozen);
  });

  it("Test Unfreeze Account", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check account is frozen
    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isTrue(user1Account.isFrozen);

    await unfreeze(
      pdaMaintainers,
      pdaConfig,
      mintAccount,
      TEST_TOKEN,
      user1ATA,
      admin,
    );

    // Check account is not frozen
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isFalse(user1Account.isFrozen);
  });

  it("Test Freeze and Unfreeze Account by Issuer", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check account is not frozen
    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isFalse(user1Account.isFrozen);

    // Freeze the account
    await freeze(
      pdaMaintainers,
      pdaConfig,
      mintAccount,
      TEST_TOKEN,
      user1ATA,
      issuer,
    );

    // Check account is frozen
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isTrue(user1Account.isFrozen);

    // Now unfreeze the account
    await unfreeze(
      pdaMaintainers,
      pdaConfig,
      mintAccount,
      TEST_TOKEN,
      user1ATA,
      issuer,
    );

    // Check account is not frozen
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isFalse(user1Account.isFrozen);
  });

  it("Test Freeze and Unfreeze Account by Transfer Agent", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Check account is not frozen
    let user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isFalse(user1Account.isFrozen);

    // Freeze the account
    await freeze(
      pdaMaintainers,
      pdaConfig,
      mintAccount,
      TEST_TOKEN,
      user1ATA,
      transferAgent,
    );

    // Check account is frozen
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isTrue(user1Account.isFrozen);

    // Now unfreeze the account
    await unfreeze(
      pdaMaintainers,
      pdaConfig,
      mintAccount,
      TEST_TOKEN,
      user1ATA,
      transferAgent,
    );

    // Check account is not frozen
    user1Account = await getAccount(
      provider.connection,
      user1ATA,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    assert.isFalse(user1Account.isFrozen);
  });

  it("Test Freeze Account by other user", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    try {
      await freeze(
        pdaMaintainers,
        pdaConfig,
        mintAccount,
        TEST_TOKEN,
        user1ATA,
        transferAgent,
      );
    } catch (e) {
      assert.equal(e.error.errorCode.code, "Unauthorized");
    }
  });

  it("Test Unfreeze Account by other user", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    try {
      await unfreeze(
        pdaMaintainers,
        pdaConfig,
        mintAccount,
        TEST_TOKEN,
        user1ATA,
        transferAgent,
      );
    } catch (e) {
      assert.equal(e.error.errorCode.code, "Unauthorized");
    }
  });

  it("Test Update Admin", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let oldAdmin = (await program.account.maintainers.fetch(pdaMaintainers))
      .admin;
    assert.equal(oldAdmin.toString(), admin.publicKey.toString());

    let updateAdmin = await program.methods
      .manageAdmin(user1.publicKey)
      .accounts({
        maintainers: pdaMaintainers,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateAdmin);

    let newAdmin = (await program.account.maintainers.fetch(pdaMaintainers))
      .admin;
    assert.equal(newAdmin.toString(), user1.publicKey.toString());

    updateAdmin = await program.methods
      .manageAdmin(admin.publicKey)
      .accounts({
        maintainers: pdaMaintainers,
        authority: user1.publicKey,
      })
      .signers([user1])
      .rpc();

    await confirmTransaction(updateAdmin);
    newAdmin = (await program.account.maintainers.fetch(pdaMaintainers)).admin;
    assert.equal(oldAdmin.toString(), admin.publicKey.toString());
  });

  it("Test Add Sub Admins", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let addSubAdmins = await program.methods
      .addSubAdminAccounts([user1.publicKey])
      .accounts({
        maintainers: pdaMaintainers,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(addSubAdmins);

    let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
    assert.isTrue(
      JSON.stringify(maintainers.subAdmins).includes(
        JSON.stringify(user1.publicKey),
      ),
    );
  });

  it("Test Remove Sub Admins", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let removeSubAdmins = await program.methods
      .removeSubAdminAccounts([user1.publicKey])
      .accounts({
        maintainers: pdaMaintainers,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(removeSubAdmins);

    let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
    assert.isFalse(
      JSON.stringify(maintainers.subAdmins).includes(
        JSON.stringify(user1.publicKey),
      ),
    );
  });

  it("Test Update Token Limit", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let limit = new BN(5000);

    let updateLimit = await program.methods
      .updateTokenLimitByToken(TEST_TOKEN, limit)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateLimit);

    let newLimit = (await program.account.tokenConfiguration.fetch(pdaConfig))
      .tokenLimit;
    assert.equal(newLimit.toNumber(), limit.toNumber());
  });

  it("Test Add Country Codes", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let codes = [2, 3];

    let addCountryCodes = await program.methods
      .addCountryCodesByToken(TEST_TOKEN, codes)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(addCountryCodes);

    let newCodes = (await program.account.tokenConfiguration.fetch(pdaConfig))
      .countryCodes;
    assert.isTrue(JSON.stringify(newCodes).includes("2"));
    assert.isTrue(JSON.stringify(newCodes).includes("3"));
  });

  it("Test Remove Country Codes", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let codes = [2, 3];

    let removeCountryCodes = await program.methods
      .removeCountryCodesByToken(TEST_TOKEN, codes)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(removeCountryCodes);

    let newCodes = (await program.account.tokenConfiguration.fetch(pdaConfig))
      .countryCodes;
    assert.isFalse(JSON.stringify(newCodes).includes("2"));
    assert.isFalse(JSON.stringify(newCodes).includes("3"));
  });

  it("Test Update Issuer", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let updateIssuer = await program.methods
      .updateIssuerByToken(TEST_TOKEN, fundManager.publicKey)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateIssuer);

    let issuer = (await program.account.tokenConfiguration.fetch(pdaConfig))
      .issuer;
    assert.equal(issuer.toString(), fundManager.publicKey.toString());
  });

  it("Test Update Tokenization Agent", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let updateTokenizationAgent = await program.methods
      .updateTokenizationAgentByToken(TEST_TOKEN, user1.publicKey)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateTokenizationAgent);

    let tokenizationAgent = (
      await program.account.tokenConfiguration.fetch(pdaConfig)
    ).tokenizationAgent;
    assert.equal(tokenizationAgent.toString(), user1.publicKey.toString());
  });

  it("Test Update Transfer Agent", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [CONFIG, TEST],
      program.programId,
    );

    let updateTransferAgent = await program.methods
      .updateTransferAgentByToken(TEST_TOKEN, user1.publicKey)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateTransferAgent);

    let transferAgent = (
      await program.account.tokenConfiguration.fetch(pdaConfig)
    ).transferAgent;
    assert.equal(transferAgent.toString(), user1.publicKey.toString());
  });
});

describe("fund_contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FundContract as Program<FundContract>;
  // const tokenProgram = anchor.workspace.TokenProgram as Program<TokenProgram>;

  const mintAuthority = anchor.web3.Keypair.generate();

  // Declare PDAs
  var pdaMaintainers,
    pdaGlobalConfig,
    pdaAgent = null;

  // Declare nft mints
  var usdcAccount = null;

  const confirmTransaction = async (tx) => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });
  };

  const createFund = async (createParams, pdaGlobalConfig, pdaAgent) => {
    let create = await program.methods
      .createFund(createParams)
      .accounts({
        globalConfig: pdaGlobalConfig,
        agent: pdaAgent,
        authority: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(create);
  };

  const shareDividendToken = async (
    shareDividendParams,
    pdaGlobalConfig,
    pdaAgent,
    toAccount,
    mintAccount,
    tokenAccount,
    signer,
  ) => {
    let signerATA = await getAssociatedTokenAddress(
      mintAccount,
      signer.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let shareDividend = await program.methods
      .shareDividends(shareDividendParams)
      .accounts({
        globalConfig: pdaGlobalConfig,
        agent: pdaAgent,
        authority: signer.publicKey,
        fromAccount: signerATA,
        toAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        customTokenProgram: TOKEN_PROGRAM,
        maintainers: pdaSuperMaintainers,
        config: pdaSuperConfig,
        whitelist: pdaSuperWhitelist,
        mintAccount,
        tokenAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

    await confirmTransaction(shareDividend);
  };

  const distributeAndBurn = async (
    distributionParams,
    pdaGlobalConfig,
    pdaAgent,
    mintAccountStable,
    mintAccountToken,
    investorStable,
    investorToken,
    fromAccountStable,
    signer,
  ) => {
    let distributeAndBurn = await program.methods
      .waterfallDistribution(distributionParams)
      .accounts({
        globalConfig: pdaGlobalConfig,
        agent: pdaAgent,
        partialFreeze: pdaSuperPartialFreeze,
        authority: signer.publicKey,
        fromAccountStable,
        customTokenProgram: TOKEN_PROGRAM,
        maintainers: pdaSuperMaintainers,
        config: pdaSuperConfig,
        whitelist: pdaSuperWhitelist,
        mintAccountStable,
        mintAccountToken,
        investorStable,
        investorToken,
        tokenAccount: investorToken,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    await confirmTransaction(distributeAndBurn);
  };

  it("Initialize test accounts", async () => {
    // Airdrop sol to the test users
    let adminSol = await provider.connection.requestAirdrop(
      admin.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(adminSol);

    let payerSol = await provider.connection.requestAirdrop(
      payer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(payerSol);

    let user1Sol = await provider.connection.requestAirdrop(
      user1.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(user1Sol);

    let user2Sol = await provider.connection.requestAirdrop(
      user2.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(user2Sol);

    let mintAuthoritySol = await provider.connection.requestAirdrop(
      mintAuthority.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(mintAuthoritySol);

    // Create mint token with decimals
    usdcAccount = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      DECIMALS,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let adminATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      usdcAccount,
      admin.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Mint tokens to user
    await mintToChecked(
      provider.connection,
      payer,
      usdcAccount,
      adminATA.address,
      mintAuthority,
      100000,
      DECIMALS,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let fundManagerATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      usdcAccount,
      fundManager.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    // Mint stable coins to fund manager accounts
    await mintToChecked(
      provider.connection,
      payer,
      usdcAccount,
      fundManagerATA.address,
      mintAuthority,
      100000,
      DECIMALS,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
  });

  it("Initialize global account", async () => {
    [pdaMaintainers] = await anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    // Test initialize instruction
    let init = await program.methods
      .init()
      .accounts({
        maintainers: pdaMaintainers,
        authority: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(init);

    let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
    assert.isTrue(
      JSON.stringify(maintainers.admins).includes(
        JSON.stringify(admin.publicKey),
      ),
    );
  });

  it("Test Add Admins", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let addAdmins = await program.methods
      .addAdminAccounts([user1.publicKey, user2.publicKey])
      .accounts({
        maintainers: pdaMaintainers,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(addAdmins);

    let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
    assert.isTrue(
      JSON.stringify(maintainers.admins).includes(
        JSON.stringify(user1.publicKey),
      ),
    );
    assert.isTrue(
      JSON.stringify(maintainers.admins).includes(
        JSON.stringify(user2.publicKey),
      ),
    );
  });

  it("Test Remove Admins", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let removeAdmins = await program.methods
      .removeAdminAccounts([user1.publicKey, user2.publicKey])
      .accounts({
        maintainers: pdaMaintainers,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(removeAdmins);

    let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
    assert.isFalse(
      JSON.stringify(maintainers.admins).includes(
        JSON.stringify(user1.publicKey),
      ),
    );
    assert.isFalse(
      JSON.stringify(maintainers.admins).includes(
        JSON.stringify(user2.publicKey),
      ),
    );
  });

  it("Test Create Fund Contract", async () => {
    // Create Test Fund
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let amount = new BN(100);
    let createParams = {
      token: TEST_TOKEN,
      fund: TEST_TOKEN,
      fundManager: fundManager.publicKey,
      assetType: { token: {} },
      issuer: "Issuer",
      targetAum: amount,
      navLaunchPrice: amount,
      ccy: "INR",
    };

    // Create Fund
    await createFund(createParams, pdaGlobalConfig, pdaAgent);

    // Check stored values
    let globalConfig =
      await program.account.globalConfig.fetch(pdaGlobalConfig);
    assert.equal(globalConfig.fund, TEST_TOKEN);
    assert.equal(globalConfig.issuer, "Issuer");
    assert.equal(Number(globalConfig.targetAum), Number(amount));
    assert.equal(Number(globalConfig.navLaunchPrice), Number(amount));
    assert.equal(globalConfig.ccy, "INR");

    let agent = await program.account.agent.fetch(pdaAgent);
    assert.equal(agent.address.toString(), admin.publicKey.toString());

    // Create Test-1 Fund
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST_1],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST_1],
      program.programId,
    );

    createParams = {
      token: TEST_1_TOKEN,
      fund: TEST_1_TOKEN,
      fundManager: issuer.publicKey,
      assetType: { token: {} },
      issuer: "Issuer",
      targetAum: amount,
      navLaunchPrice: amount,
      ccy: "INR",
    };

    // Create Fund
    await createFund(createParams, pdaGlobalConfig, pdaAgent);

    // Check stored values
    globalConfig = await program.account.globalConfig.fetch(pdaGlobalConfig);
    assert.equal(globalConfig.fund, TEST_1_TOKEN);
    assert.equal(globalConfig.issuer, "Issuer");
    assert.equal(Number(globalConfig.targetAum), Number(amount));
    assert.equal(Number(globalConfig.navLaunchPrice), Number(amount));
    assert.equal(globalConfig.ccy, "INR");

    agent = await program.account.agent.fetch(pdaAgent);
    assert.equal(agent.address.toString(), admin.publicKey.toString());
  });

  it("Test Share Dividend with Stable Coins", async () => {
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let dividend = new BN(100);
    let shareDividendParams = {
      token: TEST_TOKEN,
      coinType: { usdc: {} },
      toAccount: user1.publicKey,
      dividend,
      assetType: { stableCoin: {} },
      decimals: DECIMALS,
    };

    let user1ATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      usdcAccount,
      user1.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1BalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA.address,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let adminATA = await getAssociatedTokenAddress(
      usdcAccount,
      admin.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let adminBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          adminATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    // Share Dividends
    await shareDividendToken(
      shareDividendParams,
      pdaGlobalConfig,
      pdaAgent,
      user1ATA.address,
      usdcAccount,
      user1.publicKey,
      admin,
    );

    // Check Balances
    let adminBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          adminATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    let user1BalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA.address,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(adminBalanceAfter, adminBalanceBefore - dividend.toNumber());
    assert.equal(user1BalanceAfter, user1BalanceBefore + dividend.toNumber());
  });

  it("Test Share Dividend with Stable Coins by Fund Manager", async () => {
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let dividend = new BN(100);
    let shareDividendParams = {
      token: TEST_TOKEN,
      coinType: { usdc: {} },
      toAccount: user1.publicKey,
      dividend,
      assetType: { stableCoin: {} },
      decimals: DECIMALS,
    };

    let user1ATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      usdcAccount,
      user1.publicKey,
      undefined,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1BalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA.address,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let fundManagerATA = await getAssociatedTokenAddress(
      usdcAccount,
      fundManager.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let fundManagerBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          fundManagerATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    // Share Dividends
    await shareDividendToken(
      shareDividendParams,
      pdaGlobalConfig,
      pdaAgent,
      user1ATA.address,
      usdcAccount,
      user1.publicKey,
      fundManager,
    );

    // Check Balances
    let fundManagerBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          fundManagerATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    let user1BalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA.address,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      fundManagerBalanceAfter,
      fundManagerBalanceBefore - dividend.toNumber(),
    );
    assert.equal(user1BalanceAfter, user1BalanceBefore + dividend.toNumber());
  });

  it("Test Share Dividend with Tokens", async () => {
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let dividend = new BN(100);
    let shareDividendParams = {
      token: TEST_TOKEN,
      coinType: { usdc: {} },
      toAccount: user1.publicKey,
      dividend,
      assetType: { token: {} },
      decimals: DECIMALS,
    };

    let user1ATA = await getAssociatedTokenAddress(
      pdaSuperMintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1BalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    // Share Dividends
    await shareDividendToken(
      shareDividendParams,
      pdaGlobalConfig,
      pdaAgent,
      user1ATA,
      pdaSuperMintAccount,
      user1ATA,
      admin,
    );

    // Check Balances
    let user1BalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(user1BalanceAfter, user1BalanceBefore + dividend.toNumber());
  });

  it("Test Share Dividend with Tokens by Fund Manager", async () => {
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let dividend = new BN(100);
    let shareDividendParams = {
      token: TEST_TOKEN,
      coinType: { usdc: {} },
      toAccount: user1.publicKey,
      dividend,
      assetType: { token: {} },
      decimals: DECIMALS,
    };

    let user1ATA = await getAssociatedTokenAddress(
      pdaSuperMintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let user1BalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    // Share Dividends
    await shareDividendToken(
      shareDividendParams,
      pdaGlobalConfig,
      pdaAgent,
      user1ATA,
      pdaSuperMintAccount,
      user1ATA,
      fundManager,
    );

    // Check Balances
    let user1BalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(user1BalanceAfter, user1BalanceBefore + dividend.toNumber());
  });

  it("Test Distribute and Burn", async () => {
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let distributionAmount = new BN(100);
    let burnAmount = new BN(50);
    let distributionParams = {
      token: TEST_TOKEN,
      coinType: { usdc: {} },
      investor: user1.publicKey,
      distributionAmount,
      burnAmount,
      decimals: DECIMALS,
    };

    let investorStable = await getAssociatedTokenAddress(
      usdcAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let investorToken = await getAssociatedTokenAddress(
      pdaSuperMintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let investorTokenBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          investorToken,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let fromAccountStable = await getAssociatedTokenAddress(
      usdcAccount,
      admin.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let adminBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          fromAccountStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let user1BalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          investorStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    // Waterfall Distribution
    await distributeAndBurn(
      distributionParams,
      pdaGlobalConfig,
      pdaAgent,
      usdcAccount,
      pdaSuperMintAccount,
      investorStable,
      investorToken,
      fromAccountStable,
      admin,
    );

    // Check Balances
    let adminBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          fromAccountStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let user1BalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          investorStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let investorTokenBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          investorToken,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      adminBalanceAfter,
      adminBalanceBefore - distributionAmount.toNumber(),
    );
    assert.equal(
      user1BalanceAfter,
      user1BalanceBefore + distributionAmount.toNumber(),
    );
    assert.equal(
      investorTokenBalanceAfter,
      investorTokenBalanceBefore - burnAmount.toNumber(),
    );
  });

  it("Test Distribute and Burn by Fund Manager", async () => {
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let distributionAmount = new BN(100);
    let burnAmount = new BN(50);
    let distributionParams = {
      token: TEST_TOKEN,
      coinType: { usdc: {} },
      investor: user1.publicKey,
      distributionAmount,
      burnAmount,
      decimals: DECIMALS,
    };

    let investorStable = await getAssociatedTokenAddress(
      usdcAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let investorToken = await getAssociatedTokenAddress(
      pdaSuperMintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let investorTokenBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          investorToken,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let fromAccountStable = await getAssociatedTokenAddress(
      usdcAccount,
      fundManager.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let fundManagerBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          fromAccountStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let user1BalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          investorStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    // Waterfall Distribution
    await distributeAndBurn(
      distributionParams,
      pdaGlobalConfig,
      pdaAgent,
      usdcAccount,
      pdaSuperMintAccount,
      investorStable,
      investorToken,
      fromAccountStable,
      fundManager,
    );

    // Check Balances
    let fundManagerBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          fromAccountStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let user1BalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          investorStable,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let investorTokenBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          investorToken,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      fundManagerBalanceAfter,
      fundManagerBalanceBefore - distributionAmount.toNumber(),
    );
    assert.equal(
      user1BalanceAfter,
      user1BalanceBefore + distributionAmount.toNumber(),
    );
    assert.equal(
      investorTokenBalanceAfter,
      investorTokenBalanceBefore - burnAmount.toNumber(),
    );
  });

  it("Test Update Agent", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let agent = await program.account.agent.fetch(pdaAgent);
    assert.equal(agent.address.toString(), admin.publicKey.toString());

    let updateAgent = await program.methods
      .updateAgentByToken(TEST_TOKEN, user1.publicKey)
      .accounts({
        maintainers: pdaMaintainers,
        agent: pdaAgent,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateAgent);

    agent = await program.account.agent.fetch(pdaAgent);
    assert.equal(agent.address.toString(), user1.publicKey.toString());
  });

  it("Test Update Stable Coin Address", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let [pdaStableCoin] = anchor.web3.PublicKey.findProgramAddressSync(
      [STABLE_COIN],
      program.programId,
    );

    let stableCoinParams = {
      coinType: { usdc: {} },
      updateType: { add: { address: usdcAccount } },
    };

    let updateStableCoin = await program.methods
      .updateStableCoins(stableCoinParams)
      .accounts({
        maintainers: pdaMaintainers,
        stableCoins: pdaStableCoin,
        authority: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateStableCoin);

    // Check stable coin address
    let stableCoin =
      await program.account.stableCoinStorage.fetch(pdaStableCoin);
    assert.equal(stableCoin.usdc.toString(), usdcAccount.toString());
  });
});
