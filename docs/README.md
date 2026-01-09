# What is Strata

Strata is a deterministic data format with canonical encoding.

It defines a **single, unambiguous binary representation** for structured data. If two systems encode the same logical value, they **must** produce identical bytes and identical hashes.

This is not a convenience feature. It is a correctness guarantee.

Strata is designed for systems where byte-level stability matters more than flexibility, and where differences in encoding are considered bugs, not acceptable variation.



***

### Deterministic by design

Most data formats allow multiple valid encodings for the same value. Strata does not.

In Strata:

* Every value has exactly **one valid binary form**
* There are no equivalent encodings
* There is no normalization step
* There is no permissive decoding

If two encoders produce different bytes for the same value, at least one of them is wrong.

Determinism is enforced, not suggested.



***

### Canonical encoding

Strata's canonical encoding rules define:

* how values are represented in binary
* how integers are encoded
* how strings are encoded
* how maps are ordered
* what constitutes valid and invalid data

Canonical rules are **frozen per version**. Once finalized, they do not change.

This guarantees that data encoded today will produce the same bytes and hashes in the future, across all compliant implementations.



***

### Binary core, textual authoring

Strata consists of two layers:

* **Strata Core Binary (`.scb`)**\
  The canonical binary format.\
  This is the source of truth for hashing, storage, and transport.
* **Strata Text (`.st`)**\
  A human-readable authoring format that compiles into canonical `.scb`.\
  `.st` is a convenience layer. `.scb` is the truth.

Only `.scb` participates in determinism guarantees.



***

### What Strata is not

Strata is intentionally limited.

It is **not**:

* a schema system
* a self-describing format
* a compression format
* a streaming protocol
* a cryptographic container
* a general-purpose serialization framework

It does not support:

* floating-point numbers
* optional fields
* default values
* backward-compatible schema evolution

These constraints are deliberate. Flexibility in representation introduces ambiguity. Strata chooses precision over convenience.



***

### When Strata makes sense

Strata is designed for systems where:

* data is content-addressed or hashed
* hashes must be stable across languages
* byte-level equality matters
* correctness depends on reproducibility
* data may outlive any single implementation

If your system requires "mostly the same" bytes, Strata is not the right tool. If it requires **exactly the same bytes**, Strata exists for that purpose.



***

### Summary

Strata is a deterministic data format with canonical encoding.

Same value -> same bytes -> same hash\
Always. Everywhere. Or it is a bug.
