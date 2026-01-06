import { encodeValue, decodeValue, hashValue } from "@emagjby/strata-js";

const res = await fetch("http://localhost:3000/payload");

const contentType = res.headers.get("Content-Type");
if (contentType !== "application/strata") {
    throw new Error(
        "Northstar T2 failed: Content-Type is not application/strata",
    );
}

const buffer = await res.arrayBuffer();
const raw = new Uint8Array(buffer);

if (raw.length < 1) {
    throw new Error("Northstar T2 failed: Empty payload");
}

const decoded = decodeValue(raw);
const reEncoded = encodeValue(decoded);

const hashOriginal = hashValue(raw);
const hashReEncoded = hashValue(reEncoded);

if (hashOriginal.length !== hashReEncoded.length) {
    throw new Error(
        "Northstar T2 failed: Hash length mismatch between original and re-encoded",
    );
}

for (let i = 0; i < hashOriginal.length; i++) {
    if (hashOriginal[i] !== hashReEncoded[i]) {
        throw new Error(`Northstar T2 failed: Hash mismatch at byte ${i}`);
    }
}

console.log("Northstar T2 passed");
