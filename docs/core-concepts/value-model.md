# Value model

Strata defines a minimal, fixed value model.

Every value type exists to support deterministic, canonical encoding. Anything that introduces ambiguity is intentionally excluded.

The value model is part of the protocol contract.



***

### Design principles

The Strata value model is designed to be:

* minimal
* unambiguous
* deterministic
* cross-language stable
* future-extensible without breaking existing guarantees

There are no implicit conversions. There is no undefined behavior. There are no implementation-dependent semantics.

If a value exists, its encoding is fully defined.



***

### Supported value types

Strata supports the following core value types:

* `null`
* `bool`
* `int`
* `bytes`
* `string`
* `list`
* `map`

This set is closed within a version line.



***

### `null`

Represents the absence of a value.

* Has exactly one canonical encoding
* Carries no payload
* Has no equivalent numeric or boolean meaning

`null` is a value, not a placeholder.



***

### `bool`

Represents a boolean truth value.

* Allowed values: `true`, `false`
* Encoded as distinct canonical tags
* No numeric coercion
* No truthy or falsy semantics

Boolean values are explicit.



***

### `int`

Represents a signed integer.

* Fixed-width semantic range
* Arbitrary precision at the language level
* Canonical binary encoding
* No leading zeros
* No alternate encodings

Integers are mathematical integers, not machine integers.



***

### `bytes`

Represents an arbitrary byte sequence.

* Length-prefixed
* No interpretation
* No encoding assumptions
* No implicit text semantics

Bytes are opaque by design.



***

### `string`

Represents UTF-8 text.

* Must be valid UTF-8
* Canonically encoded
* No normalization
* No alternative encodings

Invalid UTF-8 is rejected.

Strings are text, not byte containers.



***

### `list`

Represents an ordered sequence of values.

* Order is semantically significant
* Length is explicit
* Elements are encoded sequentially
* No sparse or implicit entries

Lists preserve structure exactly.



***

### `map`

Represents a key-value mapping.

* Keys must be strings
* Keys are sorted canonically by UTF-8 byte order
* Values may be any Strata value
* Duplicate keys are not permitted

Map ordering is deterministic and enforced.



***

### Canonical ordering

Canonical ordering applies only to maps.

* Keys are compared by UTF-8 byte sequence
* Sorting is lexicographic
* Locale, language, and collation rules are ignored

This ensures cross-language determinism.



***

### What is intentionally excluded

Strata does not include:

* floating-point numbers
* optional or nullable fields
* default values
* unions or tagged variants
* schema-aware types
* timestamps
* decimals
* NaN or infinity

These features introduce ambiguity or hidden behavior.

They may exist in higher layers, not in the core.



***

### No implicit coercions

Strata does not perform:

* number to string coercion
* string to number parsing
* truthy or falsy evaluation
* automatic wrapping or unwrapping

Values are explicit and exact.



***

### Stability guarantees

The value model is frozen per version line.

* Existing value types never change semantics
* Existing encodings never change
* New value types require a new version

This guarantees that hashes and bytes remain stable indefinitely.



***

### Summary

The Strata value model is intentionally small.

Its purpose is not expressiveness. Its purpose is correctness.

If a value can be represented unambiguously, Strata supports it. If it cannot, Strata rejects it.

Everything else belongs above the core.
