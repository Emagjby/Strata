# Hashing

Hashing in Strata is not a utility feature.\
It is a **core contract**.

A Strata hash is a cryptographic commitment to a **canonical value**, not to an in-memory structure, not to source text, and not to transport framing.



***

### Core rule

A Strata hash is defined as:

> BLAKE3-256(canonical Strata Core Binary bytes)

Nothing more. Nothing less.

If two values are logically equal, they **must** hash identically.\
If two values hash identically, they **must** decode to the same value.



***

### Hash target

Hashing always operates on **canonical encoded bytes**.

Hashing never operates on:

* Strata Text (`.st`)
* Parsed ASTs
* Runtime structures
* Transport frames
* Framed streams
* Pretty-printed or inspected output

All hashing is downstream of canonical encoding.



***

### Entry point

The Rust reference implementation exposes hashing at the value level.

```
hash_value(value: &Value) -> [u8; 32]
```

Internally, hashing is equivalent to:

```
encode(value) → canonical bytes
blake3(bytes) → 32-byte digest
```

This sequence is not optional.



***

### Algorithm choice

Strata uses **BLAKE3-256**.

Reasons:

* Cryptographically strong
* Deterministic
* Fast in software
* Stable across platforms
* Well-specified

The algorithm is part of the hashing contract.

Changing it requires a new version and a new Northstar.



***

### Stability guarantees

For a finalized Strata version:

* Hashes are stable forever
* Identical values always hash identically
* Hashes do not depend on:
  * CPU architecture
  * Endianness
  * Compiler version
  * Optimization level
  * Programming language

If two independent implementations disagree on a hash, at least one is wrong.



***

### Structural sensitivity

Hashes are sensitive to structure, not intent.

Examples:

* List order affects the hash
* Map key order does not affect the hash (canonical sorting)
* Changing any value changes the hash
* Changing nesting changes the hash

This is intentional.

Strata hashes describe **exact structure**, not semantic similarity.



***

### Non-canonical inputs

Hashing never accepts non-canonical input.

If bytes are non-canonical:

* They must be decoded
* Then re-encoded canonically
* Then hashed

Hashing raw, non-canonical bytes is forbidden at the API level.

This prevents hash divergence and ambiguity.



***

### Hashing raw bytes

If raw bytes are already known to be canonical, they may be hashed directly.

This is a performance optimization, not a semantic change.

The bytes must still be:

* Valid Strata Core Binary
* Canonically encoded
* Complete and single-value

Otherwise, the result is undefined behavior.



***

### Cross-language contract

The hashing contract is global.

All implementations must agree on:

* Canonical encoding rules
* Hash algorithm
* Hash length
* Byte order

No implementation may "optimize" hashing semantics.



***

### Northstar enforcement

Hashing behavior is enforced by Northstar tests:

* T1: Hash survives encode → decode → encode across languages
* T2: Hash survives raw wire transport
* T3: Hash survives framed streaming transport

If any Northstar fails, hashing is broken.



***

### What hashing does NOT guarantee

Hashing does not guarantee:

* Schema compatibility
* Backward compatibility
* Semantic equivalence
* Business meaning

It guarantees byte-level identity only.



***

### Summary

Hashing in Strata is:

* Deterministic
* Canonical
* Structural
* Cross-language
* Non-negotiable

If hashing is wrong, Strata is wrong.

There is no fallback.
