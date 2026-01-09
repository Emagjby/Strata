# Golden vectors

Golden vectors are the **source of truth** for Strata behavior.

They define what Strata _is_, not what implementations _wish it to be_.

If an implementation disagrees with a golden vector, **the implementation is wrong**.



***

### What golden vectors are

Golden vectors are fixed, versioned files that describe:

* Canonical Strata Text input (`.st`)
* Canonical Strata Core Binary output (`.scb.hex`)
* Canonical hashes (`.hash.hex`)
* Required failure modes for invalid data

They are language-agnostic and shared by all implementations.

Rust, JavaScript, and any future language must consume the **same vectors**.



***

### What golden vectors define

Golden vectors define:

* Exact canonical byte sequences
* Exact hash outputs
* Exact decoding behavior
* Exact error kinds
* Exact error offsets

They eliminate ambiguity.



***

### Vector structure

Each positive vector consists of three files:

* `<name>.st`\
  Human-readable Strata Text input
* `<name>.scb.hex`\
  Canonical Strata Core Binary encoded as hex
* `<name>.hash.hex`\
  Canonical BLAKE3-256 hash of the canonical bytes

Example layout:

```
vectors/ 
    v1/ 
        01-basic.st 
        01-basic.scb.hex 
        01-basic.hash.hex
```



***

### Positive vectors

Positive vectors assert **correct behavior**.

For a positive vector, an implementation must:

1. Parse `.st` into a value
2. Encode that value into `.scb`
3. Produce bytes identical to `.scb.hex`
4. Produce a hash identical to `.hash.hex`

Any difference is a failure.



***

### Negative vectors

Negative vectors assert **required failure behavior**.

Each negative vector defines:

* Invalid input bytes (`.hex`)
* Required error kind
* Required byte offset

Example:

```
neg-01-invalid-tag.hex 
neg-01-invalid-tag.error.json
```

The implementation must fail with the specified error and offset.

Failing differently is incorrect. Succeeding is incorrect. Failing with a different offset is incorrect.



***

### Versioned by Northstar

Golden vectors are versioned by **Northstar guarantees**.

* `v1` vectors define canonical encoding and hashing
* `v2` vectors define decoding and inspection guarantees
* `v2.1` vectors define strict failure semantics

New vectors may only be added by introducing a **new Northstar version**.

Existing vectors are immutable.



***

### Rules of golden vectors

These rules are absolute:

* Vectors are never edited to satisfy code
* Code must be changed to satisfy vectors
* Vectors are shared across all languages
* Vectors define canonical truth

There are no exceptions.



***

### Enforcement in CI

Golden vectors are enforced continuously.

Every pull request must:

* Pass Rust vector tests
* Pass JavaScript vector tests
* Pass Northstar wire tests (T1, T2, T3)

If any vector fails, CI fails.

Broken determinism does not ship.



***

### Why this exists

Golden vectors exist to prevent:

* Silent regressions
* Accidental normalization
* Language drift
* “Almost compatible” implementations

They turn correctness into a binary outcome.



***

### Mental model

Think of golden vectors as:

* A cryptographic contract
* A constitutional document
* A test suite you are not allowed to modify

Strata does not evolve by breaking the past.

It evolves by freezing it.



***

### Summary

Golden vectors are not tests.

They are **law**.

Everything else in Strata exists to satisfy them.
