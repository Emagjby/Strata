# String & UTF-8 rules

This page defines how **strings** are represented, validated, encoded, and hashed in Strata.

Strings in Strata are **exact byte sequences**, not abstract text objects. There is no normalization, no interpretation, and no locale-dependent behavior.



***

### String type

Strata provides a single string type:

* **string**: UTF-8 encoded text

Strings represent **Unicode scalar values encoded as UTF-8 bytes**. No other encodings are permitted.



***

### UTF-8 requirement

All strings MUST be valid UTF-8.

* Encoding invalid UTF-8 is forbidden
* Decoding invalid UTF-8 MUST fail
* No replacement characters
* No lossy decoding

If a byte sequence is not valid UTF-8, it is not a valid Strata string.



***

### No Unicode normalization

Strata does **not** perform Unicode normalization.

That means:

* No NFC
* No NFD
* No NFKC
* No NFKD

Strings are compared, encoded, and hashed **exactly as provided**.

Examples:

* `"é"` (U+00E9) and `"e\u0301"` (U+0065 + U+0301) are **different values**
* They encode to different bytes
* They hash differently

This is intentional.



***

### Parsing rules (Strata Text)

In Strata Text (`.st`), strings:

* Are delimited by double quotes `"`
* Support explicit escape sequences
* Must result in valid UTF-8

Supported escapes:

* `\"` quote
* `\\` backslash
* `\n` newline
* `\r` carriage return
* `\t` tab
* `\uXXXX` (exactly 4 hex digits)

Any malformed escape sequence MUST fail parsing.



***

### Disallowed string behavior

Strings MUST NOT:

* Contain invalid UTF-8
* Be auto-normalized
* Be trimmed
* Be case-folded
* Be locale-transformed

What you write is what gets encoded.



***

### Encoding rules

Strings are encoded as:

* A string type tag
* Followed by a byte length
* Followed by raw UTF-8 bytes

Canonical encoding rules:

* Length is the number of bytes, not characters
* UTF-8 bytes are preserved verbatim
* Exactly one valid encoding per string

There is no alternate representation.



***

### Decoding rules

Decoders MUST:

* Validate UTF-8 strictly
* Reject malformed UTF-8 sequences
* Reject truncated strings
* Reject overlong or invalid encodings

Decoders MUST NOT:

* Replace invalid sequences
* Guess encodings
* Normalize content



***

### Equality and ordering

#### Equality

Two strings are equal if and only if:

* Their UTF-8 byte sequences are identical

Semantic equivalence is irrelevant.<br>

#### Ordering

When strings are ordered (e.g. map keys):

* Ordering is by **UTF-8 byte lexicographic order**
* Not by Unicode code points
* Not by locale
* Not by human collation rules

Ordering is deterministic and language-independent.



***

### Hashing implications

Strings contribute to hashes via their canonical UTF-8 byte encoding.

Because:

* UTF-8 bytes are preserved
* No normalization occurs
* Encoding is canonical

Hashes involving strings are stable across:

* languages
* platforms
* runtimes



***

### Why normalization is forbidden

Normalization introduces ambiguity:

* Different runtimes normalize differently
* Libraries disagree on defaults
* Hashes become unstable

Strata chooses:

* explicitness over convenience
* bytes over interpretation
* determinism over human-friendliness

If normalization is needed, it must occur **before** data enters Strata.



***

### Non-goals

String rules explicitly do not include:

* locale-aware collation
* case-insensitive comparison
* text shaping
* grapheme clustering

These belong in higher layers.



***

### Stability guarantee

String and UTF-8 rules are **frozen per version**.

Once finalized:

* encoding rules cannot change
* validation rules cannot change
* hashing behavior cannot change

Any modification requires:

* a new version
* a new Northstar
* explicit documentation



***

### Summary

* Strings are UTF-8 bytes
* No normalization
* No interpretation
* Canonical encoding
* Deterministic ordering and hashing

If two strings differ at the byte level, they are different values.
