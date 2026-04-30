const fs = require('fs');

const vk = JSON.parse(fs.readFileSync('build/verification_key.json'));

function toU8ArrayString(x) {
    let hex = BigInt(x).toString(16).padStart(64, '0');
    let bytes = [];
    for (let i = 0; i < 32; i++) {
        bytes.push('0x' + hex.slice(i * 2, i * 2 + 2));
    }
    return `[${bytes.join(', ')}]`;
}

function toU8Array64String(arr) {
    let hex0 = BigInt(arr[0]).toString(16).padStart(64, '0');
    let hex1 = BigInt(arr[1]).toString(16).padStart(64, '0');
    let bytes = [];
    for (let i = 0; i < 32; i++) { bytes.push('0x' + hex0.slice(i * 2, i * 2 + 2)); }
    for (let i = 0; i < 32; i++) { bytes.push('0x' + hex1.slice(i * 2, i * 2 + 2)); }
    return `[\n        ${bytes.join(', ')}\n    ]`;
}

function toU8Array128String(arr) {
    let hex01 = BigInt(arr[0][1]).toString(16).padStart(64, '0');
    let hex00 = BigInt(arr[0][0]).toString(16).padStart(64, '0');
    let hex11 = BigInt(arr[1][1]).toString(16).padStart(64, '0');
    let hex10 = BigInt(arr[1][0]).toString(16).padStart(64, '0');
    let bytes = [];
    for (let i = 0; i < 32; i++) { bytes.push('0x' + hex01.slice(i * 2, i * 2 + 2)); }
    for (let i = 0; i < 32; i++) { bytes.push('0x' + hex00.slice(i * 2, i * 2 + 2)); }
    for (let i = 0; i < 32; i++) { bytes.push('0x' + hex11.slice(i * 2, i * 2 + 2)); }
    for (let i = 0; i < 32; i++) { bytes.push('0x' + hex10.slice(i * 2, i * 2 + 2)); }
    return `[\n        ${bytes.join(', ')}\n    ]`;
}

let out = `
use groth16_solana::groth16::Groth16Verifyingkey;

pub const VERIFYING_KEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: ${vk.IC.length},
    vk_alpha_g1: ${toU8Array64String(vk.vk_alpha_1)},
    vk_beta_g2: ${toU8Array128String(vk.vk_beta_2)},
    vk_gamma_g2: ${toU8Array128String(vk.vk_gamma_2)},
    vk_delta_g2: ${toU8Array128String(vk.vk_delta_2)},
    vk_ic: &[
`;

for (let i = 0; i < vk.IC.length; i++) {
    out += `        ${toU8Array64String(vk.IC[i])},\n`;
}

out += `    ]
};
`;

fs.writeFileSync('build/vk.rs', out);
console.log('Done!');
