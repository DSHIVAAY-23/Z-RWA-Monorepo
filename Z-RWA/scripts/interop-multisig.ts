import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  getProvider,
  interopCoreProgramID,
  interopMultisigProgramID,
  interopMultisigInterface,
  baseTokenProgramID,
} from "./solanaService";
import { InteropMultisig } from "../target/types/interop_multisig";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  AdminAddress,
  MAINTAINERS,
  EXECUTER,
  THRESHOLD,
  VALIDATORS,
  VOTES,
  BASE_TOKEN_PROGRAM_ID,
  INTEROP_CORE_PROGRAM_ID,
  MINT,
  TEST,
  PAYLOAD,
} from "./constant";
import * as fs from "fs";
import path from "path";

const { provider }: any = getProvider();
if (!provider) throw new Error("Provider not available");

const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  [MINT, TEST],
  baseTokenProgramID,
);

const [pdaExecuter] = anchor.web3.PublicKey.findProgramAddressSync(
  [EXECUTER],
  interopCoreProgramID,
);

let program: any = new anchor.Program(
  interopMultisigInterface,
  provider,
) as Program<InteropMultisig>;

const [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
  [MAINTAINERS],
  program.programId,
);

const [pdaThreshold] = anchor.web3.PublicKey.findProgramAddressSync(
  [THRESHOLD],
  program.programId,
);

const [pdaValidators] = anchor.web3.PublicKey.findProgramAddressSync(
  [VALIDATORS],
  program.programId,
);

const [pdaVotes] = anchor.web3.PublicKey.findProgramAddressSync(
  [VOTES],
  program.programId,
);

const [pdaPayload] = anchor.web3.PublicKey.findProgramAddressSync(
  [PAYLOAD],
  program.programId,
);

const initInteropMultisig = async () => {
  await program.methods
    .init(1)
    .accounts({
      maintainers: pdaMaintainers,
      threshold: pdaThreshold,
      authority: AdminAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
};

const fetchMultisigMaintainers = async () => {
  let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
  console.log("Admins: ", maintainers.admins.toString());

  let threshold = await program.account.threshold.fetch(pdaThreshold);
  console.log("Threshold: ", threshold.value);

  let validators = await program.account.validators.fetch(pdaValidators);
  console.log("Validators: ", validators.addresses.toString());

  let votes = await program.account.votes.fetch(pdaVotes);
  console.log("Votes: ", votes);

  console.log({
    validators: pdaValidators.toBase58(),
    threshold: pdaThreshold.toBase58(),
    votes: pdaVotes.toBase58(),
    payload: pdaPayload.toBase58(),
  });

  let payload = await program.account.payload.fetch(pdaPayload);
  console.log("Payload: ", payload);
};

const manageRoles = async (role: any) => {
  await program.methods
    .manageRoles(role)
    .accounts({
      maintainers: pdaMaintainers,
      validators: pdaValidators,
      authority: AdminAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
};

const addValidators = async (newValidator: PublicKey) => {
  let role = {
    validators: {
      updateType: { add: { addresses: [newValidator] } },
    },
  };

  await manageRoles(role);
};

const castVote = async () => {
  let castVoteParams = {
    txHash: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
    canTransact: true,
  };

  await program.methods
    .castVotes(castVoteParams.txHash, castVoteParams.canTransact)
    .accounts({
      validators: pdaValidators,
      threshold: pdaThreshold,
      votes: pdaVotes,
      authority: AdminAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
};

const executeTransaction = async () => {
  let executeParams = {
    txHash: "0x5542E6947a86A0A1069690f61006A53B35BB56e8",
    sourceChain: "Polygon",
    sourceAddress: "0xaa19bBDF7C2FB757DD4bfbA0b1D5f3dBc89c3Bea",
    payload:
      "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002c4578377938535a53706431424d4461356d4d52653136437665767348353634457a6d45434c6678694e625633000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000045465737400000000000000000000000000000000000000000000000000000000",
  };

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

  await program.methods
    .executeTransactions(executeParams)
    .accounts({
      threshold: pdaThreshold,
      votes: pdaVotes,
      executer: pdaExecuter,
      mintAccount,
      user: userATA.address,
      caller: AdminAddress,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      interopCoreProgram: INTEROP_CORE_PROGRAM_ID,
      baseTokenProgram: BASE_TOKEN_PROGRAM_ID,
    })
    .rpc();
};

export {
  addValidators,
  initInteropMultisig,
  fetchMultisigMaintainers,
  castVote,
  executeTransaction,
};
