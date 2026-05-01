/**
 * Layer 2 — OCR field validation
 * Validates extracted text from Tesseract OCR against Indian ID document patterns.
 * Extracts DOB and calculates age. No data is stored or sent anywhere.
 */

export type DocType = 'aadhaar' | 'pan' | 'unknown';

export interface OcrValidationResult {
  valid: boolean;
  docType: DocType;
  extractedAge: number | null;
  reason?: string;
}

// ── Regex patterns ────────────────────────────────────────────────────────────
const PAN_NUMBER_REGEX   = /[A-Z]{5}[0-9]{4}[A-Z]{1}/;
const AADHAAR_NUM_REGEX  = /\d{4}\s?\d{4}\s?\d{4}/;
const DOB_REGEX          = /(\d{2})[\/\-\.](\d{2})[\/\-\.](\d{4})/;

const PAN_KEYWORDS    = ['INCOME TAX', 'PERMANENT ACCOUNT', 'GOVT OF INDIA'];
const AADHAAR_KEYWORDS = ['AADHAAR', 'UIDAI', 'UNIQUE IDENTIFICATION'];

// ── Helpers ────────────────────────────────────────────────────────────────────
function containsKeyword(text: string, keywords: string[]): boolean {
  return keywords.some((kw) => text.includes(kw));
}

function extractAge(rawText: string): number | null {
  // Try to find DOB pattern in original (non-stripped) text to preserve spaces
  const match = rawText.match(DOB_REGEX);
  if (!match) return null;

  const day   = parseInt(match[1], 10);
  const month = parseInt(match[2], 10);
  const year  = parseInt(match[3], 10);

  // Basic sanity check
  if (year < 1900 || year > new Date().getFullYear()) return null;
  if (month < 1 || month > 12) return null;
  if (day < 1 || day > 31) return null;

  const dob = new Date(year, month - 1, day);
  const today = new Date();
  let age = today.getFullYear() - dob.getFullYear();
  const m = today.getMonth() - dob.getMonth();
  if (m < 0 || (m === 0 && today.getDate() < dob.getDate())) age -= 1;

  return age;
}

// ── Main export ───────────────────────────────────────────────────────────────
export function validateExtractedFields(ocrText: string): OcrValidationResult {
  // Work with uppercase, preserve spaces for DOB regex
  const upper = ocrText.toUpperCase();
  // Compact version (no spaces) for pattern matching
  const compact = upper.replace(/\s+/g, '');

  const hasPanNumber    = PAN_NUMBER_REGEX.test(compact);
  const hasAadhaarNum   = AADHAAR_NUM_REGEX.test(upper);
  const hasPanKeyword   = containsKeyword(upper, PAN_KEYWORDS);
  const hasAadhaarKw    = containsKeyword(upper, AADHAAR_KEYWORDS);

  const isPan    = hasPanNumber && hasPanKeyword;
  const isAadhaar = hasAadhaarNum && hasAadhaarKw;

  // ── No valid Indian ID detected ───────────────────────────────────────────
  if (!isPan && !isAadhaar) {
    // Give a more specific reason
    if (hasPanNumber && !hasPanKeyword) {
      return {
        valid: false,
        docType: 'unknown',
        extractedAge: null,
        reason: 'PAN number pattern found but issuing authority text missing. Please upload a complete, unobstructed PAN card scan.',
      };
    }
    if (hasAadhaarNum && !hasAadhaarKw) {
      return {
        valid: false,
        docType: 'unknown',
        extractedAge: null,
        reason: 'Aadhaar number pattern found but UIDAI header text missing. Please upload a complete, unobstructed Aadhaar scan.',
      };
    }
    return {
      valid: false,
      docType: 'unknown',
      extractedAge: null,
      reason: 'No valid Indian ID document detected. Please upload a clear Aadhaar or PAN card image.',
    };
  }

  const docType: DocType = isPan ? 'pan' : 'aadhaar';

  // ── Age extraction ────────────────────────────────────────────────────────
  const extractedAge = extractAge(upper);

  // PAN cards don't always have DOB — allow null age for PAN (API will use default)
  if (docType === 'aadhaar' && extractedAge === null) {
    return {
      valid: false,
      docType: 'aadhaar',
      extractedAge: null,
      reason: 'Aadhaar detected but date of birth could not be read. Please ensure the DOB field is clearly visible.',
    };
  }

  if (extractedAge !== null && extractedAge < 18) {
    return {
      valid: false,
      docType,
      extractedAge,
      reason: `Age extracted as ${extractedAge}. Must be 18+ for RWA compliance proof.`,
    };
  }

  return {
    valid: true,
    docType,
    extractedAge,
  };
}
