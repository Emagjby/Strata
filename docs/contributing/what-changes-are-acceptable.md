# What changes are acceptable

Strata draws a hard line between **acceptable evolution** and **semantic drift**.

This page defines which changes may be merged without violating Strata’s guarantees, and which changes are forbidden without a new version boundary and Northstar.

If a change is not listed as acceptable here, assume it is **not acceptable**.



***

### Acceptable changes (no version bump)

The following changes are allowed within a frozen version line (e.g. v0.3.x):

#### Documentation

* Clarifying existing behavior
* Correcting inaccuracies or ambiguities
* Adding examples that reflect canonical behavior
* Expanding explanations of invariants

Documentation must describe reality, not redefine it.



***

#### Tests and verification

* Adding new golden vectors that reflect existing rules
* Adding negative vectors for malformed input
* Adding semantic vectors that assert invariants
* Strengthening CI enforcement
* Adding new Northstar tests that verify existing guarantees

Tests may tighten enforcement, but must not relax it.



***

#### Bug fixes (restorative only)

Bug fixes are acceptable **only if** they restore documented behavior.

Examples of acceptable fixes:

* Fixing an encoder that emits non-canonical bytes
* Fixing a decoder that accepts invalid input
* Fixing incorrect error offsets or error kinds
* Fixing cross-language inconsistencies

A bug fix must not introduce new semantics.



***

#### Performance improvements

* Optimizations that do not alter encoded bytes
* Optimizations that do not alter hashing output
* Optimizations that do not alter error behavior
* Optimizations that do not rely on undefined behavior

Performance is allowed only when behavior remains identical.



***

#### Tooling and DX (non-canonical)

* CLI improvements
* New CLI commands or flags
* Better inspection or formatting tools
* Debug helpers
* Developer utilities

Tooling must not influence canonical encoding or hashing.



***

### Conditionally acceptable changes (require discussion)

The following changes are not automatically rejected, but require explicit discussion and usually a new Northstar:

* New value types
* Changes to integer semantics
* Changes to string or UTF-8 handling
* Changes to map ordering rules
* Changes to framing or transport guarantees
* New layers built on top of Strata Core

These changes almost always require a new MINOR version.



***

### Unacceptable changes (will be rejected)

The following changes are **never acceptable** within a frozen version line:

* Any change that alters canonical bytes
* Any change that alters hash output
* Any change that relaxes decoding strictness
* Any change that introduces permissive decoding
* Any change that breaks existing golden vectors
* Any change that introduces implicit coercion
* Any change that hides or ignores errors

If bytes or hashes change, the change is not acceptable without a version bump.



***

### Golden vector rule

Golden vectors are law.

* Code must match vectors
* Vectors are never changed to satisfy code
* If behavior differs from vectors, the code is wrong

Any change that invalidates existing vectors is unacceptable.



***

### Northstar rule

Any change that affects semantics MUST be accompanied by:

* A new Northstar invariant
* CI enforcement of that invariant
* Documentation of the new boundary

No exceptions.



***

### Practical guideline

If you are unsure whether a change is acceptable, ask:

> Would this change allow two implementations to produce different bytes or hashes?

If yes, it is not acceptable without a new version and Northstar.



***

### Summary

Acceptable changes:

* Improve clarity
* Improve correctness
* Improve confidence
* Improve tooling

Unacceptable changes:

* Alter meaning
* Alter bytes
* Alter hashes
* Alter guarantees

Strata evolves deliberately, not accidentally.
