# Strata

**Deterministic data language with canonical binary encoding.**

This is the **reference Rust implementation** of **Strata**.

Same data -> same bytes -> same hash.  
No ambiguity. No normalization. No silent coercions.

Golden vectors are law.

---

## What this crate provides

- Canonical encoder for Strata Core Binary (`.scb`)
- Safe decoder with explicit error semantics
- Parser for Strata Text (`.st`)
- Deterministic BLAKE3 hashing
- Production-grade CLI
- Golden vector enforcement

This implementation defines **canonical truth** for all other languages.

---

## Installation

```bash
cargo add strata-rs
```

Or in `Cargo.toml`:

```toml
[dependencies]
strata-rs = "*"
```

---

## Data model

Strata supports a minimal, fixed value model:

```text
Value =
    Null
  | Bool(bool)
  | Int(i64)
  | String(String)
  | Bytes(Vec<u8>)
  | List(Vec<Value>)
  | Map(BTreeMap<String, Value>)
```

Rules:

- Integers are **signed 64-bit**
- Strings are **UTF-8**
- Map keys are **strings only**
- No floats
- No implicit conversions

---

## Canonical encoding

Encoding is **fully deterministic**.

- Maps are sorted by UTF-8 byte order
- Integers use canonical SLEB128
- Strings are stored as raw UTF-8 bytes
- Bytes are preserved verbatim

Identical values always produce identical `.scb` bytes.

---

## Decoding guarantees

The decoder:

- Never panics on input
- Rejects malformed data explicitly
- Preserves non-canonical ordering
- Allows duplicate keys (debug visibility)

Decoding reveals reality.  
Encoding enforces truth.

---

## Hashing

Hashes are computed as:

```text
BLAKE3-256(canonical_scb_bytes)
```

Example:

```rust
use strata::hash::hash_value;
use strata::value::Value;

let value = Value::Int(42);
let hash = hash_value(&value);
```

Hashes are stable across:

- Machines
- Operating systems
- Compiler versions
- Programming languages

---

## Parsing Strata Text (`.st`)

Strata Text is a human-friendly authoring format.

Example:

```st
user {
  id: 42
  name: "Gencho"
  active: true
  skills: ["rust", "systems"]
  avatar_hash: 0x9f86d081884c7d659a2feaa0c55ad015
}
```

Parsing:

```rust
use strata::parser::parse;

let value = parse(source)?;
```

---

## CLI

The Rust crate ships with a CLI.

### Compile

```bash
strata compile input.st output.scb
```

### Decode

```bash
strata decode input.scb
```

### Hash

```bash
strata hash input.st
strata hash input.scb
```

### Format

```bash
strata fmt input.st
```

---

## Error model

All failures are explicit and structured.

Decode errors include:

- `InvalidTag`
- `UnexpectedEOF`
- `InvalidVarint`
- `InvalidUtf8`
- `TrailingBytes`

Parse errors include:

- Unexpected token
- Integer out of range
- Malformed literals
- Line and column information

CLI exit codes:

| Code | Meaning        |
| ---: | -------------- |
|    0 | Success        |
|    1 | Invalid input  |
|    2 | I/O failure    |
|  100 | Internal error |

---

## Golden vectors

Canonical truth lives in `/vectors`.

The Rust implementation:

- Must match vectors exactly
- Must never change vectors to satisfy code
- Must fail when vectors say so

If Rust disagrees with vectors, **Rust is wrong**.

---

## Versioning & Northstar

Strata evolution is governed by **Northstar documents**.

- Northstar v1 – Canonical encoding
- Northstar v2 – Decode & inspection
- Northstar v2.1 – Explicit error semantics
- Northstar v3 – Cross-language parity

Any canonical change requires a new Northstar.

---

## License

MIT License

---

## Final note

This crate is intentionally strict.

It is designed for:

- Integrity-critical systems
- Deterministic pipelines
- Cross-language verification
- Audit-friendly storage

If convenience matters more than correctness, use something else.  
If correctness matters, this is the reference.
