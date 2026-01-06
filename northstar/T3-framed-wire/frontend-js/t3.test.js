import { encodeValue, decodeValue, hashValue } from "@emagjby/strata-js";

const res = await fetch("http://localhost:3000/stream");

const contentType = res.headers.get("Content-Type");
if (contentType !== "application/strata") {
    throw new Error(
        "Northstar T3 failed: Content-Type is not application/strata",
    );
}

if (!res.body) {
    throw new Error("Northstar T3 failed: Response body is null");
}

const reader = res.body.getReader();

const MAX_FRAME_BYTES = 16 * 1024 * 1024;

let buffer = new Uint8Array(0);
let expectedLen = null;

function append(a, b) {
    const merged = new Uint8Array(a.length + b.length);
    merged.set(a);
    merged.set(b, a.length);
    return merged;
}

while (true) {
    const { value, done } = await reader.read();
    if (done) break;

    buffer = append(buffer, value);

    while (true) {
        if (expectedLen === null) {
            if (buffer.length < 4) break;

            const view = new DataView(
                buffer.buffer,
                buffer.byteOffset,
                buffer.byteLength,
            );

            const len = view.getUint32(0, false);

            if (len === 0) {
                throw new Error("Northstar T3 failed: Frame length is zero");
            }

            if (len > MAX_FRAME_BYTES) {
                throw new Error("Northstar T3 failed: Frame length exceeds maximum");
            }

            expectedLen = len;
            buffer = buffer.slice(4);
        }

        if (buffer.length < expectedLen) break;

        const payload = buffer.slice(0, expectedLen);
        buffer = buffer.slice(expectedLen);
        expectedLen = null;

        const decoded = decodeValue(payload);
        const reEncoded = encodeValue(decoded);

        const hashOriginal = hashValue(payload);
        const hashReEncoded = hashValue(reEncoded);

        if (hashOriginal.length !== hashReEncoded.length) {
            throw new Error("Northstar T3 failed: Hash lengths do not match");
        }

        for (let i = 0; i < hashOriginal.length; i++) {
            if (hashOriginal[i] !== hashReEncoded[i]) {
                throw new Error("Northstar T3 failed: Hash mismatch at byte " + i);
            }
        }
    }
}

if (expectedLen !== null || buffer.length !== 0) {
    throw new Error("Northstar T3 failed: Truncated frame at end of stream");
}

console.log("Northstar T3 passed");
