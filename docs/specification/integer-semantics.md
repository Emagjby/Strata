# Integer semantics

This page defines the **semantics, constraints, and canonical rules** for integers in Strata.

Integers in Strata are **exact**, **finite**, and **unambiguous**. They are designed to behave identically across all languages and implementations.



***

### Integer type

Strata provides a single integer type:

* **int**: signed 64-bit integer

There are:

* no unsigned integers
* no floating-point numbers
* no arbitrary-precision integers in the core



***

### Valid range

Integers MUST fall within the signed 64-bit range:

* Minimum: −2⁶³
* Maximum: 2⁶³ − 1

Any value outside this range is invalid.

Parsing, encoding, or constructing an integer outside this range MUST fail.



***

### Exactness

Integers are **exact values**.

* No rounding
* No approximation
* No implicit conversion

The value `42` is always exactly `42`, regardless of language, platform, or runtime.



***

### Language interoperability

#### Rust

* Represented as `i64`
* Native, lossless representation

#### JavaScript

* Represented as `BigInt`
* `Number` MUST NOT be used
* Precision loss is forbidden

If an implementation uses a type that cannot represent the full range exactly, it is incorrect.



***

### Parsing rules

When parsing Strata Text (`.st`):

* Integers are written in decimal form
* Optional leading minus sign is allowed
* No leading `+`
* No underscores
* No exponential notation

Examples of valid integers:

* `0`
* `-1`
* `42`
* `9007199254740993`

Examples of invalid integers:

* `01`
* `+5`
* `1_000`
* `1e6`
* `3.14`



***

### Encoding rules

Integers are encoded as:

* A fixed integer type tag
* Followed by a **canonical signed integer encoding**

Canonical encoding rules:

* Exactly one valid encoding per integer
* No redundant bytes
* No alternative representations
* No multiple varint forms

If two encoders produce different byte sequences for the same integer, at least one is incorrect.



***

### Decoding rules

Decoders MUST:

* Reject malformed integer encodings
* Reject truncated encodings
* Reject encodings that overflow the valid range
* Reject non-canonical forms

Decoders MUST NOT:

* Clamp values
* Wrap values
* Coerce values
* Guess intent



***

### Canonical uniqueness

For any valid integer value:

* There exists exactly one valid binary representation
* That representation is stable across time and implementations
* That representation is used for hashing

This property is fundamental to Strata's determinism guarantees.



***

### Hashing implications

Integer values contribute to hashes via their canonical encoding.

Because encoding is:

* deterministic
* unique
* stable

Hashes involving integers are stable across:

* languages
* architectures
* compiler versions



***

### Why there is only one integer type

Multiple integer types introduce ambiguity:

* signed vs unsigned
* width differences
* implicit promotions
* language-specific behavior

Strata avoids these problems by defining:

* one integer type
* one range
* one encoding



***

### Non-goals

Integer semantics explicitly do not include:

* arbitrary-precision math
* floating-point arithmetic
* decimal fractions
* unit-aware numbers

These belong in higher layers or application-level semantics.



***

### Stability guarantee

Integer semantics are **frozen per version**.

Once a version is finalized:

* the valid range cannot change
* encoding cannot change
* hashing behavior cannot change

Any modification requires:

* a new version
* a new Northstar invariant
* explicit documentation



***

### Summary

* Signed 64-bit integers only
* Exact, finite, deterministic
* Canonical encoding
* No coercion or approximation
* Stable across languages and time

If an integer cannot be represented exactly, it does not belong in the core.
