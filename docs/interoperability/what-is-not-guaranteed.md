# What is NOT guaranteed

Strata is intentionally strict. That also means it is intentionally narrow.

This page defines what Strata **does not promise**, and why those omissions are deliberate.

***

### Schema evolution

Strata does **not** guarantee:

* Backward-compatible schema changes
* Forward-compatible decoding
* Optional fields
* Default values
* Field presence inference

There is no schema layer in core Strata.

If you need schema evolution, it must exist **outside** the format.



***

### Field compatibility

Strata does **not** guarantee:

* That a consumer understands new fields
* That missing fields can be inferred
* That renamed fields can be mapped automatically

Strata encodes values, not meaning.



***

### Floating-point behavior

Strata does **not** guarantee:

* IEEE 754 semantics
* Rounding behavior
* NaN propagation
* Cross-language float equality

Floating-point numbers are excluded from core precisely because they are not deterministic across environments.



***

### Numeric coercions

Strata does **not** guarantee:

* Automatic int ↔ float coercion
* Implicit narrowing or widening
* Numeric compatibility across types

Integers are integers. Nothing else is assumed.



***

### Performance characteristics

Strata does **not** guarantee:

* Encoding speed
* Decoding speed
* Memory allocation patterns
* Zero-copy behavior
* Streaming decode performance

Correctness comes first. Performance is implementation-specific.



***

### Memory layout

Strata does **not** guarantee:

* In-memory representation
* Object layout
* Struct alignment
* Allocation strategies

Only the wire format is defined.



***

### Error message text

Strata does **not** guarantee:

* Identical error strings
* Identical stack traces
* Identical exception types

It guarantees error _classes_, not wording.



***

### Ordering beyond maps

Strata does **not** guarantee ordering for:

* Map iteration after decoding
* Object field order in host languages
* JSON serialization output

Ordering is only guaranteed in the canonical byte encoding.



***

### Transport semantics

Strata does **not** guarantee:

* Delivery
* Reliability
* Ordering
* Security
* Authentication

Those belong to the transport layer.



***

### Compression and encryption

Strata does **not** guarantee:

* Compression
* Encryption
* Signatures
* Authentication tags

These must be layered externally.



***

### Cross-version compatibility

Strata does **not** guarantee:

* Compatibility across major versions
* Automatic migrations
* Silent upgrades

Each finalized version is frozen. New versions may add layers, but old rules do not change.



***

### Developer ergonomics

Strata does **not** guarantee:

* Easy debugging
* Human-readable binaries
* Friendly diffs
* Minimal boilerplate

Strata is optimized for correctness, not comfort.



***

### Summary

Strata does not try to be everything.

It intentionally does **not** promise:

* Schemas
* Floats
* Flexibility
* Convenience
* Evolution magic

What it promises instead is simpler:

If something works, it works _everywhere_. If something breaks, it breaks _loudly_.

Everything else is someone else’s job.
