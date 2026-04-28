const ZRWA_ORACLE_URL = process.env.ZRWA_ORACLE_URL || 
  "https://z-rwa-monorepo.vercel.app";

export async function checkZKCompliance(
  walletAddress: string
): Promise<{
  compliant: boolean;
  proofHash: string | null;
  expiresAt: Date | null;
  freshnessSeconds: number;
}> {
  try {
    const response = await fetch(
      `${ZRWA_ORACLE_URL}/api/verify/${walletAddress}`
    );
    
    if (!response.ok) {
      throw new Error(`Oracle returned ${response.status}`);
    }
    
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const data = await response.json() as any;
    
    const now = new Date();
    const verifiedAt = data.verified_at ? new Date(data.verified_at) : null;
    const expiresAt = data.expires_at ? new Date(data.expires_at) : null;
    
    const freshnessSeconds = verifiedAt 
      ? Math.floor((now.getTime() - verifiedAt.getTime()) / 1000)
      : Infinity;
    
    return {
      compliant: data.compliant,
      proofHash: data.proof_hash || null,
      expiresAt,
      freshnessSeconds,
    };
  } catch (error) {
    console.error(`Compliance check failed for ${walletAddress}:`, error);
    return {
      compliant: false,
      proofHash: null,
      expiresAt: null,
      freshnessSeconds: Infinity,
    };
  }
}
