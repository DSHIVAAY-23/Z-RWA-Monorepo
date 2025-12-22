import * as anchor from "@coral-xyz/anchor";
import {
  getProvider,
  tokenProgramID,
  tokenProgramInterface,
} from "./solanaService";
import { TokenProgram } from "../target/types/token_program";
import { Program } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import {
  TOKEN_2022_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  AdminAddress,
  MAINTAINERS,
  CONFIG,
  TEST,
  TEST_TOKEN,
  MINT,
} from "./constant";
import * as fs from "fs";
import { PublicKey } from "@solana/web3.js";

const { provider }: any = getProvider();
if (!provider) throw new Error("Provider not available");
let program: any = new anchor.Program(
  tokenProgramInterface,
  provider,
) as Program<TokenProgram>;

const [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
  [MAINTAINERS],
  program.programId,
);

const [pdaConfig] = anchor.web3.PublicKey.findProgramAddressSync(
  [CONFIG, TEST],
  program.programId,
);

const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  [MINT, TEST],
  program.programId,
);

const initTokenProgram = async () => {
  try {
    await program.account.maintainers.fetch(pdaMaintainers);
    console.log("token-program already initialized; skipping init");
    return;
  } catch {}

  await program.methods
    .init()
    .accounts({
      maintainers: pdaMaintainers,
      authority: AdminAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .rpc();
};

const fetchMaintainers = async () => {
  let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
  console.log(maintainers.admin.toString());
  console.log(maintainers.subAdmins.toString());
};

const createToken = async () => {
  let createTokenParams = {
    id: "unique",
    name: TEST_TOKEN,
    symbol: "tes",
    uri: "some/uri",
    issuer: AdminAddress,
    transferAgent: AdminAddress,
    tokenizationAgent: AdminAddress,
  };

  await program.methods
    .create(createTokenParams)
    .accounts({
      maintainers: pdaMaintainers,
      config: pdaConfig,
      mintAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .rpc();
};

const getBaseKeys = async () => {
  console.log("mint", mintAccount.toString());
  console.log("config", pdaConfig.toString());
  console.log("maintainers", pdaMaintainers.toString());

  // let supply = await provider.connection.getTokenSupply(mintAccount);
  // console.log(Number(supply.value.amount));
};

const fetchBalances = async () => {
  let userATA = await getAssociatedTokenAddress(
    mintAccount,
    AdminAddress,
    undefined,
    TOKEN_2022_PROGRAM_ID,
  );

  let supply = (await provider.connection.getTokenSupply(mintAccount)).value
    .amount;

  let userAccountBalance = Number(
    (
      await getAccount(
        provider.connection,
        userATA,
        undefined,
        TOKEN_2022_PROGRAM_ID,
      )
    ).amount,
  );

  console.log("supply: ", supply);
  console.log("user balance: ", userAccountBalance);
};

const updateTokenProgramAdmin = async (admin: PublicKey) => {
  await program.methods
    .manageAdmin(admin)
    .accounts({
      maintainers: pdaMaintainers,
      authority: AdminAddress,
    })
    .rpc();
};

export { initTokenProgram, fetchMaintainers, updateTokenProgramAdmin, getBaseKeys };
