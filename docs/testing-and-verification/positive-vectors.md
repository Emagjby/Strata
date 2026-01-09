# Positive vectors

Positive vectors define **correct, required behavior** in Strata.

They specify what _must work_, how it must work, and what the result _must be_ down to the last byte.

If an implementation fails a positive vector, it is **incorrect**.



***

### What a positive vector is

A positive vector is a fully valid Strata input with a fully defined outcome.

It asserts that:

* Parsing must succeed
* Encoding must succeed
* Canonical bytes must match exactly
* Hashes must match exactly

There is no flexibility.



***

### Files in a positive vector

Each positive vector consists of three files:

* `<name>.st`\
  Strata Text source input
* `<name>.scb.hex`\
  Canonical Strata Core Binary, hex-encoded
* `<name>.hash.hex`\
  Canonical BLAKE3-256 hash of the canonical bytes

All three files are required.



***

### Required implementation behavior

For every positive vector, an implementation must:

1. Read the `.st` file as UTF-8 text
2. Parse it into a Strata value
3. Canonically encode that value into `.scb`
4. Produce bytes **identical** to `.scb.hex`
5. Hash those bytes
6. Produce a hash **identical** to `.hash.hex`

Any deviation is a failure.



***

### What positive vectors cover

Positive vectors cover:

* All core value types
* Nested structures
* Large integers beyond JS safe integer range
* Canonical map ordering
* Deterministic list ordering
* String and bytes handling
* Structural equivalence across languages

They are designed to touch every canonical rule.



***

### Canonical equivalence

Positive vectors often include inputs that are **not written canonically**.

Example cases include:

* Maps written in non-canonical key order
* Deeply nested structures
* Mixed value types

The vector defines the **canonical result**, not the textual form.

Parsing may accept multiple surface forms. Encoding must produce exactly one canonical form.



***

### Cross-language guarantee

A positive vector must produce:

* The same `.scb` bytes in Rust
* The same `.scb` bytes in JavaScript
* The same `.scb` bytes in any future implementation

If two languages disagree, at least one is wrong.

There is no "reference output per language".



***

### Determinism requirement

Positive vectors assert **strong determinism**.

Given the same logical value:

* Byte output is identical
* Hash output is identical
* Across machines
* Across operating systems
* Across architectures
* Across time

This is a requirement, not a goal.



***

### Adding new positive vectors

New positive vectors may only be added when:

* A new Northstar version is introduced
* The behavior being tested is explicitly specified
* The change is irreversible once merged

Vectors are never added casually.



***

### What positive vectors are not

Positive vectors are not:

* Examples
* Suggestions
* Reference outputs that may drift
* Tests you can “fix” by changing expected values

They are binding.



***

### Summary

Positive vectors assert:

* This input is valid
* This output is correct
* This hash is final

If your code does not match a positive vector, your code is wrong.

No exceptions.
