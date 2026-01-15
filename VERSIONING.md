# Strata Versioning Policy

This document defines how Strata versions evolve and what changes are permitted
within a version line.

Strata versioning prioritizes **correctness and determinism**
over rapid iteration or convenience.

---

## Versioning Model

Strata follows semantic versioning with additional constraints:

```
vMAJOR.MINOR.PATCH
```

- **MAJOR**: Reserved for future use
- **MINOR**: New invariant boundaries (canonical, semantic, or contractual)
- **PATCH**: Non-breaking fixes and additions within an existing invariant

Version numbers in Strata are **contract boundaries**, not cosmetics.

---

## v0.3.x Stability Contract

As of **v0.3.x**, Strata’s core canonical model is considered **frozen**.

The following are guaranteed stable across all v0.3.x releases:

- Canonical binary encoding
- Hashing semantics
- Value model
- Decode behavior and failure modes
- Cross-language determinism guarantees

Any change affecting the above is **not permitted** in v0.3.x.

---

## Allowed Changes in v0.3.x

The following changes are allowed and do **not** require a new minor version:

- Bug fixes that restore documented behavior
- Performance improvements that do not affect bytes or hashes
- Documentation updates
- Additional tooling (CLI flags, helpers)
- New tests or test vectors
- New Northstar tests that verify existing guarantees

PATCH releases must not change:

- encoded bytes
- hashes
- decode acceptance or rejection rules

---

## v0.4.x Stability Contract (DX Line)

As of **v0.4.0**, Strata introduces a **DX-only minor version line**.

v0.4.x explicitly guarantees:

- Canonical encoding is unchanged from v0.3.x
- Hashing semantics are unchanged
- Value model is unchanged
- Decode behavior is unchanged
- Cross-language determinism is preserved

The purpose of v0.4.x is to improve:

- Developer ergonomics
- Construction APIs and helpers
- Documentation and integration clarity
- Test coverage and enforcement

No change in v0.4.x may alter canonical bytes or hashes.

---

## Allowed Changes in v0.4.x

The following changes are allowed within v0.4.x:

- Additive DX APIs (macros, helpers, factories)
- Documentation additions and corrections
- New tests and stronger enforcement
- CLI UX improvements that do not affect output bytes
- Refactors that preserve observable behavior

PATCH releases within v0.4.x must not:

- change encoded bytes
- change hashes
- relax or tighten decode rules
- change value semantics

---

## Changes That Require a New Minor Version

Any of the following changes require a **new MINOR version**
(e.g. v0.5.0) and a new Northstar:

- Changes to canonical encoding
- Changes to hashing input or algorithm
- Changes to the value model
- Changes to integer, string, bytes, list, or map semantics
- Relaxing or tightening decode rules
- Any change that alters encoded bytes for existing values

If bytes or hashes change, a minor version bump is mandatory.

---

## Northstar Requirement

All changes that introduce new invariants MUST be accompanied by:

- A new Northstar defining the invariant
- CI enforcement of the invariant
- Explicit documentation of the change

No canonical change is valid without a Northstar.

---

## Backward Compatibility

Strata does not guarantee backward compatibility across MINOR versions.

Backward compatibility, if provided, must be:

- explicit
- documented
- enforced by tests

Silent compatibility assumptions are forbidden.

---

## Philosophy

Version numbers in Strata are not cosmetic.

A version bump signals:

- a change in guarantees
- a new invariant boundary
- intentional breakage or extension

If a change feels small but alters bytes or hashes, it is **not small**.

---

## Stability Statement

- **v0.3.x** is canonically frozen for its defined scope
- **v0.4.x** is a DX-only extension line with identical canonical guarantees

Future versions may expand capabilities, but existing guarantees
will not be weakened retroactively.
