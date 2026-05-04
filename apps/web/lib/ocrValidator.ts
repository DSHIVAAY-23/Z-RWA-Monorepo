/**
 * OCR Field Validation — Z-RWA Compliance
 *
 * Validates Tesseract OCR output against Indian ID document standards.
 * Runs 3 layers of checks in order:
 *   1. Blocked keyword detection (sample/demo/test documents)
 *   2. Document type + Verhoeff checksum (Aadhaar only)
 *   3. Age sanity check (18 ≤ age ≤ 100)
 *
 * No data is stored or transmitted. All validation is client-side.
 */

export type DocType = 'aadhaar' | 'pan' | 'unknown';

export interface OcrValidationResult {
  valid: boolean;
  docType: DocType;
  extractedAge: number | null;
  reason?: string;
  /** 'blocked' | 'checksum' | 'age' | 'ocr' used for icon selection in UI */
  rejectionLayer?: 'blocked' | 'checksum' | 'age' | 'ocr';
}

// ── Blocked keywords (Layer 1) ────────────────────────────────────────────────
const BLOCKED_KEYWORDS = [
  'SAMPLE', 'NOT VALID', 'DEMO ONLY', 'TEST ONLY',
  'TESTING ONLY', 'SPECIMEN', 'FOR TESTING', 'DEMO DOCUMENT',
  'SAMPLE DOCUMENT', 'NOT A VALID',
];

// ── Document patterns ─────────────────────────────────────────────────────────
const PAN_NUMBER_REGEX  = /[A-Z]{5}[0-9]{4}[A-Z]{1}/;
const AADHAAR_NUM_REGEX = /\d{4}\s?\d{4}\s?\d{4}/;
const DOB_REGEX         = /(\d{2})[\/\-\.](\d{2})[\/\-\.](\d{4})/;

const PAN_KEYWORDS = ['INCOME TAX', 'PERMANENT ACCOUNT', 'GOVT OF INDIA'];
const AADHAAR_KEYWORDS = [
  'AADHAAR',
  'UIDAI',
  'UNIQUE IDENTIFICATION',
  'GOVERNMENT OF INDIA', // Front face header
  'GOVT OF INDIA',
  'VID',                 // VID number line on Aadhaar front
  'MERA AADHAAR',
  'BHARAT SARKAR',       // भारत सरकार transliterated
  'DOB',                 // "DOB:" appears on Aadhaar front
];

// ── Helpers ────────────────────────────────────────────────────────────────────
function containsKeyword(text: string, keywords: string[]): boolean {
  return keywords.some((kw) => text.includes(kw));
}

function extractAge(rawText: string): number | null {
  const match = rawText.match(DOB_REGEX);
  if (!match) return null;

  const day   = parseInt(match[1], 10);
  const month = parseInt(match[2], 10);
  const year  = parseInt(match[3], 10);

  if (year < 1900 || year > new Date().getFullYear()) return null;
  if (month < 1 || month > 12) return null;
  if (day < 1 || day > 31) return null;

  const dob   = new Date(year, month - 1, day);
  const today = new Date();
  let age = today.getFullYear() - dob.getFullYear();
  const m = today.getMonth() - dob.getMonth();
  if (m < 0 || (m === 0 && today.getDate() < dob.getDate())) age -= 1;

  return age;
}

// ── Layer 2: Verhoeff checksum ────────────────────────────────────────────────

/**
 * verhoeffCheck — validates an Aadhaar number using the Verhoeff algorithm.
 *
 * The Verhoeff algorithm is a checksum method based on the dihedral group D5.
 * UIDAI uses it to ensure every valid 12-digit Aadhaar number satisfies a
 * mathematical property, making random number guessing detectable.
 *
 * Rules enforced:
 *  - Input must be exactly 12 digits after stripping spaces/dashes
 *  - First digit cannot be 0 or 1 (UIDAI allocation rule)
 *  - The checksum digit (last digit) must produce c === 0
 *
 * @param aadhaarRaw - Raw Aadhaar string (may include spaces or dashes)
 * @returns true if checksum is valid, false otherwise
 */
