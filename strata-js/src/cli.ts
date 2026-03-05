#!/usr/bin/env bun

import fs from "node:fs";
import process from "node:process";

import { parse } from "./parser.js";
import { encodeValue } from "./encode.js";
import { decodeValue } from "./decode.js";
import { hashBytes } from "./hash.js";

import { DecodeError } from "./decode_error.js";
import { ParseError } from "./parse_error.js";

import { inspectValue } from "./inspect.js";
import { fmt, FormatOptions } from "./fmt.js";

function exitOk(): never {
    process.exit(0);
}

function exitInvalid(msg?: string): never {
    if (msg) {
        console.error(msg);
    }
    process.exit(1);
}

function exitIo(err: unknown): never {
    if (err instanceof Error) {
        console.error(err.message);
    }
    process.exit(2);
}

function exitInternal(err: unknown): never {
    console.error("internal error");
    if (err instanceof Error) {
        console.error(err.stack);
    }
    process.exit(100);
}

function hex(bytes: Uint8Array): string {
    return Array.from(bytes, (b) => b.toString(16).padStart(2, "0")).join("");
}

function cmdCompile(input: string, output: string): void {
    const source = fs.readFileSync(input, "utf8");
    const value = parse(source);
    const encoded = encodeValue(value);
    fs.writeFileSync(output, encoded);
}

function cmdDecode(input: string): void {
    const encoded = fs.readFileSync(input);
    const value = decodeValue(encoded);
    console.log(JSON.stringify(inspectValue(value), null, 2));
}

function cmdHash(input: string): void {
    let encoded: Uint8Array;

    if (input.endsWith(".st")) {
        const source = fs.readFileSync(input, "utf8");
        const value = parse(source);
        encoded = encodeValue(value);
    } else {
        encoded = fs.readFileSync(input);
    }

    const hashed = hashBytes(encoded);
    console.log(hex(hashed));
}

function cmdFmt(
    input: string,
    options: FormatOptions = FormatOptions.PRETTY,
): void {
    const value = input.endsWith(".scb")
        ? decodeValue(fs.readFileSync(input))
        : parse(fs.readFileSync(input, "utf8"));
    const formatted = fmt(options, value);
    console.log(formatted);
}

function main(): void {
    const [, , cmd, ...args] = process.argv;

    if (!cmd) {
        exitInvalid("no command provided");
    }

    switch (cmd) {
        case "compile": {
            if (args.length !== 2) {
                exitInvalid("usage: strata-js compile <input.st> <output.scb>");
            }
            cmdCompile(args[0]!, args[1]!);
            return exitOk();
        }

        case "decode": {
            if (args.length !== 1) {
                exitInvalid("usage: strata-js decode <input.scb>");
            }
            cmdDecode(args[0]!);
            return exitOk();
        }

        case "hash": {
            if (args.length !== 1) {
                exitInvalid("usage: strata-js hash <input.st|input.scb>");
            }
            cmdHash(args[0]!);
            return exitOk();
        }

        case "fmt": {
            if (args.length < 1 || args.length > 3)
                return exitInvalid("usage: strata-js fmt [format] <input.st>");
            if (args.length === 2) {
                return exitInvalid(
                    "usage: --format [type] must be provided if format is provided",
                );
            }
            if (args.length === 1) {
                cmdFmt(args[0]!);
                return exitOk();
            }

            if (args[0] !== "--format") {
                return exitInvalid(
                    "usage: first argument must be --format if format is provided",
                );
            }

            if (args[1] !== "pretty" && args[1] !== "ast") {
                return exitInvalid("usage: format type must be one of: pretty, ast");
            }

            const format =
                args[1] === "pretty" ? FormatOptions.PRETTY : FormatOptions.AST;

            cmdFmt(args[2]!, format);
            return exitOk();
        }

        case "--help": {
            console.log(`
Strata CLI (JavaScript)

Commands:
  compile <input.st> <output.scb>
  decode <input.scb>
  hash <input.st|input.scb>
  fmt [format] <input.st>
            `);
            process.exit(0);
        }

        default:
            exitInvalid(`unknown command: ${cmd}`);
    }
}

try {
    main();
} catch (err) {
    if (err instanceof DecodeError || err instanceof ParseError) {
        console.error("error: ", err.message);
        process.exit(1);
    }

    if (err instanceof Error && "code" in err && (err as any).code === "ENOENT") {
        exitIo(err);
    }

    exitInternal(err);
}
