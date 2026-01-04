import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { V } from "../value_factory.js";
import { hashValueHex } from "../hash.js";
import { Value } from "../value.js";

const VECTORS_DIR = join(process.cwd(), "..", "vectors");

function readText(path: string): string {
    return readFileSync(path, "utf8").trim();
}

test("hash matches v1/01-basic.hash.hex", () => {
    const config = V.map([
        ["enabled", V.bool(true)],
        ["retries", V.int(3n)],
        ["name", V.string("strata")],
        ["empty", V.null()],
    ]);

    const root: Value = V.map([["config", config]]);

    const expected = readText(join(VECTORS_DIR, "v1", "01-basic.hash.hex"));
    const actual = hashValueHex(root);

    assert.equal(actual, expected);
});

test("hash matches v1/02-map-order.hash.hex", () => {
    const data = V.map([
        ["z", V.int(1n)],
        ["a", V.int(2n)],
        ["m", V.int(3n)],
    ]);

    const root: Value = V.map([["data", data]]);

    const expected = readText(join(VECTORS_DIR, "v1", "02-map-order.hash.hex"));
    const actual = hashValueHex(root);

    assert.equal(actual, expected);
});

test("hash matches v1/03-bigint-bytes.hash.hex", () => {
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

    const expected = readText(
        join(VECTORS_DIR, "v1", "03-bigint-bytes.hash.hex"),
    );
    const actual = hashValueHex(root);

    assert.equal(actual, expected);
});
