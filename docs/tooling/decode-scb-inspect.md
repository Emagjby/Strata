# Decode .scb -> inspect

The `decode` command reads **Strata Core Binary (`.scb`)** and renders a **human-readable inspection view**.

This operation is **read-only**. It does not normalize, rewrite, or reinterpret data.



***

### Purpose

`decode` exists to:

* Inspect canonical Strata Core Binary
* Verify encoded structure and values
* Debug or audit `.scb` payloads
* Confirm correctness after transport or storage

It is a visibility tool, not a transformation.



***

### Command

```
strata-js decode <input.scb>
```



***

### What "inspect" means

Inspection converts binary Strata values into a **lossless, readable representation**.

Important properties:

* Every decoded value maps directly to a Strata value type
* No implicit coercion is performed
* No re-encoding occurs
* Byte-level meaning is preserved

The output is intended for humans, not machines.



***

### Decoding pipeline

The command performs the following steps:

1. **Read**
   * Raw bytes are read exactly as stored
   * No framing or envelope is assumed
2. **Decode**
   * Tags and payloads are parsed
   * Canonical varints are decoded
   * UTF-8 is validated
   * Structural rules are enforced
3. **Validate**
   * Trailing bytes are rejected
   * Invalid tags or malformed data cause failure
   * Offsets are tracked precisely
4. **Inspect**
   * The decoded value is rendered as structured JSON-like output
   * Integers are rendered as strings (to preserve full precision)
   * Bytes are rendered as byte arrays



***

### Output format

Inspection output is **not Strata Text**.

It is an informational view designed to:

* Be readable
* Preserve meaning
* Avoid ambiguity

Examples of representation:

* `int` → stringified integer
* `bytes` → array of byte values
* `map` → key-value object
* `list` → ordered array

This output is **not guaranteed to round-trip** back into `.scb`.



***

### Error handling

Decoding fails if:

* An invalid tag is encountered
* A value is truncated
* UTF-8 is malformed
* Varints overflow or terminate incorrectly
* Trailing bytes exist

On failure:

* The command exits with a non-zero code
* The error offset is reported
* No partial output is printed

Errors are precise and positional.



***

### Canonical strictness

`decode` enforces **strict canonical decoding**:

* Non-canonical encodings are rejected
* Ambiguous encodings are rejected
* Extra bytes are rejected

If decoding succeeds, the input is valid Strata Core Binary.



***

### Relationship to encoding

Decoding does not imply re-encoding.

However:

* A decoded value may be re-encoded
* Re-encoding must produce byte-identical output
* Any mismatch indicates corruption or a bug

This property is enforced by Northstar tests.



***

### Example

```
strata-js decode payload.scb
```

This:

* Reads `payload.scb`
* Validates it fully
* Prints an inspection view to stdout



***

### Summary

`decode` is the lens into canonical data.

It answers one question:

> “What does these exact bytes mean under Strata rules?”

Nothing more. Nothing less.
