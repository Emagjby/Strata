import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

import { encodeValue } from "../encode.js";
import { decodeValue } from "../decode.js";
import { hashValueHex } from "../hash.js";
import { parse } from "../parser.js";
import { DecodeError } from "../decode_error.js";
import { Value } from "../value.js";

const VECTORS_DIR = join(process.cwd(), "..", "vectors");

function readHex(path: string): Uint8Array {
    const text = readFileSync(path, "utf8").trim();
    assert.ok(text.length % 2 === 0, "Hex string must have even length");

    const out = new Uint8Array(text.length / 2);
    for (let i = 0; i < text.length; i += 2) {
        out[i / 2] = Number.parseInt(text.slice(i, i + 2), 16);
    }
    return out;
}

function readText(path: string): string {
    return readFileSync(path, "utf8");
}

function readJson(path: string): any {
    return JSON.parse(readFileSync(path, "utf8"));
}

function runPositiveVector(relPath: string) {
    const base = join(VECTORS_DIR, relPath);

    const stPath = base + ".st";
    const scbHexPath = base + ".scb.hex";
    const hashHexPath = base + ".hash.hex";

    const source = readText(stPath);

    const parsedValue: Value = parse(source);

    const encoded = encodeValue(parsedValue);

    const expectedScb = readHex(scbHexPath);

    assert.deepEqual(
        [...encoded],
        [...expectedScb],
        `SCB mismatch for vector ${relPath}`,
    );

    //hash
    const actualHash = hashValueHex(parsedValue);
    const expectedHash = readText(hashHexPath).trim();

    assert.equal(actualHash, expectedHash, `Hash mismatch for vector ${relPath}`);
}

function runNegativeDecodeVector(relPath: string) {
    const base = join(VECTORS_DIR, relPath);

    const hexPath = base + ".hex";
    const errorPath = base + ".error.json";

    const bytes = readHex(hexPath);
    const expectedError = readJson(errorPath);

    let thrown: unknown;

    try {
        decodeValue(bytes);
    } catch (err) {
        thrown = err;
    }

    assert.ok(
        thrown instanceof DecodeError,
        `Expected DecodeError for ${relPath}`,
    );

    const err = thrown as DecodeError;

    if (expectedError.kind === "InvalidTag") {
        assert.equal(err.kind.type, "InvalidTag");
    } else {
        assert.equal(err.kind.type, expectedError.kind);
    }

    assert.equal(
        err.offset,
        expectedError.offset,
        `offset mismatch for ${relPath}`,
    );
}

// v1
test("vector v1/01-basic", () => runPositiveVector("v1/01-basic"));
test("vector v1/02-map-order", () => runPositiveVector("v1/02-map-order"));
test("vector v1/03-bigint-bytes", () =>
    runPositiveVector("v1/03-bigint-bytes"));

// v2
test("vector v2/01-decode-roundtrip", () =>
    runPositiveVector("v2/01-decode-roundtrip"));

test("vector v2/02-noncanonical-map-order", () =>
    runPositiveVector("v2/02-noncanonical-map-order"));

test("vector v2/03-nested-structure", () =>
    runPositiveVector("v2/03-nested-structure"));

test("neg v2.1/01-invalid-tag", () =>
    runNegativeDecodeVector("v2.1/neg-01-invalid-tag"));

test("neg v2.1/02-truncated-string", () =>
    runNegativeDecodeVector("v2.1/neg-02-truncated-string"));

test("neg v2.1/03-varint-overflow", () =>
    runNegativeDecodeVector("v2.1/neg-03-varint-overflow"));

test("neg v2.1/04-invalid-utf8", () =>
    runNegativeDecodeVector("v2.1/neg-04-invalid-utf8"));
