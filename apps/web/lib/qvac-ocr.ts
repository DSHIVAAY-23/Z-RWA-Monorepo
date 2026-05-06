import {
  ocr,
  startQVACProvider,
  loadModel,
  OCR_LATIN_RECOGNIZER_1,
  OCR_CRAFT_DETECTOR,
  type OCRTextBlock,
} from '@qvac/sdk';

let providerInitPromise: Promise<string> | null = null;

async function ensureProvider(): Promise<string> {
  if (providerInitPromise) return providerInitPromise;

  providerInitPromise = (async () => {
    const start = Date.now();
    console.log('[QVAC] Initializing local AI engine...');
    
    // We skip startQVACProvider() because it triggers DHT bootstrapping, 
    // which is slow and not needed for local-only inference.
    // getRPC() is handled internally by loadModel().
    
    console.log('[QVAC] Loading OCR model (Latin + CRAFT Detector)...');
    const modelId = await loadModel({
      modelSrc: OCR_LATIN_RECOGNIZER_1.src,
      modelType: 'onnx-ocr',
      modelConfig: {
        detectorModelSrc: OCR_CRAFT_DETECTOR.src
      }
    });
    
    console.log(`[QVAC] Engine ready in ${Date.now() - start}ms (Model ID: ${modelId})`);
    return modelId;
  })();

  return providerInitPromise;
}

export type QVACOcrResult = {
  text: string;
  confidence: number;
  engine: 'qvac-local';
  blocks: OCRTextBlock[];
};

/**
 * Extracts text from a document image using the QVAC OCR engine (server-side).
 * Performs local inference using ONNX models (Latin Recognizer + CRAFT Detector).
 *
 * Privacy note: image bytes are processed entirely within this server process.
 * No data is forwarded to any third-party API.
 */
export async function extractTextFromDocument(
  image: Buffer | string
): Promise<QVACOcrResult> {
  console.log('[QVAC] Local OCR engine — zero data transmitted');

  const modelId = await ensureProvider();

  const { blocks: blocksPromise } = ocr({
    modelId: modelId,
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
