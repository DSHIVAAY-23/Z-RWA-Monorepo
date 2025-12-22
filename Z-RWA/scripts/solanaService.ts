/* scripts/solanaService.ts
   Patched: requires IDL JSONs, robust keypair lookup, stable Uint8Array conversion for bs58.
*/

import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { TokenProgram } from "../target/types/token_program";
import { FundContract } from "../target/types/fund_contract";
import { TreasuryBond } from "../target/types/treasury_bond";
import { BaseTokenProgram } from "../target/types/base_token_program";
import { InteropCore } from "../target/types/interop_core";
import { InteropMultisig } from "../target/types/interop_multisig";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import * as fs from "fs";
import * as path from "path";

import {
  AdminPrivateKey,
  FUND_CONTRACT_PROGRAM_ID,
  BOND_CONTRACT_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  BASE_TOKEN_PROGRAM_ID,
  INTEROP_CORE_PROGRAM_ID,
  INTEROP_MULTISIG_PROGRAM_ID,
  WORMHOLE_MESSAGING_PROGRAM_ID,
} from "./constant";

/* Load IDL JSONs via require() to avoid import interop issues with ts-node */
const tokenProgramIDL: any = require("../target/idl/token_program.json");
const fundContractIDL: any = require("../target/idl/fund_contract.json");
const bondContractIDL: any = require("../target/idl/treasury_bond.json");
const baseTokenProgramIDL: any = require("../target/idl/base_token_program.json");
const interopCoreIDL: any = require("../target/idl/interop_core.json");
const interopMultisigIDL: any = require("../target/idl/interop_multisig.json");

/* Defensive checks */
if (!tokenProgramIDL) throw new Error("Failed to load target/idl/token_program.json");
if (!fundContractIDL) throw new Error("Failed to load target/idl/fund_contract.json");
if (!bondContractIDL) throw new Error("Failed to load target/idl/treasury_bond.json");
if (!baseTokenProgramIDL) throw new Error("Failed to load target/idl/base_token_program.json");
if (!interopCoreIDL) throw new Error("Failed to load target/idl/interop_core.json");
if (!interopMultisigIDL) throw new Error("Failed to load target/idl/interop_multisig.json");

export const tokenProgramID = new PublicKey(TOKEN_PROGRAM_ID);
export const fundProgramID = new PublicKey(FUND_CONTRACT_PROGRAM_ID);
export const bondProgramID = new PublicKey(BOND_CONTRACT_PROGRAM_ID);
export const baseTokenProgramID = new PublicKey(BASE_TOKEN_PROGRAM_ID);
export const interopCoreProgramID = new PublicKey(INTEROP_CORE_PROGRAM_ID);
export const interopMultisigProgramID = new PublicKey(INTEROP_MULTISIG_PROGRAM_ID);
export const wormholeMessagingProgramID = new PublicKey(WORMHOLE_MESSAGING_PROGRAM_ID);

/* Keep the same JSON -> object conversion used by your scripts */
export const tokenProgramInterface = JSON.parse(JSON.stringify(tokenProgramIDL));
export const fundProgramInterface = JSON.parse(JSON.stringify(fundContractIDL));
export const bondProgramInterface = JSON.parse(JSON.stringify(bondContractIDL));
export const baseTokenProgramInterface = JSON.parse(JSON.stringify(baseTokenProgramIDL));
export const interopCoreInterface = JSON.parse(JSON.stringify(interopCoreIDL));
export const interopMultisigInterface = JSON.parse(JSON.stringify(interopMultisigIDL));

/* Network and options
 * Prefer local validator if SOLANA_URL or Anchor.toml provider cluster is localnet.
 */
//const solanaNetwork = process.env.SOLANA_URL || "http://127.0.0.1:8899";
const solanaNetwork = process.env.SOLANA_URL || "https://api.devnet.solana.com";

const opts: any = {
  preflightCommitment: "processed",
};

