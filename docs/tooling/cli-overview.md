# CLI overview

The Strata CLI is a **reference-grade toolchain** for working with Strata data.

It is designed for:

* Authoring Strata Text (`.st`)
* Producing canonical Strata Core Binary (`.scb`)
* Inspecting and validating encoded data
* Computing stable hashes

The CLI does not provide convenience features. It exists to expose the exact behavior of the specification.



***

### Design principles

The CLI follows the same principles as Strata itself:

* Deterministic behavior
* No hidden normalization
* No best-effort recovery
* Explicit failure on invalid input

If the CLI accepts something, it is valid Strata. If it rejects something, it is invalid Strata.



***

### Scope of the CLI

The CLI operates at three layers:

1. **Text authoring**
   * Parse `.st` into a value model
2. **Canonical encoding**
   * Encode values into `.scb`
3. **Verification**
   * Decode, inspect, and hash canonical data

It does not:

* Infer schemas
* Apply defaults
* Rewrite data
* Perform migrations
* Guess intent



***

### Supported commands

The JavaScript reference CLI exposes the following commands:

* `compile` – compile Strata Text to canonical binary
* `decode` – decode binary into a structured inspection format
* `hash` – compute canonical hashes
* `fmt` – normalize and inspect Strata Text

Each command is a thin wrapper around the reference implementation.



***

### Failure model

All CLI commands follow the same failure rules:

* Invalid Strata input exits with error code `1`
* I/O failures exit with error code `2`
* Internal errors exit with error code `100`

No command continues after an error.



***

### Determinism guarantee

Given the same input:

* The CLI always produces the same bytes
* The same hash is always printed
* Errors always occur at the same offset

This makes the CLI suitable for:

* CI enforcement
* Reproducible builds
* Cross-language debugging
* Vector generation



***

### Relationship to the specification

The CLI is not an alternative interpretation of Strata.

It is:

* A concrete execution of the spec
* A validator for implementations
* A debugging lens for vectors and Northstars

If the CLI behavior disagrees with the specification, the CLI is wrong.



***

### Relationship to other implementations

Other languages may provide their own CLIs.

However:

* All CLIs must behave identically
* Output differences are bugs
* Error differences are bugs

CLI parity is enforced indirectly through golden vectors and Northstar tests.



***

### Summary

The Strata CLI is intentionally minimal and strict.

It exists to:

* Make canonical behavior observable
* Make invalid states impossible to ignore
* Make regressions immediately visible

If you are surprised by what the CLI does,\
that surprise reveals a misunderstanding of the spec.

That is its purpose.
