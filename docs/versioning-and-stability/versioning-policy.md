# Versioning policy

Strata versioning exists to protect **determinism, hashes, and trust**.

Version numbers are not cosmetic.\
They define **hard boundaries of correctness**.



***

### Version format

Strata uses semantic versioning with stricter rules than typical libraries:

v`MAJOR`.`MINOR`.`PATCH`

* **MAJOR**\
  Reserved for future use. Not currently incremented.
* **MINOR**\
  Introduces changes to canonical rules, semantics, or guarantees.
* **PATCH**\
  Fixes and additions that do **not** change bytes, hashes, or decode behavior.



***

### Stability first, iteration second

Strata optimizes for correctness over velocity.

If a change alters:

* encoded bytes
* hashes
* canonical ordering
* decode acceptance
* error semantics

it is **not a patch**.



***

### Frozen contract (v0.3.x)

As of v0.3.x, the following are **frozen and guaranteed**:

* Canonical encoding rules
* Binary layouts and tags
* Value model
* Integer semantics
* UTF-8 string rules
* Map ordering
* Hashing input and algorithm
* Decode failure behavior
* Cross-language determinism

All v0.3.x implementations must agree **byte-for-byte**.



***

### Allowed changes within a patch version

PATCH releases may include:

* Bug fixes that restore documented behavior
* Performance improvements
* Tooling improvements (CLI, helpers)
* Documentation updates
* Additional tests or vectors
* New Northstar tests that enforce existing guarantees

PATCH releases must NOT change:

* emitted bytes
* hash output
* decode acceptance or rejection rules



***

### What requires a new MINOR version

A new MINOR version is required if **anything canonical changes**.

Examples:

* Adding or modifying a value type
* Changing integer, string, or map semantics
* Altering canonical ordering rules
* Modifying hashing input or algorithm
* Tightening or relaxing decode rules
* Changing error kinds or offsets
* Any change that alters existing `.scb` bytes

If hashes change, a minor bump is mandatory.



***

### Northstar requirement

Any MINOR version change must include:

* A new Northstar invariant
* CI enforcement of that invariant
* Explicit documentation of the new guarantee

No canonical change is valid without a Northstar.



***

### Backward compatibility

Strata does **not** promise backward compatibility across MINOR versions.

If compatibility is provided, it must be:

* explicit
* documented
* tested

Silent compatibility assumptions are forbidden.



***

### Philosophy

If two systems disagree on bytes, **the version boundary was crossed**.

Strata versions communicate:

* what is frozen
* what is guaranteed
* where trust boundaries lie

If a change feels small but alters hashes, it is **not small**.



***

### Stability statement

Strata v0.3.x is stable and production-ready for its defined scope.

Future versions may expand Strata with new layers, but existing guarantees will never be weakened retroactively.
