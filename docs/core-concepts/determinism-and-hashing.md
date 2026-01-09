# Determinism & Hashing

Determinism is the observable consequence of canonical encoding.\
Hashing is the measurable proof that determinism holds.

Strata treats hashing not as a feature, but as a **contract**.

If determinism breaks, hashing exposes it immediately.



***

### What determinism means in Strata

In Strata, determinism means:

* The same logical value **always** encodes to the same bytes
* The same bytes **always** produce the same hash
* Independent implementations **must** agree at the byte level
* No runtime, platform, or language variance is permitted

Determinism is not probabilistic.\
It is absolute.

If two systems disagree, the system is broken.



***

### Why hashing is central

Hashing is the simplest and strongest test of determinism.

If two values are logically equal and canonically encoded, then:

* Their byte sequences are identical
* Their hashes are identical

No semantic comparison is required.\
No structural traversal is required.

Equality is proven by bytes.



***

### Hashing contract

Strata defines a strict hashing contract:

* Hashes are computed over **canonical encoded bytes**
* Hashes never depend on input form or authoring format
* Hashes never depend on decode or re-encode behavior
* Hashes never change within a finalized version line

Violating any of these rules invalidates the implementation.



***

### Hashing algorithm

Strata specifies:

* A fixed cryptographic hash function
* A fixed output size
* A fixed input definition (canonical bytes)

The hash function is **part of the protocol**, not an implementation detail.

Changing the hash function requires a new version line.



***

### Encoding vs hashing responsibility

Hashing does not define correctness.\
Encoding does.

Hashing only reflects what encoding produced.

This separation is intentional:

* Encoding enforces canonical truth
* Hashing proves canonical truth

A correct hash with incorrect encoding is impossible.



***

### No semantic hashing

Strata does not support:

* semantic hashing
* structure-based hashing
* field-order-independent hashing
* value-normalized hashing

All hashing is byte-based.

If the bytes differ, the hash differs.\
If the hash differs, determinism has been violated.



***

### Cross-language determinism

Determinism applies across:

* programming languages
* CPU architectures
* operating systems
* runtimes and VMs
* endianness
* memory layouts

A Strata encoder written in Rust and one written in JavaScript must emit identical bytes for the same value.

This is enforced by:

* canonical encoding rules
* golden vectors
* Northstar guarantees



***

### Determinism failures

If determinism fails, the failure is observable as:

* hash mismatch
* vector test failure
* Northstar regression
* broken cache keys
* broken signatures
* broken content addressing

Strata is designed so that failures are **loud**, not subtle.



***

### Determinism vs flexibility

Many formats trade determinism for flexibility:

* multiple integer encodings
* unordered maps
* optional defaults
* implicit normalization
* permissive decoders

Strata explicitly rejects this tradeoff.

Determinism is the priority.\
Flexibility must exist above the core, not inside it.



***

### Summary

In Strata:

* Canonical encoding guarantees determinism
* Determinism guarantees stable hashing
* Hashing proves correctness
* Disagreement is a bug, not an edge case

If hashes match, systems agree.\
If hashes differ, something is wrong.

That is the contract.
