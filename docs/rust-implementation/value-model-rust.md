# Value model (Rust)

The Rust value model defines the **authoritative in-memory representation** of Strata data.

It is deliberately small, closed, and explicit.\
Every value that can exist in Strata must map _exactly_ to one of these variants.

There are no extension hooks, no dynamic typing, and no implicit conversions.



***

### Core principle

The value model exists to guarantee that:

* Every value has a single, unambiguous meaning
* Every value can be encoded canonically
* Every value can be decoded without loss
* Every value behaves identically across languages

If a concept cannot be represented deterministically, it does not belong here.



***

### The Value enum

The Rust implementation defines the following value variants:

* Null
* Bool
* Int
* String
* Bytes
* List
* Map

This set is **fixed per version**.

No additional variants may be introduced without a new version boundary.



***

### Null

Represents the absence of a value.

Properties:

* Has no payload
* Encodes to a single tag byte
* Carries no semantic ambiguity

Null is distinct from false, zero, empty string, or empty list.



***

### Bool

Represents a boolean value.

Properties:

* Only two valid values: true and false
* Each has its own distinct canonical tag
* No truthy or falsy coercion

Boolean values are never inferred or auto-converted.



***

### Int

Represents a signed integer.

Properties:

* Signed 64-bit (`i64`)
* No floats
* No decimals
* No widening
* No narrowing

Integers are encoded using canonical SLEB128.

If a number does not fit into a signed 64-bit range, it is rejected.

This rule is non-negotiable and enforced at parse time.



***

### String

Represents textual data.

Properties:

* UTF-8 only
* Stored as raw UTF-8 bytes
* No normalization
* No transcoding
* No lossy conversion

Strings must be valid UTF-8 at all times.

Invalid UTF-8 is rejected during decoding.



***

### Bytes

Represents raw binary data.

Properties:

* Arbitrary byte sequences
* No encoding assumptions
* No interpretation
* No transformation

Bytes are preserved exactly as provided.

This type exists explicitly to avoid abusing strings for binary data.



***

### List

Represents an ordered sequence of values.

Properties:

* Order is significant
* Length is explicit
* Items are encoded in sequence
* Nested lists are allowed

Lists preserve insertion order exactly.

Reordering a list changes the canonical encoding and hash.



***

### Map

Represents a key-value structure.

Properties:

* Keys are strings only
* Values are any Strata value
* Internally stored as `BTreeMap`
* Canonically ordered by UTF-8 byte order of keys

Maps enforce ordering during encoding, not during parsing.

Duplicate keys are resolved by last-write-wins at parse time.



***

### Why this model is strict

This model intentionally excludes:

* Floating-point numbers
* Optional fields
* Default values
* Implicit coercions
* User-defined types

Each excluded feature introduces ambiguity or instability at the byte level.

Strata chooses determinism over convenience.



***

### Cross-language implications

Every non-Rust implementation must:

* Represent integers as signed 64-bit values
* Preserve UTF-8 exactly
* Sort map keys identically
* Reject values Rust would reject

If a language cannot represent this model faithfully, it cannot implement Strata correctly.



***

### Stability guarantee

Once a Strata version is finalized:

* The value model is frozen
* Semantics cannot change
* Existing values must encode identically forever

New concepts must be added in new versions, never retroactively.



***

### Summary

The Rust value model is:

* Minimal
* Closed
* Deterministic
* Non-negotiable

It exists to make correctness boring and failures explicit.

Everything else in Strata is built on top of this foundation.
