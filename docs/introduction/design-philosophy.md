# Design Philosophy

Strata is designed around a small number of uncompromising principles. Every decision in the format follows directly from them.

This page explains the philosophy behind Strata, not its mechanics.



***

### Determinism is not optional

In Strata, determinism is a requirement, not a preference.

If two systems encode the same logical value, they must produce identical bytes. Any outcome where this is not true is considered a defect.

This principle overrides convenience, performance, and flexibility.



***

### One value, one encoding

Every Strata value has exactly one valid binary representation.

There are no equivalent encodings. There is no normalization step. There is no permissive decoding mode.

If an encoding differs, the values are not the same.

This eliminates ambiguity at the byte level and makes hashing, comparison, and verification reliable.



***

### Correctness over convenience

Many data formats optimize for developer ergonomics. Strata optimizes for correctness.

This leads to deliberate constraints:

* a minimal value model
* no floating-point numbers
* no implicit conversions
* strict validation rules

These constraints are not limitations of implementation. They are guarantees of behavior.



***

### Explicit failure is better than silent recovery

Strata rejects malformed or invalid data explicitly.

It does not attempt to:

* guess intent
* repair corrupted input
* normalize non-canonical forms

Silent recovery hides errors. Explicit failure surfaces them early.

This makes systems easier to reason about, audit, and secure.



***

### Stability is a contract

Once a Strata version is finalized, its rules do not change.

There are:

* no silent behavior changes
* no implicit migrations
* no compatibility assumptions

If a change would affect bytes, hashes, or semantics, it requires a new version boundary and a new invariant.

This allows Strata data to remain valid indefinitely.



***

### Layers, not features

Strata is intentionally small.

It does not attempt to solve:

* schema evolution
* compression
* encryption
* transport framing
* application-level semantics

These concerns are handled in higher layers.

By keeping the core minimal, Strata remains reliable and predictable at its foundation.



***

### Trust is built from invariants

Strata does not rely on conventions or "best practices". It relies on enforced invariants.

Canonical encoding, strict decoding, and shared test vectors ensure that implementations behave identically.

Trust emerges from repeatability, not interpretation.



***

### Summary

Strata is designed for systems where:

* bytes must be identical
* hashes must be stable
* behavior must be predictable
* correctness must be provable

It is intentionally strict. It is intentionally limited. Those qualities are what make it dependable.
