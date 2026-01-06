import { encodeValue, decodeValue, hashValue } from "@emagjby/strata-js";

const res = await fetch("http://localhost:3000/payload");
const { bytes_base64, hash_hex } = await res.json();

const bytes = Uint8Array.from(atob(bytes_base64), (c) => c.charCodeAt(0));
const decoded = decodeValue(bytes);
const reEncoded = encodeValue(decoded);
const reHashed = hashValue(reEncoded);

const hex = Array.from(reHashed)
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");

if (hex !== hash_hex) {
    throw new Error("Northstar T1 violated");
}

console.log("Northstar T1 passed");
