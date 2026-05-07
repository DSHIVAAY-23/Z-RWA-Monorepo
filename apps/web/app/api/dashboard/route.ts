import { NextResponse } from 'next/server';

async function getDodoPayments(walletAddress: string) {
  try {
    const resp = await fetch(
      'https://api.dodopayments.com/payments?limit=10',
      { headers: { 'Authorization': `Bearer ${process.env.DODO_API_KEY}` } }
    );
    const data = await resp.json();
    return (data.items || []).filter(
      (p: any) => p.metadata?.wallet_address === walletAddress
    );
  } catch (error) {
    console.error('[Dodo] Failed to fetch payments:', error);
    return [];
  }
}

async function getRWATokenBalance(walletAddress: string) {
  try {
    const { Connection, PublicKey } = await import('@solana/web3.js');
    const connection = new Connection(
      process.env.RPC_ENDPOINT || 'https://api.devnet.solana.com'
    );
    const RWA_MINT = '8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa';
    const tokenAccounts = await connection.getParsedTokenAccountsByOwner(
      new PublicKey(walletAddress),
      { mint: new PublicKey(RWA_MINT) }
    );
    const balance = tokenAccounts.value[0]
      ?.account?.data?.parsed?.info?.tokenAmount?.uiAmount || 0;
    return balance;
  } catch (error) {
    console.error('[Solana] Token balance error:', error);
    return 0;
  }
}

async function getComplianceStatus(walletAddress: string) {
  try {
    const baseUrl = process.env.NEXT_PUBLIC_APP_URL ||
                    'https://z-rwa-monorepo.vercel.app';
    const response = await fetch(`${baseUrl}/api/verify/${walletAddress}`);
    const data = await response.json();
    return {
      verified: data.compliant || false,
      proofHash: data.proof_hash || null,
      verifiedAt: data.verified_at || null
    };
  } catch {
    return { verified: false, proofHash: null, verifiedAt: null };
  }
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const wallet = searchParams.get('wallet');

  if (!wallet) {
    return NextResponse.json({ error: 'Wallet address required' }, { status: 400 });
  }

  const [payments, tokenBalance, compliance] = await Promise.all([
    getDodoPayments(wallet),
    getRWATokenBalance(wallet),
    getComplianceStatus(wallet)
  ]);

  return NextResponse.json({
    wallet,
    stats: {
      proofs_generated: compliance.verified ? 1 : 0,
      tokens_owned: tokenBalance,
      payments_done: payments.length
    },
    compliance,
    transactions: payments.map((p: any) => ({
      payment_id: p.payment_id,
      status: p.status === 'succeeded' ? 'COMPLETE' : p.status,
      amount: `₹${p.amount / 100}`,
      proof_hash: p.metadata?.proof_hash || null,
      mint_address: p.metadata?.mint_address ||
                    '8GWCAZsHLMw3XaBACPxZzSz5Q2bqSKAZXx8NwYqkJcaa',
      created_at: p.created_at
    }))
  });
}
