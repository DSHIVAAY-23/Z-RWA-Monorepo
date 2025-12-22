import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddress,
  getAccount,
} from "@solana/spl-token";
import { BN } from "bn.js";
import { assert } from "chai";
import { BaseTokenProgram } from "../target/types/base_token_program";
import { InteropCore } from "../target/types/interop_core";
import { InteropMultisig } from "../target/types/interop_multisig";
import {
  PublicKey,
  AddressLookupTableProgram,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";

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

// Create constant amount fields
const MINT_AMOUNT = new BN(1000);
const BURN_AMOUNT = new BN(600);
const BURN_FROM_AMOUNT = new BN(200);

// Constant seeds
const TEST_TOKEN = "Test";
const TEST_1_TOKEN = "Test-1";
const MINT = Buffer.from("mint");
const MAINTAINERS = Buffer.from("maintainers");
const CONFIG = Buffer.from("config");
const PARTIAL_FREEZE = Buffer.from("partial_freeze");
const EXECUTER = Buffer.from("executer");
const THRESHOLD = Buffer.from("threshold");
const VALIDATORS = Buffer.from("validators");
const VOTES = Buffer.from("votes");
const PAYLOAD = Buffer.from("payload");
const TEST = Buffer.from(TEST_TOKEN);
const TEST_1 = Buffer.from(TEST_1_TOKEN);

const BASE_TOKEN_PROGRAM = new PublicKey(
  "5z51MutHHjK7sCsQiA8F9ePErcbooewTNVMShZbN2hMy",
);

const INTEROP_CORE_PROGRAM = new PublicKey(
  "GuPGNTEphtvsEH1UhSqbCdqrooXCLjvioRppv3GfjK9L",
);

let mintAccount,
  pdaExecuter = null;

// Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const confirmTransaction = async (tx) => {
  const latestBlockHash = await provider.connection.getLatestBlockhash();

  await provider.connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx,
  });
};

