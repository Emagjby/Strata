import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { encodeValue } from "../encode.js";
import { V } from "../value_factory.js";
import { Value } from "../value.js";

const VECTORS_DIR = join(process.cwd(), "..", "vectors");

function readHex(path: string): Uint8Array {
    const text = readFileSync(path, "utf-8").trim();
    assert.equal(text.length % 2, 0, "Hex string must have even length");

    const out = new Uint8Array(text.length / 2);
    for (let i = 0; i < text.length; i += 2) {
        out[i / 2] = Number.parseInt(text.slice(i, i + 2), 16);
    }
    return out;
}

test("encode matches v1/01-basic.scb.hex", () => {
    const config = V.map([
        ["enabled", V.bool(true)],
        ["retries", V.int(3n)],
        ["name", V.string("strata")],
        ["empty", V.null()],
    ]);

    const root: Value = V.map([["config", config]]);

    const expected = readHex(join(VECTORS_DIR, "v1", "01-basic.scb.hex"));
    const actual = encodeValue(root);

    assert.deepEqual([...actual], [...expected]);
});

test("encode matches v1/03-bigint-bytes.scb.hex", () => {
    const avatar = new Uint8Array([
        0x9f, 0x86, 0xd0, 0x81, 0x88, 0x4c, 0x7d, 0x65, 0x9a, 0x2f, 0xea, 0xa0,
        0xc5, 0x5a, 0xd0, 0x15,
    ]);

    const profile = V.map([
        ["id", V.int(9007199254740993n)],
        ["avatar_hash", V.bytes(avatar)],
        [
            "tags",
            V.list([V.string("logistics"), V.string("state"), V.string("integrity")]),
        ],
    ]);

    const root: Value = V.map([["profile", profile]]);

    const expected = readHex(join(VECTORS_DIR, "v1", "03-bigint-bytes.scb.hex"));
    const actual = encodeValue(root);

    assert.deepEqual([...actual], [...expected]);
});
