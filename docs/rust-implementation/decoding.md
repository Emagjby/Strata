# Decoding

Decoding is the act of converting **Strata Core Binary (.scb)** bytes back into an in-memory Strata value.

In Rust, decoding is **strict**, **explicit**, and **non-normalizing**.

Decoding reveals what is on the wire.\
It does not correct it. It does not reinterpret it. It does not forgive it.



***

### Core principle

Decoding answers one question only:

> "Are these bytes a valid Strata value according to the specification?"

If the answer is no, decoding must fail.

There is no partial success and no silent recovery.



***

### Entry point

Decoding starts from a byte slice and produces a `Value`.

```
decode(input: &[u8]) -> Result<Value, DecodeError>
```

The decoder walks the byte stream sequentially and consumes exactly one value.



***

### Single-value contract

A valid Strata payload contains **exactly one value**.

Rules:

* One value must decode successfully
* All bytes must be consumed
* Any trailing bytes are an error

Trailing data is never ignored.

This rule is essential for hash stability and wire safety.



***

### Tag-driven decoding

Decoding begins by reading a single tag byte.

The tag determines:

* Which value type is expected
* How many bytes must follow
* How to interpret the payload

Unknown tags are rejected immediately.

There is no extension space inside a version.



***

### Integer decoding

Integers are decoded as:

* Canonical SLEB128
* Interpreted as signed 64-bit integers

Rules:

* Varint overflow is rejected
* Truncated varints are rejected
* Values outside i64 range are rejected

No clamping. No wrapping.



***

### String decoding

Strings are decoded as:

* Length via canonical ULEB128
* Exact number of bytes
* UTF-8 validation

Rules:

* Length must be satisfied exactly
* UTF-8 must be valid
* Invalid UTF-8 is an error
* Truncation is an error

Decoded strings are not normalized or transformed.



***

### Bytes decoding

Bytes are decoded as:

* Length via canonical ULEB128
* Raw byte slice copied verbatim

Rules:

* Length must be satisfied exactly
* No interpretation is applied
* Zero-length bytes are allowed

Bytes remain bytes.



***

### List decoding

Lists are decoded as:

* Element count via canonical ULEB128
* Recursive decoding of each element

Rules:

* Element count must be satisfied exactly
* Truncation is an error
* Nested decoding errors bubble up immediately

List order is preserved exactly as encoded.



***

### Map decoding

Maps are decoded as:

* Entry count via canonical ULEB128
* Repeated key-value pairs

Rules:

* Keys must decode as strings
* Any non-string key is an error
* Duplicate keys are allowed during decoding
* Last key wins in the resulting map

Decoding does not enforce canonical ordering.\
Encoding does.



***

### Non-canonical data

Decoding deliberately accepts some non-canonical forms:

* Unsorted map entries
* Duplicate keys
* Values produced by older or incorrect encoders

This is intentional.

Decoding exposes reality.\
Encoding enforces truth.



***

### Error semantics

All decoding failures are explicit and typed.

Common errors include:

* InvalidTag
* UnexpectedEOF
* InvalidVarint
* InvalidUtf8
* TrailingBytes

Each error includes an exact byte offset.

There is no generic "invalid data" error.



***

### Safety guarantees

The decoder:

* Never panics on malformed input
* Never reads out of bounds
* Never allocates unbounded memory
* Never assumes correctness

Every byte is accounted for.



***

### Decoding vs encoding

Decoding is permissive enough to inspect reality.\
Encoding is strict enough to define canonical truth.

Roundtrip rule:\
decode(encode(value)) == value

The reverse is intentionally not guaranteed.



***

### Stability guarantee

Within a finalized version:

* Decoding rules are frozen
* Error semantics are frozen
* Byte offsets must remain stable

Changing decoding behavior requires a new Northstar and a new version.



***

### Summary

Rust decoding is:

* Strict
* Transparent
* Explicit
* Auditable

If decoding feels unforgiving, that is by design.

Forgiveness is how corruption hides.
