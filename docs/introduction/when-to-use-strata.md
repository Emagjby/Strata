# When to use Strata

Strata is not a general-purpose data format. It is designed for systems where **byte-level determinism is a requirement**, not a preference.

If your system treats differences in encoding as acceptable, Strata is probably the wrong choice. If your system treats them as bugs, Strata may be exactly what you need.



***

### Use Strata when correctness depends on bytes

Strata is a good fit when the exact binary representation of data matters.

Typical examples include:

* Content-addressed storage
* Hash-based identifiers
* Digital signatures
* Deterministic builds or artifacts
* Distributed systems that compare hashes
* Cross-language pipelines that must agree exactly
* Long-lived data that must remain stable over time

In these systems, two logically identical values producing different bytes is not a cosmetic issue.\
It is a correctness failure.

Strata eliminates this class of problems by construction.



***

### Use Strata when hashes are part of your contract

If your system exposes or relies on hashes, determinism is not optional.

Strata guarantees that:

* Hashes are computed over canonical bytes
* Identical values always hash identically
* Hashes are stable across languages and platforms
* Hashes remain stable within a finalized version

This makes Strata suitable for:

* Merkle trees
* Content addressing
* Cache keys
* Audit logs
* Verifiable data pipelines

If a hash mismatch is unacceptable, Strata is designed for that environment.



***

### Use Strata for cross-language correctness

Strata is designed to be implemented independently in multiple languages.

Each implementation must:

* produce identical canonical bytes
* compute identical hashes
* reject the same invalid inputs

Correctness is enforced using shared golden vectors and cross-language tests.

If your system spans Rust, JavaScript, or other runtimes and must behave identically at the byte level, Strata provides a common ground.



***

### Use Strata when data must outlive implementations

Strata is designed for **temporal stability**.

Once a Strata version is finalized:

* encoding rules are frozen
* hashing behavior is frozen
* semantics do not drift

This makes Strata suitable for:

* archival data
* long-lived identifiers
* systems with strict backward reasoning
* environments where re-encoding old data must never change meaning

If your data needs to remain valid and verifiable years later, this matters.



***

### Summary

Use Strata when:

* identical values must produce identical bytes
* hashes are part of system correctness
* cross-language determinism matters
* stability matters more than flexibility

Strata exists for systems where "close enough" is not enough.
