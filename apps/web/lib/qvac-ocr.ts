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
    
    // On Vercel, serverless functions have a 50MB size limit, so we cannot bundle 95MB models.
    // Instead, we download them from the repo to /tmp on the first request.
    const isVercel = process.env.VERCEL === '1';
    const bundledHome = isVercel ? '/tmp/qvac' : path.join(process.cwd(), 'qvac-data');
    
    if (isVercel) {
      console.log(`[QVAC] Vercel detected. Ensuring models are in ${bundledHome}...`);
      const fs = require('fs');
      const https = require('https');
      
      const modelsToDownload = [
        { 
          name: '7c3f97207b725d40_recognizer_latin.onnx', 
          url: 'https://raw.githubusercontent.com/DSHIVAAY-23/Z-RWA-Monorepo/main/apps/web/qvac-data/.qvac/models/7c3f97207b725d40_recognizer_latin.onnx' 
        },
        { 
          name: 'e5341e191b8b1ea6_detector_craft.onnx', 
          url: 'https://raw.githubusercontent.com/DSHIVAAY-23/Z-RWA-Monorepo/main/apps/web/qvac-data/.qvac/models/e5341e191b8b1ea6_detector_craft.onnx' 
        }
      ];

      const modelDir = path.join(bundledHome, '.qvac', 'models');
      if (!fs.existsSync(modelDir)) fs.mkdirSync(modelDir, { recursive: true });

      for (const model of modelsToDownload) {
        const dest = path.join(modelDir, model.name);
        if (!fs.existsSync(dest)) {
          console.log(`[QVAC] Downloading ${model.name}...`);
          await new Promise((resolve, reject) => {
            const file = fs.createWriteStream(dest);
            https.get(model.url, (response) => {
              response.pipe(file);
              file.on('finish', () => { file.close(); resolve(true); });
            }).on('error', (err) => { fs.unlink(dest); reject(err); });
          });
          console.log(`[QVAC] Downloaded ${model.name}`);
        }
      }
    }

    console.log(`[QVAC] Setting HOME to: ${bundledHome}`);
    process.env.HOME = bundledHome;
    
    // Ensure bare binary is in PATH
    const projectRoot = process.cwd();
    const bareBinPath = path.join(projectRoot, 'node_modules', 'bare-runtime', 'bin');
    process.env.PATH = `${bareBinPath}:${process.env.PATH}`;
    console.log(`[QVAC] Updated PATH with: ${bareBinPath}`);
    
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
