# CLI

The Strata CLI is a **thin, deterministic interface** over the core library.

It exists to:

* Exercise canonical behavior
* Enable inspection and tooling
* Prove end-to-end correctness

It does **not** add semantics. It does **not** relax rules. It does **not** hide errors.

The CLI behaves like a systems tool, not a developer convenience wrapper.



***

### Design principles

The CLI follows four hard rules:

1. No panics on user input
2. Deterministic output for identical input
3. Explicit failure modes
4. Zero semantic interpretation

The CLI is a consumer of Strata, not a special case.



***

### Command overview

The CLI exposes four commands:

* compile
* decode
* hash
* fmt

Each command is orthogonal and composable.



***

### `compile`

Compile Strata Text (`.st`) into canonical Strata Core Binary (`.scb`).

Purpose:

* Produce canonical bytes
* Enforce encoding rules
* Reject non-canonical constructs

Usage:&#x20;

```
strata compile input.st output.scb
```

Behavior:

* Parses Strata Text
* Canonicalizes the value
* Writes exact `.scb` bytes
* Fails on any canonical violation

This command defines the **entry point into canonical truth**.



***

### `decode`

Decode Strata Core Binary (`.scb`) into a human-readable form.

Purpose:

* Inspect canonical or non-canonical data
* Debug malformed payloads
* Verify transport correctness

Usage:&#x20;

```
strata decode input.scb
```

Behavior:

* Decodes bytes into a Value
* Preserves observed structure
* Does not re-encode or normalize
* Rejects malformed input explicitly

Decoding reveals reality. It does not enforce truth.



***

### `hash`

Compute the canonical hash of a value.

Purpose:

* Content addressing
* Verification
* Integrity checks

Usage:&#x20;

```
strata hash input.st strata hash input.scb
```

Behavior:

* If input is `.st`: parse → encode → hash
* If input is `.scb`: decode → re-encode → hash
* Always hashes canonical bytes only
* Output is lowercase hex

Hashing never includes:

* Framing
* Metadata
* Transport headers
*

***

### `fmt`

Format Strata Text (`.st`) for readability.

Purpose:

* Cosmetic cleanup
* Consistent presentation
* Developer ergonomics

Usage:&#x20;

```
strata fmt input.st
```

Behavior:

* Parses Strata Text
* Emits formatted Strata Text
* Does not change semantics
* Does not affect canonical output

Formatting is explicitly **non-semantic**.



***

### Exit codes

The CLI uses stable, documented exit codes:

* 0 → Success
* 1 → Invalid input (parse, encode, decode failure)
* 2 → I/O failure
* 100 → Internal error

Exit codes are part of the contract.



***

### Error output

Errors are written to stderr.

Format:

* Short
* Structured
* Human-readable

Example:&#x20;

```
error: decode failed reason: invalid tag 0x99 offset: 12
```

No stack traces. No debugging noise. No recovery guesses.



***

### Determinism guarantees

For the same input:

* Output bytes are identical
* Hashes are identical
* Errors are identical
* Exit codes are identical

CLI determinism is required for:

* CI pipelines
* Reproducible builds
* Verification tooling



***

### Relation to Northstars

The CLI is exercised by:

* Northstar T1 (wire determinism)
* Northstar T2 (raw wire determinism)
* Northstar T3 (framed streaming determinism)

If the CLI behavior diverges, a Northstar fails.



***

### What the CLI is not

The CLI is intentionally not:

* A schema system
* A migration tool
* A validator framework
* A convenience DSL
* A debugger with recovery

Those belong in higher layers.



***

### Summary

The Strata CLI is:

* Minimal
* Deterministic
* Strict
* Test-enforced

It exists to prove correctness, not to soften it.

If the CLI accepts something invalid, Strata has failed.
