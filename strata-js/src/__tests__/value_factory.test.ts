import test from "node:test";
import assert from "node:assert/strict";

import {
    Value as FactoryValue,
    V,
    encodeValue,
    decodeValue,
    hashValue,
} from "../index.js";
import type { Value } from "../value.js";

test("old factory API (V.*) still works (regression)", () => {
    const v: Value = V.map([
        ["null", V.null()],
        ["bool", V.bool(true)],
        ["int", V.int(42n)],
        ["string", V.string("strata")],
        ["bytes", V.bytes(new Uint8Array([0xde, 0xad]))],
        ["list", V.list([V.int(1n), V.int(2n)])],
    ]);

    assert.equal(v.kind, "map");
    assert.equal(v.value.get("int")?.kind, "int");
});

test("Value is the primary factory export and V is an alias", () => {
    assert.equal(V, FactoryValue);

    const v = FactoryValue.int(1n);
    assert.deepEqual(v, { kind: "int", value: 1n });
});

test("listOf creates list values", () => {
    const v = FactoryValue.listOf(
        FactoryValue.int(1n),
        FactoryValue.int(2n),
        FactoryValue.int(3n),
    );

    assert.equal(v.kind, "list");
    assert.equal(v.value.length, 3);
    assert.deepEqual(v.value[0], FactoryValue.int(1n));
});

test("mapObj creates map values from object", () => {
    const v = FactoryValue.mapObj({
        a: FactoryValue.int(1n),
        b: FactoryValue.string("x"),
    });

    assert.equal(v.kind, "map");
    assert.deepEqual(v.value.get("a"), FactoryValue.int(1n));
    assert.deepEqual(v.value.get("b"), FactoryValue.string("x"));
});

test("mapOf supports duplicate keys with last-write-wins", () => {
    const v = FactoryValue.mapOf(
        ["a", FactoryValue.int(1n)],
        ["a", FactoryValue.int(2n)],
        ["b", FactoryValue.bool(true)],
    );

    assert.equal(v.kind, "map");
    assert.deepEqual(v.value.get("a"), FactoryValue.int(2n));
    assert.deepEqual(v.value.get("b"), FactoryValue.bool(true));
});

test("bytesFrom accepts common byte-oriented inputs", () => {
    const u8 = new Uint8Array([0xde, 0xad]);

    assert.deepEqual(Array.from(FactoryValue.bytesFrom(u8).value), [0xde, 0xad]);

    assert.deepEqual(
        Array.from(FactoryValue.bytesFrom(u8.buffer).value),
        [0xde, 0xad],
    );

    assert.deepEqual(
        Array.from(FactoryValue.bytesFrom([0xde, 0xad]).value),
        [0xde, 0xad],
    );
});

test("bytesFrom rejects invalid byte values", () => {
    assert.throws(() => FactoryValue.bytesFrom([256] as any));
    assert.throws(() => FactoryValue.bytesFrom([-1] as any));
    assert.throws(() => FactoryValue.bytesFrom([1.5] as any));
});

test("bytesHex parses strict hexadecimal", () => {
    const v = FactoryValue.bytesHex("deadBEEF");
    assert.deepEqual(Array.from(v.value), [0xde, 0xad, 0xbe, 0xef]);
});

test("bytesHex rejects invalid hexadecimal", () => {
    assert.throws(() => FactoryValue.bytesHex("abc" as any)); // odd length
    assert.throws(() => FactoryValue.bytesHex("zz" as any)); // non-hex
    assert.throws(() => FactoryValue.bytesHex("0x00" as any)); // prefix
});

test("invalid uses still fail (strictness preserved)", () => {
    // @ts-expect-error
    assert.throws(() => V.int(42));

    // @ts-expect-error
    assert.throws(() => V.bytes([1, 2, 3]));
});

test("encode/hash stability: helper-built values match manual equivalents", () => {
    const built: Value = FactoryValue.mapOf(
        ["a", FactoryValue.int(1n)],
        ["a", FactoryValue.int(2n)], // last wins
        ["b", FactoryValue.listOf(FactoryValue.int(3n), FactoryValue.int(4n))],
        ["c", FactoryValue.bytesHex("deadbeef")],
    );

    const manual: Value = V.map([
        ["a", V.int(2n)],
        ["b", V.list([V.int(3n), V.int(4n)])],
        ["c", V.bytes(new Uint8Array([0xde, 0xad, 0xbe, 0xef]))],
    ]);

    const builtBytes = encodeValue(built);
    const manualBytes = encodeValue(manual);

    assert.deepEqual(builtBytes, manualBytes);

    const builtHash = hashValue(builtBytes);
    const manualHash = hashValue(manualBytes);

    assert.deepEqual(builtHash, manualHash);

    // Optional roundtrip sanity
    assert.deepEqual(decodeValue(builtBytes), decodeValue(manualBytes));
});
