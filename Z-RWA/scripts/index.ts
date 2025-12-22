import * as token_program from "./token-program";
import * as fund from "./fund-contract";
import * as base from "./base-token-program";
import * as core from "./interop-core";
import * as multisig from "./interop-multisig";


import { PublicKey } from "@solana/web3.js";
import { fundWalletIfNeeded } from "./solanaService";

import { ChainName } from "@certusone/wormhole-sdk";
import BN from "bn.js";

const callTheFunction = async () => {
  console.log("Triggering functions , please wait !");
  // ==============================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

  await fundWalletIfNeeded(0.5, 2);

  //  await token_program.initTokenProgram();
  // await fund.initFundContract();
   await base.initBaseTokenProgram();
  //  await base.createToken();
  await base.fetchBaseMaintainers();

   // await core.initInteropCore();
   // await multisig.initInteropMultisig();
   await base.getBaseKeys();

  console.log("Functions Triggered, success !");
  console.log("sent =>>>>>>>>");
  // ==============================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

  // ==============================================>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
};

callTheFunction();

// npm start run
