# What requires a new version

Strata versions are not cosmetic. A version change signals a **change in guarantees**.

This page defines exactly which changes require crossing a version boundary and why.



***

### The core rule

If a change alters **bytes, hashes, or meaning**, it requires a **new MINOR version**.

No exceptions. No shortcuts. No "it’s basically the same".



***

### Changes that REQUIRE a new MINOR version

The following changes **always** require a new MINOR version (e.g. v0.4.0) and a new Northstar:

#### Canonical encoding changes

* Any change to binary layout
* Any change to tags
* Any change to length encoding
* Any change to ordering rules
* Any change to canonical map sorting

If the same value encodes to different bytes, a new version is mandatory.



***

#### Hashing changes

* Changing the hash algorithm
* Changing hash length
* Changing hash input definition
* Hashing anything other than canonical `.scb` bytes

Hashes are a contract. Breaking them breaks systems.



***

#### Value model changes

* Adding or removing value types
* Changing integer ranges
* Changing string semantics
* Changing byte semantics
* Introducing floats into the core model
* Introducing optional or implicit types

If existing values gain new interpretations, a new version is required.



***

#### Decode behavior changes

* Accepting data that was previously rejected
* Rejecting data that was previously accepted
* Changing error kinds
* Changing error offsets
* Relaxing or tightening strictness

Decoding behavior is part of the public contract.



***

#### Semantic meaning changes

* Changing how values are interpreted
* Changing map key uniqueness rules
* Changing integer sign behavior
* Changing UTF-8 validation rules

If meaning changes without bytes changing, it is still a breaking change.



***

#### Cross-language behavior changes

* Rust and JS diverging in behavior
* Platform-dependent behavior introduced
* Runtime-specific semantics introduced

If two implementations disagree, determinism is broken.



***

### Changes that do NOT require a new version

The following changes are allowed within a PATCH release:

#### Implementation fixes

* Bug fixes that restore documented behavior
* Performance optimizations
* Refactors with identical outputs

If bytes and hashes remain identical, the change is allowed.



***

#### Tooling and DX

* CLI improvements
* New CLI commands
* Better error messages
* Formatting tools
* Inspectors and helpers

Tooling must not alter canonical behavior.



***

#### Documentation

* Clarifications
* Corrections
* Examples
* Expanded explanations

Documentation can always improve.



***

#### Tests

* New golden vectors
* Additional negative vectors
* New Northstar tests that verify existing guarantees

Tests may grow stricter, not looser.



***

### Special case: additive layers

New features may be added **without breaking versions** if they:

* compile into canonical Strata
* do not reinterpret existing bytes
* do not alter hashes for existing values

Examples:

* new authoring formats
* new transport protocols
* verification layers
* envelopes
* framing strategies

These are layers, not core changes.



***

### Northstar requirement

Any change that requires a new version MUST be accompanied by:

* a new Northstar invariant
* CI enforcement
* documentation updates

If a change cannot be expressed as a Northstar, it is not ready.



***

### Philosophy

In Strata:

* "small" changes can be breaking
* breaking changes are treated seriously
* stability is intentional

If you are unsure whether a change requires a new version, assume it does.

Correctness beats convenience.



***

### Summary

If bytes change → new version\
If hashes change → new version\
If meaning changes → new version

Strata does not drift. It advances deliberately.
