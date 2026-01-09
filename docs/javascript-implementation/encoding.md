# Encoding

The JavaScript encoder is responsible for producing **canonical Strata Core Binary** (`.scb`) from an in-memory Strata value.

This encoder is **not permissive**. It does not guess. It does not normalize. It does not forgive.

Same value in -> same bytes out. Always.



***

### Role of the encoder

Encoding is the act of turning a structured Strata value into bytes.

In JavaScript, encoding:

* Consumes a validated `Value`
* Emits canonical binary
* Enforces ordering rules
* Produces bytes suitable for hashing and transport

Encoding is where **truth is enforced**.



***

### Entry point

The public entry point is:

```
encodeValue(value: Value): Uint8Array
```

This function:

* Allocates a fresh byte buffer
* Walks the value recursively
* Emits tags, lengths, and payloads in canonical order
* Returns raw bytes

It never mutates the input value.



***

### Tag-first encoding

Every encoded value begins with a **type tag**.

Tags are single bytes that identify the value kind.

Examples:

* null → 0x00
* false → 0x01
* true → 0x02
* int → 0x10
* string → 0x20
* bytes → 0x21
* list → 0x30
* map → 0x40

The tag fully determines how the following bytes are interpreted.



***

### Integer encoding

Integers are encoded as:

* Tag: 0x10
* Payload: canonical SLEB128

Rules:

* Input value MUST be a `bigint`
* Value MUST fit signed 64-bit range
* Encoding MUST be minimal
* No leading zero or sign-extension bytes

```
out.push(TAG_INT) 
pushBytes(out, encodeSLEB128(value))
```

If two integers are numerically equal, their byte encoding is identical.



***

### String encoding

Strings are encoded as:

* Tag: 0x20
* Length: ULEB128 (byte length)
* Payload: raw UTF-8 bytes

Rules:

* Length is measured in bytes, not characters
* UTF-8 encoding is exact
* No normalization
* No escaping
* No terminator byte

```
out.push(TAG_STRING) 
pushBytes(out, encodeULEB128(len)) 
pushBytes(out, utf8Bytes)
```



***

### Bytes encoding

Bytes are encoded as:

* Tag: 0x21
* Length: ULEB128
* Payload: raw bytes

Rules:

* Bytes are preserved verbatim
* No interpretation
* No transformation

```
out.push(TAG_BYTES) 
pushBytes(out, encodeULEB128(len)) 
pushBytes(out, value.value)
```



***

### List encoding

Lists are encoded as:

* Tag: 0x30
* Count: ULEB128
* Payload: encoded elements in order

Rules:

* Order is preserved exactly
* Count is the number of elements
* Each element is encoded recursively

```
out.push(TAG_LIST) 
pushBytes(out, encodeULEB128(count)) 
for item in list: encodeInto(item)
```

Lists are order-sensitive by definition.



***

### Map encoding

Maps are encoded as:

* Tag: 0x40
* Count: ULEB128
* Payload: key-value pairs in canonical order

#### Canonical ordering

Before encoding, map entries are sorted by: **UTF-8 byte lexicographic order of keys**

Not locale order. Not Unicode code points. Raw UTF-8 bytes.

```
entries.sort(compareUtf8Bytes)
```

This guarantees cross-language determinism.



***

#### Map entry encoding

Each entry is encoded as:

1. Key (encoded exactly like a String)
2. Value (encoded recursively)

```
out.push(TAG_STRING) 
encode key length 
encode key bytes 
encode value
```

Keys are always strings. Any other key type is invalid by definition.



***

### Canonical guarantees

The encoder guarantees:

* Deterministic output
* No alternative encodings
* Stable hashes
* Cross-language equivalence

If two independent implementations encode the same value, their byte output MUST match exactly.

Anything else is a bug.



***

### What the encoder does not do

The encoder does NOT:

* Validate schemas
* Deduplicate values
* Reject semantic duplicates
* Enforce business rules
* Normalize strings
* Coerce types

Its job is narrow and absolute.



***

### Failure behavior

Encoding can fail only for structural violations:

* Invalid UTF-8 (non-JS implementations)
* Invalid integer values
* Internal invariants violated

Failures are explicit and synchronous.



***

### Relationship to hashing

Hashing is defined as:

```
hash = BLAKE3(encodeValue(value))
```

Encoding correctness directly determines hash correctness.

If encoding is wrong, hashing is wrong. There is no recovery layer.



***

### Summary

JavaScript encoding in Strata is:

* Recursive
* Deterministic
* Canonical
* Order-enforcing
* Hash-safe

Encoding is not a convenience layer. It is the gatekeeper of truth.

Once bytes are emitted, reality is fixed.

