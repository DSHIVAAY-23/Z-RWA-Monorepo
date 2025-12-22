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
import { BN } from "bn.js";
import { assert } from "chai";
import { TreasuryBond } from "../target/types/treasury_bond";

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
const DECIMALS = 6;

// Constant seeds
const TEST_TOKEN = "Test";
const TEST_1_TOKEN = "Test-1";
const MAINTAINERS = Buffer.from("maintainers");
const STABLE_COIN = Buffer.from("stable_coin");
const TEST = Buffer.from(TEST_TOKEN);
const TEST_1 = Buffer.from(TEST_1_TOKEN);
const GLOBAL_CONFIG = Buffer.from("global_config");
const AGENT = Buffer.from("agent");

describe("treasury_bond", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TreasuryBond as Program<TreasuryBond>;

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

  const createBond = async (createParams, pdaGlobalConfig, pdaAgent) => {
    let create = await program.methods
      .createBond(createParams)
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

  const shareStableCoin = async (
    shareStableCoinParams,
    pdaGlobalConfig,
    pdaAgent,
    toAccount,
    mintAccount,
    signer,
  ) => {
    let signerATA = await getAssociatedTokenAddress(
      mintAccount,
      signer.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );

    let shareStableCoin = await program.methods
      .shareStableCoins(shareStableCoinParams)
      .accounts({
        globalConfig: pdaGlobalConfig,
        agent: pdaAgent,
        authority: signer.publicKey,
        fromAccount: signerATA,
        toAccount,
        mintAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();

    await confirmTransaction(shareStableCoin);
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

  it("Test Create Treasury Bond Contract", async () => {
    // Create Test Fund
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let createParams = {
      token: TEST_TOKEN,
      issueSize: new BN(100),
      faceValue: new BN(100),
      couponRate: 7,
      accruedInterest: 8,
      maturityDate: new BN(21315411),
      issuerName: "Issuer",
      couponFrequency: "Monthly",
    };

    // Create Bond
    await createBond(createParams, pdaGlobalConfig, pdaAgent);

    // Check stored values
    let globalConfig =
      await program.account.globalConfig.fetch(pdaGlobalConfig);
    assert.equal(globalConfig.bondName, createParams.token);
    assert.equal(
      globalConfig.issueSize.toNumber(),
      createParams.issueSize.toNumber(),
    );
    assert.equal(
      globalConfig.faceValue.toNumber(),
      createParams.faceValue.toNumber(),
    );
    assert.equal(globalConfig.couponRate, createParams.couponRate * 100);
    assert.equal(globalConfig.accruedInterest, createParams.accruedInterest);
    assert.equal(
      globalConfig.maturityDate.toNumber(),
      createParams.maturityDate.toNumber(),
    );
    assert.equal(globalConfig.issuerName, "Issuer");
    assert.equal(globalConfig.couponFrequency, createParams.couponFrequency);
    assert.equal(
      globalConfig.treasuryManager.toString(),
      admin.publicKey.toString(),
    );

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
      issueSize: new BN(100),
      faceValue: new BN(100),
      couponRate: 7,
      accruedInterest: 8,
      maturityDate: new BN(21315411),
      issuerName: "Issuer",
      couponFrequency: "Monthly",
    };

    // Create Bond
    await createBond(createParams, pdaGlobalConfig, pdaAgent);

    // Check stored values
    globalConfig = await program.account.globalConfig.fetch(pdaGlobalConfig);
    assert.equal(globalConfig.bondName, createParams.token);
    assert.equal(
      globalConfig.issueSize.toNumber(),
      createParams.issueSize.toNumber(),
    );
    assert.equal(
      globalConfig.faceValue.toNumber(),
      createParams.faceValue.toNumber(),
    );
    assert.equal(globalConfig.couponRate, createParams.couponRate * 100);
    assert.equal(globalConfig.accruedInterest, createParams.accruedInterest);
    assert.equal(
      globalConfig.maturityDate.toNumber(),
      createParams.maturityDate.toNumber(),
    );
    assert.equal(globalConfig.issuerName, "Issuer");
    assert.equal(globalConfig.couponFrequency, createParams.couponFrequency);
    assert.equal(
      globalConfig.treasuryManager.toString(),
      admin.publicKey.toString(),
    );

    agent = await program.account.agent.fetch(pdaAgent);
    assert.equal(agent.address.toString(), admin.publicKey.toString());
  });

  it("Test Share Stable Coins", async () => {
    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    let payment = new BN(100);
    let shareStableCoinParams = {
      token: TEST_TOKEN,
      coinType: { usdc: {} },
      toAccount: user1.publicKey,
      payment,
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

    // Share Stable Coins
    await shareStableCoin(
      shareStableCoinParams,
      pdaGlobalConfig,
      pdaAgent,
      user1ATA.address,
      usdcAccount,
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
    assert.equal(adminBalanceAfter, adminBalanceBefore - payment.toNumber());
    assert.equal(user1BalanceAfter, user1BalanceBefore + payment.toNumber());
  });

  // it("Test Share Dividend with Stable Coins by Fund Manager", async () => {
  //   [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [GLOBAL_CONFIG, TEST],
  //     program.programId
  //   );

  //   [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
  //     [AGENT, TEST],
  //     program.programId
  //   );

  //   let dividend = new BN(100);
  //   let shareStableCoinParams = {
  //     token: TEST_TOKEN,
  //     coinType: { usdc: {} },
  //     toAccount: user1.publicKey,
  //     dividend,
  //     assetType: { stableCoin: {} },
  //     decimals: DECIMALS,
  //   };

  //   let user1ATA = await getOrCreateAssociatedTokenAccount(
  //     provider.connection,
  //     payer,
  //     usdcAccount,
  //     user1.publicKey,
  //     undefined,
  //     undefined,
  //     undefined,
  //     TOKEN_2022_PROGRAM_ID
  //   );

  //   let user1BalanceBefore = Number(
  //     (
  //       await getAccount(
  //         provider.connection,
  //         user1ATA.address,
  //         undefined,
  //         TOKEN_2022_PROGRAM_ID
  //       )
  //     ).amount
  //   );

  //   let fundManagerATA = await getAssociatedTokenAddress(
  //     usdcAccount,
  //     fundManager.publicKey,
  //     undefined,
  //     TOKEN_2022_PROGRAM_ID
  //   );

  //   let fundManagerBalanceBefore = Number(
  //     (
  //       await getAccount(
  //         provider.connection,
  //         fundManagerATA,
  //         undefined,
  //         TOKEN_2022_PROGRAM_ID
  //       )
  //     ).amount
  //   );

  //   // Share Dividends
  //   await shareStableCoin(
  //     shareStableCoinParams,
  //     pdaGlobalConfig,
  //     pdaAgent,
  //     user1ATA.address,
  //     usdcAccount,
  //     fundManager
  //   );

  //   // Check Balances
  //   let fundManagerBalanceAfter = Number(
  //     (
  //       await getAccount(
  //         provider.connection,
  //         fundManagerATA,
  //         undefined,
  //         TOKEN_2022_PROGRAM_ID
  //       )
  //     ).amount
  //   );
  //   let user1BalanceAfter = Number(
  //     (
  //       await getAccount(
  //         provider.connection,
  //         user1ATA.address,
  //         undefined,
  //         TOKEN_2022_PROGRAM_ID
  //       )
  //     ).amount
  //   );
  //   assert.equal(
  //     fundManagerBalanceAfter,
  //     fundManagerBalanceBefore - dividend.toNumber()
  //   );
  //   assert.equal(user1BalanceAfter, user1BalanceBefore + dividend.toNumber());
  // });

  it("Test Update Agent", async () => {
    [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
      [MAINTAINERS],
      program.programId,
    );

    [pdaAgent] = anchor.web3.PublicKey.findProgramAddressSync(
      [AGENT, TEST],
      program.programId,
    );

    [pdaGlobalConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL_CONFIG, TEST],
      program.programId,
    );

    let agent = await program.account.agent.fetch(pdaAgent);
    assert.equal(agent.address.toString(), admin.publicKey.toString());

    let updateAgent = await program.methods
      .updateAgentByToken(TEST_TOKEN, user1.publicKey)
      .accounts({
        maintainers: pdaMaintainers,
        agent: pdaAgent,
        globalConfig: pdaGlobalConfig,
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
