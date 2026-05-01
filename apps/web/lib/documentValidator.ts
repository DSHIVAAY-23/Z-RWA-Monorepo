/**
 * Layer 1 — File-level document validation
 * Validates file type, size, and image aspect ratio before OCR runs.
 * No data leaves the browser.
 */

export interface FileValidationResult {
  valid: boolean;
  reason?: string;
}

const ALLOWED_TYPES = ['image/jpeg', 'image/png', 'image/webp'];
const MIN_SIZE_BYTES = 10 * 1024;       // 10 KB
const MAX_SIZE_BYTES = 10 * 1024 * 1024; // 10 MB
const MIN_RATIO = 1.3;
const MAX_RATIO = 2.0;

function getImageAspectRatio(file: File): Promise<number> {
  return new Promise((resolve, reject) => {
    const url = URL.createObjectURL(file);
    const img = new Image();
    img.onload = () => {
      URL.revokeObjectURL(url);
      resolve(img.naturalWidth / img.naturalHeight);
    };
    img.onerror = () => {
      URL.revokeObjectURL(url);
      reject(new Error('Could not load image to check dimensions.'));
    };
    img.src = url;
  });
}

export async function validateDocumentFile(file: File): Promise<FileValidationResult> {
  // 1. Type check
  if (!ALLOWED_TYPES.includes(file.type)) {
    return {
      valid: false,
      reason: `Unsupported file type "${file.type}". Please upload a JPEG, PNG, or WebP image.`,
    };
  }

  // 2. Size check
  if (file.size < MIN_SIZE_BYTES) {
    return {
      valid: false,
      reason: 'File is too small (< 10 KB). Please upload a clear, full-resolution scan of your ID.',
    };
  }
  if (file.size > MAX_SIZE_BYTES) {
    return {
      valid: false,
      reason: 'File is too large (> 10 MB). Please compress the image and try again.',
    };
  }

  // 3. Aspect ratio check (ID cards are landscape, ratio ~1.5:1 to 1.9:1)
  try {
    const ratio = await getImageAspectRatio(file);
    if (ratio < MIN_RATIO || ratio > MAX_RATIO) {
      return {
        valid: false,
        reason: `Image dimensions don't match a standard ID card shape (ratio ${ratio.toFixed(2)}:1). Please upload a properly cropped ID scan.`,
      };
    }
  } catch {
    return { valid: false, reason: 'Could not validate image dimensions.' };
  }

  return { valid: true };
}
