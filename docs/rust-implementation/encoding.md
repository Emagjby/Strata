# Encoding

Encoding is the act of converting an in-memory Strata value into **Strata Core Binary (.scb)**.

In Rust, encoding is **fully deterministic**, **side-effect free**, and **canonical by construction**.

There is exactly one valid byte representation for any given Strata value.



***

### Core guarantee

For any value `V`:

* Encoding `V` always produces the same byte sequence
* Encoding does not depend on runtime, platform, or environment
* Encoding never guesses or normalizes input
* Encoding either succeeds canonically or fails explicitly

If two encoders produce different bytes for the same value, at least one of them is wrong.



***

### Entry point

Encoding begins with a Strata `Value` and produces a byte vector.

```
encode(&value) -> Vec
```

The encoder walks the value recursively and writes bytes directly to an output buffer.



***

### Canonical by construction

Canonicality is enforced during encoding, not after.

The encoder guarantees:

* Fixed tags for each value type
* Canonical varint encoding
* Canonical key ordering for maps
* Exact byte preservation for strings and bytes

There is no separate "canonicalization" step.



***

### Tag emission

Every value begins with a single-byte tag that identifies its type.

Examples:

* Null → 0x00
* Bool false → 0x01
* Bool true → 0x02
* Int → 0x10
* String → 0x20
* Bytes → 0x21
* List → 0x30
* Map → 0x40

Tags are fixed by the specification and must never change within a version.



***

### Integer encoding

Integers are encoded as:

* Tag byte (Int)
* Canonical SLEB128 payload

Rules:

* Only signed 64-bit integers are allowed
* No alternate encodings
* No leading redundancy
* No truncation

If an integer cannot be encoded canonically, encoding fails.



***

### String encoding

Strings are encoded as:

* Tag byte (String)
* Length encoded as canonical ULEB128
* Raw UTF-8 bytes

Rules:

* UTF-8 must already be valid
* No normalization
* No escaping
* No transformation

Rust strings are UTF-8 by construction, but the checks exist for cross-language parity.



***

### Bytes encoding

Bytes are encoded as:

* Tag byte (Bytes)
* Length encoded as canonical ULEB128
* Raw bytes copied verbatim

No interpretation is applied.

Bytes are not strings and are never decoded as text.



***

### List encoding

Lists are encoded as:

* Tag byte (List)
* Element count as canonical ULEB128
* Each element encoded sequentially

Rules:

* Order is preserved exactly
* Nested structures are encoded depth-first
* Empty lists are valid and canonical

Any change in order produces different bytes.



***

### Map encoding

Maps are encoded as:

* Tag byte (Map)
* Entry count as canonical ULEB128
* Entries encoded in **sorted key order**

Rules:

* Keys must be strings
* Keys are sorted by UTF-8 byte order
* Each key is encoded exactly like a string
* Values are encoded immediately after their keys

Rust uses `BTreeMap` to guarantee ordering at the data structure level.



***

### Duplicate keys

Encoding assumes maps are already resolved.

If duplicate keys existed during parsing:

* The last value wins
* Only the final value is encoded
* No trace of duplication remains

Encoding reflects the final semantic state only.



***

### Error behavior

Encoding can fail only in strictly defined cases:

* Invalid UTF-8 (non-Rust implementations)
* Invalid integer range
* Internal invariant violations

Encoding never silently fixes or ignores errors.



***

### Framing vs encoding

Encoding produces **pure Strata Core Binary**.

Framing is a separate concern and must not alter encoded bytes.

```
encode(value) != encode_framed(value)
```

Encoding defines truth.\
Framing defines transport.



***

### Stability guarantee

Within a finalized version:

* Encoding rules are frozen
* Byte layouts must never change
* Hashes derived from encoded bytes remain stable forever

Breaking encoding rules requires a new version and a new Northstar.



***

### Summary

Rust encoding is:

* Deterministic
* Canonical
* Explicit
* Non-negotiable

If encoding feels strict, that is intentional.

Correctness is cheaper than ambiguity.
