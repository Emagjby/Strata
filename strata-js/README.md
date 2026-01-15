# @emagjby/strata-js

**Deterministic Strata implementation for JavaScript.**

This package is the **JavaScript parity implementation** of **Strata**.

> Same data → same bytes → same hash.  
> No ambiguity. No normalization. No silent coercions.

If Rust and JavaScript ever disagree, **that is a bug**.

---

## What is Strata?

Strata is a **strict, minimal data model** with a **fully deterministic binary encoding**.

It is designed for systems where:

- data integrity is non-negotiable
- hashes must be stable forever
- cross-language verification is required
- ambiguity is unacceptable

Strata draws a hard line between **representation** and **truth**:

- `Value` is an in-memory representation
- canonical `.scb` bytes define truth
- hashes are computed only from canonical bytes

---

## What this package provides

This package provides:

- Canonical encoder for Strata Core Binary (`.scb`)
- Safe decoder with explicit error semantics
- Parser for Strata Text (`.st`)
- Deterministic BLAKE3 hashing
- CLI tooling (mirrors Rust CLI)
- Golden vector enforcement
- Behavioral parity with the Rust reference implementation

This package does **not** define canonical truth.  
The Rust implementation does.

---

## Install

Library:

```bash
npm install @emagjby/strata-js
```

CLI:

```bash
npm install -g @emagjby/strata-js
```

---

## Quickstart (Library)

### Parse `.st` → encode `.scb` → hash

```js
import { parse, encodeValue, hashValueHex } from "@emagjby/strata-js";

const source = `{
  name: "alice"
  active: true
  count: 3
}`;

const value = parse(source);
const scb = encodeValue(value); // Uint8Array (canonical bytes)

// Hashing is defined over canonical bytes
console.log(hashValueHex(scb));
```

---

### Decode `.scb` bytes back into a Value

```js
import { decodeValue, encodeValue } from "@emagjby/strata-js";

const originalScb = new Uint8Array([0x00]); // example only

const value = decodeValue(originalScb);
const roundtrippedScb = encodeValue(value);

// Guaranteed: encode(decode(bytes)) === bytes
// NOT guaranteed: decode(encode(value)) === value
console.log(roundtrippedScb);
```

---

### Hash an existing Value

```js
import { parse, encodeValue, hashValueHex } from "@emagjby/strata-js";

const value = parse("[1, 2, 3]");
const bytes = encodeValue(value);

console.log(hashValueHex(bytes));
```

---

## Constructing Values (JavaScript)

The recommended construction API is the **`Value` factory**.

The legacy alias `V` remains supported for backwards compatibility.

```js
import { Value } from "@emagjby/strata-js";

const value = Value.mapOf(
  ["id", Value.int(42n)],
  ["name", Value.string("Gencho")],
  ["active", Value.bool(true)],
  ["skills", Value.listOf(Value.string("rust"), Value.string("systems"))],
  [
    "meta",
    Value.mapOf(
      ["a", Value.int(1n)],
      ["a", Value.int(2n)], // last-write-wins
    ),
  ],
);
```

Available helpers:

- `Value.null()`
- `Value.bool(boolean)`
- `Value.int(bigint)` **(BigInt only)**
- `Value.string(string)`
- `Value.bytes(Uint8Array)`
- `Value.list(Value[])`
- `Value.map(Iterable<[string, Value]>)`

DX helpers (additive):

- `Value.listOf(...Value)`
- `Value.mapObj({ [key]: Value })`
- `Value.mapOf(...[string, Value])`
- `Value.bytesFrom(Uint8Array | ArrayBuffer | number[] | Iterable<number>)`
- `Value.bytesHex(hexString)` (strict hex)

Duplicate map keys resolve via **last-write-wins**.

---

## JavaScript Value model (important)

Strata values are **not JSON** and are intentionally strict.

Rules:

- Integers are `bigint` (JS `number` is rejected)
- Bytes are `Uint8Array`
- Maps are `ReadonlyMap<string, Value>`
- No floats
- No implicit conversions

Type shape (abridged):

- `{ kind: "null" }`
- `{ kind: "bool", value: boolean }`
- `{ kind: "int", value: bigint }`
- `{ kind: "string", value: string }`
- `{ kind: "bytes", value: Uint8Array }`
- `{ kind: "list", value: readonly Value[] }`
- `{ kind: "map", value: ReadonlyMap<string, Value> }`

If you need JSON interoperability, you must define an **explicit mapping**.

---

## Canonical encoding & determinism

- Encoding is fully deterministic
- Map keys are sorted by UTF-8 byte order during encoding
- Duplicate keys overwrite earlier entries (last-write-wins)
- Hashing is defined over canonical `.scb` bytes

If your system needs “mostly the same bytes”, Strata is not the tool.  
If it needs **exactly the same bytes**, it is.

---

## CLI

The `strata-js` CLI mirrors the Rust CLI.

Commands:

- `compile` – compile `.st` → canonical `.scb`
- `decode` – decode `.scb` for inspection
- `hash` – compute deterministic hash
- `fmt` – parse and pretty-print `.st`

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

- `.st` is parsed and canonically encoded first
- `.scb` bytes are hashed directly
- Output is lowercase hex

### Decode

```bash
strata-js decode input.scb
```

### Format

```bash
strata-js fmt input.st
```

### Exit codes

- `0` success
- `1` invalid input
- `2` I/O failure
- `100` internal error

---

## Golden vectors

Correctness is enforced using **golden vectors** shared with the Rust implementation.

Golden vectors are law:

- If this package disagrees with vectors, **this package is wrong**
- Vectors are not adjusted to match buggy behavior

---

## What Strata does NOT do

Strata intentionally does not provide:

- Schemas or validation rules
- Optional fields or defaults
- Backward-compatible schema evolution
- Floating point numbers
- Streaming or framing rules
- Compression, encryption, or authentication

---

## Documentation

Canonical integration documentation lives in the **Integration Reference**:

https://strata.emagjby.com/docs

This is the **single source of truth** for integrators.

---

## Links

- Documentation: https://strata.emagjby.com/docs
- Monorepo: https://github.com/Emagjby/Strata
- JS source: https://github.com/Emagjby/Strata/tree/main/strata-js

---

## License

MIT License

---

## Final note

This package is intentionally strict.

If convenience matters more than correctness, use something else.  
If correctness matters, this is the JavaScript implementation.
