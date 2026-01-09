# Canonical rules

This page defines what **canonical** means in Strata.

Canonical rules are not guidelines.\
They are **hard invariants** enforced by every correct implementation.

If two implementations disagree on canonical output, at least one of them is wrong.



***

### Canonical definition

A Strata value has **exactly one valid binary representation**.

No alternatives. No equivalent forms. No normalization step.

Given the same logical value, all correct implementations must produce **identical bytes**.



***

### Scope of canonical rules

Canonical rules apply to:

* Binary encoding of all value types
* Integer representation
* UTF-8 string encoding
* Byte sequence representation
* Map key ordering
* Hash input definition

Canonical rules do **not** apply to:

* Transport framing
* Streaming boundaries
* Envelopes or wrappers
* Compression layers
* Encryption or signatures
* Protocol metadata

Canonicality exists strictly at the **value -> bytes** boundary.



***

### Single representation rule

For every Strata value:

* there is one valid encoding
* all other encodings are invalid
* invalid encodings must be rejected

There is no concept of:

* equivalent encodings
* permissive decoding
* post-decode normalization

Canonical form is absolute.



***

### Encoding authority

Encoding is the **source of truth**.

Encoding enforces canonical rules by construction.

If a value violates a rule:

* encoding fails
* no bytes are produced

Encoding never:

* guesses intent
* repairs data
* coerces types
* relaxes constraints



***

### Decoding relationship to canonicality

Decoding does not enforce canonical form.

Decoding:

* reconstructs structure
* exposes malformed or hostile input
* preserves observed ordering where applicable

Canonicality is enforced **only during encoding**.

This separation is intentional.

> Decoding reveals reality.\
> Encoding enforces truth.



***

### Canonical ordering

Where ordering is defined, it is deterministic and absolute.

Examples:

* Map keys are ordered by UTF-8 byte sequence
* Lists preserve explicit order
* Bytes preserve exact sequence

No locale rules. No Unicode normalization. No platform influence.



***

### Canonical rejection

Non-canonical input must not be silently accepted.

Examples of invalid canonical states:

* multiple encodings for the same integer
* non-canonical map ordering
* invalid UTF-8
* duplicate map keys at encode time
* trailing bytes after a value

Implementations must fail explicitly.



***

### Hash canonicality

Hashes are computed over **canonical bytes only**.

Hash input:

* includes only the canonical binary encoding
* excludes transport, framing, or metadata
* is identical across platforms and languages

If two systems hash different bytes for the same value, canonical rules were violated earlier.



***

### Frozen guarantees

Once canonical rules are finalized for a version:

* they do not change
* they are never weakened
* they are never retroactively reinterpreted

Any change that alters canonical bytes:

* requires a new version boundary
* requires a new Northstar invariant
* must be explicitly documented



***

### Non-goals of canonical rules

Canonical rules do not attempt to be:

* human-friendly
* flexible
* forward-compatible by default
* schema-aware
* self-describing

These concerns belong in higher layers, not in the canonical core.



***

### Philosophy

Canonical rules exist to answer one question conclusively:

> "Are these bytes correct?"

If the answer is uncertain, the system must refuse to proceed.



***

### Summary

* one value -> one encoding
* encoding enforces canonical truth
* decoding does not normalize
* hashes depend on canonical bytes
* canonical rules are frozen per version

Canonical encoding is the foundation that everything else stands on.
