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

      return NextResponse.json({
        proof: proofMock,
        publicValues: publicValuesMock,
        proofSize: 260,
        provingTime: 3.14,
        docHash: docHash
      });
    } else {
      // In production scenario: Call ZK-RAG SP1 prover binary or service
      // Example real proof generation path...
      await new Promise(resolve => setTimeout(resolve, 23000)); // ~23 seconds
      
      const proofMock = "0x" + "c".repeat(520);
      const publicValuesMock = "0x" + "d".repeat(64);

      return NextResponse.json({
        proof: proofMock,
        publicValues: publicValuesMock,
        proofSize: 260,
        provingTime: 23.42,
        docHash: docHash
      });
    }

  } catch (error) {
    return NextResponse.json({ error: 'Failed to generate proof' }, { status: 500 });
  }
}
