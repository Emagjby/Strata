# Northstar T2 - Raw wire determinism

**Northstar T2** guarantees that Strata Core Binary survives _raw transport_ without assistance, envelopes, or protocol-level interpretation.

If a Strata payload is transmitted as raw bytes, decoded, re-encoded, and re-hashed, the result **must be identical**.

If this guarantee fails, Strata is not protocol-safe.



***

### What Northstar T2 guarantees

Northstar T2 asserts the following invariant:

> Canonical Strata Core Binary bytes may be transmitted as raw bytes, decoded and re-encoded, and must yield identical canonical bytes and identical hashes.

No framing.\
No metadata.\
No helpers.

Just bytes.



***

### What makes T2 different from T1

Northstar T1 allows an outer transport to help carry bytes, for example via base64 or JSON envelopes.

Northstar T2 removes that safety net.

In T2:

* Bytes are transmitted exactly as produced
* No encoding layer is allowed to reinterpret them
* The receiver must treat the payload as opaque binary data

This is closer to real-world usage.



***

### Transport assumptions

Northstar T2 assumes:

* A byte-oriented transport (HTTP body, file, socket, message queue)
* A declared content type (for example `application/strata`)
* No transformation by intermediaries

If the transport mutates bytes, the test must fail.



***

### What is explicitly forbidden

Northstar T2 forbids:

* Base64 encoding
* JSON envelopes
* Schema-aware reconstruction
* Content normalization
* Re-encoding before hashing
* Ignoring malformed payloads

The payload is either valid canonical Strata or it is rejected.



***

### Why T2 exists

Many formats appear stable until they are sent as raw bytes.

Failures often occur due to:

* Hidden text encodings
* Truncation
* Padding or alignment changes
* Implicit framing assumptions
* Transport-layer normalization

Northstar T2 exists to prove that **Strata is safe as pure binary**, without scaffolding.



***

### Failure meaning

If Northstar T2 fails, it means one of the following is true:

* Canonical encoding is not self-contained
* Decoding accepts non-canonical byte sequences
* Re-encoding is not idempotent
* Hashing depends on representation rather than bytes
* The format relies on transport helpers

Any of these breaks Strata’s guarantees.



***

### Relationship to other Northstars

Northstar T2 assumes Northstar T1 already holds.

T1 proves cross-language determinism.\
T2 proves raw-wire safety.

Higher Northstars build on both.



***

### Stability guarantee

Once Northstar T2 is finalized:

* Its invariant must never change
* All future Strata versions must pass it
* Any new feature must preserve raw-wire determinism

If a change breaks T2, it requires a new major version and explicit declaration.

Northstar T2 defines Strata’s minimum bar for protocol usage.
