# Rust <-> JS parity

Strata is designed to be **language-agnostic by construction**.

Rust and JavaScript are independent implementations, but they are required to produce **identical observable results** for the same logical input.

Parity is not an aspiration.\
It is a requirement enforced by canonical rules and golden vectors.



***

### What parity means

Rust ↔ JavaScript parity guarantees that:

* The same Strata Text produces the same logical value
* The same value encodes to the same Strata Core Binary bytes
* The same bytes hash to the same BLAKE3 digest
* The same invalid input fails with the same error class and offset

If any of these differ, parity is broken.



***

### Canonical encoding as the anchor

Parity exists because encoding is fully specified.

Both implementations:

* Use identical tag values
* Use canonical SLEB128 / ULEB128 rules
* Encode strings as raw UTF-8 bytes
* Encode bytes verbatim
* Sort map keys by UTF-8 byte order
* Reject implicit coercions

There is no freedom left to diverge.



***

### Value model equivalence

Rust and JavaScript use different in-memory representations, but the **logical model is identical**.

* Rust `i64` ↔ JavaScript `bigint`
* Rust `String` ↔ JavaScript `string`
* Rust `Vec<u8>` ↔ JavaScript `Uint8Array`
* Rust `BTreeMap<String, Value>` ↔ JavaScript `Map<string, Value>`

Only the canonical bytes matter.\
In-memory layout is irrelevant.



***

### Parsing parity

Strata Text parsing is aligned across languages.

Both parsers:

* Enforce the same grammar
* Reject the same malformed literals
* Reject integers outside the signed 64-bit range
* Support the same shorthand map syntax
* Ignore whitespace and comments identically

Differences in tokenization are permitted internally, but **parse results must match**.



***

### Encoding parity

For any valid value:

* Rust `encode(value)`
* JavaScript `encodeValue(value)`

Must produce byte-identical output.

This is verified continuously via shared golden vectors.



***

### Hash parity

Hashing is defined as:

Code:\
BLAKE3-256(canonical\_scb\_bytes)

Both implementations:

* Hash only canonical bytes
* Never hash Strata Text directly
* Never normalize or reinterpret values
* Never include framing bytes

If two hashes differ, at least one implementation is wrong.



***

### Decode parity

Decoding is intentionally strict.

Both implementations:

* Reject unknown tags
* Reject truncated input
* Reject invalid varints
* Reject invalid UTF-8
* Reject trailing bytes

Error class and offset must match golden vectors.

Decode parity ensures observability is identical across languages.



***

### Golden vectors as the arbiter

Parity is enforced by shared vectors.

The vectors directory is language-neutral.

* Rust must satisfy vectors
* JavaScript must satisfy vectors
* Vectors are never updated to satisfy implementations

Parity failures are implementation bugs, not spec ambiguities.



***

### What parity does not require

Parity does **not** require:

* Identical performance
* Identical memory usage
* Identical error message strings
* Identical internal APIs
* Identical public ergonomics

Only observable behavior is constrained.



***

### Why Rust and JavaScript first

Rust and JavaScript represent opposite ends of the spectrum:

* Low-level vs high-level
* Static vs dynamic
* Native vs managed

If these two can agree perfectly, other languages can follow.

They form the reference parity baseline.



***

### Summary

Rust ↔ JavaScript parity guarantees that:

* Data is portable
* Hashes are stable
* Systems can interoperate safely
* Language choice does not affect correctness

If Rust and JavaScript disagree, Strata has failed.

Parity is the proof that canonical encoding works.
