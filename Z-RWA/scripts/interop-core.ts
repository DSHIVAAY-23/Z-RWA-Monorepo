import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  getProvider,
  interopCoreProgramID,
  interopCoreInterface,
  interopMultisigProgramID,
  baseTokenProgramInterface,
} from "./solanaService";
import { InteropCore } from "../target/types/interop_core";
import { BaseTokenProgram } from "../target/types/base_token_program";
import { Program } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import {
  TOKEN_2022_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  AdminAddress,
  MAINTAINERS,
  EXECUTER,
  BASE_TOKEN_PROGRAM_ID,
  MINT,
  TEST,
} from "./constant";
import * as fs from "fs";

const { provider }: any = getProvider();
if (!provider) throw new Error("Provider not available");

let program: any = new anchor.Program(
  baseTokenProgramInterface,
  provider,
) as Program<BaseTokenProgram>;

const [mintAccount] = anchor.web3.PublicKey.findProgramAddressSync(
  [MINT, TEST],
  program.programId,
);

program = new anchor.Program(
  interopCoreInterface,
  provider,
) as Program<InteropCore>;

const [pdaMaintainers] = anchor.web3.PublicKey.findProgramAddressSync(
  [MAINTAINERS],
  program.programId,
);

const [pdaExecuter] = anchor.web3.PublicKey.findProgramAddressSync(
  [EXECUTER],
  program.programId,
);

const initParams = {
  multisig: interopMultisigProgramID,
  deployedChain: "Solana",
};

const initInteropCore = async () => {
  try {
    await program.account.maintainers.fetch(pdaMaintainers);
    await program.account.executer.fetch(pdaExecuter);
    console.log("interop-core already initialized; skipping init");
    return;
  } catch {}

  await program.methods
    .init(initParams)
    .accounts({
      maintainers: pdaMaintainers,
      executer: pdaExecuter,
      authority: AdminAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .rpc();
};

const fetchCoreMaintainers = async () => {
  let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
  console.log("Admins: ", maintainers.admins.toString());

  let executer = await program.account.executer.fetch(pdaExecuter);
  console.log("Executer: ", executer.address.toString());
};

const sendMintInstruction = async () => {
  let sendParams = {
    destChain: "Polygon",
    destAddress: "0x62152c32120A55dB65f9773EEb8544fB2ebF6800",
    investor: "0x0B70373D5BA5b0Da8672fF62704bFD117211C2C2",
    token: "0xAE4D4a1b78dA1b50E5e50E2CE462ADC9eD6d3c08",
    amount: "100000000000000000000",
    orderId: new BN(101),
    action: { mint: {} },
  };

  let tx = await program.methods
    .sendInstructions(sendParams)
    .accounts({
      caller: AdminAddress,
    })
    .rpc();
  console.log(
    `Tx URL: `,
    `https://explorer.solana.com/tx/${tx.toString()}?cluster=devnet`,
  );

  // // Get transaction from its signature
  // const txs = await anchor.getProvider().connection.getTransaction(tx);

  // const eventParser = new EventParser(program.programId, new BorshCoder(program.idl));
  // const events = eventParser.parseLogs(txs.meta.logMessages);
  // for (let event of events) {
  //  console.log(event);
  // }
};

const executeInstruction = async () => {
  let executeParams = {
    sourceChain: "Polygon",
    sourceAddress: "0xaa19bBDF7C2FB757DD4bfbA0b1D5f3dBc89c3Bea",
    payload:
      "000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000006400000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002c4578377938535a53706431424d4461356d4d52653136437665767348353634457a6d45434c6678694e625633000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000045465737400000000000000000000000000000000000000000000000000000000",
  };

  const rawPayerKeypair = JSON.parse(
    fs.readFileSync("/Users/tarunjaiswal/.config/solana/id.json", "utf-8"),
  );
  const adminKey = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(rawPayerKeypair),
  );
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
    .executeInstructions(
      executeParams.sourceChain,
      executeParams.sourceAddress,
      executeParams.payload,
    )
    .accounts({
      executer: pdaExecuter,
      caller: AdminAddress,
      mintAccount,
      user: userATA.address,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      baseTokenProgram: BASE_TOKEN_PROGRAM_ID,
    })
    .rpc();
};

export {
  sendMintInstruction,
  initInteropCore,
  fetchCoreMaintainers,
  executeInstruction,
};
