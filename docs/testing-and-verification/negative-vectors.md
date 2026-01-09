# Negative vectors

Negative vectors define **required failure behavior** in Strata.

They specify inputs that **must be rejected**, how they must be rejected, and **where** the failure must be reported.

If an implementation accepts a negative vector, it is **incorrect**.



***

### What a negative vector is

A negative vector is an intentionally invalid Strata Core Binary input.

It asserts that:

* Decoding must fail
* The failure must be explicit
* The error kind must match exactly
* The error offset must match exactly

Silent acceptance is forbidden.



***

### Files in a negative vector

Each negative vector consists of two files:

* `<name>.hex`\
  Raw Strata Core Binary bytes, hex-encoded
* `<name>.error.json`\
  The expected failure description

Both files are mandatory.



***

### Error contract

The `.error.json` file defines:

* `kind`\
  The exact error category that must be raised
* `offset`\
  The byte offset at which the error is detected

This is part of the public contract.



***

### Required implementation behavior

For every negative vector, an implementation must:

1. Attempt to decode the provided bytes
2. Fail during decoding
3. Produce the exact error kind
4. Report the exact byte offset

If the error kind differs, the implementation is wrong.\
If the offset differs, the implementation is wrong.



***

### Covered failure modes

Negative vectors cover all required decoder failure cases, including:

* Invalid tags
* Truncated inputs
* Varint overflow
* Invalid UTF-8 sequences
* Trailing bytes after a complete value

These failures are not optional.



***

### Strictness is intentional

Strata decoders are intentionally strict.

They do not:

* Guess intent
* Normalize invalid data
* Recover from malformed input
* Ignore trailing bytes

Decoding reveals reality. Invalid input must fail loudly and precisely.



***

### Cross-language determinism

Negative vectors must fail identically across all implementations.

This includes:

* Same error kind
* Same byte offset
* Same failure point

A Rust decoder and a JavaScript decoder must disagree on nothing.



***

### Why offsets matter

Offsets are not debugging aids.

They are part of the determinism contract.

Precise offsets ensure:

* Debuggable corruption
* Auditability
* Binary-level reproducibility
* Identical behavior across languages

An error without a correct offset is incomplete.



***

### Adding new negative vectors

New negative vectors may only be added when:

* A new Northstar version defines a new failure mode
* The failure semantics are fully specified
* The error is irreversible once merged

Negative vectors are never weakened.



***

### What negative vectors are not

Negative vectors are not:

* Fuzz tests
* Soft validation checks
* Suggestions for behavior
* Recoverable parsing hints

They are hard requirements.



***

### Summary

Negative vectors assert:

* This input is invalid
* This error must be raised
* At this exact byte offset

If your decoder accepts invalid data, it is broken.

If it fails differently, it is also broken.

There is no partial correctness.
