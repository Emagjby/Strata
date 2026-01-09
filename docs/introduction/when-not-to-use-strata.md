# When NOT to use Strata

Strata is intentionally strict. It trades flexibility and convenience for determinism and correctness.

If those tradeoffs do not align with your needs, Strata is not the right tool.

This is by design.



***

### Do not use Strata for evolving schemas

Strata does not support:

* optional fields
* default values
* field versioning
* backward-compatible schema evolution

Once a value is encoded, its meaning is fixed by its structure. Adding, removing, or reshaping fields requires explicit agreement at a higher layer.

If your system depends on gradually evolving schemas with soft compatibility rules, Strata will feel restrictive.



***

### Do not use Strata if you need floating-point numbers

Strata does not support floating-point values.

This is intentional.

Floating-point numbers introduce:

* multiple valid binary representations
* platform-specific behavior
* rounding ambiguity
* non-deterministic serialization

If your domain depends on floats, scientific computing, or approximate values, Strata is not appropriate.



***

### Do not use Strata for human-facing data interchange

Strata Core Binary (`.scb`) is not designed to be:

* human-readable
* self-describing
* forgiving of mistakes

Strata Text (`.st`) exists for authoring, but the source of truth is binary.

If humans are expected to read, write, or debug raw wire data frequently, Strata will slow them down.



***

### Do not use Strata if permissive decoding is acceptable

Strata rejects invalid or non-canonical data explicitly.

It does not:

* coerce types
* normalize values
* accept multiple encodings
* silently repair malformed input

If your system expects "best effort" decoding or tolerance for inconsistent data, Strata will be frustrating.



***

### Do not use Strata when flexibility matters more than correctness

Strata prioritizes correctness over convenience.

That means:

* fewer value types
* no implicit behavior
* strict boundaries
* slower evolution

If rapid iteration, loose contracts, or developer ergonomics are the primary goal, other formats are a better fit.



***

### Do not use Strata as a transport protocol

Strata defines **data encoding**, not transport.

It does not include:

* framing rules
* streaming semantics
* compression
* encryption
* authentication

Those concerns must be handled externally.

If you need an all-in-one wire protocol, Strata is intentionally incomplete.



***

### Summary

Strata is not designed for:

* flexible schemas
* floating-point data
* permissive decoding
* rapid evolution
* human-oriented wire formats

These are not missing features. They are deliberate exclusions.

Strata exists for systems where correctness must be provable and bytes must be identical. If that is not your problem, choose a different tool.
