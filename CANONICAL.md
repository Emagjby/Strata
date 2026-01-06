# Strata Canonical Encoding

This document defines what "canonical" means in Strata and what is frozen as of v0.3.x.

Canonical rules are not guidelines.  
They are enforced invariants.

---

## Definition of Canonical

A Strata value has exactly **one valid binary representation**.

If two encoders produce different bytes for the same logical value, at least one implementation is incorrect.

There is no concept of:

- equivalent encodings
- permissive decoding
- normalization after decoding

Canonical encoding is absolute.

---

## Canonical Scope

Canonical rules apply to:

- Binary encoding of values
- Ordering of map keys
- Integer representation
- String encoding (UTF-8)
- Byte sequences
- Hashing input

Canonical rules do **not** apply to:

- Transport framing
- Streaming boundaries
- Envelopes or wrappers
- Compression or encryption

---

## Frozen Guarantees (v0.3.x)

As of v0.3.x, the following are frozen and must not change:

- Encoding of all value types
- Binary layout and byte order
- Map key ordering rules
- Hash input definition
- Decode failure conditions

Any change to the above requires:

- a new Northstar
- a new minor version
- explicit documentation

---

## Decode Strictness

Strata decoding is strict by design.

The decoder MUST fail on:

- malformed input
- truncated values
- invalid UTF-8
- trailing bytes
- invalid tags or lengths

Silent recovery is forbidden.

---

## Hashing Contract

Hashes are computed over **canonical encoded bytes only**.

Hashing:

- does not include transport headers
- does not include framing metadata
- does not depend on runtime or platform

If two implementations hash different bytes for the same value, canonical rules have been violated.

---

## Non-Goals

Canonical encoding explicitly does not attempt to be:

- human-readable
- self-describing
- schema-aware
- forward-flexible

These concerns are intentionally left outside the canonical layer.

---

## Stability Statement

Canonical rules defined in this document are stable for all v0.3.x releases.

They will not change without a new version boundary and a new Northstar.