export function verhoeffCheck(aadhaarRaw: string): boolean {
  // Verhoeff multiplication table
  const d: number[][] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
    [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
    [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
    [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
    [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
  ];
  // Verhoeff permutation table
  const p: number[][] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
    [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
    [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
    [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
    [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
    [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
    [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
  ];

  const digits = aadhaarRaw.replace(/\D/g, '');

  // Must be exactly 12 digits
  if (digits.length !== 12) return false;

  // First digit cannot be 0 or 1 (UIDAI allocation rule)
  if (['0', '1'].includes(digits[0])) return false;

  // Verhoeff algorithm — reverse traversal
  const arr = digits.split('').reverse().map(Number);
  let c = 0;
  arr.forEach((digit, i) => {
    c = d[c][p[i % 8][digit]];
  });

  return c === 0;
}

// ── Main export ───────────────────────────────────────────────────────────────
export function validateExtractedFields(ocrText: string): OcrValidationResult {
  const upper   = ocrText.toUpperCase();
  const compact = upper.replace(/\s+/g, '');

  // ── LAYER 1: Block sample/demo/test documents ─────────────────────────────
  const foundBlockedKeyword = BLOCKED_KEYWORDS.find((kw) => upper.includes(kw));
  if (foundBlockedKeyword) {
    return {
      valid: false,
      docType: 'unknown',
      extractedAge: null,
      rejectionLayer: 'blocked',
      reason: `Document rejected: contains "${foundBlockedKeyword}". Sample/demo documents are not accepted. Please upload a real Aadhaar or PAN card.`,
    };
  }

  // ── Document type detection ───────────────────────────────────────────────
  const hasPanNumber  = PAN_NUMBER_REGEX.test(compact);
  const hasAadhaarNum = AADHAAR_NUM_REGEX.test(upper);
  const hasPanKeyword = containsKeyword(upper, PAN_KEYWORDS);
  const hasAadhaarKw  = containsKeyword(upper, AADHAAR_KEYWORDS);

  const isPan     = hasPanNumber && hasPanKeyword;
  const isAadhaar = hasAadhaarNum && hasAadhaarKw;

  if (!isPan && !isAadhaar) {
    if (hasPanNumber && !hasPanKeyword) {
      return {
        valid: false,
        docType: 'unknown',
        extractedAge: null,
        rejectionLayer: 'ocr',
        reason: 'PAN number pattern found but issuing authority text missing. Please upload a complete, unobstructed PAN card scan.',
      };
    }
    if (hasAadhaarNum && !hasAadhaarKw) {
      return {
        valid: false,
        docType: 'unknown',
        extractedAge: null,
        rejectionLayer: 'ocr',
        reason: 'Aadhaar number found but header text missing. Please upload a complete, unobstructed Aadhaar scan.',
      };
    }
    return {
      valid: false,
      docType: 'unknown',
      extractedAge: null,
      rejectionLayer: 'ocr',
      reason: 'No valid Indian ID document detected. Please upload a clear Aadhaar or PAN card image.',
    };
  }

  const docType: DocType = isPan ? 'pan' : 'aadhaar';

  // ── LAYER 2: Verhoeff checksum (Aadhaar only) ─────────────────────────────
  if (isAadhaar) {
    const aadhaarMatch = upper.match(AADHAAR_NUM_REGEX);
    if (aadhaarMatch) {
      const isChecksumValid = verhoeffCheck(aadhaarMatch[0]);
      if (!isChecksumValid) {
        return {
          valid: false,
          docType: 'unknown',
          extractedAge: null,
          rejectionLayer: 'checksum',
          reason: 'Invalid Aadhaar number — checksum verification failed. Please upload a valid Aadhaar card.',
        };
      }
    }
  }

  // ── Age extraction ────────────────────────────────────────────────────────
  const extractedAge = extractAge(upper);

  if (docType === 'aadhaar' && extractedAge === null) {
    return {
      valid: false,
      docType: 'aadhaar',
      extractedAge: null,
      rejectionLayer: 'ocr',
      reason: 'Aadhaar detected but date of birth could not be read. Please ensure the DOB field is clearly visible.',
    };
  }

  // ── LAYER 3: Age sanity check ─────────────────────────────────────────────
  if (extractedAge !== null) {
    if (extractedAge < 18) {
      return {
        valid: false,
        docType: 'unknown',
        extractedAge,
        rejectionLayer: 'age',
        reason: 'Age verification failed — must be 18 or older for compliance proof.',
      };
    }
    if (extractedAge > 100) {
      return {
        valid: false,
        docType: 'unknown',
        extractedAge: null,
        rejectionLayer: 'age',
        reason: 'Invalid date of birth detected. Please upload a valid document.',
      };
    }
  }

  return {
    valid: true,
    docType,
    extractedAge,
  };
}
