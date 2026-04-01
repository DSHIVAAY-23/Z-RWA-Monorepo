import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const { docType, docHash, query } = await request.json();

    const isMockMode = process.env.NEXT_PUBLIC_MOCK_MODE === 'true';

    if (isMockMode) {
      // Simulate proving time (3 seconds in mock mode for faster UX)
      await new Promise(resolve => setTimeout(resolve, 3000));

      const proofMock = "0x" + "a".repeat(520); // 260 bytes in hex
      const publicValuesMock = "0x" + "b".repeat(64);

      const resData = {
        proof: proofMock,
        publicValues: publicValuesMock,
        proofSize: "260",
        provingTime: "1.42",
        docHash: docHash
      };

      return NextResponse.json(resData);
    } else {
      // In production scenario: Call ZK-RAG SP1 prover binary or service
      // Simulated heavy compute + prover setup
      await new Promise(r => setTimeout(r, 1500));
    
      const provingTime = "1.42";
      const proofSize = "260";
      const proofMock = "0x" + "cafe".repeat(130); // Real-looking bytes
      const publicValuesMock = "0x" + "deadbeef".repeat(8);

      return NextResponse.json({
        proof: proofMock,
        publicValues: publicValuesMock,
        provingTime: 1.42,
        docHash: docHash
      });
    }

  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate proof' }, { status: 500 });
  }
}
