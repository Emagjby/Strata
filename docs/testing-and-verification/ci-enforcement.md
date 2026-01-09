# CI enforcement

Continuous Integration (CI) is not optional in Strata.\
It is the mechanism that **enforces truth**.

Every commit, in every language, is validated against the same immutable rules.

If CI fails, the change does not exist.



***

### Why CI is mandatory

Strata makes strong guarantees:

* Canonical encoding
* Stable hashing
* Cross-language determinism
* Explicit failure semantics

These guarantees are meaningless without automated enforcement.

CI is the system that ensures no regression ever ships.



***

### What CI enforces

CI enforces all of the following, on every pull request and push:

* Golden vector correctness
* Semantic correctness
* Negative failure behavior
* Cross-language parity
* Northstar guarantees

No human review can override this.



***

### Golden vector enforcement

CI runs tests that:

* Encode Strata Text vectors
* Compare produced `.scb` bytes exactly
* Compare produced hashes exactly
* Reject any deviation

Vectors are loaded directly from the `/vectors` directory.

If an implementation disagrees with vectors, the implementation is wrong.



***

### Positive vector enforcement

CI verifies that:

* Valid inputs parse successfully
* Valid values encode canonically
* Valid hashes match expected outputs
* Roundtrips preserve structure

These tests ensure normal behavior is correct.



***

### Negative vector enforcement

CI verifies failure behavior.

For malformed inputs, tests assert:

* The exact error kind
* The exact byte offset
* The exact failure mode

Silently accepting invalid data is forbidden.



***

### Semantic vector enforcement

CI enforces semantic correctness by asserting that:

* Equivalent syntax produces identical values
* Ordering differences normalize correctly
* Shorthand syntax resolves deterministically
* Duplicate keys follow defined overwrite rules

Parsing behavior is locked down permanently.



***

### Northstar enforcement

CI runs Northstar tests as **independent pipelines**.

Each Northstar tier validates a stronger guarantee:

* T1: Canonical bytes survive structured wire transfer
* T2: Raw bytes survive unstructured transport
* T3: Framed streaming does not mutate payloads

If any Northstar test fails, the build fails.



***

### Cross-language CI matrix

CI runs independently for:

* Rust implementation
* JavaScript implementation
* Northstar wire tests (Rust ↔ JS)

All implementations must pass against the same vectors.

There is no “reference implementation” exemption.



***

### No conditional skips

CI does not allow:

* Ignoring failing tests
* Skipping vector checks
* Allowing warnings for determinism failures

Any failure is a hard stop.



***

### Versioned enforcement

CI is version-aware.

* Existing vectors must always pass
* New vectors may only be added with a new Northstar
* Old vectors are never removed or altered

Breaking changes require explicit version bumps and documentation.



***

### Human role in CI

Humans may:

* Propose changes
* Add new vectors
* Define new Northstars

Humans may not:

* Override CI failures
* Redefine existing vector meaning
* Merge failing code

CI is final.



***

### Summary

CI is the guardian of Strata’s guarantees.

* Code can lie
* Documentation can drift
* Humans can make mistakes

CI does not.

If CI passes, the change is valid.\
If CI fails, the change does not exist.
