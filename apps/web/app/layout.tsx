import type { Metadata } from 'next';
import './globals.css';
import SolanaWalletProvider from '../components/WalletProvider';

export const metadata: Metadata = {
  title: 'Z-RWA | Private Compliance',
  description: 'Zero-Knowledge RWA Compliance.',
};

import { ThemeProvider } from '../components/ThemeProvider';

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body>
        <ThemeProvider attribute="class" defaultTheme="dark" enableSystem>
          <SolanaWalletProvider>
            {children}
          </SolanaWalletProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
