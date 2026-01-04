import { Value } from "./value.js";
import { encodeULEB128, encodeSLEB128 } from "./varint.js";

const te = new TextEncoder();

const TAG_NULL = 0x00;
const TAG_FALSE = 0x01;
const TAG_TRUE = 0x02;

const TAG_INT = 0x10;
const TAG_STRING = 0x20;
const TAG_BYTES = 0x21;

const TAG_LIST = 0x30;
const TAG_MAP = 0x40;

export function encodeValue(value: Value): Uint8Array {
    const out: number[] = [];
    encodeInto(value, out);
    return Uint8Array.from(out);
}

function encodeInto(value: Value, out: number[]): void {
    switch (value.kind) {
        case "null": {
            out.push(TAG_NULL);
            return;
        }

        case "bool": {
            out.push(value.value ? TAG_TRUE : TAG_FALSE);
            return;
        }

        case "int": {
            out.push(TAG_INT);
            pushBytes(out, encodeSLEB128(value.value));
            return;
        }

        case "string": {
            out.push(TAG_STRING);
            const encoded = te.encode(value.value);
            pushBytes(out, encodeULEB128(BigInt(encoded.length)));
            pushBytes(out, encoded);
            return;
        }

        case "bytes": {
            out.push(TAG_BYTES);
            pushBytes(out, encodeULEB128(BigInt(value.value.length)));
            pushBytes(out, value.value);
            return;
        }

        case "list": {
            out.push(TAG_LIST);
            pushBytes(out, encodeULEB128(BigInt(value.value.length)));
            for (const item of value.value) {
                encodeInto(item, out);
            }
            return;
        }

        case "map": {
            out.push(TAG_MAP);

            // Canonical ordering
            const entries = [...value.value.entries()];
            entries.sort(([a], [b]) => compareUtf8Bytes(a, b));

            pushBytes(out, encodeULEB128(BigInt(entries.length)));

            for (const [key, value] of entries) {
                // key encoded exactly like a String
                out.push(TAG_STRING);
                const keyBytes = te.encode(key);
                pushBytes(out, encodeULEB128(BigInt(keyBytes.length)));
                pushBytes(out, keyBytes);

                encodeInto(value, out);
            }

            return;
        }
    }
}

function pushBytes(out: number[], bytes: Uint8Array): void {
    for (const b of bytes) {
        out.push(b);
    }
}

// Compare keys by UTF-8 byte order (lexicographic)
function compareUtf8Bytes(a: string, b: string): number {
    const ab = te.encode(a);
    const bb = te.encode(b);

    const min = Math.min(ab.length, bb.length);

    for (let i = 0; i < min; i++) {
        const diff = ab[i]! - bb[i]!;
        if (diff !== 0) {
            return diff;
        }
    }

    return ab.length - bb.length;
}