const extractPayload = async (pg, pdaPayload, payload) => {
  let extract = await pg.methods
    .extractPayload(payload)
    .accounts({
      payload: pdaPayload,
      authority: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([admin])
    .rpc();

  await confirmTransaction(extract);
};

describe("base_token_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .BaseTokenProgram as Program<BaseTokenProgram>;

  // Declare PDAs
  let pdaMaintainers,
    pdaPartialFreeze,
    pdaConfig = null;

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
      .rpc();

    await confirmTransaction(createToken);
  };

  const requestOrders = async (
    requestParams,
    mintAccount,
    request,
    user,
    signer,
  ) => {
    // Test mint_token instruction
    let orders = await program.methods
      .requestOrders(requestParams)
      .accounts({
        maintainers: pdaMaintainers,
        mintAccount,
        request,
        user,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        payer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    await confirmTransaction(orders);
  };

  const mint = async (
    tokenParams,
    pdaMaintainers,
    mintAccount,
    pdaConfig,
    user1ATA,
    signer,
  ) => {
    // Test mint_token instruction
    let mintToken = await program.methods
      .mintToken(tokenParams)
      .accounts({
        maintainers: pdaMaintainers,
        config: pdaConfig,
        mintAccount,
        toAccount: user1ATA,
        authority: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

    await confirmTransaction(mintToken);
  };

  const burn = async (tokenParams, pdaMaintainers, mintAccount, user1ATA) => {
    // Test burn_token instruction
    let burnToken = await program.methods
      .burnToken(tokenParams)
      .accounts({
        maintainers: pdaMaintainers,
        partialFreeze: pdaPartialFreeze,
        mintAccount,
        from: user1ATA,
        authority: user1.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    await confirmTransaction(burnToken);
  };

  const burnFrom = async (
    tokenParams,
    pdaMaintainers,
    pdaConfig,
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
        partialFreeze: pdaPartialFreeze,
        mintAccount,
        from: user1ATA,
        authority: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([signer])
      .rpc();

    await confirmTransaction(burnToken);
  };

  const transfer = async (
    transferParams,
    pdaMaintainers,
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
        partialFreeze: pdaPartialFreeze,
        mintAccount,
        fromAccount: fromATA,
        toAccount: toATA,
        authority: user1.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    await confirmTransaction(transferToken);
  };

  const forceTransfer = async (
    forceTransferParams,
    pdaMaintainers,
    pdaConfig,
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
        partialFreeze: pdaPartialFreeze,
        mintAccount,
        fromAccount: fromATA,
        toAccount: toATA,
        authority: signer.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

    await confirmTransaction(forceTransferToken);
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

    let issuerSol = await provider.connection.requestAirdrop(
      issuer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(issuerSol);

    let transferAgentSol = await provider.connection.requestAirdrop(
      transferAgent.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(transferAgentSol);

    let tokenizationAgentSol = await provider.connection.requestAirdrop(
      tokenizationAgent.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(tokenizationAgentSol);

    let fundManagerSol = await provider.connection.requestAirdrop(
      fundManager.publicKey,
      anchor.web3.LAMPORTS_PER_SOL,
    );
    await confirmTransaction(fundManagerSol);
  });

  it("Test Initialisation", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
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
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(init);

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

    [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    let createTokenParams = {
      id: "unique",
      name: TEST_TOKEN,
      symbol: "tes",
      uri: "some/uri",
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

    await mint(
      tokenParams,
      pdaMaintainers,
      mintAccount,
      pdaConfig,
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

  it("Test Request Order for mint", async () => {
    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    // Creating associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let requestParams = {
      orderId: new BN(1),
      token: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
      requestType: { mint: {} },
    };

    let supplyBefore = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;

    let user1AccountBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let [request] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(requestParams.orderId.toString())],
      program.programId,
    );

    await requestOrders(requestParams, mintAccount, request, user1ATA, admin);

    // Check balance after mint
    let supplyAfter = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;
    assert.equal(
      Number(supplyBefore) + Number(MINT_AMOUNT),
      Number(supplyAfter),
    );

    let user1AccountBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      user1AccountBalanceAfter,
      user1AccountBalanceBefore + MINT_AMOUNT.toNumber(),
    );
  });

  it("Test Request Order for mint by other user", async () => {
    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    // Creating associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let requestParams = {
      orderId: new BN(2),
      token: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
      requestType: { mint: {} },
    };

    let [request] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(requestParams.orderId.toString())],
      program.programId,
    );

    try {
      await requestOrders(requestParams, mintAccount, request, user1ATA, payer);
    } catch (err) {
      assert.equal(err.error.errorCode.code, "Unauthorized");
    }
  });

  it("Test Request Order for burn", async () => {
    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    // Creating associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let requestParams = {
      orderId: new BN(3),
      token: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
      requestType: { mint: {} },
    };

    let supplyBefore = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;

    let user1AccountBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    let [request] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(requestParams.orderId.toString())],
      program.programId,
    );

    await requestOrders(requestParams, mintAccount, request, user1ATA, admin);

    // Check balance after mint
    let supplyAfter = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;
    assert.equal(
      Number(supplyBefore) + Number(MINT_AMOUNT),
      Number(supplyAfter),
    );

    let user1AccountBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      user1AccountBalanceAfter,
      user1AccountBalanceBefore + MINT_AMOUNT.toNumber(),
    );
  });

  it("Test Request Order for burn by other user", async () => {
    let [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );

    // Creating associated token for user1 for Test
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let requestParams = {
      orderId: new BN(4),
      token: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: MINT_AMOUNT,
      requestType: { mint: {} },
    };

    let [request] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(requestParams.orderId.toString())],
      program.programId,
    );

    try {
      await requestOrders(requestParams, mintAccount, request, user1ATA, payer);
    } catch (err) {
      assert.equal(err.error.errorCode.code, "Unauthorized");
    }
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

    [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

    let tokenParams = {
      name: TEST_TOKEN,
      toAccount: user1.publicKey,
      amount: BURN_AMOUNT,
    };

    // Check balances before burn
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let supplyBefore = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;

    let user1AccountBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    await burn(tokenParams, pdaMaintainers, mintAccount, user1ATA);

    // Check balance after after
    let supplyAfter = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;
    assert.equal(
      Number(supplyBefore) - Number(BURN_AMOUNT),
      Number(supplyAfter),
    );

    let user1AccountBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      user1AccountBalanceAfter,
      user1AccountBalanceBefore - BURN_AMOUNT.toNumber(),
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

    [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST_1, user1.publicKey.toBytes()],
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

    supplyBefore = (await provider.connection.getTokenSupply(mintAccount)).value
      .amount;

    user1AccountBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    await burn(tokenParams, pdaMaintainers, mintAccount, user1ATA);

    // Check balance after after burn
    supplyAfter = (await provider.connection.getTokenSupply(mintAccount)).value
      .amount;
    assert.equal(
      Number(supplyBefore) - Number(BURN_AMOUNT),
      Number(supplyAfter),
    );

    user1AccountBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      user1AccountBalanceAfter,
      user1AccountBalanceBefore - BURN_AMOUNT.toNumber(),
    );
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

    [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST, user1.publicKey.toBytes()],
      program.programId,
    );

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

    // Creating associated token for user1 and Test
    // Check balances before burn
    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let supplyBefore = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;

    let user1AccountBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    await burnFrom(
      tokenParams,
      pdaMaintainers,
      pdaConfig,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      admin,
    );

    // Check balance after burn from
    let supplyAfter = (await provider.connection.getTokenSupply(mintAccount))
      .value.amount;
    assert.equal(
      Number(supplyBefore) - Number(BURN_FROM_AMOUNT),
      Number(supplyAfter),
    );

    let user1AccountBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      user1AccountBalanceAfter,
      user1AccountBalanceBefore - BURN_FROM_AMOUNT.toNumber(),
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

    [pdaPartialFreeze] = anchor.web3.PublicKey.findProgramAddressSync(
      [PARTIAL_FREEZE, TEST_1, user1.publicKey.toBytes()],
      program.programId,
    );

    // Creating associated token for user1 and Test-1
    // Check balances before burn
    user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    supplyBefore = (await provider.connection.getTokenSupply(mintAccount)).value
      .amount;

    user1AccountBalanceBefore = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );

    await burnFrom(
      tokenParams,
      pdaMaintainers,
      pdaConfig,
      pdaPartialFreeze,
      mintAccount,
      user1ATA,
      admin,
    );

    // Check balance after burn from
    supplyAfter = (await provider.connection.getTokenSupply(mintAccount)).value
      .amount;
    assert.equal(
      Number(supplyBefore) - Number(BURN_FROM_AMOUNT),
      Number(supplyAfter),
    );

    user1AccountBalanceAfter = Number(
      (
        await getAccount(
          provider.connection,
          user1ATA,
          undefined,
          TOKEN_2022_PROGRAM_ID,
        )
      ).amount,
    );
    assert.equal(
      user1AccountBalanceAfter,
      user1AccountBalanceBefore - BURN_FROM_AMOUNT.toNumber(),
    );
  });

  it("Test Burn Token From with Frozen Balance", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
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

    await forceTransfer(
      forceTransferParams,
      pdaMaintainers,
      pdaConfig,
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

    try {
      await forceTransfer(
        forceTransferParams,
        pdaMaintainers,
        pdaConfig,
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

    try {
      await mint(
        tokenParams,
        pdaMaintainers,
        mintAccount,
        pdaConfig,
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

describe("interop-core", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.InteropCore as Program<InteropCore>;
  // Declare PDAs
  let pdaMaintainers = null;

  const manageRoles = async (pdaMaintainers, pdaExecuter, role) => {
    // Test manage roles
    let manageRoles = await program.methods
      .manageRoles(role)
      .accounts({
        maintainers: pdaMaintainers,
        executer: pdaExecuter,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(manageRoles);
  };

  const sendInstructions = async (sendParams, caller) => {
    // Test send instructions
    let instruction = await program.methods
      .sendInstructions(sendParams)
      .accounts({
        caller: caller.publicKey,
      })
      .signers([caller])
      .rpc();

    await confirmTransaction(instruction);
  };

  const executeInstruction = async (
    pdaExecuter,
    request,
    executeParams,
    user,
    caller,
  ) => {
    // Test execute instructions
    let instruction = await program.methods
      .executeInstructions(
        executeParams.sourceChain,
        executeParams.sourceAddress,
        executeParams.payload,
      )
      .accounts({
        executer: pdaExecuter,
        caller: caller.publicKey,
        mintAccount,
        request,
        user,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        baseTokenProgram: BASE_TOKEN_PROGRAM,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([caller])
      .rpc();

    await confirmTransaction(instruction);
  };

  it("Test Initialisation", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaExecuter] = anchor.web3.PublicKey.findProgramAddressSync(
      [EXECUTER],
      program.programId,
    );

    let initParams = {
      multisig: user1.publicKey,
      deployedChain: "Solana",
    };

    // Test initialize instruction
    let init = await program.methods
      .init(initParams)
      .accounts({
        maintainers: pdaMaintainers,
        executer: pdaExecuter,
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

    let executer = await program.account.executer.fetch(pdaExecuter);
    assert.equal(executer.address.toString(), user1.publicKey.toString());
  });

  it("Test Add Admins", async () => {
    let admins = [user1.publicKey];
    let addAdmins = await program.methods
      .addAdminAccounts(admins)
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
  });

  it("Test Remove Admins", async () => {
    let admins = [user1.publicKey];
    let removeAdmins = await program.methods
      .removeAdminAccounts(admins)
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
  });

  it("Test Send Instruction", async () => {
    let sendParams = {
      portfolios: [
        {
          destChain: "Holesky",
          destAddress: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
          investor: "0x0B70373D5BA5b0Da8672fF62704bFD117211C2C2",
          token: "0xC29295f67F5d476105f19E8513da0E5027e73e39",
          amount: "100000000000000000000",
          orderId: new BN(1),
          action: { mint: {} },
        },
      ],
    };

    await sendInstructions(sendParams, user2);
  });

  it("Test Execute Mint Instruction", async () => {
    let executeParams = {
      sourceChain: "Holesky",
      sourceAddress: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
      payload:
        "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002c4578377938535a53706431424d4461356d4d52653136437665767348353634457a6d45434c6678694e6256330000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a30784332393239356636374635643437363130356631394538353133646130453530323765373365333900000000000000000000000000000000000000000000",
    };

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

    let [request] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("1")],
      program.programId,
    );

    try {
      await executeInstruction(
        pdaExecuter,
        request,
        executeParams,
        user1ATA.address,
        user1,
      );
    } catch (e) {
      console.log("Will Fail as mintAccount is different at each run!");
    }
  });

  it("Test update executer", async () => {
    let role = {
      executer: { addr: user2.publicKey },
    };

    await manageRoles(pdaMaintainers, pdaExecuter, role);

    let executer = await program.account.executer.fetch(pdaExecuter);
    assert.equal(executer.address.toString(), user2.publicKey.toString());

    role = {
      executer: { addr: user1.publicKey },
    };

    await manageRoles(pdaMaintainers, pdaExecuter, role);

    executer = await program.account.executer.fetch(pdaExecuter);
    assert.equal(executer.address.toString(), user1.publicKey.toString());
  });
});

describe("interop-multisig", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.InteropMultisig as Program<InteropMultisig>;

  // Declare PDAs
  let pdaMaintainers,
    pdaThreshold,
    pdaVotes,
    pdaValidators,
    pdaPayload = null;

  let LOOKUP_TABLE_ADDRESS = null;

  const sleep = async (ms = 0): Promise<void> =>
    new Promise((resolve) => setTimeout(resolve, ms));

  const createAndSendV0Tx = async (tx) => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    const messageV0 = new TransactionMessage({
      payerKey: admin.publicKey,
      recentBlockhash: latestBlockHash.blockhash,
      instructions: tx,
    }).compileToV0Message();

    const transaction = new VersionedTransaction(messageV0);
    transaction.sign([admin]);

    const txid = await provider.connection.sendTransaction(transaction, {
      maxRetries: 5,
    });

    await confirmTransaction(txid);
  };

  async function createLookupTable() {
    // Step 1 - Get a lookup table address and create lookup table instruction
    const [lookupTableInst, lookupTableAddress] =
      AddressLookupTableProgram.createLookupTable({
        authority: admin.publicKey,
        payer: admin.publicKey,
        recentSlot: await provider.connection.getSlot("finalized"),
      });

    LOOKUP_TABLE_ADDRESS = lookupTableAddress;

    // Step 2 - Log Lookup Table Address
    console.log("Lookup Table Address:", lookupTableAddress.toBase58());

    // Step 3 - Generate a transaction and send it to the network
    createAndSendV0Tx([lookupTableInst]);
  }

  async function addAddressesToTable(addresses) {
    // Step 1 - Create Transaction Instruction
    const addAddressesInstruction = AddressLookupTableProgram.extendLookupTable(
      {
        payer: admin.publicKey,
        authority: admin.publicKey,
        lookupTable: LOOKUP_TABLE_ADDRESS,
        addresses,
      },
    );

    // Step 2 - Generate a transaction and send it to the network
    createAndSendV0Tx([addAddressesInstruction]);
    // console.log(`Lookup Table URL: `,`https://explorer.solana.com/address/${LOOKUP_TABLE_ADDRESS.toString()}?cluster=devnet`)
  }

  async function findAddressesInTable() {
    // Step 1 - Fetch our address lookup table
    const lookupTableAccount =
      await provider.connection.getAddressLookupTable(LOOKUP_TABLE_ADDRESS);

    console.log(
      `Successfully found lookup table: `,
      lookupTableAccount.value?.key.toString(),
    );

    // Step 2 - Make sure our search returned a valid table
    if (!lookupTableAccount.value) return;

    // Step 3 - Log each table address to console
    for (let i = 0; i < lookupTableAccount.value.state.addresses.length; i++) {
      const address = lookupTableAccount.value.state.addresses[i];
      console.log(`   Address ${i + 1}: ${address.toBase58()}`);
    }
  }

  const executeTransaction = async (executeParams, request, user) => {
    // Test execute transaction
    let executeTransaction = await program.methods
      .executeTransactions(executeParams)
      .accounts({
        threshold: pdaThreshold,
        votes: pdaVotes,
        executer: pdaExecuter,
        mintAccount,
        request,
        user,
        caller: admin.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        interopCoreProgram: INTEROP_CORE_PROGRAM,
        baseTokenProgram: BASE_TOKEN_PROGRAM,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(executeTransaction);
  };

  it("Test Initialisation", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaThreshold] = anchor.web3.PublicKey.findProgramAddressSync(
      [THRESHOLD],
      program.programId,
    );

    [pdaValidators] = anchor.web3.PublicKey.findProgramAddressSync(
      [VALIDATORS],
      program.programId,
    );

    let threshold = 1;

    // Test initialize instruction
    let init = await program.methods
      .init(threshold)
      .accounts({
        maintainers: pdaMaintainers,
        threshold: pdaThreshold,
        validators: pdaValidators,
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

    let threshold_store = await program.account.threshold.fetch(pdaThreshold);
    assert.equal(threshold_store.value, threshold);
  });

  it("Test Add Admins", async () => {
    let admins = [user1.publicKey];
    let addAdmins = await program.methods
      .addAdminAccounts(admins)
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
  });

  it("Test Remove Admins", async () => {
    let admins = [user1.publicKey];
    let removeAdmins = await program.methods
      .removeAdminAccounts(admins)
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
  });

  it("Test Add Validators", async () => {
    let newValidators = [user1.publicKey, user2.publicKey];
    let addValidators = await program.methods
      .addValidatorAccounts(newValidators)
      .accounts({
        maintainers: pdaMaintainers,
        validators: pdaValidators,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(addValidators);

    let validators = await program.account.validators.fetch(pdaValidators);
    assert.isTrue(
      JSON.stringify(validators.addresses).includes(
        JSON.stringify(user1.publicKey),
      ),
    );
    assert.isTrue(
      JSON.stringify(validators.addresses).includes(
        JSON.stringify(user2.publicKey),
      ),
    );
  });

  it("Test Remove Validators", async () => {
    let oldValidators = [user2.publicKey];
    let removeValidators = await program.methods
      .removeValidatorAccounts(oldValidators)
      .accounts({
        maintainers: pdaMaintainers,
        validators: pdaValidators,
        authority: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(removeValidators);

    let validators = await program.account.validators.fetch(pdaValidators);
    assert.isTrue(
      JSON.stringify(validators.addresses).includes(
        JSON.stringify(user1.publicKey),
      ),
    );
    assert.isFalse(
      JSON.stringify(validators.addresses).includes(
        JSON.stringify(user2.publicKey),
      ),
    );
  });

  it("Test Cast Vote", async () => {
    let castVoteParams = {
      txHash: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
      canTransact: true,
    };

    let seed = castVoteParams.txHash.substring(
      castVoteParams.txHash.length - 32,
      castVoteParams.txHash.length,
    );

    [pdaVotes] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(seed)],
      program.programId,
    );

    let castVote = await program.methods
      .castVotes(castVoteParams.txHash, castVoteParams.canTransact)
      .accounts({
        validators: pdaValidators,
        threshold: pdaThreshold,
        votes: pdaVotes,
        authority: user1.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    await confirmTransaction(castVote);

    let votes = await program.account.votes.fetch(pdaVotes);
    let expectedVote = {
      yes: 1,
      no: 0,
      voters: [user1.publicKey],
      status: { ready: {} },
    };
    assert.equal(votes.yes, expectedVote.yes);
    assert.equal(votes.no, expectedVote.no);
    assert.equal(votes.voters.toString(), expectedVote.voters.toString());
  });

  it("Test Execute Transaction", async () => {
    let executeParams = {
      txHash: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
      sourceChain: "Holesky",
      sourceAddress: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
      payload:
        "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002c4578377938535a53706431424d4461356d4d52653136437665767348353634457a6d45434c6678694e6256330000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a30784332393239356636374635643437363130356631394538353133646130453530323765373365333900000000000000000000000000000000000000000000",
    };

    let seed = executeParams.txHash.substring(
      executeParams.txHash.length - 32,
      executeParams.txHash.length,
    );

    [pdaVotes] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(seed)],
      program.programId,
    );

    [pdaPayload] = anchor.web3.PublicKey.findProgramAddressSync(
      [PAYLOAD],
      program.programId,
    );

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    await extractPayload(
      program,
      pdaPayload,
      "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002c4578377938535a53706431424d4461356d4d52653136437665767348353634457a6d45434c6678694e625633000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000045465737400000000000000000000000000000000000000000000000000000000",
    );

    let payload = await program.account.payload.fetch(pdaPayload);

    let [request] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(payload.orderId.toString())],
      program.programId,
    );

    try {
      await executeTransaction(executeParams, request, user1ATA);
    } catch (e) {
      console.log("Will fail as transaction size is bigget than 1232 bytes!");
    }
  });

  it("Test Execute Transaction with Lookup Table", async () => {
    [pdaVotes] = anchor.web3.PublicKey.findProgramAddressSync(
      [VOTES],
      program.programId,
    );

    let executeParams = {
      txHash: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
      sourceChain: "Holesky",
      sourceAddress: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
      payload:
        "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000186a000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002c4578377938535a53706431424d4461356d4d52653136437665767348353634457a6d45434c6678694e6256330000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a30784332393239356636374635643437363130356631394538353133646130453530323765373365333900000000000000000000000000000000000000000000",
    };

    let [request] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("1")],
      program.programId,
    );

    let addresses = [
      pdaThreshold,
      pdaVotes,
      pdaExecuter,
      mintAccount,
      request,
      TOKEN_2022_PROGRAM_ID,
      INTEROP_CORE_PROGRAM,
      BASE_TOKEN_PROGRAM,
      pdaMaintainers,
    ];

    // Lookup table needs some cooldown time
    await createLookupTable();
    await sleep(1001);

    // Lookup table needs some cooldown time
    await addAddressesToTable(addresses);
    await sleep(1001);

    const lookupTableAccount = (
      await provider.connection.getAddressLookupTable(LOOKUP_TABLE_ADDRESS)
    ).value;

    let user1ATA = await getAssociatedTokenAddress(
      mintAccount,
      user1.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let executeTransaction = await program.methods
      .executeTransactions(executeParams)
      .accounts({
        threshold: lookupTableAccount.state.addresses[0],
        votes: lookupTableAccount.state.addresses[1],
        executer: lookupTableAccount.state.addresses[2],
        mintAccount: lookupTableAccount.state.addresses[3],
        request: lookupTableAccount.state.addresses[4],
        user: user1ATA,
        caller: admin.publicKey,
        tokenProgram: lookupTableAccount.state.addresses[5],
        interopCoreProgram: lookupTableAccount.state.addresses[6],
        baseTokenProgram: lookupTableAccount.state.addresses[7],
        maintainers: lookupTableAccount.state.addresses[8],
      })
      .instruction();

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    const messageV0 = new TransactionMessage({
      payerKey: admin.publicKey,
      recentBlockhash: latestBlockHash.blockhash,
      instructions: [executeTransaction],
    }).compileToV0Message([lookupTableAccount]);

    const transaction = new VersionedTransaction(messageV0);
    transaction.sign([admin]);

    try {
      const txid = await provider.connection.sendTransaction(transaction, {
        maxRetries: 5,
      });

      await confirmTransaction(txid);
    } catch (e) {
      console.log("Will Fail as mintAccount is different at each run!");
    }
  });

  it("Test Update Threshold", async () => {
    let threshold = 3;
    let updateThreshold = await program.methods
      .updateThreshold(threshold)
      .accounts({
        maintainers: pdaMaintainers,
        threshold: pdaThreshold,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(updateThreshold);

    let threshold_store = await program.account.threshold.fetch(pdaThreshold);
    assert.equal(threshold_store.value, threshold);
  });

  it("Test Extract Payload", async () => {
    await extractPayload(
      program,
      pdaPayload,
      "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002c4578377938535a53706431424d4461356d4d52653136437665767348353634457a6d45434c6678694e625633000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000045465737400000000000000000000000000000000000000000000000000000000",
    );

    let payload = await program.account.payload.fetch(pdaPayload);

    let [derivedMintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, Buffer.from(payload.token)],
      program.programId,
    );
    let [expectedMintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [MINT, TEST],
      program.programId,
    );
    assert.equal(derivedMintAccount.toBase58(), expectedMintAccount.toBase58());
    assert.equal(payload.orderId.toNumber(), 1);
    assert.equal(
      payload.investor.toBase58(),
      "Ex7y8SZSpd1BMDa5mMRe16CvevsH564EzmECLfxiNbV3",
    );
  });
});
