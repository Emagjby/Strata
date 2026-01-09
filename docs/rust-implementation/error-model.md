# Error model

Strata treats errors as **first-class data**, not side effects.

Errors are not strings. They are not panics. They are not implicit.

Every failure is explicit, structured, and intentional.



***

### Design goals

The error model exists to guarantee:

* No panics on user input
* Deterministic failure behavior
* Precise diagnostics
* Cross-language parity
* Testable failure semantics

If something fails, it must fail **loudly, predictably, and correctly**.



***

### Unified error surface

All public operations converge on a single top-level error type.

```
pub enum StrataError { Parse(ParseError), Encode(EncodeError), Decode(DecodeError), Io(std::io::Error), Internal(&'static str), }
```

This allows callers to:

* Handle errors exhaustively
* Route failures correctly
* Preserve intent across layers

No information is lost by flattening errors into strings.



***

### Parse errors

Parse errors originate from Strata Text processing.

They always include **location information**.

```
pub struct ParseError { pub kind: ParseErrorKind, pub span: Span, }
```

```
pub struct Span { pub offset: usize, pub line: usize, pub column: usize, }
```

#### Parse error kinds

Syntax errors:

* UnexpectedToken
* MalformedBytesLiteral

Semantic errors:

* IntegerOutOfRange

Parse errors are raised when the input text violates the grammar or literal rules.

There is no recovery or backtracking.



***

### Encode errors

Encode errors originate during canonical encoding.

```
pub enum EncodeError { DuplicateKey, InvalidUtf8, InvalidInteger, }
```

Encoding is the point where **canonical truth is enforced**.

Encode failures mean:

* The Value cannot be represented canonically
* Canonical rules would be violated
* Output bytes would not be lawful

Encoding must fail rather than guess.



***

### Decode errors

Decode errors originate when reading `.scb` bytes.

```
pub struct DecodeError { pub kind: DecodeErrorKind, pub offset: usize, }
```

The offset is the **exact byte position** where decoding failed.

#### Decode error kinds

* InvalidTag(u8)
* UnexpectedEOF
* InvalidVarint
* InvalidUtf8
* TrailingBytes

These errors distinguish:

* Malformed input
* Truncated input
* Structural corruption

No error is ambiguous.



***

### Decode safety rules

The decoder guarantees:

* No panics on arbitrary input
* Bounded memory usage
* Exact error offsets
* Deterministic failure classification

Malformed input is rejected explicitly.

Hostile input is safe.



***

### CLI error behavior

The CLI reflects the same error model.

Exit codes:

* 0 → success
* 1 → invalid input
* 2 → I/O failure
* 100 → internal error

Errors are written to stderr.

Stack traces are never shown.



***

### Error output philosophy

Errors are:

* Human-readable
* Minimal
* Actionable

Example format:

```
error: decode failed reason: invalid tag 0x99 offset: 12
```

No noise. No speculation. No recovery suggestions.



***

### Golden vector enforcement

Negative test vectors assert exact failure behavior.

For each malformed input:

* Error kind must match
* Error offset must match

If an implementation fails differently, it is incorrect.

Failure semantics are part of the specification.



***

### Cross-language contract

The Rust error model defines **behavioral law**.

Other languages must:

* Produce equivalent error kinds
* Report equivalent failure positions
* Reject the same malformed inputs

Errors are part of determinism.



***

### Why this strictness exists

Silent failure is data corruption. Ambiguous failure is a security risk.

Strata systems are expected to operate in:

* Cryptographic pipelines
* Content-addressed systems
* Distributed verification
* Audit-sensitive environments

In these domains, **how** something fails matters as much as success.



***

### Summary

The Strata error model is:

* Explicit
* Structured
* Deterministic
* Test-enforced

Errors are not accidents.

They are guarantees.
