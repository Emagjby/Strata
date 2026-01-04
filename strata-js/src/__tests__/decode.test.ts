import test from "node:test";
import assert from "node:assert/strict";
import { decodeValue } from "../decode.js";
import { encodeValue } from "../encode.js";
import { V } from "../value_factory.js";
import { DecodeError } from "../decode_error.js";

test("decode simple int", () => {
    const bytes = new Uint8Array([0x10, 0x01]);
    const v = decodeValue(bytes);
    assert.equal(v.kind, "int");
    assert.equal(v.value, 1n);
});

test("encode -> decode roundtrip", () => {
    const value = V.map([
        ["a", V.int(1n)],
        ["b", V.bool(true)],
        ["c", V.list([V.null(), V.string("hi")])],
    ]);

    const encoded = encodeValue(value);
    const decoded = decodeValue(encoded);

    assert.deepEqual(decoded, value);
});

test("invalid tag throws DecodeError", () => {
    const bytes = new Uint8Array([0xff]);

    assert.throws(
        () => decodeValue(bytes),
        (err: unknown) =>
            err instanceof DecodeError &&
            err.kind.type === "InvalidTag" &&
            err.offset === 0,
    );
});
