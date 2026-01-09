# Golden vectors (Rust)

Golden vectors are the **highest authority** in Strata.

They define canonical truth across:

* Implementations
* Languages
* Versions
* Platforms

Code adapts to vectors. Vectors never adapt to code.



***

### What is a golden vector

A golden vector is a **complete, canonical specification sample** consisting of:

* Strata Text source (`.st`)
* Canonical Core Binary (`.scb.hex`)
* Canonical hash (`.hash.hex`)
* Optional negative cases (`.hex` + `.error.json`)

Together, these define **what must happen**, byte-for-byte.

If an implementation disagrees with a vector, the implementation is wrong.



***

### Why golden vectors exist

Golden vectors exist to eliminate ambiguity.

They ensure:

* Cross-language determinism
* Stable hashing across versions
* Reproducible verification
* Spec enforcement without prose interpretation

Vectors are executable specification.



***

### Vector directory structure

Vectors live in the shared `/vectors` directory.

Logical layout:

* v1/ → Canonical encoding rules
* v2/ → Decode and inspection behavior
* v2.1/ → Explicit failure semantics
* Future versions extend forward only

Each version is append-only.



***

### Positive vectors

A positive vector defines **valid canonical behavior**.

Each positive vector includes:

* `<name>.st`\
  Human-authored Strata Text
* `<name>.scb.hex`\
  Canonical Core Binary encoded as hex
* `<name>.hash.hex`\
  Canonical BLAKE3-256 hash of `.scb`

Required properties:

* Parsing `.st` must succeed
* Encoding must exactly match `.scb.hex`
* Hashing must exactly match `.hash.hex`

No normalization is allowed. No alternative encodings are allowed.



***

### Negative vectors

Negative vectors define **required failures**.

Each negative decode vector includes:

* `<name>.hex`\
  Raw bytes to decode
* `<name>.error.json`\
  Expected error kind and byte offset

Negative vectors assert that:

* Decoding must fail
* The failure kind is exact
* The failure offset is exact

Rejecting malformed data is part of the contract.



***

### Error precision requirement

For negative vectors, implementations must:

* Fail at the correct byte offset
* Emit the correct error category
* Not recover or guess intent

Example failure classes:

* InvalidTag
* UnexpectedEOF
* InvalidVarint
* InvalidUtf8
* TrailingBytes

Incorrect errors are considered failures.



***

### Canonical authority rule

Golden vectors override:

* Documentation prose
* Comments
* README descriptions
* Developer intent
* Historical behavior

If text and vectors disagree, vectors win.



***

### Versioning and vectors

Each vector set is tied to a Northstar version.

Rules:

* Existing vectors are immutable
* New behavior requires new vectors
* Canonical changes require a new Northstar
* Old vectors must continue to pass

This prevents silent regressions.



***

### Cross-language enforcement

All implementations must pass:

* Rust
* JavaScript
* Future languages

Vectors guarantee that: Same value → same bytes → same hash

Language differences are irrelevant.



***

### CI and enforcement

Golden vectors are enforced in CI.

Failure modes:

* Byte mismatch
* Hash mismatch
* Wrong error kind
* Wrong error offset

Any failure blocks release.



***

### What vectors do not do

Golden vectors do not:

* Describe intent
* Explain rationale
* Teach usage
* Provide ergonomics

They only state facts.



***

### Mental model

Think of golden vectors as:

* Cryptographic test cases
* Legal contracts
* Immutable records

They are not examples. They are law.



***

### Summary

Golden vectors are the backbone of Strata.

They guarantee:

* Determinism
* Stability
* Verifiability
* Trust

If vectors pass, Strata is correct. If vectors fail, Strata is broken.

There is no middle ground.
