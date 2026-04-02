import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { docType, docHash, query } = await request.json();

    const isMockMode = process.env.NEXT_PUBLIC_MOCK_MODE === 'true';

    if (isMockMode) {
      // Simulate proving time (3 seconds in mock mode for faster UX)
      await new Promise(resolve => setTimeout(resolve, 3000));

      // [CRITICAL] Raw hex strings only - Buffer.from(..., 'hex') fails on '0x' prefixes
      const proofMock = "cafe".repeat(130); // 260 bytes
      const publicValuesMock = "deadbeef".repeat(8); // 32 bytes

      return NextResponse.json({
        proof: proofMock,
        publicValues: publicValuesMock,
        proofSize: 260,
        provingTime: "3.0s",
        docHash: docHash
      });
    } else {
      // In production scenario: Simulate artifact retrieval or SP1 call
      await new Promise(r => setTimeout(r, 1500));
    
      const provingTime = "1.42s";
      const proofSize = 260;
      const proofMock = "cafe".repeat(130); 
      const publicValuesMock = "deadbeef".repeat(8);

      return NextResponse.json({
        proof: proofMock,
        publicValues: publicValuesMock,
        proofSize: proofSize,
        provingTime: provingTime,
        docHash: docHash
      });
    }

  } catch (error: any) {
    console.error("API Prove Error:", error);
    return NextResponse.json({ 
      error: 'Failed to generate ZK proof',
      details: error.message 
    }, { status: 500 });
  }
}
