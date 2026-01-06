# Contributing to Strata

Thank you for your interest in contributing to Strata.

Strata is a deterministic data format with strict canonical guarantees.
As such, changes are held to a high standard.

---

## Scope of Contributions

We welcome contributions in the following areas:

- Documentation improvements
- Additional tests and golden vectors
- Bug fixes that do not alter canonical behavior
- Cross-language parity improvements

Changes to canonical encoding rules require explicit discussion and
a new Northstar invariant.

---

## Canonical Stability

Strata prioritizes stability over features.

- Canonical encoding behavior must not change without a Northstar
- Existing golden vectors must remain valid
- Hash stability is treated as a contract

If a proposed change would alter bytes or hashes, it will be rejected
unless it is explicitly ratified.

---

## Submitting Changes

- Fork the repository
- Make focused, minimal commits
- Ensure all tests pass
- Describe _why_ the change is correct, not just _what_ it does

---

## Code of Conduct

All contributors are expected to follow the Code of Conduct.
