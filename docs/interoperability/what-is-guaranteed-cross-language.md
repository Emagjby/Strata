# What is guaranteed cross-language

## What Is Guaranteed Cross-Language

Strata is designed to make **cross-language behavior boring**.

If two independent implementations follow the specification, they must agree. Not approximately. Not logically. **Byte-for-byte.**

This page defines exactly what is guaranteed across languages.



***

### Canonical bytes

For any valid Strata value:

* Canonical encoding is unique
* Encoding produces exactly one byte sequence
* That byte sequence is identical across implementations

If Rust and JavaScript encode the same value, the output bytes must match exactly.

No normalization. No heuristics. No platform variance.



***

### Hash stability

Hashes are computed over canonical bytes.

This guarantees:

* Identical hashes across languages
* Identical hashes across machines
* Identical hashes across time

If two implementations disagree on a hash, at least one is wrong.

Hashing is not an optimization. It is a contract.



***

### Value semantics

The following semantics are guaranteed across languages:

* null, bool, int, string, bytes, list, map
* Integers are signed 64-bit
* Strings are UTF-8
* Byte arrays are raw and opaque
* Lists preserve order
* Maps are sorted canonically by UTF-8 byte order of keys

A value constructed in one language must decode to the same structure in another.



***

### Map ordering

Map ordering is deterministic and enforced.

* Keys are strings only
* Keys are sorted by UTF-8 byte order
* Sorting happens during encoding
* Decoding preserves canonical order

Different insertion orders must not affect encoded output.



***

### Integer encoding

Integers are encoded canonically:

* Signed 64-bit range
* Encoded using SLEB128
* No alternative encodings
* No implicit widening or narrowing

The same integer must always encode to the same byte sequence.



***

### Error behavior

Certain failures are guaranteed to be detected consistently:

* Invalid tags
* Truncated input
* Invalid varints
* Invalid UTF-8
* Trailing bytes

Error categories and offsets must align across implementations.

Exact error wording may differ. Error meaning must not.



***

### Decode then re-encode stability

For any valid canonical payload:

* decode(bytes) succeeds
* encode(decoded) produces identical bytes
* hash(bytes) equals hash(encode(decoded))

This is enforced by Northstar tests.

Roundtripping is mandatory.



***

### Cross-language tests

Strata guarantees are enforced through:

* Shared golden vectors
* Northstar T1, T2, and T3 tests
* Independent implementations
* Zero shared code

Passing these tests is required for compliance.



***

### Transport independence

Strata guarantees survive transport layers:

* Files
* HTTP
* Streams
* Framed protocols

As long as canonical bytes are preserved, behavior is identical.

Transport does not affect meaning.



***

### Determinism over convenience

Strata guarantees determinism even when inconvenient.

This includes:

* Rejecting malformed input
* Rejecting ambiguous encodings
* Refusing silent coercions
* Failing early and explicitly

Convenience is optional. Correctness is not.



***

### Summary

Strata guarantees that:

* Same value → same bytes
* Same bytes → same hash
* Same bytes → same decoded structure
* Same failures → same error class

Across languages. Across platforms. Across time.

If this sounds strict, that is the point.
