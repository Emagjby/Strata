# Common misconceptions

Strata is deliberately strict and opinionated. That tends to create misunderstandings. This page addresses the most common ones.



***

### "Strata is just another serialization format"

No.

Strata is not primarily about serialization. It is about **canonical representation**.

Serialization is incidental. The real contract is:

* one logical value
* one binary representation
* one hash

If you only need to move data around, Strata is probably unnecessary.



***

### "Strata is too strict to be practical"

Strictness is the point.

Permissive formats work well until correctness depends on bytes. At that point, ambiguity becomes a liability.

Strata removes ambiguity up front so systems do not have to reason about it later.



***

### "Strata should support schema evolution like Protobuf"

Schema evolution and canonical encoding conflict.

Allowing optional fields, defaults, or reordering introduces multiple valid encodings. That breaks determinism.

Strata intentionally keeps schemas out of the core. Schema layers can exist above it.



***

### "Strata Text is the canonical format"

No.

Strata Core Binary (.scb) is canonical. Strata Text (.st) is an authoring format.

Multiple `.st` files may describe the same value. Only one `.scb` representation is valid.



***

### "Decoders should accept non-canonical data and normalize it"

No.

Silent normalization hides bugs and weakens guarantees.

Strata decoders either:

* accept canonical data
* or fail explicitly

There is no in-between.



***

### "Map key order shouldn’t matter"

Map key order absolutely matters at the byte level.

Strata enforces canonical ordering so that:

* all encoders emit keys in the same order
* hashes are stable
* re-encoding is idempotent

Logical equality implies byte equality.



***

### "Hashes are just a convenience feature"

Hashes are part of the contract.

Strata is designed so that:

* hashing is deterministic
* hashing input is unambiguous
* hashing behavior is stable across languages

If hashes change unexpectedly, something is broken.



***

### "Strata is slow because it uses BigInt"

BigInt is used for correctness, not for convenience.

Performance remains predictable because:

* encoding rules are simple
* there are no fallback paths
* no normalization passes exist

In practice, performance is stable and debuggable.



***

### "Strata will eventually add everything JSON has"

No.

Strata is intentionally small.

New features are added only if they:

* preserve canonical guarantees
* do not introduce ambiguity
* can be enforced by Northstars

Convenience alone is not sufficient justification.



***

### "Future versions will break existing data"

Only if you opt into them.

Strata does not silently evolve.

If bytes or hashes change:

* the version changes
* a new Northstar is introduced
* the break is explicit and documented

Existing versions remain valid.



***

### "Strata is trying to replace everything"

It is not.

Strata is a foundational layer. It does one thing well and leaves the rest to higher layers.

It fits best where correctness is non-negotiable.



***

### Final note

Most misconceptions come from expecting Strata to behave like flexible formats.

It does not.

That is intentional.
