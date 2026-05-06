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
    // Race with a 2.5s timeout — if QVAC provider isn't running it hangs rather than throwing
    const start = Date.now();
    try {
      const qvacTimeout = new Promise<never>((_, reject) =>
        setTimeout(() => reject(new Error('QVAC engine timeout')), 30000)
      );
      const result = await Promise.race([extractTextFromDocument(buffer), qvacTimeout]);
      console.log(`[QVAC] Success in ${Date.now() - start}ms`);
      return NextResponse.json({
        text: result.text,
        confidence: result.confidence,
        engine: result.engine,
      });
    } catch (qvacErr: any) {
      // QVAC provider not running on this host — fall back to Tesseract (server-side)
      console.warn('[QVAC] Falling back to Tesseract:', qvacErr?.message);

      const tesseractStart = Date.now();
      const { createWorker } = await import('tesseract.js');
      const worker = await createWorker('eng');
      const {
        data: { text, confidence },
      } = await worker.recognize(buffer);
      await worker.terminate();

      console.log(`[OCR] Tesseract fallback complete in ${Date.now() - tesseractStart}ms`);

      return NextResponse.json({
        text,
        confidence,
        engine: 'tesseract-fallback',
      });
    }
  } catch (err: any) {
    console.error('[QVAC] Fatal error:', err?.message);
    return NextResponse.json({ error: err?.message ?? 'QVAC scanning failed' }, { status: 500 });
  }
}
