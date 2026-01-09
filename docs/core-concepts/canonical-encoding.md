# Canonical encoding

Canonical encoding is the foundation of Strata.

It is the reason Strata exists, and the reason every other guarantee holds.

In Strata, a value has **exactly one valid binary representation**.\
No alternatives. No "equivalent" encodings. No normalization steps after the fact.

If two encoders produce different bytes for the same logical value, **at least one of them is wrong**.



***

### What "canonical" means in Strata

Canonical encoding means:

* A Strata value maps to **one and only one** byte sequence
* Encoding is **fully deterministic**
* Decoding does **not** repair, normalize, or reinterpret data
* Hashing is performed over **canonical bytes only**

There is no concept of:

* permissive encoding
* equivalent representations
* platform-dependent behavior
* runtime-dependent output

Canonical encoding is not a guideline.\
It is a **hard invariant**.



***

### Why canonical encoding matters

Canonical encoding enables guarantees most data formats cannot provide:

* Stable hashing across languages and runtimes
* Verifiable equality without semantic comparison
* Cross-language reproducibility
* Auditability and long-term storage correctness
* Protocol safety without hidden behavior

Without canonical encoding:

* hashes diverge
* signatures become unstable
* caches fragment
* distributed systems silently disagree

Strata chooses correctness over convenience.



***

### Canonical vs "normalized" formats

Many formats claim determinism but rely on **normalization**:

* keys reordered after parsing
* values coerced during encoding
* floats normalized implicitly
* decoders accepting multiple forms

Normalization happens _after_ ambiguity has already entered the system.

Strata rejects this model entirely.

In Strata:

* ambiguity is **not representable**
* invalid input is **rejected**
* correctness is enforced at encode time



***

### Scope of canonical rules

Canonical rules apply to:

* Binary encoding of values
* Ordering of map keys
* Integer representation
* String encoding (UTF-8)
* Byte sequences
* Hash input definition

Canonical rules **do not** apply to:

* Transport framing
* Streaming boundaries
* Envelopes or wrappers
* Compression or encryption
* Application-level protocols

These layers are explicitly outside the canonical core.



***

### Encoding vs decoding

Encoding and decoding have different responsibilities.

#### Encoding

Encoding is where truth is enforced.

* Only canonical representations may be emitted
* Invalid values are rejected
* Duplicate map keys are forbidden
* Non-canonical states cannot be produced

#### Decoding

Decoding is observational.

* Non-canonical ordering may be preserved
* Duplicate keys may exist for inspection
* No normalization is applied
* Malformed input fails explicitly

> Encoding enforces truth.\
> Decoding reveals reality.



***

### Canonical encoding and hashing

All Strata hashes are computed over **canonical encoded bytes**.

This means:

* Hashes do not depend on language
* Hashes do not depend on platform
* Hashes do not depend on runtime behavior
* Hashes are stable for the lifetime of a version line

If two implementations produce different hashes for the same value, canonical rules have been violated.



***

### Stability guarantee

Canonical encoding rules are **frozen within a version line**.

For example:

* All v0.3.x releases share identical canonical encoding
* Bytes and hashes must never change within that line
* Any change to canonical rules requires a new minor version and a new Northstar

This is a requirement, not a goal.



***

### Summary

Canonical encoding in Strata means:

* One value -> one byte sequence
* One byte sequence -> one hash
* Zero ambiguity
* Zero normalization
* Zero silent behavior

If correctness matters, canonical encoding is not optional.

It is the contract.
