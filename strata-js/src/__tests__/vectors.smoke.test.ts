import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const VECTORS_DIR = join(process.cwd(), "..", "vectors");

test("JS can see vectors directory", () => {
    const entries = readdirSync(VECTORS_DIR);
    assert.ok(entries.includes("v1"));
    assert.ok(entries.includes("v2"));
    assert.ok(entries.includes("v2.1"));
});

test("JS can read a known vector file", () => {
    const path = join(VECTORS_DIR, "v1", "01-basic.st");
    const content = readFileSync(path, "utf-8");

    assert.ok(content.includes("config"));
    assert.ok(content.includes("enabled"));
});
