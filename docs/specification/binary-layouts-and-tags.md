# Binary layouts & Tags

This page defines the **binary structure** of Strata Core Binary (`.scb`) and the **type tags** used to encode values.

Binary layout is not an implementation detail. It is part of the Strata contract.



***

### Overview

Strata Core Binary is a **self-describing**, tag-based binary format.

Every encoded value consists of:

1. A **type tag** (1 byte)
2. Zero or more **payload bytes**, depending on the type

There is no padding, alignment, or implicit metadata.



***

### Design goals

The binary layout is designed to be:

* Deterministic
* Minimal
* Unambiguous
* Efficient to parse
* Independent of platform or architecture

There are no optional fields and no alternative encodings.



***

### Type tag table

Each value begins with exactly one byte identifying its type.

```
0x00 Null
0x01 False
0x02 True
0x10 Int
0x20 String
0x21 Bytes
0x30 List
0x40 Map
```

These tags are fixed and versioned.



***

### Tag semantics

#### Null

```
0x00
```

* No payload
* Represents absence of value



***

#### Boolean

```
0x02 true
0x02 true
```

* No payload
* Boolean values are explicit
* No numeric coercion is allowed



***

#### Integer

```
0x10
```

* Payload is a signed LEB128 integer
* Always interpreted as signed 64-bit
* No alternative integer widths exist



***

#### String

```
0x20
```

* Length is number of UTF-8 bytes
* Payload is raw UTF-8
* No normalization or transformation is applied



***

#### Bytes

```
0x21
```

* Length is number of bytes
* Payload is copied verbatim
* Intended for hashes, blobs, and opaque data



***

#### List

```
0x30
```

* Count specifies number of elements
* Elements are encoded sequentially
* Order is preserved and significant



***

#### Map

```
0x40
```

* Keys are always encoded as String values
* Entries MUST be in canonical order during encoding



***

### Length and count encoding

All lengths and counts use **ULEB128** encoding.

Properties:

* Compact for small values
* Unbounded in theory
* Restricted by semantic limits in Strata

Invalid or overflowing varints are decoding errors.



***

### Nested encoding

Values may be nested arbitrarily.

Example structure:

```
Int
Map
String
Int
```

There are no delimiters or terminators. Structure is defined entirely by tags and lengths.



***

### End-of-input rule

A valid `.scb` payload MUST:

* Contain exactly one root value
* Consume all bytes in the input

Trailing bytes are forbidden.



***

### Error handling

Decoders MUST fail on:

* Unknown tags
* Truncated payloads
* Invalid varints
* Invalid UTF-8
* Trailing bytes

Decoding failures are explicit and structured.



***

### Why tags matter

Fixed tags ensure:

* Fast dispatch in decoders
* Stable cross-language behavior
* No schema negotiation
* No ambiguity during parsing

Tags are part of the wire contract.



***

### Non-goals

The binary layout does not support:

* Version negotiation
* Optional fields
* Backward-compatible tag reuse
* Compression markers
* Encryption markers

Those belong outside Strata Core.



***

### Stability guarantee

Binary layout and tag assignments are frozen per version.

Changing any of the following requires a new version and Northstar:

* Tag values
* Payload structure
* Length encoding rules
* Interpretation of bytes



***

### Summary

* Every value starts with a 1-byte tag
* Payload structure is fully deterministic
* Lengths and counts use ULEB128
* Maps and lists are explicit and bounded
* No trailing bytes are allowed

The binary layout is the foundation of Strata's determinism and safety.
