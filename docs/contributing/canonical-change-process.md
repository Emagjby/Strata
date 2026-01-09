# Canonical change process

Canonical changes are the most serious class of change in Strata.

They alter **what bytes are produced**, **what hashes are computed**, or **what behavior is guaranteed**. Because of this, canonical changes follow a strict, explicit process.

There are no shortcuts.



***

### What counts as a canonical change

A change is considered **canonical** if it affects any of the following:

* Binary encoding rules
* Hashing input or algorithm
* Value model or type semantics
* Integer, string, or map ordering rules
* Decode acceptance or rejection behavior
* Error kinds or error offsets
* Cross-language determinism guarantees

If a change can cause different bytes or hashes for the same logical value, it is canonical.



***

### Canonical changes are never silent

Canonical behavior MUST NOT change:

* implicitly
* gradually
* through bug fixes
* through refactors
* through performance optimizations

If behavior changes, it must be declared.



***

### Required steps for a canonical change

Every canonical change MUST follow all steps below.

Skipping any step invalidates the change.



***

#### 1. Define the new invariant

Before any code is written, the change must be expressed as a **precise invariant**.

The invariant must answer:

* What is newly guaranteed?
* What is no longer guaranteed?
* What inputs are affected?
* What outputs change?
* Why the change is necessary

Vague goals are not acceptable. The invariant must be mechanically testable.



***

#### 2. Create a new Northstar

Every canonical change requires a **new Northstar**.

The Northstar must:

* Encode the invariant in executable form
* Fail if the invariant is violated
* Be independent of implementation details
* Run in CI
* Block regressions

No canonical change exists without a Northstar.



***

#### 3. Add or update golden vectors

Canonical changes MUST introduce new vectors.

Rules:

* Existing vectors are never modified
* New vectors live under a new version directory
* Vectors must cover:
  * Positive cases
  * Negative cases
  * Edge cases
* Vectors must fully reflect the new invariant

Vectors define the new truth.



***

#### 4. Bump the MINOR version

Canonical changes REQUIRE a new MINOR version.

Examples:

* v0.3.x → v0.4.0
* v0.4.x → v0.5.0

PATCH releases are forbidden from containing canonical changes.



***

#### 5. Update documentation explicitly

Documentation must be updated to:

* Describe the new behavior
* Describe what changed
* Describe what is no longer valid
* Describe upgrade implications

Documentation must not hide or soften the change.



***

#### 6. Update all implementations

All reference implementations must be updated to match the new vectors:

* Rust
* JavaScript
* Any future language bindings

Partial updates are not allowed.

If one implementation lags, the change is incomplete.



***

#### 7. Enforce in CI

CI must enforce the new reality:

* New Northstar must run
* New vectors must be validated
* Old vectors must still pass (unless explicitly versioned out)

If CI does not fail on violation, the change is invalid.



***

### Backward compatibility is optional, not assumed

Canonical changes MAY be backward compatible, but they are not required to be.

If backward compatibility is provided:

* It must be explicit
* It must be documented
* It must be tested
* It must not weaken guarantees

Silent compatibility assumptions are forbidden.



***

### Rejected canonical changes

A canonical change will be rejected if:

* It lacks a Northstar
* It alters behavior without a version bump
* It modifies existing golden vectors
* It relies on undefined behavior
* It introduces permissive decoding
* It weakens determinism or hash stability



***

### Philosophy

Strata treats canonical behavior as **law**, not preference.

A canonical change is not a refactor. It is a declaration of a new truth.

If the truth changes, it must be announced, tested, versioned, and enforced.



***

### Summary

Canonical changes require:

* A new invariant
* A new Northstar
* New golden vectors
* A new MINOR version
* Updated documentation
* Updated implementations
* CI enforcement

Anything less is not a canonical change.
