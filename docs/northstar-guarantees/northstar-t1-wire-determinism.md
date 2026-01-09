# Northstar T1 - Wire determinism

**Northstar T1** guarantees that Strata is _wire-deterministic across languages_.

If a Strata value is encoded on one system, transmitted as canonical bytes, and decoded on another system implemented in a different language, the resulting canonical bytes **and hash must be identical**.

If this guarantee fails, Strata is not wire-safe.



***

### What Northstar T1 guarantees

Northstar T1 asserts the following invariant:

> A Strata value encoded in one language, decoded in another, and re-encoded **must produce identical canonical bytes and identical hashes**.

This guarantee applies regardless of:

* Programming language
* Runtime
* Platform
* Internal data representations

Only canonical bytes matter.



***

### What crosses the boundary

In Northstar T1, **only canonical bytes cross the system boundary**.

Specifically:

* The backend emits canonical Strata Core Binary bytes
* The frontend receives those bytes
* The frontend decodes them into its own in-memory model
* The frontend re-encodes them canonically
* Hashes are compared byte-for-byte

No structured data, schemas, or ASTs are transmitted.



***

### What is explicitly forbidden

Northstar T1 forbids:

* Sharing encoding or decoding logic
* Transmitting JSON, ASTs, or structured payloads
* Normalizing values during transport
* Reconstructing values from semantic hints
* Hashing anything other than canonical bytes

If any of these occur, the test is invalid.



***

### Why T1 exists

Many formats appear interoperable but fail silently when:

* Map ordering differs
* Integer encodings vary
* Language-specific defaults leak in
* Hashes depend on internal structure

Northstar T1 exists to prove that **Strata has exactly one meaning on the wire**, regardless of language.



***

### Failure meaning

If Northstar T1 fails, it means one of the following is true:

* Canonical encoding is inconsistent
* Decoding allows non-canonical interpretation
* Re-encoding is not strictly canonical
* Hashing is not computed over canonical bytes
* Cross-language behavior has diverged

Any of these is a critical failure.



***

### Relationship to the core spec

The core specification defines _how_ values are encoded canonically.

Northstar T1 verifies that those rules survive real cross-language usage without dilution.

It is the first and lowest Northstar.\
All higher Northstars assume T1 holds.



***

### Stability guarantee

Once Northstar T1 is finalized:

* Its behavior must never change
* All future versions of Strata must satisfy it
* Any new feature must preserve this invariant

If T1 ever breaks, Strata must bump its major version and explicitly declare the break.

Northstar T1 is non-negotiable.
