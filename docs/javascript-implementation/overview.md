# Overview

## JavaScript Implementation Overview

The JavaScript implementation of Strata provides a **fully deterministic, specification-compliant** runtime for encoding, decoding, hashing, and parsing Strata data.

It is not a convenience wrapper. It is a **first-class, law-bound implementation** that must match Rust byte-for-byte.

Package name: `@emagjby/strata-js`\
Status: **Parity implementation (v0.3.x)**



***

### Design goals

The JavaScript implementation exists to prove and enforce:

* Cross-language determinism
* Canonical encoding stability
* Identical hashing semantics
* Explicit failure behavior
* Vector-driven correctness

JavaScript is treated as a hostile environment by default. Precision loss, implicit coercions, and runtime magic are actively rejected.



***

### Scope of the implementation

The JS implementation includes:

* Strata Core Binary encoder
* Strata Core Binary decoder
* Deterministic hashing (BLAKE3-256)
* Strata Text (`.st`) lexer and parser
* Canonical map ordering
* Structured error model
* CLI with stable exit semantics
* Golden vector enforcement

Everything required to round-trip `.st → .scb → .hash` deterministically.



***

### Runtime requirements

* Node.js 20+
* ES2022
* Native BigInt support
* UTF-8 compliant TextEncoder / TextDecoder

No browser shims. No polyfills. No transpiled runtime hacks.



***

### Integer strategy (BigInt-first)

JavaScript numbers are **not allowed**.

All Strata integers are represented as `bigint`.

Rules enforced at runtime:

* Any attempt to construct an `Int` from `number` throws
* All varint logic operates on `bigint`
* Overflow is explicitly detected and rejected

This guarantees parity with Rust `i64` semantics.



***

### Architecture overview

High-level module responsibilities:

* `encode.ts`\
  Canonical encoder for Strata Core Binary
* `decode.ts`\
  Strict, offset-aware decoder with structured errors
* `hash.ts`\
  BLAKE3-256 hashing over canonical bytes
* `parser.ts`\
  Recursive descent parser for Strata Text
* `lexer.ts`\
  Tokenization with span tracking and strict literals
* `value.ts` / `value_factory.ts`\
  Immutable Value model and constructors
* `cli.ts`\
  Production-grade CLI with deterministic behavior
* `__tests__/`\
  Golden vectors, negative cases, parity enforcement

Each module has a single responsibility. No cross-layer leakage is allowed.



***

### Canonical authority

The JavaScript implementation does **not** define truth.

Truth comes from:

* Golden vectors
* Northstar documents
* Canonical rules

JS code must conform. If JS disagrees, JS is wrong.



***

### Error philosophy

Errors are values, not strings.

All failures include:

* Error kind
* Byte offset (for decode)
* Line and column (for parse)

There is:

* No panic on user input
* No silent recovery
* No guesswork

Failure is part of the contract.



***

### CLI parity

The JavaScript CLI mirrors the Rust CLI exactly:

Commands:

* compile
* decode
* hash
* fmt

Exit codes:

* 0 success
* 1 invalid input
* 2 I/O failure
* 100 internal error

Output formats and semantics are intentionally boring and stable.



***

### Golden vector enforcement

JS directly consumes `/vectors` from the repository root.

Rules:

* Vectors are not copied
* Vectors are not transformed
* Vectors are law

Tests assert:

* Byte-for-byte encoding
* Hash equality
* Exact error kind and offset

Any failure blocks release.



***

### What this implementation is not

The JavaScript implementation is not:

* A schema system
* A JSON replacement
* A flexible serializer
* A forgiving parser
* A browser-first library

It is a correctness engine.



***

### Mental model

Think of `@emagjby/strata-js` as:

* A verifier
* A canonical executor
* A hostile-environment proof
* A legal witness to determinism

If Rust and JavaScript agree, Strata is real.

If they ever disagree, Strata has failed.



***

### Summary

The JavaScript implementation exists to answer one question:

**Can Strata survive JavaScript?**

If the answer is yes, it can survive anywhere.

That is why this implementation exists. That is why it is strict. That is why it is boring.

And that is why it matters.
