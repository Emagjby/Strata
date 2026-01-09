# Upgrade strategy

This page defines how Strata systems are expected to **evolve safely** over time.

Upgrades in Strata are deliberate, explicit, and verifiable.\
There is no concept of "transparent" or "automatic" upgrades.

***

### Core principle

**Upgrading Strata is a conscious act.**

If a system upgrades Strata, it must:

* know what changed
* know what stayed the same
* know which guarantees still apply

Anything else is unsafe.



***

### Version pinning is mandatory

All production systems MUST pin a Strata MINOR version.

Examples:

* v0.3.x
* v0.4.x
* v1.0.x

Relying on “latest” is forbidden for integrity-critical systems.

Pinning ensures:

* stable canonical bytes
* stable hashes
* stable decode behavior



***

### Patch upgrades (PATCH)

PATCH upgrades are always safe within a pinned MINOR version.

They may include:

* bug fixes
* performance improvements
* documentation changes
* additional tests
* tooling improvements

PATCH upgrades MUST NOT:

* change encoded bytes
* change hashes
* change decode acceptance or rejection rules

**Recommended action:**\
Auto-upgrade PATCH versions.



***

### Minor upgrades (MINOR)

MINOR upgrades introduce **new guarantees or new semantics**.

They MAY include:

* new value types
* new canonical rules
* new hashing contracts
* new Northstars
* new layers on top of core

They MAY break:

* encoded bytes
* hashes
* decoding behavior
* compatibility with previous MINOR versions

**Recommended action:**\
Treat MINOR upgrades as migrations.



***

### Major upgrades (MAJOR)

MAJOR upgrades are reserved for:

* fundamental redesigns
* breaking architectural changes
* paradigm shifts

There are currently no MAJOR upgrades defined.



***

### How to upgrade safely

A correct Strata upgrade follows this sequence:

1. Read the new Northstar documents
2. Identify which guarantees changed
3. Run vectors for both old and new versions
4. Decide which version governs stored data
5. Upgrade tooling and code
6. Re-encode data only if explicitly required

Skipping steps is a correctness violation.



***

### Data upgrade strategy

Strata never upgrades stored data implicitly.

Possible strategies:

#### Strategy A: Versioned storage

* Store data with an associated Strata version
* Decode using the version it was written with
* Encode new data using the new version

#### Strategy B: Explicit migration

* Decode old data with old version
* Transform at the Value level
* Re-encode using new version
* Accept that hashes will change

#### Strategy C: Frozen core, layered extensions

* Keep core Strata version fixed
* Add higher-level semantics externally
* Avoid touching canonical bytes



***

### Hash stability expectations

Hashes are only stable **within a version line**.

If a migration changes:

* canonical encoding
* hashing input
* value semantics

Then hash changes are expected and correct.

Assuming cross-version hash stability is a bug.



***

### Cross-language upgrades

All languages MUST upgrade together.

Invalid upgrade patterns:

* Rust on v0.4.x, JS on v0.3.x
* Mixed encoders in the same pipeline
* Partial Northstar adoption

Cross-language determinism only holds when versions match.



***

### Tooling support

Strata tooling is designed to:

* reject unsupported versions
* fail loudly on incompatibility
* surface version mismatches early

Silent fallback is forbidden.



***

### Why this strategy works

This upgrade model ensures:

* zero ambiguity
* explicit responsibility
* long-term reproducibility
* audit-friendly evolution

Systems that upgrade Strata never guess. They decide.



***

### Summary

* PATCH upgrades are safe
* MINOR upgrades are migrations
* Compatibility is explicit
* Data is never silently rewritten
* Hash changes are intentional, not accidental

Strata upgrades are boring by design.\
Boring is how correctness survives.
