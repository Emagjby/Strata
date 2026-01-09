# Hash

The `hash` command computes the **canonical Strata hash** of a value.

This hash is the cryptographic fingerprint of Strata data. If two hashes are equal, the underlying canonical bytes are equal.



***

### Purpose

`hash` exists to provide:

* Content addressing
* Integrity verification
* Deterministic identifiers
* Cross-language equality checks

It is the final arbiter of equality in Strata.



***

### Command

```
strata-js hash <input.st | input.scb>
```

The command accepts **either**:

* Strata Text (`.st`)
* Strata Core Binary (`.scb`)

Both paths must produce the **same hash** for equivalent data.



***

### Hashing pipeline

#### When hashing `.st`

1. Parse Strata Text into a value
2. Canonically encode the value into `.scb`
3. Hash the canonical bytes

#### When hashing `.scb`

1. Read raw bytes
2. Validate canonical decoding
3. Hash the bytes directly

There is no difference in the final result.



***

### Hash algorithm

Strata uses:

* **BLAKE3**
* Fixed output length (32 bytes)
* Hash computed over **canonical encoded bytes only**

No metadata, framing, or transport context is included.



***

### Canonical contract

The hash obeys the following rules:

* Hash(value) == Hash(encode(value))
* Hash(.st) == Hash(.scb)
* Hashes are identical across languages
* Hashes never change within a frozen version

If this contract is violated, it is a **Strata bug**.



***

### Output format

The CLI prints the hash as:

* Lowercase hexadecimal
* Fixed length
* No prefix

Example:
