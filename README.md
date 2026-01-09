# Strata

_A deterministic binary data format with canonical encoding._

Designed for stable hashing, cross-language determinism, and safe transport.

[View the Documentation](https://strata.emagjby.com/docs)

## Why Strata exists

Most data formats optimize for convenience.
Strata optimizes for correctness.

Strata exists to make structured data deterministic across languages, runtimes, and transports.
If two systems encode the same value, they must produce the same bytes and the same hash.

Anything else is considered a bug.

## What it guarantees

- Canonical, unambiguous binary encoding
- Stable hashing over canonical bytes
- Identical behavior across independent implementations
- Strict decoding with explicit failure modes
- Transport-independent correctness (files, HTTP, streaming)

## What it explicitly does not do

Strata intentionally does not handle:

- Schemas or validation rules
- Streaming or framing protocols
- Compression
- Encryption or signatures
- Floating point numbers
- Implicit coercions or normalization

These concerns are deliberately kept outside the core format.

## Value Model

Strata supports a small, fixed set of value types:

- null
- bool
- int _(signed 64-bit)_
- string _(UTF-8)_
- bytes
- list
- map _(string keys, canonical order)_

There are no floats, no optional types, and no undefined behavior.

## Implementations

This repository contains independent implementations:

- Rust (strata-rs)

Reference implementation, CLI, and full test suite.

- JavaScript (strata-js)

Fully deterministic implementation with shared vectors and CI enforcement.

Both implementations must agree byte-for-byte.

## Northstar Tests

Strata correctness is enforced through Northstar tests, each proving a specific invariant:

- T1 – Cross-language determinism with an envelope
- T2 – Raw wire determinism (no encoding, no helpers)
- T3 – Framed streaming determinism under arbitrary chunking

Northstars run in CI and block regressions.

## Stability & Versioning

As of v0.3.x, the following are frozen:

- Canonical encoding
- Hashing semantics
- Value model
- Decode behavior

Any change affecting bytes, hashes, or semantics requires a new minor version and a new Northstar.

## Installation

Rust:

```sh
cargo add strata-rs
```

Javascript:

```sh
npm install strata-js
```

## Status

Status: Stable

Strata v0.3.x is considered production-ready for its defined scope.
Future work may introduce verification or trust layers, but the core format is complete

Documentation hosted with support from [GitBook](https://gitbook.com/) Community Plan.
