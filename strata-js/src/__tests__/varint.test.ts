import test from "node:test";
import assert from "node:assert/strict";
import {
    encodeULEB128,
    decodeULEB128,
    encodeSLEB128,
    decodeSLEB128,
} from "../varint.js";

test("ULEB128 basic values", () => {
    assert.deepEqual(encodeULEB128(0n), Uint8Array.from([0x00]));
    assert.deepEqual(encodeULEB128(1n), Uint8Array.from([0x01]));
    assert.deepEqual(encodeULEB128(127n), Uint8Array.from([0x7f]));
    assert.deepEqual(encodeULEB128(128n), Uint8Array.from([0x80, 0x01]));
});

test("ULEB128 round trip", () => {
    const values = [0n, 1n, 127n, 128n, 1024n, 9007199254740993n];

    for (const value of values) {
        const encoded = encodeULEB128(value);
        const decoded = decodeULEB128(encoded);
        assert.equal(decoded.value, value);
        assert.equal(decoded.nextOffset, encoded.length);
    }
});

test("SLEB128 basic values", () => {
    const values = [
        0n,
        1n,
        -1n,
        63n,
        -64n,
        127n,
        -128n,
        9007199254740993n,
        -9007199254740993n,
    ];

    for (const value of values) {
        const encoded = encodeSLEB128(value);
        const decoded = decodeSLEB128(encoded);
        assert.equal(decoded.value, value);
        assert.equal(decoded.nextOffset, encoded.length);
    }
});
