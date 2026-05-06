import { NextRequest, NextResponse } from 'next/server';
import { extractTextFromDocument } from '../../../lib/qvac-ocr';

// Must be nodejs runtime — QVAC native binaries are not Edge-compatible
export const runtime = 'nodejs';

export async function POST(request: NextRequest) {
  try {
    const formData = await request.formData();
    const file = formData.get('image') as File | null;

    if (!file) {
      return NextResponse.json({ error: 'No image provided' }, { status: 400 });
    }

    const arrayBuffer = await file.arrayBuffer();
    const buffer = Buffer.from(arrayBuffer);

    // Primary: QVAC server-side OCR (zero external data transmission)
    // Race with a 5s timeout — if QVAC provider isn't running it hangs rather than throwing
    try {
      const qvacTimeout = new Promise<never>((_, reject) =>
        setTimeout(() => reject(new Error('QVAC provider timeout')), 5000)
      );
      const result = await Promise.race([extractTextFromDocument(buffer), qvacTimeout]);
      return NextResponse.json({
        text: result.text,
        confidence: result.confidence,
        engine: result.engine,
      });
    } catch (qvacErr: any) {
      // QVAC provider not running on this host — fall back to Tesseract (server-side)
      console.warn('[QVAC] Provider unavailable, falling back to Tesseract:', qvacErr?.message);

      const { createWorker } = await import('tesseract.js');
      const worker = await createWorker('eng');
      const {
        data: { text, confidence },
      } = await worker.recognize(buffer);
      await worker.terminate();

      return NextResponse.json({
        text,
        confidence,
        engine: 'tesseract-fallback',
      });
    }
  } catch (err: any) {
    console.error('[OCR] Fatal error:', err?.message);
    return NextResponse.json({ error: err?.message ?? 'OCR failed' }, { status: 500 });
  }
}
