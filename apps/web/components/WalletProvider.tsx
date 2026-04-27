'use client';

import { useMemo } from 'react';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets';
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui';

require('@solana/wallet-adapter-react-ui/styles.css');

export default function SolanaWalletProvider({ children }: { children: React.ReactNode }) {
  const network = WalletAdapterNetwork.Devnet;

  // Uses NEXT_PUBLIC_QUICKNODE_RPC_URL or standard RPC_URL
  const endpoint = useMemo(() => 
    process.env.NEXT_PUBLIC_QUICKNODE_RPC_URL || 
    process.env.NEXT_PUBLIC_RPC_URL || 
    'https://frequent-alpha-pool.solana-devnet.quiknode.pro/5f06a41cf6e077af5ca7ac464fbf1caed5c84d42/', 
  []);

  const wallets = useMemo(
    () => [
      new PhantomWalletAdapter({ network }),
      new SolflareWalletAdapter({ network }),
    ],
    [network]
  );
  return (
    // @ts-ignore - Next.js/React 18 types conflict with Solana Wallet Adapter
    <ConnectionProvider endpoint={endpoint}>
      {/* @ts-ignore */}
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>{children}</WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
