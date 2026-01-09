# Overview

The Rust implementation of Strata is the **reference implementation**.

It defines what Strata _means_, not just how it is encoded.\
Every other language implementation is measured against this one.

If there is ambiguity, Rust is the tie-breaker.



***

### Purpose of the Rust implementation

The Rust crate exists to provide:

* Canonical encoding truth
* Strict decoding semantics
* Deterministic hashing
* Executable specification via tests
* Golden vector enforcement
* A production-grade CLI

It is intentionally conservative, explicit, and strict.

Convenience is never allowed to weaken guarantees.



***

### Reference status

The Rust implementation is treated as:

* The canonical encoder
* The canonical decoder
* The canonical hash producer
* The canonical interpretation of the spec

Other implementations must:

* Match Rust output byte-for-byte
* Match Rust hashes exactly
* Fail on the same invalid inputs
* Accept the same valid inputs

If a discrepancy exists, the non-Rust implementation is wrong.



***

### Architectural overview

The crate is structured around clear, isolated concerns:

* `value`\
  Core in-memory representation
* `encode`\
  Canonical binary encoding
* `decode`\
  Strict binary decoding with offsets
* `hash`\
  Deterministic BLAKE3 hashing
* `lexer`\
  Tokenization of Strata Text
* `parser`\
  AST construction from tokens
* `framing`\
  Optional wire framing (non-canonical)
* `error`\
  Explicit, typed error model

Each module has a single responsibility and no hidden coupling.



***

### Value model

Rust uses a closed, explicit value model:

* Null
* Bool
* Int (signed 64-bit)
* String (UTF-8)
* Bytes
* List
* Map (string keys only)

There are:

* No floats
* No implicit conversions
* No auto-widening
* No lossy parsing

This strictness is intentional and enforced.



***

### Encoding philosophy

Encoding is:

* Fully deterministic
* Fully canonical
* Independent of runtime state
* Independent of platform
* Independent of compiler version

Maps are ordered. Integers are canonical. Strings are raw UTF-8. Bytes are preserved verbatim.

Identical values always produce identical bytes.



***

### Decoding philosophy

Decoding is:

* Strict
* Total
* Non-panicking
* Explicitly failing

The decoder:

* Rejects malformed data
* Rejects trailing bytes
* Rejects invalid varints
* Rejects invalid UTF-8
* Surfaces precise offsets

Decoding reveals exactly what is on the wire. It does not "fix" data.



***

### Hashing guarantees

Hashes are defined as:

* BLAKE3
* 256-bit
* Over canonical `.scb` bytes only

Hashing is:

* Stable
* Cross-language
* Cross-platform
* Reproducible

Any change in bytes must change the hash. No exceptions.



***

### Strata Text support

The Rust implementation includes a full Strata Text parser.

Strata Text exists for:

* Authoring
* Review
* Debugging
* Documentation

It is not used for transport. It is not canonical. It always compiles into canonical binary.



***

### CLI as a first-class feature

The CLI is not a demo. It is part of the contract.

It allows:

* Compilation
* Decoding
* Hashing
* Formatting
* Inspection

CLI behavior is tested, versioned, and stable.



***

### Testing as specification

Tests are not optional.

The Rust implementation enforces:

* Unit tests
* Roundtrip tests
* Semantic tests
* Negative tests
* Golden vectors
* Northstar guarantees

Tests encode meaning. Vectors encode truth.



***

### Golden vectors are law

Golden vectors define:

* What must encode
* What must hash
* What must fail
* What must never change

If Rust disagrees with vectors, Rust is wrong.

Vectors are never updated to match bugs.



***

### Versioning discipline

The Rust implementation evolves only through:

* Explicit versioning
* New Northstars
* Additive guarantees

Breaking canonical behavior requires:

* A new Northstar
* A new version boundary
* Updated documentation

Nothing changes silently.



***

### Summary

The Rust implementation is:

* The canonical Strata definition
* The enforcement layer
* The regression barrier
* The executable spec

It prioritizes correctness over ergonomics and clarity over cleverness.

Everything else follows it.
