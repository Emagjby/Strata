# @emagjby/strata-js

Deterministic Strata implementation for JavaScript.

Strata is a deterministic binary data format with canonical encoding: identical logical values must produce identical bytes (and therefore identical hashes) across all implementations.

This package is a **parity implementation** with the Rust reference implementation. If Rust and JavaScript ever disagree, **that is a bug**.

- Canonical encoding: one value → one byte representation
- Hashing: stable hash over canonical bytes (BLAKE3-256)
- Strata Text (`.st`): human authoring format that compiles to canonical Strata Core Binary (`.scb`)

Docs: https://strata.emagjby.com/docs

## Install

Library:

```bash
npm install @emagjby/strata-js
```

CLI:

```bash
npm install -g @emagjby/strata-js
```

## Quickstart (Library)

### Parse Strata Text (`.st`) → encode (`.scb`) → hash

```js
import { parse, encodeValue, hashValueHex } from "@emagjby/strata-js";

const source = `{
  name: "alice"
  active: true
  count: 3
}`;

const value = parse(source);
const scb = encodeValue(value); // Uint8Array canonical bytes

// Hashing is defined over canonical bytes.
console.log(hashValueHex(value));
```

### Decode `.scb` bytes back into a Strata value

```js
import { decodeValue, encodeValue } from "@emagjby/strata-js";

// Given canonical bytes (or bytes captured from storage/wire)
const originalScb = new Uint8Array([0x00]); // example only

const value = decodeValue(originalScb);
const roundtrippedScb = encodeValue(value);

// Strata guarantees: decode(encode(value)) == value
// The reverse is intentionally NOT guaranteed.
console.log(roundtrippedScb);
```

### Hash a value (canonical re-encode)

If you already have a JS `Value` and want a canonical hash:

```js
import { hashValueHex, parse } from "@emagjby/strata-js";

const value = parse("[1, 2, 3]");
console.log(hashValueHex(value));
```

## CLI

The `strata-js` CLI mirrors the Rust CLI.

Commands:

- `compile` – compile `.st` → canonical `.scb`
- `decode` – decode `.scb` into a stable inspection format
- `hash` – compute deterministic hash (BLAKE3-256)
- `fmt` – parse `.st` and print a structured inspection format

### Compile

```bash
strata-js compile input.st output.scb
```

### Hash

```bash
strata-js hash input.st
strata-js hash input.scb
```

Behavior:

- If input is `.st`, it is parsed and canonically encoded first
- If input is `.scb`, bytes are hashed directly
- Output is lowercase hex

### Decode

```bash
strata-js decode input.scb
```

### Fmt

```bash
strata-js fmt input.st
```

### Exit codes

- `0` success
- `1` invalid input
- `2` I/O failure
- `100` internal error

## JavaScript value model (important)

Strata values in JS are **not JSON** and are intentionally strict.

- Integers are `bigint` (JS `number` MUST NOT be used)
- Bytes are `Uint8Array`
- Maps are `ReadonlyMap<string, Value>`

If you need to bridge into JSON:

- `bigint` cannot be directly serialized by `JSON.stringify`
- You must choose an explicit representation (commonly decimal strings)

Type shape (abridged):

- `{ kind: "null" }`
- `{ kind: "bool", value: boolean }`
- `{ kind: "int", value: bigint }`
- `{ kind: "string", value: string }`
- `{ kind: "bytes", value: Uint8Array }`
- `{ kind: "list", value: readonly Value[] }`
- `{ kind: "map", value: ReadonlyMap<string, Value> }`

## Determinism notes

- Hashing is defined over **canonical encoded bytes**, not decoded structures.
- Decoding exists to inspect reality; encoding defines canonical truth.
- Map keys are canonically ordered by UTF-8 byte order during encoding.

If your system needs “mostly the same bytes”, Strata is not the tool. If it needs _exactly the same bytes_, it is.

## Golden vectors

Strata correctness is enforced via golden vectors (authored in `.st`) and cross-implementation tests.

Golden vectors are not examples; they are law:

- If this package disagrees with a golden vector, **the implementation is wrong**.
- Vectors are not adjusted to match buggy behavior.

## What Strata does NOT do

Strata intentionally does not provide:

- Schemas or validation rules
- Optional fields or default values
- Backward-compatible schema evolution
- Floating point numbers
- Streaming/framing rules (transport concerns are external)
- Compression, encryption, or authentication

## Links

- Documentation: https://strata.emagjby.com/docs
- Repo: https://github.com/Emagjby/Strata
- Strata JS package source: https://github.com/Emagjby/Strata/tree/main/strata-js
