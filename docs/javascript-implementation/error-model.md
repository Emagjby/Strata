# Error model

The JavaScript implementation of Strata uses a **strict, explicit error model**.

Errors are not values.\
Errors are not recoverable.\
Errors are not normalized.

If something is wrong, execution stops.

This is intentional.



***

### Design goals

The error model exists to guarantee:

* Deterministic failure
* Precise diagnostics
* No silent coercion
* No partial success

If an operation succeeds, the result is correct. If it fails, the failure is explicit and actionable.



***

### Error categories

The JavaScript implementation defines two primary error domains:

* Parse errors (Strata Text)
* Decode errors (Strata Core Binary)

Encoding errors are prevented structurally by construction.



***

### ParseError

`ParseError` represents failures while parsing Strata Text (`.st`).

#### When ParseError occurs

ParseError is thrown when:

* The input does not conform to grammar
* An integer is out of range
* A bytes literal is malformed
* Tokens appear in invalid positions
* Extra input remains after parsing



***

#### ParseError structure

A ParseError includes:

* kind: structured error reason
* offset: byte offset in source
* line: 1-based line number
* column: 1-based column number

This enables editor integration and precise diagnostics.



***

#### ParseError kinds

*   UnexpectedToken\
    Raised when the parser encounters an unexpected token.

    Includes:

    * expected: textual description
    * found: actual token
* MalformedBytesLiteral\
  Raised when a bytes literal is invalid.
* IntegerOutOfRange\
  Raised when an integer cannot fit in signed 64-bit range.

Parsing stops immediately at the first error.



***

### DecodeError

`DecodeError` represents failures while decoding Strata Core Binary (`.scb`).

These errors indicate malformed, truncated, or invalid binary input.



***

#### When DecodeError occurs

DecodeError is thrown when:

* An invalid tag is encountered
* The input ends unexpectedly
* A varint is malformed or overflows
* UTF-8 decoding fails
* Trailing bytes remain after decoding



***

#### DecodeError structure

A DecodeError includes:

* kind: structured error type
* offset: byte offset in the binary input

Offsets always refer to the canonical byte stream.



***

#### DecodeError kinds

* **InvalidTag**\
  Encountered when a tag byte is not recognized.
* **UnexpectedEOF**\
  Raised when input ends before a value completes.
* **InvalidVarint**\
  Raised when ULEB128 or SLEB128 decoding fails.
* **InvalidUtf8**\
  Raised when string bytes are not valid UTF-8.
* **TrailingBytes**\
  Raised when extra bytes remain after a value is decoded.



***

### Deterministic offsets

Offsets are always:

* Absolute
* Stable
* Byte-accurate

They are safe to log, persist, or compare across runs.



***

### No recovery semantics

The JavaScript implementation does not:

* Skip invalid bytes
* Continue after errors
* Attempt best-effort parsing
* Downgrade errors to warnings

Any recovery must be handled by the caller.



***

### CLI behavior

The JavaScript CLI maps errors to exit codes:

* Exit 0: Success
* Exit 1: Invalid input (ParseError or DecodeError)
* Exit 2: I/O error
* Exit 100: Internal error

Errors are printed to stderr.



***

### Cross-language parity

The JavaScript error model mirrors the Rust implementation:

* Same error categories
* Same failure boundaries
* Same semantic meaning

Exact error text may differ.\
Error meaning must not.



***

### Golden vector enforcement

Negative vectors explicitly test error cases.

If JavaScript produces:

* A different error kind
* A different offset
* Or succeeds where failure is expected

The implementation is incorrect.

Vectors are law.



***

### Summary

The JavaScript error model is:

* Explicit
* Deterministic
* Non-recoverable
* Diagnostics-first

Strata does not tolerate ambiguity.

If input is wrong, the system must say so loudly and precisely.
