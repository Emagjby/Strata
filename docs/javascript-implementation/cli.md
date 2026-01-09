# CLI

The JavaScript implementation of Strata ships with a **fully deterministic CLI**.

The CLI is not a wrapper. It is a first-class reference tool.

It follows the same guarantees as the core library: same input, same bytes, same hash, same failure.



***

### Purpose

The JavaScript CLI exists to:

* Compile Strata Text into canonical binary
* Decode Strata Core Binary for inspection
* Hash values deterministically
* Validate cross-language parity
* Provide a zero-dependency reference workflow

It is designed for correctness, not convenience.



***

### Installation

Install from npm:

```
npm install -g @emagjby/strata-js
```

This exposes the `strata-js` executable.



***

### Command overview

The CLI exposes four commands:

* compile
* decode
* hash
* fmt

All commands are deterministic and side-effect free.



***

### compile

Compiles Strata Text (`.st`) into Strata Core Binary (`.scb`).

```
strata-js compile input.st output.scb
```

Behavior:

* Parses input as Strata Text
* Enforces grammar and integer bounds
* Produces canonical `.scb` bytes
* Overwrites output file if it exists

Failure conditions:

* ParseError
* I/O failure
* Internal error



***

### decode

Decodes Strata Core Binary into a structured, inspectable form.

```
strata-js decode input.scb
```

Behavior:

* Reads raw bytes
* Decodes strictly
* Prints a structured JSON-like representation

Notes:

* Output is for inspection only
* Ordering reflects decoded structure
* This is not Strata Text output

Failure conditions:

* DecodeError
* I/O failure



***

### hash

Computes the canonical hash of a Strata value.

```
strata-js hash input.st strata-js hash input.scb
```

Behavior:

* If input is `.st`, it is parsed and encoded first
* If input is `.scb`, bytes are hashed directly
* Output is lowercase hex
* Hash algorithm: BLAKE3-256

Guarantee:

* Hashes match Rust exactly



***

### fmt

Formats Strata Text into a normalized inspection form.

```
strata-js fmt input.st
```

Behavior:

* Parses Strata Text
* Outputs a canonical JSON-like view
* Intended for debugging and inspection

Notes:

* This is not a pretty-printer
* Formatting is not reversible
* Output is not Strata Text



***

### Exit codes

The CLI uses explicit exit codes:

* 0: Success
* 1: Invalid input (ParseError or DecodeError)
* 2: I/O error
* 100: Internal error

These codes are stable and script-safe.



***

### Error handling

The CLI does not recover from errors.

* ParseError and DecodeError are printed
* Execution stops immediately
* No partial output is produced

This ensures deterministic failure semantics.



***

### Determinism guarantees

The JavaScript CLI guarantees:

* Canonical encoding
* Stable hashing
* Cross-platform behavior
* Byte-identical output across runs

If the CLI output differs from Rust: the JavaScript implementation is wrong.



***

### Cross-language validation

The CLI is used directly in Northstar tests.

It is expected to:

* Decode Rust-produced bytes
* Re-encode identically
* Re-hash identically

Failure in CLI parity is a Northstar violation.



***

### Intended usage

The JavaScript CLI is intended for:

* Tooling
* CI pipelines
* Cross-language verification
* Debugging canonical behavior

It is not designed for casual editing or recovery workflows.



***

### Summary

The JavaScript CLI is:

* Deterministic
* Strict
* Canonical
* Reference-grade

If it succeeds, the result is correct. If it fails, the input is invalid.

There is no middle ground.
