# Hashing contract

Hashing in Strata is not an optimization. It is a **formal contract**.

If two systems compute different hashes for the same logical value, **at least one of them is incorrect**.

There is no acceptable deviation.



***

### What is hashed

Strata hashes are computed over **canonical Strata Core Binary (.scb) bytes only**.

Not over:

* Strata Text
* decoded in-memory structures
* transport envelopes
* framing metadata
* file headers
* runtime-specific representations

Only canonical `.scb` bytes are valid hash input.

If it is not canonical, it is not hashable.



***

### Hash function

Strata uses **BLAKE3-256**.

Properties required by the system:

* deterministic
* fast
* cryptographically secure
* identical across platforms
* stable over time

The hash output is exactly **32 bytes**.



***

### Hash determinism

Hash determinism is guaranteed by construction.

Because:

* canonical encoding is unique
* encoding rules are frozen per version
* all implementations encode identically

The following always holds:

Same logical value\
-> same canonical bytes\
-> same hash

This is a hard invariant.



***

### Hash computation order

Hashing always occurs **after canonical encoding**.

```
hash = BLAKE3(canonical_scb_bytes)
```

There is no alternative order. There is no shortcut. There is no pre-hashing.



***

### No normalization before hashing

Strata forbids any transformation before hashing.

The following are explicitly forbidden:

* map reordering
* whitespace trimming
* integer normalization
* UTF-8 repair
* trailing byte removal
* schema-driven coercion

If input is non-canonical, hashing must not proceed.



***

### Hash stability across languages

All implementations must produce:

* identical canonical bytes
* identical hash output

for the same logical value.

Language choice is irrelevant. Runtime choice is irrelevant. Platform choice is irrelevant.

If two languages disagree, one implementation is wrong.



***

### Hash stability across time

Once a Strata version is finalized:

* canonical encoding is frozen
* hash semantics are frozen

Hashes produced today must match hashes produced in the future for the same value under the same version.

This is not a goal. It is a requirement.



***

### Version boundaries

If a change would alter:

* canonical bytes
* hash input
* hash output

it requires:

* a new minor version
* a new Northstar
* explicit documentation

Hash changes are never silent.



***

### Security implications

This hashing contract enables:

* content-addressed storage
* tamper detection
* digital signatures
* reproducible builds
* distributed consensus
* audit-grade verification

Any deviation breaks these guarantees.



***

### What Strata does not do

Strata hashing does not:

* include transport metadata
* support multiple hash algorithms
* allow runtime configuration
* attempt backward-compatible hashing
* accept non-canonical input

Simplicity is part of the safety model.



***

### Summary

* hashes are computed over canonical `.scb` bytes only
* BLAKE3-256 is mandatory
* encoding precedes hashing
* non-canonical data is not hashable
* hashes are stable across languages and time
* breaking the contract requires a new version

In Strata, **hashes are law**.
