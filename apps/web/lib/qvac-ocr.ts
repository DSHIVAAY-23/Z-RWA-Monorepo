import {
  ocr,
  startQVACProvider,
  type OCRTextBlock,
} from '@qvac/sdk';

const MODEL_ID = process.env.QVAC_OCR_MODEL_ID ?? 'qvac-ocr-latin';

let providerReady = false;

async function ensureProvider(): Promise<void> {
  if (providerReady) return;
  await startQVACProvider();
  providerReady = true;
}

export type QVACOcrResult = {
  text: string;
  confidence: number;
  engine: 'qvac-local';
  blocks: OCRTextBlock[];
};

/**
 * Extracts text from a document image using the QVAC OCR engine (server-side).
 * Requires QVAC provider process running on the server — see QVAC_OCR_MODEL_ID env var.
 *
 * Privacy note: image bytes are processed entirely within this server process.
 * No data is forwarded to any third-party API.
 */
export async function extractTextFromDocument(
  image: Buffer | string
): Promise<QVACOcrResult> {
  console.log('[QVAC] Local OCR engine — zero data transmitted');

  await ensureProvider();

  const { blocks: blocksPromise } = ocr({
    modelId: MODEL_ID,
    image,
    options: { paragraph: false },
  });

  const blocks = await blocksPromise;

  const text = blocks.map((b) => b.text).join('\n');
  const avgConfidence =
    blocks.length > 0
      ? blocks.reduce((sum, b) => sum + (b.confidence ?? 1), 0) / blocks.length
      : 0;

  return {
    text,
    confidence: avgConfidence,
    engine: 'qvac-local',
    blocks,
  };
}
