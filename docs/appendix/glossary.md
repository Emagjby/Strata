# Glossary

This glossary defines the precise meaning of terms used throughout the Strata documentation.\
Terms are normative unless explicitly stated otherwise.



***

### Canonical Encoding

A property of Strata where each logical value has **exactly one valid binary representation**.

If two encodings differ in bytes, at least one of them is incorrect.

Canonical encoding is enforced, not optional.



***

### Canonical Bytes

The byte sequence produced by encoding a value according to Strata’s canonical rules.

Canonical bytes are the **only** valid input to hashing and comparison.



***

### Determinism

The guarantee that identical input values always produce identical output bytes and hashes across:

* languages
* platforms
* runtimes
* time

Determinism is a core invariant of Strata.



***

### Value

A typed data element in Strata.

Strata values are limited to a fixed set of core types: null, bool, int, string, bytes, list, map.



***

### Value Model

The complete set of value types and their semantics as defined by Strata.

The value model is frozen per version line.



***

### Strata Core Binary (.scb)

The canonical binary encoding of a Strata value.

* Machine-oriented
* Deterministic
* Hash-stable
* Not human-readable

.scb is the only form used for hashing and wire transmission.



***

### Strata Text (.st)

A human-readable authoring format for Strata values.

* Parsed into the value model
* Compiled into canonical .scb
* Never hashed directly

.st exists for ergonomics, not transport.



***

### Encoding

The process of converting a Strata value into canonical .scb bytes.

Encoding must always produce the same bytes for the same value.



***

### Decoding

The process of converting .scb bytes back into a Strata value.

Decoding is strict and must fail on malformed or non-canonical input.



***

### Hashing

The process of computing a cryptographic hash over canonical .scb bytes.

Hashes identify content. They do not imply trust or security by themselves.



***

### Hashing Contract

The rule that hashing is performed over canonical bytes only, and that identical values must produce identical hashes across implementations.



***

### Map

A Strata value type consisting of string keys mapped to values.

Maps are unordered semantically but encoded in a canonical sorted order.



***

### Canonical Map Ordering

The rule that map keys are sorted by UTF-8 byte lexicographic order during encoding.

This ordering is mandatory and language-independent.



***

### Golden Vectors

Files that define the source of truth for Strata behavior.

Golden vectors specify:

* canonical .scb bytes
* canonical hashes
* required failure modes

If code disagrees with vectors, the code is wrong.



***

### Northstar

A formally defined invariant that Strata must uphold.

Northstars are enforced by tests and CI and block regressions.



***

### Northstar T

A concrete test that proves a specific invariant, such as wire determinism or cross-language equivalence.

Examples include T1, T2, and T3.



***

### Wire Determinism

The guarantee that Strata data survives transmission across systems without byte-level or hash-level change.



***

### Framing

A transport-layer mechanism used to separate Strata payloads in streams.

Framing must never modify Strata bytes or affect hashing.



***

### Strict Decoding

A decoding policy where invalid input causes explicit failure instead of recovery, coercion, or normalization.

Strict decoding is required.



***

### Failure Semantics

The defined behavior of decoders when encountering invalid input.

Failures include error kind and byte offset.



***

### Semantic Versioning (Strata)

Strata uses semantic versioning with additional constraints prioritizing canonical stability.

Minor versions define new invariant boundaries.



***

### Frozen Guarantees

Properties of Strata that are guaranteed not to change within a version line.

Frozen guarantees form contractual promises.



***

### Regression

Any change that breaks an existing invariant, vector, hash, or Northstar.

Regressions are treated as critical failures.



***

### Transport Independence

The principle that Strata correctness does not depend on how bytes are moved, stored, or framed.



***

### Non-Goals

Explicitly excluded concerns such as schemas, encryption, compression, or floating-point arithmetic.

Non-goals are intentional design decisions, not missing features.



***

### Canonical Violation

Any behavior that produces ambiguous, multiple, or unstable encodings for the same logical value.

Canonical violations are bugs.



***

### Law

The rule that vectors and invariants define correctness, not implementations.

In Strata: If code disagrees with the law, the code is wrong.
