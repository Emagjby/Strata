# Strata Text parser

Strata Text (`.st`) exists for **humans**.\
The parser exists to turn that human-friendly syntax into an exact, deterministic **Value**.

The parser is not permissive, not heuristic, and not forgiving.\
It is a compiler frontend with explicit failure semantics.



***

### Role of the parser

The Strata Text parser performs exactly one job:

> Convert Strata Text into a Strata Value model **without ambiguity**.

It does **not**:

* Infer types
* Coerce values
* Normalize structure
* Apply canonical ordering
* Perform hashing

Parsing is upstream of all canonical behavior.



***

### Pipeline position

The full Strata pipeline looks like this:

```
.st source
-> lexing
-> parsing
-> Value
-> canonical encoding
-> .scb
-> hashing / transport / storage
```

The parser stops at **Value**.

Canonical guarantees begin **after parsing**.



***

### Lexer responsibility

The lexer transforms raw text into tokens.

Token categories include:

* Literals: null, true, false, integers, strings, bytes
* Identifiers
* Punctuation: `{ } [ ] : ,`
* End-of-file

Whitespace and comments are ignored.

Supported comments:

* Line comments starting with `//`
* Line comments starting with `#`

Comments never appear in the token stream.



***

### Lexical constraints

The lexer enforces strict lexical rules:

* Integers must fit in signed 64-bit
* Bytes literals must be valid hex
* Strings must be valid UTF-8
* Strings cannot span multiple lines
* Identifiers are ASCII-only

Invalid characters cause immediate failure.

There is no recovery.



***

### Integer parsing

Integers are parsed as signed 64-bit values.

Rules:

* Optional leading `-`
* Base-10 digits only
* No underscores
* No suffixes
* No floats

If the value exceeds the i64 range, parsing fails.

```
9223372036854775808 -> error
-9223372036854775809 -> error
```

This is intentional.



***

### Bytes literals

Bytes are expressed explicitly using hex syntax.

Format:

```
0xDEADBEEF
```

Rules:

* Must start with `0x`
* Must contain an even number of hex digits
* Must contain at least one byte
* Case-insensitive hex digits allowed

Malformed bytes literals are rejected.



***

### String literals

Strings use double quotes.

Supported escapes:

* "
* \\
* \n
* \r
* \t
* \uXXXX (exactly four hex digits)

Rules:

* Strings must be valid UTF-8
* No multiline strings
* No implicit normalization

Unicode codepoints are inserted as-is.



***

### Grammar overview

The parser is recursive-descent and LL(1).

Value grammar (simplified):

```
Value = null | true | false | Int | String | Bytes | List | Map
```

Containers:

* List: `[ value, value, ... ]`
* Map: `{ key: value, ... }`

Trailing commas are allowed.



***

### Map syntax and shorthand

Maps support two forms.

Explicit form:

```
{ a: 1, b: 2 }
```

Shorthand form:

```
user { id: 42 }
```

Shorthand expands to:

```
{ user: { id: 42 } }
```

Shorthand nesting is allowed.



***

### Duplicate keys

During parsing:

* Duplicate keys are allowed
* Last-write-wins semantics apply

Example:

```
a { x: 1 x: 2 }
```

Resulting Value:

```
{ a: { x: 2 } }
```

This behavior is **not canonical**.

Duplicate keys are resolved during parsing to allow inspection and debugging.

Canonical encoding later enforces uniqueness.



***

### Structural guarantees

Parsing guarantees:

* Correct Value construction
* Preserved structure
* Exact literal interpretation
* Deterministic output for identical input

Parsing does **not** guarantee:

* Canonical ordering
* Canonical encoding
* Hash stability

Those belong to later stages.



***

### Error model

All parser failures are explicit and structured.

Errors include:

* Unexpected token
* Malformed bytes literal
* Integer out of range
* Invalid escape sequence
* Extra input after value

Each error includes:

* Exact byte offset
* Line number
* Column number

There are no panics on user input.



***

### End-of-input enforcement

After parsing a value, the parser requires EOF.

Extra tokens after a valid value cause failure.

This prevents partial parses and silent truncation.



***

### Round-trip expectations

Valid `.st` input must satisfy:

```
parse -> encode -> decode -> Value
```

Where:

* Value' is structurally equivalent to the parsed Value
* Ordering differences may exist
* Semantic meaning is preserved

This property is enforced by round-trip tests.



***

### Why the parser is strict

Strata Text is a **source language**, not a configuration hack.

Strict parsing ensures:

* Predictability
* Tooling safety
* Deterministic compilation
* Cross-language portability

Relaxing the parser would leak ambiguity into canonical layers.



***

### Summary

The Strata Text parser is:

* Deterministic
* Strict
* Explicit
* Non-magical

It converts human-written text into an exact Value.

Everything that matters happens after.
