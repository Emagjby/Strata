# Hashing

Hashing in Strata is not a convenience feature.\
It is a **contract**.

If hashing changes, **Strata is broken**.

The JavaScript implementation follows the hashing rules defined by the Strata specification and enforced by Northstar guarantees. No shortcuts. No reinterpretation.



***

### Hashing definition

A Strata hash is defined as:

```
BLAKE3-256(canonical_scb_bytes)
```

That is the _entire_ rule.

There is:

* No salting
* No domain separation
* No schema influence
* No metadata

Only canonical bytes matter.



***

### Hashing APIs

The JavaScript implementation exposes three hashing utilities.

#### Hash raw bytes

```
hashBytes(bytes: Uint8Array): Uint8Array
```

This function:

* Accepts already-encoded Strata Core Binary
* Hashes bytes exactly as provided
* Does **not** validate canonicality
* Produces a raw 32-byte hash

This is used by:

* Northstar T2
* Northstar T3
* Raw wire verification



***

#### Hash a value

```
hashValue(value: Value): Uint8Array
```

This function:

* Canonically encodes the value
* Hashes the resulting bytes
* Returns the raw hash

This is the most common API for application usage.



***

#### Hash as hex

```
hashValueHex(value: Value): string
```

This is a convenience wrapper that:

* Calls `hashValue`
* Formats the result as lowercase hexadecimal
* Produces a stable, comparable string

Used heavily in:

* CLI output
* Golden vectors
* Tests



***

### Canonical dependency

Hashing **always depends on canonical encoding**.

That means:

* Map keys are sorted
* Integers are canonical SLEB128
* Strings are exact UTF-8 bytes
* Lists preserve order
* Bytes are preserved verbatim

If two values are logically equal, their hashes **must match**.

If hashes differ, something upstream is wrong.



***

### No hashing of decoded structures

Hashing never operates on decoded structures directly.

This is intentional.

Decoded structures may originate from:

* Non-canonical encodings
* Debug payloads
* Wire captures

Hashing requires canonical truth, not decoded reality.

Therefore:

* `hashValue` re-encodes
* `hashBytes` hashes raw bytes explicitly

The caller chooses.



***

### Determinism guarantees

JavaScript hashing is deterministic across:

* Node.js versions
* Operating systems
* CPU architectures
* Endianness
* JS runtimes

This is guaranteed by:

* Canonical encoding
* Fixed BLAKE3 parameters
* Explicit byte handling

If Rust and JS hashes differ, **JavaScript is wrong** or **Rust is wrong**. There is no third option.



***

### Relationship to Northstar tests

#### Northstar T1

T1 validates that:

* Rust encodes + hashes
* JS decodes
* JS re-encodes
* JS re-hashes

And the hashes match **bit-for-bit**.

This proves:

* Canonical parity
* Hash parity
* Cross-language determinism



***

#### Northstar T2

T2 validates hashing on **raw wire bytes**.

JS must:

* Hash raw `.scb`
* Decode
* Re-encode
* Hash again

Both hashes must match.

This proves:

* Hashing does not depend on envelopes
* No implicit normalization occurs



***

#### Northstar T3

T3 validates hashing under framed streaming transport.

Frames:

* Define boundaries
* Do not touch payload bytes

JS hashes reconstructed payloads and verifies equality.

This proves:

* Hashing survives streaming
* Hashing ignores framing



***

### Security properties

Strata hashing provides:

* Strong collision resistance (BLAKE3)
* Content-addressability
* Replay detection
* Tamper detection
* Deterministic signatures

What it does **not** provide:

* Authentication
* Encryption
* Access control

Those are higher layers.



***

### Golden vector enforcement

All JavaScript hashing is validated against golden vectors.

For each vector:

* Encode value
* Hash value
* Compare against `.hash.hex`

If the hash differs:

* The implementation must be fixed
* The vector must never be changed

Vectors are law.



***

### Failure modes

Hashing itself does not fail.

Failures occur _before_ hashing:

* Encoding errors
* Invalid values
* Parser failures

Once bytes exist, hashing always succeeds.



***

### Summary

JavaScript hashing in Strata is:

* Canonical-dependent
* Byte-exact
* Cross-language stable
* Northstar-enforced
* Non-negotiable

If two systems produce different hashes for the same value,\
Strata has already failed somewhere earlier.

Hashing is not where ambiguity is allowed.
