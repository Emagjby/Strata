# Strata

_A deterministic binary data format with canonical encoding._

Designed for stable hashing, cross-language determinism, and safe transport.

[View the Documentation](https://strata.emagjby.com/)

---

## Why Strata exists

Most data formats optimize for convenience.  
Strata optimizes for **correctness**.

Strata exists to make structured data deterministic across languages, runtimes, and transports.
If two systems encode the same value, they must produce the **same bytes** and the **same hash**.

Anything else is considered a bug.

---

## What it guarantees

- Canonical, unambiguous binary encoding
- Stable hashing over canonical bytes
- Identical behavior across independent implementations
- Strict decoding with explicit failure modes
- Transport-independent correctness (files, HTTP, streaming)

Determinism is not a best-effort goal.  
It is the primary invariant.

---

## What it explicitly does not do

Strata intentionally does not handle:

- Schemas or validation rules
- Optional fields or defaults
- Streaming or framing protocols
- Compression
- Encryption or signatures
- Floating point numbers
- Implicit coercions or normalization

These concerns are deliberately kept **outside** the core format.

---

## Value model

Strata supports a small, fixed set of value types:

- `null`
- `bool`
- `int` (signed integer)
- `string` (UTF-8)
- `bytes`
- `list`
- `map` (string keys, canonical order)

There are no floats, no optional types, and no undefined behavior.

---

## Implementations

This repository contains independent implementations that must agree byte-for-byte:

### Rust — `strata-rs`

- Reference implementation
- Defines canonical truth
- Full encoder, decoder, parser, and CLI
- Golden vectors and Northstar enforcement

### JavaScript — `strata-js`

- Parity implementation with Rust
- Canonical encoding and hashing
- Shared golden vectors and CI enforcement
- First-class DX APIs

If Rust and JavaScript ever disagree, **that is a bug**.

---

## Northstar invariants

Strata correctness is enforced through **Northstar tests**.
Each Northstar defines and locks a specific invariant boundary.

Examples include:

- Cross-language determinism
- Raw wire determinism
- Framed streaming determinism

Northstars run in CI and block regressions.

---

## Stability & versioning

Strata uses semantic versioning with strict rules.

- **v0.3.x**: Canonical core frozen
- **v0.4.x**: Developer-experience and documentation only  
  (no canonical, hashing, or semantic changes)

Any change affecting bytes, hashes, or semantics requires:

- a new minor version
- a new Northstar
- explicit documentation

Version numbers are contract boundaries, not cosmetics.

---

## Installation

Rust:

```sh
cargo add strata-rs
```

JavaScript:

```sh
npm install @emagjby/strata-js
```

---

## Status

**Status: Stable**

Strata’s canonical core is complete and production-ready.
Future versions may expand tooling and ergonomics, but existing guarantees
will not be weakened retroactively.

---

Documentation is hosted with support from  
[GitBook](https://gitbook.com/).
