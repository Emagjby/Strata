import { blake3 } from "@noble/hashes/blake3.js";
import { encodeValue } from "./encode.js";
import { Value } from "./value.js";

export function hashBytes(bytes: Uint8Array): Uint8Array {
    return blake3.create({ dkLen: 32 }).update(bytes).digest();
}

export function hashValue(value: Value): Uint8Array {
    const encoded = encodeValue(value);
    return hashBytes(encoded);
}

export function toHex(bytes: Uint8Array): string {
    let out = "";
    for (const byte of bytes) {
        out += byte.toString(16).padStart(2, "0");
    }
    return out;
}

export function hashValueHex(value: Value): string {
    const hash = hashValue(value);
    return toHex(hash);
}
