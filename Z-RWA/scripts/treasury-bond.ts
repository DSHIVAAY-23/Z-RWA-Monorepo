import * as anchor from "@coral-xyz/anchor";
import {
  getProvider,
  bondProgramID,
  bondProgramInterface,
} from "./solanaService";
import { TreasuryBond } from "../target/types/treasury_bond";
import { Program } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import {
  AdminAddress,
  MAINTAINERS,
  CONFIG,
  TEST,
  TEST_TOKEN,
  MINT,
} from "./constant";
import { PublicKey } from "@solana/web3.js";

const { provider }: any = getProvider();
if (!provider) throw new Error("Provider not available");
let program: any = new anchor.Program(
  bondProgramInterface,
  provider,
) as Program<TreasuryBond>;

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

const initBondContract = async () => {
  await program.methods
    .init()
    .accounts({
      maintainers: pdaMaintainers,
      authority: AdminAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();
};

const fetchBondMaintainers = async () => {
  console.log("maintainer key: ", pdaMaintainers.toBase58());
  let maintainers = await program.account.maintainers.fetch(pdaMaintainers);
  console.log(maintainers.admins.toString());
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

export { fetchBondMaintainers, updateTokenProgramAdmin, initBondContract };
