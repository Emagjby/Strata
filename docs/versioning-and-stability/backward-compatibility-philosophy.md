# Backward compatibility philosophy

Strata treats backward compatibility as a **design choice**, not an assumption.

This page explains what Strata does, what it does not do, and why.



***

### The core stance

Strata does **not** guarantee backward compatibility across MINOR versions.

This is intentional.

Backward compatibility that is implicit, undocumented, or accidental is a liability in deterministic systems.



***

### Why backward compatibility is not automatic

In Strata, guarantees are stronger than convenience:

* Bytes are contracts
* Hashes are identities
* Semantics are invariants

If compatibility is assumed without being enforced, correctness degrades silently.

Strata refuses silent compatibility.



***

### Compatibility vs determinism

Many formats optimize for:

* evolution
* flexibility
* gradual migration

Strata optimizes for:

* exactness
* verifiability
* long-term reproducibility

These goals conflict.

When they do, Strata chooses determinism.



***

### What backward compatibility WOULD require

Backward compatibility in Strata must be:

* explicit
* documented
* testable
* enforced by vectors and Northstars

If compatibility cannot be proven mechanically, it is not considered compatible.



***

### Allowed forms of compatibility

Backward compatibility MAY exist when:

* New versions introduce additive layers
* Existing canonical bytes remain valid
* Hashes of existing values remain unchanged
* Old values retain identical semantics

In such cases, compatibility is **earned**, not assumed.



***

### Forbidden forms of compatibility

The following are explicitly rejected:

* Silent reinterpretation of bytes
* Auto-upgrading data on decode
* “Best effort” decoding
* Version sniffing that alters semantics
* Accepting malformed data for convenience

If old data behaves differently, compatibility is broken.



***

### Compatibility boundaries

Compatibility in Strata is defined at these boundaries:

* Canonical `.scb` bytes
* Hash output
* Decode acceptance and failure modes
* Cross-language equivalence

Anything outside these boundaries is not compatibility-critical.



***

### Version lines are trust zones

Each MINOR version defines a **trust zone**:

* v0.3.x is one trust zone
* v0.4.x is another

Within a trust zone:

* behavior is frozen
* guarantees are stable

Across trust zones:

* behavior may change
* guarantees may evolve

Trust does not automatically cross version lines.



***

### Why this is safer

This philosophy ensures:

* no accidental breakage
* no ambiguous migrations
* no reliance on undocumented behavior
* no long-term entropy in meaning

Systems depending on Strata can reason about data **without guesswork**.



***

### Practical implication

If you need:

* long-lived hashes
* content addressing
* cryptographic verification
* distributed consensus

You must pin a Strata version explicitly.

This is not a weakness. It is how correctness is preserved.



***

### Summary

Strata does not promise backward compatibility by default.

When compatibility exists:

* it is explicit
* it is tested
* it is enforced

Silence is not compatibility. Assumption is not compatibility.

Only proofs count.
