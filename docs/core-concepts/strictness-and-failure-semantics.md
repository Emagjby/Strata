# Strictness & Failure semantics

Strata is strict by design.

Strictness is not an implementation detail or a configuration option.\
It is a **core semantic property** of the system.

When Strata encounters invalid input, it **fails immediately and explicitly**.

There is no recovery, no coercion, and no silent repair.



***

### Why strictness exists

Strata exists to guarantee correctness at the byte level.

Any system that:

* hashes data
* signs data
* reaches consensus on data
* addresses data by content

cannot afford ambiguity.

Leniency introduces ambiguity. Ambiguity breaks determinism. Determinism is non-negotiable.

Strictness is how Strata protects its guarantees.



***

### Failure is part of the contract

In Strata, failure is not an error state.\
It is an **expected and meaningful outcome**.

A failed operation communicates one thing clearly:

> "This input violates the Strata contract."

That signal must never be weakened.



***

### Encoding failure semantics

Encoding is **authoritative** and **enforcing**.

Encoding fails if:

* a value cannot be represented canonically
* an invariant would be violated
* an unsupported construct is encountered
* the value model is breached

Encoding never:

* guesses intent
* coerces types
* normalizes data silently
* repairs invalid input

If encoding fails, **no bytes are produced**.

This ensures that invalid data can never enter the system in canonical form.



***

### Decoding failure semantics

Decoding is strict but interpretive.

Decoding fails if:

* an invalid tag is encountered
* input is truncated
* a varint overflows
* UTF-8 is invalid
* trailing bytes remain
* structural rules are violated

Decoding does not attempt recovery.

Malformed data is rejected immediately, at the exact offset where the violation occurs.



***

### Error locality

All Strata failures are **local and precise**.

Errors report:

* the category of failure
* the exact byte offset (or source span)
* no speculative follow-up behavior

This makes Strata suitable for:

* auditing
* fuzzing
* security-sensitive environments
* protocol boundaries



***

### No silent normalization

Strata never normalizes data silently.

Examples of behaviors Strata explicitly forbids:

* reordering maps on decode
* accepting non-canonical varints
* trimming trailing data
* fixing invalid UTF-8
* repairing malformed frames

If input is wrong, it stays wrong.



***

### Failure vs flexibility

Many formats trade strictness for convenience:

* permissive decoders
* "best effort" parsing
* automatic repair
* backward-compatibility heuristics

Strata does not.

Flexibility in representation leads to ambiguity. Ambiguity breaks hashing and verification. Strata chooses correctness.



***

### Security implications

Strict failure semantics:

* prevent downgrade attacks
* prevent canonicalization attacks
* prevent hash-collision ambiguity
* prevent parser differentials across languages

Every implementation fails the same way for the same input.

This is intentional.



***

### Philosophy

Strata follows a simple rule:

> If the system cannot prove correctness, it must refuse to operate.

Silence is dangerous. Explicit failure is safe.



***

### Summary

* strictness is foundational, not optional
* encoding enforces invariants
* decoding rejects malformed input
* failure is explicit and precise
* no normalization, no recovery, no guessing

In Strata, **failure is correctness asserting itself**.
