# How to contribute

Strata welcomes contributions, but they are held to a **higher bar than most projects**.

Strata is not a feature-driven library. It is a correctness-driven system.

Every contribution is evaluated against one question:

> Does this preserve or strengthen Strata's guarantees?

If the answer is unclear, the change will not be accepted.



***

### What contributions are welcome

The following types of contributions are encouraged:

* Documentation improvements and clarifications
* Additional tests and test coverage
* New golden vectors
* Bug fixes that restore documented behavior
* Cross-language parity fixes
* Tooling improvements that do not affect canonical bytes

These contributions improve confidence without changing meaning.



***

### What contributions require discussion

The following changes require prior discussion before submission:

* Any change touching encoding or decoding logic
* Any change that could affect canonical output
* Any change that could affect hashing
* Any change to failure modes or error offsets
* Any change that introduces new semantics

These are not automatically rejected, but they must be justified.



***

### What contributions are not accepted

The following changes will be rejected:

* Changes that alter canonical bytes without a new Northstar
* Changes that break existing golden vectors
* Changes that introduce permissive decoding
* Changes that hide or soften errors
* Changes that prioritize convenience over determinism

Strata does not accept "mostly correct" behavior.



***

### Contribution workflow

A correct contribution follows this flow:

1. Fork the repository
2. Create a focused branch
3. Make a minimal, well-scoped change
4. Add or update tests if applicable
5. Run all tests and Northstars locally
6. Submit a pull request with a clear explanation

Large or vague pull requests will be rejected.



***

### Explaining your change

When submitting a contribution, explain:

* What invariant the change preserves or enforces
* Why the change is correct
* Why existing behavior was wrong or incomplete
* Why this change does not violate frozen guarantees

Descriptions like "refactor" or "cleanup" without justification are insufficient.



***

### Review process

All pull requests are reviewed for:

* Canonical safety
* Cross-language determinism
* Vector compliance
* Failure-mode correctness

If reviewers cannot prove correctness, the change will not be merged.



***

### Philosophy

Strata is intentionally conservative.

This is not bureaucracy. This is how deterministic systems stay trustworthy.

Contributing to Strata means treating correctness as a first-class feature.



***

### Summary

* Small, precise changes are preferred
* Correctness beats convenience
* Tests and vectors matter more than code style
* Determinism is non-negotiable

If you enjoy building systems where "almost" is unacceptable, you are in the right place.