export const getProvider = (): {
  provider: AnchorProvider;
  connection: web3.Connection;
} => {
  try {
    const connection = new web3.Connection(solanaNetwork, opts.preflightCommitment);

    // Resolve keypair path: prefer env var, else default ~/.config/solana/id.json
    const envKey = process.env.SOLANA_KEYPAIR || "";
    const home = process.env.HOME || process.env.USERPROFILE || "";
    const defaultKeyPath = path.join(home, ".config", "solana", "id.json");

    const candidatePaths = [
      envKey && envKey.trim().length ? envKey.trim() : null,
      defaultKeyPath,
    ].filter(Boolean) as string[];

    let keypairPath: string | null = null;
    for (const p of candidatePaths) {
      const expanded = p.startsWith("~") ? path.join(home, p.slice(1)) : p;
      if (fs.existsSync(expanded)) {
        keypairPath = expanded;
        break;
      }
    }

    if (!keypairPath) {
      throw new Error(
        `No Solana keypair found. Tried: ${candidatePaths.join(
          ", ",
        )}. Set env SOLANA_KEYPAIR to the path of your id.json or create ~/.config/solana/id.json`,
      );
    }

    // Read key file content
    const raw = fs.readFileSync(keypairPath, "utf-8").trim();

    // Parse into an intermediate 'any' variable to avoid TS narrowing issues
    let parsed: any = null;
    try {
      parsed = JSON.parse(raw);
      // handle case where JSON file is { "secret": [...] } or { data: [...] }
      if (parsed && typeof parsed === "object" && !Array.isArray(parsed)) {
        if (Array.isArray(parsed.secret)) {
          parsed = parsed.secret;
        } else if (Array.isArray(parsed.data)) {
          parsed = parsed.data;
        }
      }
    } catch (err) {
      // Not JSON — maybe a base58 string
      parsed = raw;
    }

    // Normalize to Uint8Array
    let secretKeyUint8: Uint8Array;

    if (Array.isArray(parsed)) {
      // normal id.json: array of numbers
      secretKeyUint8 = Uint8Array.from(parsed as number[]);
    } else if (Buffer.isBuffer(parsed)) {
      // Buffer -> Uint8Array
      secretKeyUint8 = new Uint8Array(parsed);
    } else if (typeof parsed === "string") {
      // base58 string -> decode then convert to Uint8Array explicitly
      try {
        const decoded = bs58.decode(parsed); // Buffer-like
        // convert Buffer (or any iterable) to Uint8Array
        secretKeyUint8 = Uint8Array.from((decoded as unknown) as number[]);
      } catch (err) {
        throw new Error("Failed to decode base58 keypair string.");
      }
    } else if (parsed && (ArrayBuffer.isView(parsed) || parsed instanceof Uint8Array)) {
      // ArrayBuffer view or Uint8Array
      secretKeyUint8 = parsed as Uint8Array;
    } else {
      // Last-ditch try
      try {
        secretKeyUint8 = Uint8Array.from(parsed as any);
      } catch (err) {
        throw new Error("Unable to parse keypair file into secret key (expected array-of-numbers or base58).");
      }
    }

    // Create keypair
    const privateKeyWallet = anchor.web3.Keypair.fromSecretKey(secretKeyUint8);

    const provider: any = new AnchorProvider(connection, new NodeWallet(privateKeyWallet), opts);
    console.log("Using provider with wallet:", provider.wallet.publicKey.toBase58());
    return { provider, connection };
  } catch (error) {
    console.log("provider:solana error:", error);
    throw error;
  }
};

/**
 * Request an airdrop on devnet if the wallet balance is below the threshold.
 * No-op on mainnet as requestAirdrop will fail there.
 */
export const fundWalletIfNeeded = async (
  minimumSol = 0.5,
  airdropSol = 2,
): Promise<void> => {
  const { provider, connection } = getProvider();
  const walletPubkey = provider.wallet.publicKey;

  try {
    const balanceLamports = await connection.getBalance(walletPubkey, "processed");
    const balanceSol = balanceLamports / web3.LAMPORTS_PER_SOL;
    if (balanceSol >= minimumSol) return;

    const sig = await connection.requestAirdrop(
      walletPubkey,
      airdropSol * web3.LAMPORTS_PER_SOL,
    );
    await connection.confirmTransaction(sig, "confirmed");
    const newBalance = (await connection.getBalance(walletPubkey)) / web3.LAMPORTS_PER_SOL;
    console.log(
      `Airdropped ${airdropSol} SOL to ${walletPubkey.toBase58()}. New balance: ${newBalance} SOL`,
    );
  } catch (e) {
    // Best-effort: log and continue; some RPCs/rates may fail
    console.log("Airdrop attempt failed (non-fatal):", e);
  }
};
