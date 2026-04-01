/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/z_rwa.json`.
 */
export type ZRwa = {
  "address": "3SN3zAmuW5HWgJy5mcWjvy8vwDZRLosEajqydbuxiEZC",
  "metadata": {
    "name": "zRwa",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "verifyAndMint",
      "discriminator": [
        18,
        182,
        110,
        12,
        166,
        47,
        186,
        41
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "destination",
          "writable": true
        },
        {
          "name": "mintAuthority",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  105,
                  110,
                  116,
                  95,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "docs": [
            "The generic Verifier program is often needed if sp1-solana calls a separate program.",
            "But sp1-solana 3.0 might use the precompile. We leave it out unless required."
          ],
          "address": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "proof",
          "type": "bytes"
        },
        {
          "name": "publicValues",
          "type": "bytes"
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidProof",
      "msg": "The provided SP1 proof is invalid."
    }
  ]
};
