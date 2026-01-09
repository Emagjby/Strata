# Comparison to other formats

Strata exists in a very specific design space. This page compares it to commonly used formats to clarify when Strata is the right choice and when it is not.



***

### Strata vs JSON

**JSON**

* Human-readable
* Flexible and forgiving
* Multiple valid representations of the same value
* Key order is semantically irrelevant
* No canonical binary form
* Hashing JSON reliably requires normalization layers

**Strata**

* Machine-first, canonical
* Exactly one valid binary representation per value
* Key order is fixed and enforced
* Designed for stable hashing and reproducibility
* Strict decoding with explicit failures

**Summary**

JSON optimizes for ease of use. Strata optimizes for correctness.

If two JSON encoders produce different bytes, that is normal. If two Strata encoders do, one of them is wrong.



***

### Strata vs MessagePack

**MessagePack**

* Compact binary format
* Multiple integer encodings
* Optional canonical mode
* Decoders often accept non-canonical input
* Hash stability is optional, not enforced

**Strata**

* Canonical encoding is mandatory
* One integer representation
* No permissive decode paths
* Canonical form is not a mode, it is the format

**Summary**

MessagePack can be canonical if you are careful. Strata is canonical even if you are careless.



***

### Strata vs Protocol Buffers

**Protocol Buffers**

* Schema-driven
* Designed for forward and backward compatibility
* Field reordering is allowed
* Unknown fields are preserved
* Binary output is not stable across implementations or versions

**Strata**

* Schema-agnostic
* No field reordering
* No unknown fields
* Binary output is stable by definition

**Summary**

Protobuf optimizes for evolving APIs. Strata optimizes for invariant data.

If evolution is your priority, Protobuf wins. If byte identity matters, Strata wins.



***

### Strata vs CBOR

**CBOR**

* Flexible binary format
* Optional canonical rules
* Supports tags, floats, and extensions
* Multiple valid encodings for many values

**Strata**

* No optional rules
* No tags or extensions
* No floats in core
* Minimal value model

**Summary**

CBOR is a toolkit. Strata is a contract.



***

### Strata vs BSON

**BSON**

* Designed for document databases
* Focused on storage and queryability
* Includes type metadata and size fields
* Not designed for canonical hashing

**Strata**

* Designed for transport and hashing
* Minimal overhead
* No implicit metadata
* Hash-first design

**Summary**

BSON is a database format. Strata is an integrity format.



***

### Strata vs Custom Canonical Layers

Many systems build their own canonical layers on top of JSON or Protobuf.

This usually involves:

* key sorting
* numeric normalization
* strict schema enforcement
* ad-hoc hashing rules

Strata bakes these guarantees into the format itself.

**Summary**

Strata replaces fragile conventions with enforced invariants.



***

### When Strata is the Wrong Choice

Do not use Strata if you need:

* Schema evolution with backward compatibility
* Floating point arithmetic
* Human-editable storage
* Partial or permissive decoding
* Rich type systems

Strata will actively resist these use cases.



***

### When Strata is the Right Choice

Use Strata when you need:

* Stable content addressing
* Cryptographic hashing
* Cross-language determinism
* Reproducible builds
* Trustless or adversarial environments
* Protocols where bytes matter



***

### Final Perspective

Most formats optimize for humans. Some optimize for networks. Very few optimize for invariants.

Strata optimizes for invariants.
