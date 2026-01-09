# Security conciderations

Strata is not a security protocol. It is a deterministic data format with strict correctness guarantees.

This page explains what Strata does and does not protect against, and how it should be used safely in security-sensitive systems.



***

### What Strata Guarantees

Strata provides strong guarantees at the **representation level**:

* Identical values always produce identical bytes
* Identical bytes always produce identical hashes
* Canonical encodings are enforced, not optional
* Malformed or ambiguous input is rejected explicitly

These properties are foundational for building secure systems, but they are not security mechanisms by themselves.



***

### What Strata Does NOT Do

Strata intentionally does **not** provide:

* Encryption
* Authentication
* Authorization
* Key management
* Access control
* Replay protection
* Confidentiality guarantees

Strata should never be treated as a secure channel or a cryptographic protocol.



***

### Hashing Is Not Security

Strata hashing provides **identity**, not **protection**.

* Hashes identify content
* Hashes do not hide content
* Hashes do not prove trust by themselves

Hashing is safe to use for:

* content addressing
* integrity checks
* deduplication
* deterministic identifiers

Hashing is NOT sufficient for:

* authentication
* tamper-proof storage without signatures
* access control

If you need trust, you must layer cryptography on top.



***

### Recommended Security Model

Strata is designed to be embedded inside higher-level security systems.

Typical safe layering looks like:

* Strata for canonical representation
* Cryptographic hashing over canonical bytes
* Digital signatures over hashes
* Transport encryption (TLS, Noise, QUIC, etc.)
* Application-level authorization

Strata ensures that what you sign or hash is unambiguous. It does not decide who you trust.



***

### Input Validation and Hostile Data

Strata decoders are strict by design.

* Invalid tags cause immediate failure
* Truncated values cause failure
* Invalid UTF-8 is rejected
* Trailing bytes are rejected

This reduces ambiguity but does not eliminate all attack vectors.

Implementations should still:

* Enforce size limits
* Limit recursion depth
* Apply resource constraints
* Treat untrusted input as hostile

Canonical does not mean safe from denial-of-service.



***

### Resource Exhaustion Risks

Strata allows arbitrarily nested structures and large values.

Attackers may attempt to exploit this by sending:

* Extremely large lists or maps
* Deeply nested structures
* Very large byte or string values

Mitigations must be applied at the application or transport layer:

* Maximum payload size
* Maximum nesting depth
* Timeouts during decoding
* Memory usage limits

These limits are intentionally not defined by the core format.



***

### Map Semantics and Duplicate Keys

Canonical Strata maps cannot contain duplicate keys.

However, malformed or hostile input may include duplicates.

Decoders may:

* Preserve last-write-wins behavior for inspection
* Reject non-canonical data during validation

Applications must never treat decoded non-canonical data as trusted canonical state without re-encoding and verification.



***

### Transport and Framing Risks

Strata does not define framing or transport semantics.

When transmitting Strata:

* Framing must not alter payload bytes
* Framing metadata must not be included in hashing
* Partial or truncated frames must be rejected

All transport security concerns live outside the Strata layer.



***

### Versioning and Downgrade Attacks

Strata versions define different guarantees.

Applications should:

* Pin accepted Strata versions explicitly
* Reject unknown or unsupported versions
* Avoid automatic upgrades or silent compatibility

Version negotiation must be explicit to avoid downgrade or confusion attacks.



***

### Summary

Strata is safe by design, but not secure by itself.

It provides:

* determinism
* reproducibility
* strict validation

It does not provide:

* secrecy
* trust
* authorization

Used correctly, Strata is an ideal foundation for secure systems. Used incorrectly, it offers no protection at all.

Correctness is necessary for security. It is never sufficient on its own.
