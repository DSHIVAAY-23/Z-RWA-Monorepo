// Utility functions to convert SnarkJS output to Solana groth16-solana byte arrays

function reverseEndianness(hexString: string): Uint8Array {
  const bytes = new Uint8Array(hexString.length / 2);
  for (let i = 0; i < hexString.length; i += 2) {
    bytes[i / 2] = parseInt(hexString.substr(i, 2), 16);
  }
  return bytes.reverse();
}

function to32ByteBuffer(bigIntStr: string | bigint): Uint8Array {
  // Convert to BigInt, then to hex string, pad to 64 chars (32 bytes)
  const hex = BigInt(bigIntStr).toString(16).padStart(64, '0');
  // groth16-solana uses big-endian layout for bytes internally for these structures usually,
  // or it relies on arkworks which might be little-endian or big-endian.
  // Actually, groth16-solana (based on arkworks) expects big-endian bytes (or little-endian depending on the macro).
  // The standard Solana verifier expects big endian. Let's return big endian array.
  const bytes = new Uint8Array(32);
  for (let i = 0; i < 32; i++) {
    bytes[i] = parseInt(hex.slice(i * 2, i * 2 + 2), 16);
  }
  return bytes;
}

export function convertProofForSolana(proof: any) {
  // proof.pi_a is [x, y, 1]
  const proof_a = new Uint8Array(64);
  proof_a.set(to32ByteBuffer(proof.pi_a[0]), 0);
  proof_a.set(to32ByteBuffer(proof.pi_a[1]), 32);

  // proof.pi_b is [[x1, x2], [y1, y2], [1, 0]]
  // SnarkJS outputs [x2, x1], we must be careful with G2 ordering.
  // Standard solidity/solana exports expect: x[1], x[0], y[1], y[0] or similar.
  const proof_b = new Uint8Array(128);
  proof_b.set(to32ByteBuffer(proof.pi_b[0][1]), 0);
  proof_b.set(to32ByteBuffer(proof.pi_b[0][0]), 32);
  proof_b.set(to32ByteBuffer(proof.pi_b[1][1]), 64);
  proof_b.set(to32ByteBuffer(proof.pi_b[1][0]), 96);

  // proof.pi_c is [x, y, 1]
  const proof_c = new Uint8Array(64);
  proof_c.set(to32ByteBuffer(proof.pi_c[0]), 0);
  proof_c.set(to32ByteBuffer(proof.pi_c[1]), 32);

  return { proof_a, proof_b, proof_c };
}

export function convertPublicSignalsForSolana(publicSignals: string[]) {
  // Returns an array of 32-byte arrays
  return publicSignals.map((signal) => to32ByteBuffer(signal));
}
