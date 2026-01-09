# Strata Text parse

Strata Text (`.st`) is a **human-facing authoring format**, not a data interchange format.

The JavaScript parser exists to:

* Parse `.st` into the Strata value model
* Reject invalid or ambiguous input
* Preserve semantic intent
* Hand off canonical truth to the encoder

Parsing is **strict**, **deterministic**, and **lossless at the semantic level**.



***

### Purpose of Strata Text

Strata Text exists for:

* Authoring configuration
* Writing golden vectors
* Manual inspection and review
* Deterministic source input

It does **not** exist for:

* Network transport
* Partial parsing
* Lenient recovery
* Schema-driven validation

Text is temporary. Canonical bytes are forever.



***

### Parsing pipeline

The JavaScript parser follows a fixed pipeline:

1. Lex source text into tokens
2. Parse tokens into a Value tree
3. Reject trailing input
4. Return an in-memory Strata Value

At no point does parsing:

* Reorder data
* Normalize values
* Infer types
* Apply defaults



***

### Lexer responsibilities

The lexer converts raw text into tokens with exact source locations.

It tracks:

* Byte offset
* Line number
* Column number

#### Supported tokens

* Literals:
  * null
  * true / false
  * integers (signed, base-10)
  * strings
  * bytes (`0x...`)
* Identifiers (map keys)
* Structural tokens:
  * `{ }`
  * `[ ]`
  * `:`
  * `,`
* EOF

Whitespace and comments are ignored.



***

### Comments

The lexer supports:

* `// line comments`
* `# line comments`

Comments are treated as whitespace and have no semantic meaning.



***

### Integer rules

Integers:

* Are parsed as `bigint`
* Must fit in signed 64-bit range
* Reject overflow immediately
* Reject malformed syntax

```
Allowed: 0, -1, 42, 9007199254740993
Rejected: 1.2, 1e6, 0xFF, 9223372036854775808
```

Parsing never clamps or wraps.



***

### String rules

Strings:

* Use double quotes `"..."`
* Support escape sequences:
  * `\"`
  * `\\`
  * `\n`
  * `\r`
  * `\t`
  * `\uXXXX`
* Cannot span lines
* Reject invalid escapes

Non-ASCII bytes are rejected during lexing.

Unicode enters the system through explicit escapes only.



***

### Bytes literal

Bytes are written as hex:

```
0xDEADBEEF
```

Rules:

* Must start with `0x`
* Must contain an even number of hex digits
* At least one byte is required
* Case-insensitive hex digits allowed

Bytes are stored exactly as written.



***

### Identifiers and keywords

Identifiers:

* Begin with letter or `_`
* May contain letters, digits, `_`

Keywords:

* `null`
* `true`
* `false`

Keywords are reserved and cannot be used as identifiers.



***

### Grammar overview

Strata Text grammar is minimal and explicit.

#### Lists

```
[1, 2, 3]
```

Rules:

* Comma-separated
* Trailing commas allowed
* Order preserved



***

#### Maps

```
{ a: 1, b: 2 }
```

Rules:

* Keys are identifiers
* Values are any Strata value
* Trailing commas allowed



***

#### Shorthand maps

Strata supports shorthand nesting:

```
user { id: 42 }
```

Which is equivalent to:

```
{ user: { id: 42 } }
```

This is purely syntactic sugar and does not affect canonical output.



***

### Separator rules

Maps support:

* Commas
* Newlines as implicit separators

This enables readable layouts without ambiguity.



***

### Parser guarantees

The parser guarantees:

* Full consumption of input
* No silent recovery
* Precise error locations
* Deterministic AST construction

If parsing succeeds, the result is a valid Strata Value.



***

### Error model

All parse failures throw a `ParseError`.

Errors include:

* Unexpected token
* Malformed bytes literal
* Integer out of range

Each error includes:

* Line
* Column
* Byte offset

This is critical for authoring and debugging.



***

### Relationship to encoding

The parser does **not** enforce canonical ordering.

For example:

* Map key order is preserved as written

Canonical ordering is enforced **only during encoding**.

Parsing reveals intent.\
Encoding enforces truth.



***

### Golden vector usage

All golden vectors are authored in `.st`.

The JS parser must:

* Parse vectors identically to Rust
* Produce identical values
* Enable byte-for-byte canonical encoding

If a vector fails to parse:

* The parser is wrong
* The vector is not changed



***

### Failure philosophy

The parser is intentionally strict.

It does not:

* Guess intent
* Autocorrect syntax
* Permit partial values

If the input is wrong, parsing fails.



***

### Summary

The JavaScript Strata Text parser is:

* Strict
* Deterministic
* Explicit
* Author-focused
* Canonical-safe

It exists to bridge human intent to canonical binary truth.

Everything after parsing is non-negotiable.
