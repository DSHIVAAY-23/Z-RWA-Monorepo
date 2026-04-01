import type { Metadata } from 'next';
import './globals.css';
import SolanaWalletProvider from '../components/WalletProvider';

export const metadata: Metadata = {
  title: 'Z-RWA | Private Compliance',
  description: 'Zero-Knowledge RWA Compliance.',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  // Force dark mode class on html so dark variants apply
  return (
    <html lang="en" className="dark">
      <body>
        <SolanaWalletProvider>
          {children}
        </SolanaWalletProvider>
      </body>
    </html>
  );
}
