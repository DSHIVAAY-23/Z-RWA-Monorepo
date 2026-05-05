import { NextResponse } from 'next/server';
import { paymentStore } from '../../../../lib/paymentStore';

export async function GET(request: Request, { params }: { params: { paymentId: string } }) {
  try {
    const paymentId = params.paymentId;
    if (!paymentId) return NextResponse.json({ error: "Missing paymentId" }, { status: 400 });

    const state = paymentStore.get(paymentId);
    if (!state) {
        return NextResponse.json({ status: "not_found" }, { status: 404 });
    }

    return NextResponse.json({
        status: state.status,
        proofHash: state.proofHash,
        txSignature: state.txSignature,
        tokenAddress: state.tokenAddress
    });
  } catch (error) {
      console.error(error);
      return NextResponse.json({ error: "Internal error" }, { status: 500 });
  }
}
