import test from "node:test";
import assert from "node:assert/strict";
import { V } from "../value_factory.js";
import { Value } from "../value.js";

test("can represent all core value types", () => {
    const v: Value = V.map([
        ["null", V.null()],
        ["bool", V.bool(true)],
        ["int", V.int(42n)],
        ["string", V.string("strata")],
        ["bytes", V.bytes(new Uint8Array([0xde, 0xad]))],
        ["list", V.list([V.int(1n), V.int(2n), V.int(3n)])],
    ]);

    assert.equal(v.kind, "map");
});

test("rejects number for int", () => {
    // @ts-expect-error
    assert.throws(() => V.int(42));
});

test("supports BigInt beyond JS safe integer range", () => {
    const big = 9007199254740993n;
    const v = V.int(big);

    assert.equal(v.value, big);
});
