# Golden vectors (JS)

Golden vectors are the **final authority** for correctness in the JavaScript implementation of Strata.

They are not examples.\
They are not tests of convenience.\
They are law.

If the JavaScript implementation disagrees with a golden vector, the implementation is wrong.



***

### What are golden vectors

Golden vectors are canonical reference artifacts stored in the shared `/vectors` directory.

Each vector defines, with zero ambiguity:

* The Strata Text source (`.st`)
* The canonical binary encoding (`.scb.hex`)
* The canonical hash (`.hash.hex`)
* For negative cases, the expected error and offset

Vectors are shared across all implementations.



***

### Purpose

Golden vectors exist to guarantee:

* Canonical encoding correctness
* Stable hashing
* Cross-language determinism
* Regression prevention
* Spec enforcement

They represent frozen truth for a given Northstar version.



***

### Vector structure

A positive vector consists of:

* `<name>.st`\
  Human-authored Strata Text
* `<name>.scb.hex`\
  Canonical Strata Core Binary, hex-encoded
* `<name>.hash.hex`\
  Canonical BLAKE3-256 hash of the SCB bytes

A negative vector consists of:

* `<name>.hex`\
  Raw invalid binary input
* `<name>.error.json`\
  Expected error kind and offset



***

### JavaScript enforcement model

The JavaScript implementation enforces vectors in three dimensions:

1. Encoding
2. Hashing
3. Decoding failure semantics

All enforcement happens in automated tests.



***

### Encoding enforcement

For positive vectors:

* `.st` is parsed into a Value
* Value is encoded using `encodeValue`
* Resulting bytes must match `.scb.hex` exactly

Byte-for-byte equality is required.

Any difference is a failure.



***

### Hash enforcement

For positive vectors:

* The parsed Value is hashed using canonical encoding
* Resulting hex must match `.hash.hex` exactly

This ensures:

* Encoding correctness
* Hash correctness
* No hidden normalization



***

### Decode enforcement (negative vectors)

For negative vectors:

* Raw bytes are decoded using `decodeValue`
* Decoding must throw `DecodeError`
* Error kind must match expected
* Error offset must match expected

If decoding succeeds, or fails differently, the test fails.



***

### Test coverage

JavaScript golden vector tests verify:

* v1 canonical encoding
* v2 decode roundtrips
* v2 non-canonical input handling
* v2.1 strict error semantics
* Hash stability across versions

Every vector is exercised end-to-end.



***

### Cross-language parity

Golden vectors are shared with Rust.

This guarantees:

* Rust and JavaScript encode identically
* Rust and JavaScript hash identically
* Rust and JavaScript fail identically

Language differences are irrelevant. Canonical truth is universal.



***

### Regression blocking

Golden vectors are the primary regression barrier.

Any change that:

* Alters encoding
* Alters hashing
* Alters error semantics

Will immediately fail tests.

This prevents accidental drift and silent breakage.



***

### Northstar alignment

Each vector set corresponds to a Northstar milestone:

* v1: Canonical encoding
* v2: Decode behavior
* v2.1: Explicit error semantics

Adding a new Northstar requires:

* New vectors
* New enforcement
* No modification of old vectors



***

### Rules

* Vectors must never be modified to satisfy code
* Code must be modified to satisfy vectors
* Old vectors are immutable
* New behavior requires new vectors

This is non-negotiable.



***

### Summary

Golden vectors in JavaScript are:

* Canonical
* Shared
* Immutable
* Enforced

They are the contract between:

* Specification
* Implementation
* Time

If JavaScript passes all vectors, it is correct. If it fails one, it is not.
