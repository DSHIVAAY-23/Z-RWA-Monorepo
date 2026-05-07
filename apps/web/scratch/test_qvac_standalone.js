
const { ocr, startQVACProvider, loadModel, OCR_LATIN_RECOGNIZER_1, OCR_CRAFT_DETECTOR } = require('@qvac/sdk');
const fs = require('fs');
const path = require('path');

async function test() {
  try {
    console.log('Starting QVAC provider...');
    await startQVACProvider();
    console.log('Provider started.');

    console.log('Loading OCR model with detector...');
    const modelId = await loadModel({
      modelSrc: OCR_LATIN_RECOGNIZER_1.src,
      modelType: 'ocr',
      modelConfig: {
        detectorModelSrc: OCR_CRAFT_DETECTOR.src
      }
    });
    console.log(`Model loaded with ID: ${modelId}`);

    const imagePath = path.join(__dirname, 'aadhaar.jpg');
    console.log(`Starting OCR on ${imagePath}...`);
    
    const { blocks } = ocr({
      modelId: modelId,
      image: imagePath,
    });

    console.log('Waiting for blocks...');
    const result = await blocks;
    console.log('OCR Result:', JSON.stringify(result, null, 2));
    
    process.exit(0);
  } catch (e) {
    console.error('OCR Error:', e);
    process.exit(1);
  }
}

test();
