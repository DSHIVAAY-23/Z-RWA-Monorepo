import { PublicKey } from "@solana/web3.js";

export const TOKEN_PROGRAM_ID: string =
  "33pHXYQbe41JJSA7oXor6h7JFY74eqH25xtSjBysmTYo";
export const FUND_CONTRACT_PROGRAM_ID: string =
  "H9txrMrTfGU6LXYWBzMHzmzuQWbNYcW1vHFLRcxCYiKn";
export const BOND_CONTRACT_PROGRAM_ID: string =
  "DFBmTsrcfpp8pvU2zF1JoFzVW6VG8KEnNnmisbFKyEXv";
export const BASE_TOKEN_PROGRAM_ID: string =
  "7iaDbVGbVJdhZcKXWQmMw783nBqfDyx6K8V4yF6Kv8iq";
export const INTEROP_CORE_PROGRAM_ID: string =
  "5dyQmihDcQqCBerQg82J19QxMByjpxbwdtmCuPkT9ePD";
export const INTEROP_MULTISIG_PROGRAM_ID: string =
  "2uuBHq3teujBvfE3AnRm4LZFYk7sHUiC36Z9MdkgcJ2N";
export const WORMHOLE_MESSAGING_PROGRAM_ID: string =
  "9wXFsMPMZcaCBMTVUYyB8Pwx59invxh1aTkwtnDhKLCQ";
export const usdcAddress: string =
  "5jfx4RgPRbEXYMLFVEXtwpHZ4chG7MCgyuxS8r4Y7z8m";
export const AdminPrivateKey: string =
  "3DYMbsz2YA3PLNP5XjEiLNnDewgsywgdJmZphSTwBy4vigmc6u6W5HmZa57WvHi3mvqVpLtGyxTs9wLazPJEoEDo";
export const AptosKey: string =
  "3383D26B73B3A5BD6104E723AA6C7E53AB5AD02AC5F60F81A3C2AA36F1B1446D";
export const AvalancheKey: string =
  "762d10f59b45a5783b7a50ba406be15584b6dc859fb21b4144f375671150ada4";
// export const AdminPrivateKey: string =
//   "5TUcWtxoMWeC51MnUJAciG9fMda6Pjo4DEVvwbQaDsxPv2rq8Mp28eUBVhAJhJPpiiV5CetjZiFfNVkaQUESiVms";
// export const AdminAddress: PublicKey = new PublicKey(
//   "CRiYxizqywdhEpHRk1pPFuWCPfMbpURjoS3vj9cWgGCH");

export const AdminAddress: PublicKey = new PublicKey(
  "GsPrDLXoqVbcWwofYpRZFJg4h5dzHEjyNfPyzPrcUKGd",
);

// export const AdminAddress: PublicKey = new PublicKey(
//   "7rLxamNEF6576w8ApGBvcW3D19KihChTmPhgtizViL7t",
// );

export const MINT = Buffer.from("mint");
export const MAINTAINERS = Buffer.from("maintainers");
export const CONFIG = Buffer.from("config");
export const PARTIAL_FREEZE = Buffer.from("partial_freeze");
export const EXECUTER = Buffer.from("executer");
export const THRESHOLD = Buffer.from("threshold");
export const VALIDATORS = Buffer.from("validators");
export const RECEIVED = Buffer.from("received");
export const VOTES = Buffer.from("votes");
export const PAYLOAD = Buffer.from("payload");
export const TEST_TOKEN = "Test";
export const TEST = Buffer.from(TEST_TOKEN);
