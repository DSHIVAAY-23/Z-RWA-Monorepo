"use server";
import { GoldRushClient } from "@covalenthq/client-sdk";

const client = new GoldRushClient(
  process.env.NEXT_PUBLIC_GOLDRUSH_API_KEY || ""
);

export async function getWalletTransactions(walletAddress: string) {
  try {
    const resp = await client.TransactionService
      .getAllTransactionsForAddressByPage(
        "solana-mainnet",
        walletAddress,
        { noLogs: true, pageSize: 10 }
      );
    return resp.data?.items || [];
  } catch (error) {
    console.error("[GoldRush] Error fetching transactions:", error);
    return [];
  }
}

export async function getWalletTokenBalances(walletAddress: string) {
  try {
    const resp = await client.BalanceService
      .getTokenBalancesForWalletAddress(
        "solana-mainnet", 
        walletAddress
      );
    return resp.data?.items || [];
  } catch (error) {
    console.error("[GoldRush] Error fetching balances:", error);
    return [];
  }
}
