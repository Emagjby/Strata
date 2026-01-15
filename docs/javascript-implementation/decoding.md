# Decoding

Decoding is the inverse of encoding, but **not** its mirror.

Encoding enforces truth.\
Decoding reveals reality.

The JavaScript decoder takes raw Strata Core Binary (`.scb`) bytes and reconstructs a structured `Value` while **preserving all semantic guarantees** and **rejecting malformed input explicitly**.

***

### Role of the decoder

Decoding exists to answer one question only:

> "What do these bytes actually say?"

It does **not**:

* Normalize
* Repair
* Guess intent
* Rewrite structure

If the bytes are wrong, decoding fails. If the bytes are non-canonical, decoding still succeeds, but the structure is preserved exactly.

***

### Entry point

The public API is:

```
decodeValue(input: Uint8Array): Value
```

This function:

* Consumes raw bytes
* Walks the byte stream sequentially
* Reconstructs a `Value`
* Ensures the input is fully consumed

If any bytes remain after decoding, decoding fails.

***

### Decoder model

Decoding is implemented as a **stateful cursor** over the byte array.

Internal state:

* `offset`: current read position
* `input`: immutable byte buffer

All reads advance the cursor. All errors report the exact offset where decoding failed.

***

### Tag dispatch

Decoding always begins by reading a single tag byte.

```
tag = readByte()
```

The tag determines:

* Value type
* How many bytes follow
* How to interpret the payload

Unknown tags are **fatal errors**.

***

### Primitive decoding

#### Null and booleans

Tags:

* 0x00 → null
* 0x01 → false
* 0x02 → true

These values have no payload.

```
return Value.null() | Value.bool(false) | Value.bool(true)
```

***

### Integer decoding

Integers are decoded as:

* Tag: 0x10
* Payload: SLEB128

Rules:

* Decoded as `bigint`
* Must fit signed 64-bit range
* Overflow is rejected
* Truncated input is rejected

```
decodeSLEB128(input, offset)
```

Any malformed varint results in `InvalidVarint`.

***

### String decoding

Strings are decoded as:

* Tag: 0x20
* Length: ULEB128
* Payload: UTF-8 bytes

Rules:

* Length is byte count
* UTF-8 decoding is strict
* Invalid UTF-8 is rejected
* No normalization is applied

```
bytes = readSlice(len) 
string = TextDecoder(fatal=true).decode(bytes)
```

UTF-8 failure produces `InvalidUtf8` at the byte offset where decoding started.

***

### Bytes decoding

Bytes are decoded as:

* Tag: 0x21
* Length: ULEB128
* Payload: raw bytes

Rules:

* Bytes are preserved exactly
* No interpretation
* No transformation

```
return Value.bytes(bytes)
```

***

### List decoding

Lists are decoded as:

* Tag: 0x30
* Count: ULEB128
* Payload: `count` encoded values

Rules:

* Order is preserved exactly
* Count controls iteration
* Nested decoding is recursive

```
for i in 0..count: items.push(decodeValue())
```

Truncated lists fail with `UnexpectedEOF`.

***

### Map decoding

Maps are decoded as:

* Tag: 0x40
* Count: ULEB128
* Payload: key-value pairs

Rules:

* Keys MUST decode to strings
* Values may be any valid value
* Duplicate keys are allowed
* Insertion order reflects wire order

```
key = decodeValue() 
if key.kind !== "string" -> error value = decodeValue()
```

Maps are stored in a `Map<string, Value>`.

***

### Duplicate keys

Decoding **does not reject duplicate keys**.

If a key appears multiple times:

* The last value wins in the JS map
* Earlier entries are overwritten
* No error is raised

This is intentional and mirrors Rust behavior.

Decoding shows reality. Encoding enforces rules.

***

### Trailing bytes check

After decoding a single root value, the decoder checks:

```
if remaining() !== 0 -> TrailingBytes
```

This prevents:

* Concatenated payloads
* Framing leaks
* Ambiguous inputs

A valid `.scb` file encodes exactly one value.

***

### Error model

All decoding failures throw `DecodeError`.

Error kinds:

* InvalidTag
* UnexpectedEOF
* InvalidVarint
* InvalidUtf8
* TrailingBytes

Each error includes:

* Exact byte offset
* Structured error kind

No silent failures. No partial values.

***

### Canonical vs non-canonical input

The decoder:

* Accepts non-canonical encodings
* Preserves structure
* Allows re-encoding into canonical form

This enables:

* Inspection
* Debugging
* Migration
* Verification

Canonicality is enforced at **encode time**, not decode time.

***

### Relationship to Northstar tests

Northstar T2 and T3 rely directly on decoder correctness.

If decoding:

* Drops bytes
* Reorders data
* Misinterprets tags

Then wire determinism is broken.

The decoder is therefore a **security boundary**.

***

### Summary

JavaScript decoding in Strata is:

* Strict
* Offset-aware
* Non-normalizing
* Structure-preserving
* Failure-explicit

It does not lie. It does not guess. It tells you exactly what the bytes mean, or it refuses to speak.
