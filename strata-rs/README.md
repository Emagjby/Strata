# Strata

**Deterministic data language with canonical binary encoding.**

This crate is the **reference Rust implementation** of **Strata**.

> Same data → same bytes → same hash.  
> No ambiguity. No normalization. No silent coercions.

Golden vectors are law.

[View the Documentation](https://strata.emagjby.com/)

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

## What this crate provides

This crate defines **canonical truth** for Strata.

It provides:

- Canonical encoder for Strata Core Binary (`.scb`)
- Safe decoder with explicit, structured errors
- Parser for Strata Text (`.st`)
- Deterministic BLAKE3 hashing
- Production-grade CLI
- Golden vector enforcement
- Reference semantics for all other implementations

If another language disagrees with this crate, **the other language is wrong**.

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

## Value model

Strata supports a **fixed, closed value model**:

```text
Value =
    Null
  | Bool(bool)
  | Int
  | String
  | Bytes
  | List(Value*)
  | Map(string → Value)
```

Rules:

- Integers are explicit (no floats, no implicit coercions)
- Strings are UTF-8
- Map keys are strings only
- Bytes are raw bytes
- Duplicate map keys are allowed
- No floats
- No NaN
- No normalization

The model is intentionally small.  
Power comes from determinism, not expressiveness.

---

## Constructing values (Rust)

Values may be constructed manually or using **DX macros**.  
Macros are **pure sugar** and do **not** affect canonical encoding or hashing.

```rust
use strata::value::Value;

let value = map! {
    "id" => int!(42),
    "name" => string!("Gencho"),
    "active" => bool!(true),
    "skills" => list![
        string!("rust"),
        string!("systems"),
    ],
    "meta" => map! {
        "a" => int!(1),
        "a" => int!(2), // last-write-wins
    },
};
```

Available macros:

- `null!()`
- `bool!(...)`
- `int!(...)`
- `string!(...)`
- `bytes!(...)`
- `list![ ... ]`
- `map!{ "k" => v, ... }`

Macros construct the **exact same `Value` structures** as manual code.

---

## Canonical encoding

Encoding is **fully deterministic**.

Rules include:

- Maps are sorted by UTF-8 byte order
- Integers use canonical varint encoding
- Strings are stored as raw UTF-8 bytes
- Bytes are preserved verbatim
- Duplicate keys resolve via **last-write-wins**

Identical values always produce identical `.scb` bytes.

```rust
use strata::encode::encode_value;

let bytes = encode_value(&value)?;
```

---

## Decoding guarantees

The decoder is **strict and transparent**.

It:

- Never panics on input
- Rejects malformed data explicitly
- Preserves observable structure
- Allows duplicate keys for inspection

Decoding reveals reality.  
Encoding enforces truth.

```rust
use strata::decode::decode_value;

let value = decode_value(&bytes)?;
```

---

## Hashing

Hashes are computed as:

```text
BLAKE3-256(canonical_scb_bytes)
```

Example:

```rust
use strata::hash::hash_bytes;

let hash = hash_bytes(&bytes);
```

Hashes are stable across:

- machines
- operating systems
- compiler versions
- programming languages

---

## Parsing Strata Text (`.st`)

Strata Text is a **human-friendly authoring format**.

Example:

```st
user {
  id: 42
  name: "Gencho"
  active: true
  skills: ["rust", "systems"]
}
```

Parsing:

```rust
use strata::parser::parse;

let value = parse(source)?;
```

Text is **never canonical**.  
Only encoded `.scb` bytes are.

---

## CLI

This crate ships with a CLI for inspection and tooling.

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

All failures are **explicit and structured**.

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

This implementation:

- Must match vectors exactly
- Must never change vectors to satisfy code
- Must fail when vectors say so

If vectors disagree with code, **code is wrong**.

---

## Documentation

Canonical integration documentation lives in the **Integration Reference**:

- Value model
- Encoding / decoding boundaries
- Hashing contract
- Rust and JavaScript examples
- Strictness rules and footguns

The Integration Reference is the **single source of truth** for integrators.

---

## Versioning & Northstar

Strata evolution is governed by **Northstar documents**.

- Northstar v1 – Canonical encoding
- Northstar v2 – Decode & inspection
- Northstar v2.1 – Explicit error semantics
- Northstar v3 – Cross-language parity
- Northstar v4 – Developer ergonomics & documentation

Any canonical change requires a new Northstar.

---

## License

MIT License

---

## Final note

This crate is intentionally strict.

It is designed for:

- integrity-critical systems
- deterministic pipelines
- cross-language verification
- audit-friendly storage

If convenience matters more than correctness, use something else.  
If correctness matters, this is the reference.
