# Value types

This page defines the **value types** supported by Strata and the guarantees associated with each.

The value model is intentionally minimal. Every type exists because it can be encoded **unambiguously** and **canonically**.

If a type introduces ambiguity, it does not belong in the core.



***

### Overview

Strata supports a fixed set of value types:

* null
* bool
* int
* string
* bytes
* list
* map

No additional core types exist.

There are:

* no floats
* no optional types
* no implicit defaults
* no undefined behavior



***

### Null

**Meaning**\
Represents the explicit absence of a value.

**Properties**

* Has exactly one representation
* Carries no payload
* Is not interchangeable with zero, false, or empty values

**Canonical guarantees**

* One tag
* No associated data
* No alternative encodings



***

### Bool

**Meaning**\
Represents a boolean truth value.

**Allowed values**

* true
* false

**Canonical guarantees**

* Each value has a distinct tag
* No numeric or textual coercion
* No truthy / falsy semantics



***

### Int

**Meaning**\
Represents a signed integer.

**Range**

* Signed 64-bit
* From −2⁶³ to 2⁶³ − 1

**Properties**

* Exact integer semantics
* No rounding
* No overflow tolerance
* No implicit widening or narrowing

**Canonical guarantees**

* Exactly one binary representation per integer
* No alternative encodings
* No leading zeros
* No multiple varint forms



***

### String

**Meaning**\
Represents textual data.

**Encoding**

* UTF-8
* Stored as raw UTF-8 bytes

**Properties**

* Strings are sequences of bytes interpreted as UTF-8
* No Unicode normalization
* No locale-dependent behavior

**Canonical guarantees**

* Invalid UTF-8 is rejected
* Byte sequence is preserved exactly
* Identical text always produces identical bytes\\



***

### Bytes

**Meaning**\
Represents raw binary data.

**Use cases**

* Hashes
* Binary payloads
* Cryptographic material
* Packed external formats

**Properties**

* Opaque sequence of bytes
* No interpretation
* No encoding assumptions

**Canonical guarantees**

* Length is explicit
* Bytes are preserved verbatim
* No transformation or normalization



***

### List

**Meaning**\
Represents an ordered sequence of values.

**Properties**

* Order is significant
* Elements may be of mixed types
* Length is explicit

**Canonical guarantees**

* Order is preserved exactly
* Each element is encoded in sequence
* No reordering or normalization



***

### Map

**Meaning**\
Represents key–value associations.

**Key restrictions**

* Keys must be strings
* Keys must be unique

**Properties**

* Values may be any Strata value
* Ordering is not semantic

**Canonical guarantees**

* Keys are sorted by UTF-8 byte order during encoding
* Duplicate keys are rejected at encode time
* Exactly one canonical ordering exists



***

### Structural completeness

All Strata values are composed recursively from these types.

There are:

* no extension hooks
* no hidden metadata
* no tagged unions
* no implicit containers

Every structure can be reduced to combinations of these primitives.



***

### Why the model is minimal

Each value type satisfies three conditions:

1. It can be represented without ambiguity
2. It can be encoded canonically
3. It behaves identically across languages

Types that violate any of these conditions are excluded from the core.



***

### Non-goals

The value model does not attempt to represent:

* floating-point arithmetic
* time or dates
* schemas
* optionality
* precision hints
* domain-specific semantics

Such concerns belong in **higher layers**, not in the canonical core.



***

### Stability guarantee

The value model is **frozen per version**.

Once a version is finalized:

* no types are removed
* existing types do not change semantics
* canonical representations remain stable

Any modification to the value model requires:

* a new version
* a new Northstar invariant
* explicit documentation



***

### Summary

* Small, fixed set of value types
* No ambiguity
* No coercion
* No implicit behavior
* Canonical by construction

The value model is the vocabulary of Strata.\
Everything else builds on top of it.
