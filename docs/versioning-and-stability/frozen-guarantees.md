# Frozen guarantees

Frozen guarantees define the **non-negotiable contract** of a Strata version line.

Once a guarantee is frozen, it **must not change** without crossing a version boundary and introducing a new Northstar invariant.

Frozen means immutable, not deprecated.



***

### What "frozen" means in Strata

A frozen guarantee is:

* enforced by tests
* enforced by CI
* relied upon by downstream systems
* safe to depend on indefinitely within a version line

If a frozen guarantee is violated, **the implementation is wrong**, not the data.



***

### Frozen guarantees in v0.3.x

As of **v0.3.x**, the following guarantees are frozen:

#### Canonical encoding

* Every Strata value has exactly one valid binary representation
* No alternate encodings are permitted
* No normalization after decoding

#### Binary layout

* Tag values
* Length encodings
* Byte order
* Structural layout of lists and maps

#### Value model

* Supported value types
* Absence of floats
* Absence of optional types
* Absence of implicit coercions

#### Integer semantics

* Signed 64-bit integer range
* SLEB128 encoding rules
* Overflow behavior

#### String semantics

* UTF-8 encoding
* Invalid UTF-8 rejection
* No normalization or transformation

#### Map semantics

* String keys only
* Canonical UTF-8 byte ordering
* Unique keys in canonical form

#### Hashing contract

* Hashes computed over canonical `.scb` bytes only
* Hash input excludes transport, framing, and metadata
* Hash output length and algorithm

#### Decode behavior

* Strict decoding
* Exact failure modes
* Error kinds and offsets
* Rejection of trailing bytes

#### Cross-language determinism

* Rust and JavaScript must produce identical bytes
* Rust and JavaScript must produce identical hashes
* Disagreement is treated as a regression



***

### What frozen guarantees protect

Frozen guarantees exist to protect:

* content addressing
* cryptographic hashing
* signatures
* distributed consensus
* cache keys
* reproducible builds
* long-term storage

Once data is written, its meaning must never drift.



***

### What frozen guarantees do NOT include

Frozen guarantees do **not** cover:

* transport protocols
* framing formats
* envelopes
* compression
* encryption
* schema systems
* developer ergonomics
* CLI UX

These may evolve freely as long as canonical bytes remain unchanged.



***

### Adding new capabilities safely

New features may be added by:

* introducing new layers above canonical encoding
* introducing new formats that compile into canonical Strata
* introducing new tooling or protocols
* introducing new Northstar invariants

New capabilities must not reinterpret existing bytes.



***

### Violation rules

If a change:

* alters canonical bytes
* alters hashes
* alters decode acceptance
* alters error semantics

then one of the following is required:

* new MINOR version
* new Northstar
* updated documentation
* explicit CI enforcement

If none of these are present, the change is invalid.



***

### Stability promise

Frozen guarantees in v0.3.x will not change.

Future versions may extend Strata, but they will do so by **adding layers**, not by rewriting the past.

This is a promise, not a goal.
