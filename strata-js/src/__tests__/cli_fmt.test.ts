import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { spawnSync } from "node:child_process";

const CLI_PATH = join(process.cwd(), "src", "cli.ts");

test("cli fmt handles .scb inputs", () => {
    const dir = mkdtempSync(join(tmpdir(), "strata-js-"));
    const input = join(dir, "input.scb");

    try {
        writeFileSync(
            input,
            Buffer.from([
                0x40, 0x01, 0x20, 0x07, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
                0x20, 0x04, 0x74, 0x65, 0x73, 0x74,
            ]),
        );

        const result = spawnSync(
            "npx",
            ["tsx", CLI_PATH, "fmt", "--format", "pretty", input],
            { encoding: "utf8" },
        );

        assert.equal(result.status, 0, result.stderr);
        assert.ok(result.stdout.includes("options: \"test\""));
    } finally {
        rmSync(dir, { recursive: true, force: true });
    }
});
