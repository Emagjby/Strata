# Terminology

This document defines the terms used throughout the Strata documentation. All terms are used consistently and precisely.



***

### Value

A **Value** is the abstract, language-independent representation of data in Strata.

Examples:

* `null`
* `true`
* `42`
* `"hello"`
* `[1, 2, 3]`
* `{ key: "value" }`

Values exist independently of encoding, storage, or transport.



***

### Canonical Encoding

**Canonical encoding** is the process of converting a Value into its single, unique binary representation.

In Strata:

* every Value has exactly one canonical encoding
* any deviation is invalid
* canonical encoding is fully deterministic

Canonical encoding produces **Strata Core Binary**.



***

### Strata Core Binary (.scb)

**Strata Core Binary** (`.scb`) is the canonical binary representation of a Value.

Properties:

* deterministic
* unambiguous
* hash-stable
* transport-agnostic

`.scb` is the source of truth for hashing, storage, and verification.



***

### Strata Text (.st)

**Strata Text** (`.st`) is a human-readable authoring format.

It exists to:

* write Strata data by hand
* review and debug structures
* generate canonical `.scb`

`.st` is not canonical. Only `.scb` is.



***

### Canonical Bytes

**Canonical bytes** are the exact byte sequence produced by canonical encoding.

They are:

* the input to hashing
* the unit of comparison
* the boundary of correctness

If canonical bytes differ, the Values are not equal.



***

### Hash

A **Hash** in Strata is the cryptographic digest of canonical bytes.

Rules:

* hashes are computed over `.scb` bytes only
* identical Values produce identical hashes
* hash stability is guaranteed within a version

Hashes are never computed over text, ASTs, or decoded structures.



***

### Determinism

**Determinism** means that Strata behavior does not depend on:

* programming language
* platform
* runtime
* implementation details

Given the same Value, all correct implementations produce identical results.



***

### Strict Decoding

**Strict decoding** means that malformed or non-canonical data is rejected explicitly.

Decoders must fail on:

* invalid tags
* truncated input
* invalid UTF-8
* trailing bytes
* malformed varints

There is no permissive mode.



***

### Non-Canonical Data

**Non-canonical data** is any input that does not follow canonical encoding rules.

Examples:

* duplicate map keys
* incorrect ordering
* malformed encodings

Non-canonical data may be inspectable, but it is never considered valid Strata.



***

### Golden Vectors

**Golden vectors** are authoritative test fixtures defining correct behavior.

They specify:

* exact canonical bytes
* expected hashes
* required failure modes

If an implementation disagrees with golden vectors, the implementation is wrong.



***

### Northstar

A **Northstar** is a formally defined invariant that governs Strata behavior.

Northstars:

* define guarantees
* lock behavior
* prevent regressions
* gate breaking changes

Any semantic or canonical change requires a new Northstar.



***

### Implementation

An **implementation** is a concrete realization of Strata in a specific language.

All implementations must:

* follow the same specification
* pass all golden vectors
* produce identical canonical output

Language differences are irrelevant.



***

### Summary

These terms define the vocabulary of Strata.

They are not interchangeable. They are not informal. They are the foundation of precise communication and reliable systems.
