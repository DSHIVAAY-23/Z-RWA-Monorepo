import * as anchor from "@coral-xyz/anchor";
import {
  getProvider,
  baseTokenProgramInterface,
} from "./solanaService";
import { BaseTokenProgram } from "../target/types/base_token_program";
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
import path from "path";

const { provider }: any = getProvider();
if (!provider) throw new Error("Provider not available");
let program: any = new anchor.Program(
  baseTokenProgramInterface,
  provider,
) as Program<BaseTokenProgram>;

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

const initBaseTokenProgram = async () => {
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

const fetchBaseMaintainers = async () => {
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

const requestOrders = async () => {
 const keyPath = process.env.SOLANA_KEYPAIR || path.join(process.env.HOME || process.env.USERPROFILE || "/", ".config", "solana", "id.json");
const rawKeyTxt = fs.readFileSync(keyPath, "utf-8").trim();
let parsedKey: any;
try {
  parsedKey = JSON.parse(rawKeyTxt);
  if (parsedKey && typeof parsedKey === "object" && !Array.isArray(parsedKey)) {
    if (Array.isArray(parsedKey.secret)) parsedKey = parsedKey.secret;
    else if (Array.isArray(parsedKey.data)) parsedKey = parsedKey.data;
  }
} catch (e) { 
  parsedKey = rawKeyTxt;
}
const secretKeyUint8 = Array.isArray(parsedKey)
  ? Uint8Array.from(parsedKey as number[])
  : Buffer.isBuffer(parsedKey)
    ? new Uint8Array(parsedKey)
    : typeof parsedKey === "string"
      ? Uint8Array.from((require("@coral-xyz/anchor/dist/cjs/utils/bytes").bs58.decode(parsedKey)) as unknown as number[])
      : Uint8Array.from(parsedKey as any);

const adminKey = anchor.web3.Keypair.fromSecretKey(secretKeyUint8);

  let userATA = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    adminKey,
    mintAccount,
    AdminAddress,
    undefined,
    undefined,
    undefined,
    TOKEN_2022_PROGRAM_ID,
  );

  let requestParams = {
    orderId: new BN(1),
    token: TEST_TOKEN,
    toAccount: AdminAddress,
    amount: new BN(100),
    requestType: { mint: {} },
  };

  console.log(userATA.address.toBase58());

  await program.methods
    .requestOrders(requestParams)
    .accounts({
      mintAccount,
      user: userATA.address,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .rpc();
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

export {
  initBaseTokenProgram,
  fetchBaseMaintainers,
  createToken,
  getBaseKeys,
  requestOrders,
  fetchBalances,
};
