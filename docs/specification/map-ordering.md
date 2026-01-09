# Map ordering

This page defines how **maps** are ordered, encoded, validated, and compared in Strata.

Map ordering is one of the most critical rules in Strata. It is the primary mechanism that eliminates ambiguity in structured data.



***

### Map type overview

A Strata map is a collection of:

* **string keys**
* **associated values**

Maps are:

* unordered by definition at the semantic level
* ordered **canonically** at the encoding level

Canonical ordering is mandatory.



***

### Key constraints

Map keys MUST:

* Be strings
* Be valid UTF-8
* Be unique within a map

Maps with duplicate keys are **not canonical**.



***

### Canonical ordering rule

Maps are encoded with entries sorted by:

**UTF-8 byte lexicographic order of keys**

Important details:

* Ordering is done on UTF-8 bytes
* Not Unicode code points
* Not locale-aware
* Not case-insensitive
* Not human-friendly

This rule is absolute and language-independent.



***

### Ordering algorithm

The canonical ordering algorithm is:

1. Encode each key as UTF-8 bytes
2. Compare byte-by-byte lexicographically
3. Shorter key wins if one is a prefix of the other
4. Emit entries in sorted order

No alternative ordering is allowed.



***

### Examples

Example keys:

* `"a"`
* `"aa"`
* `"b"`
* `"Z"`

UTF-8 byte order determines the sequence, not alphabetical intent.

Different languages, runtimes, or libraries MUST arrive at the same order.



***

### Encoding behavior

During encoding:

* Map entries MUST be sorted canonically
* Duplicate keys MUST be rejected
* The emitted byte sequence MUST follow canonical order

If an encoder emits unordered or duplicate keys, it is incorrect.



***

### Decoding behavior

During decoding:

* Entry order MUST be read as-is
* Canonical order MUST NOT be enforced
* Duplicate keys MAY appear in decoded output

Decoding is observational, not corrective.

This distinction is intentional.



***

### Duplicate keys

#### Canonical encoding

* Duplicate keys are forbidden
* Encoding MUST fail if duplicates exist

#### Decoding non-canonical data

* Duplicate keys MAY be present
* Decoders MUST NOT panic
* Decoders MAY preserve last-write-wins semantics
* Decoders MUST NOT treat such data as canonical

Duplicate keys are a property of malformed or hostile input.



***

### Why ordering matters

Without canonical ordering:

* Identical maps can encode differently
* Hashes become unstable
* Cross-language determinism breaks
* Distributed systems disagree

Canonical map ordering is non-negotiable.



***

### Hashing implications

Because map ordering is canonical:

* Hashes of maps are stable
* Key order in source text is irrelevant
* Language-specific map iteration does not affect output

Hashes are computed over canonical encoded bytes only.



***

### Non-goals

Map ordering explicitly does not provide:

* insertion-order preservation
* sorted views for human readability
* custom comparator hooks
* locale-sensitive sorting

Those belong in higher layers.



***

### Stability guarantee

Map ordering rules are frozen per version.

Once finalized:

* ordering rules cannot change
* comparison semantics cannot change
* encoding behavior cannot change

Any change requires:

* a new version
* a new Northstar
* explicit documentation



***

### Summary

* Maps use string keys only
* Keys are ordered by UTF-8 byte order
* Encoding enforces canonical order
* Decoding observes raw order
* Duplicate keys are non-canonical

Canonical map ordering is a cornerstone of Strata’s determinism.
