import path from 'path';
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
    const bundledHome = process.env.VERCEL === '1' ? '/tmp/qvac' : path.join(process.cwd(), 'qvac-data');
    
    console.log(`[QVAC] Ensuring models are in ${bundledHome}...`);
    const fs = require('fs');
    const https = require('https');
    
    const modelsToDownload = [
      { 
        name: '7c3f97207b725d40_recognizer_latin.onnx', 
        url: 'https://raw.githubusercontent.com/DSHIVAAY-23/Z-RWA-Monorepo/b35fb170ff4a620cb9e7007c468474001226a554/apps/web/qvac-data/.qvac/models/7c3f97207b725d40_recognizer_latin.onnx' 
      },
      { 
        name: 'e5341e191b8b1ea6_detector_craft.onnx', 
        url: 'https://raw.githubusercontent.com/DSHIVAAY-23/Z-RWA-Monorepo/b35fb170ff4a620cb9e7007c468474001226a554/apps/web/qvac-data/.qvac/models/e5341e191b8b1ea6_detector_craft.onnx' 
      }
    ];

    const modelDir = path.join(bundledHome, '.qvac', 'models');
    if (!fs.existsSync(modelDir)) fs.mkdirSync(modelDir, { recursive: true });

    for (const model of modelsToDownload) {
      const dest = path.join(modelDir, model.name);
      if (!fs.existsSync(dest)) {
        console.log(`[QVAC] Downloading ${model.name} to ${dest}...`);
        const startTime = Date.now();
        await new Promise((resolve, reject) => {
          const file = fs.createWriteStream(dest);
          const request = https.get(model.url, { timeout: 10000 }, (response) => {
            if (response.statusCode !== 200) {
              reject(new Error(`Failed to download ${model.name}: ${response.statusCode}`));
              return;
            }
            
            let downloaded = 0;
            const total = parseInt(response.headers['content-length'] || '0', 10);
            
            response.on('data', (chunk) => {
              downloaded += chunk.length;
              if (Date.now() % 1000 < 100) { // Log roughly every second
                console.log(`[QVAC] Progress for ${model.name}: ${((downloaded / total) * 100).toFixed(1)}%`);
              }
            });

            response.pipe(file);
            file.on('finish', () => { 
              file.close(); 
              console.log(`[QVAC] Finished ${model.name} in ${Date.now() - startTime}ms`);
              resolve(true); 
            });
          });
          
          request.on('error', (err) => { 
            fs.unlink(dest, () => {}); 
            reject(err); 
          });
          
          request.on('timeout', () => {
            request.destroy();
            reject(new Error(`Download timeout for ${model.name}`));
          });
        });
      }
    }

    console.log(`[QVAC] Setting HOME to: ${bundledHome}`);
    process.env.HOME = bundledHome;
    
    // Ensure bare binary is in PATH
    const projectRoot = process.cwd();
    const bareBinPath = path.join(projectRoot, 'node_modules', 'bare-runtime', 'bin');
    process.env.PATH = `${bareBinPath}:${process.env.PATH}`;
    console.log(`[QVAC] Updated PATH with: ${bareBinPath}`);
    
    console.log('[QVAC] Loading OCR model (Latin + CRAFT Detector)...');
    try {
      const modelId = await loadModel({
        modelSrc: OCR_LATIN_RECOGNIZER_1.src,
        modelType: 'onnx-ocr',
        modelConfig: {
          detectorModelSrc: OCR_CRAFT_DETECTOR.src
        }
      });
      
      console.log(`[QVAC] Engine ready in ${Date.now() - start}ms (Model ID: ${modelId})`);
      return modelId;
    } catch (err) {
      console.error('[QVAC] Failed to load model:', err);
      throw err;
    }
  })();

  return providerInitPromise;
}

export type QVACOcrResult = {
  text: string;
  confidence: number;
  engine: 'qvac-local';
  blocks: OCRTextBlock[];
};

export async function extractTextFromDocument(
  imageBuffer: Buffer
): Promise<QVACOcrResult> {
  try {
    const modelId = await ensureProvider();
    
    // Create a temporary file for the image
    const tempImagePath = path.join(process.env.HOME || '/tmp', `ocr_${Date.now()}.jpg`);
    const fs = require('fs');
    fs.writeFileSync(tempImagePath, imageBuffer);

    console.log(`[QVAC] Starting OCR on ${tempImagePath}...`);
    const { blocks: blocksPromise } = ocr({
      modelId: modelId,
      image: tempImagePath,
    });

    const blocks = await blocksPromise;
    
    // Cleanup temp image
    try { fs.unlinkSync(tempImagePath); } catch (e) {}

    const fullText = blocks.map((b) => b.text).join('\n');
    const avgConfidence = blocks.length > 0 
      ? blocks.reduce((acc, b) => acc + b.confidence, 0) / blocks.length 
      : 0;

    return {
      text: fullText,
      confidence: avgConfidence,
      engine: 'qvac-local',
      blocks,
    };
  } catch (error) {
    console.error('[QVAC] OCR Error:', error);
    throw error;
  }
}
