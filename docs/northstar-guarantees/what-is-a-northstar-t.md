# What is a Northstar T

A **Northstar T** is a _formal, executable guarantee_ about Strata’s behavior under real-world conditions.

Each Northstar T defines a **non-negotiable property** that Strata must uphold across implementations, languages, runtimes, and transports. These guarantees are not theoretical. They are enforced continuously through cross-language tests.

If a Northstar T fails, Strata is considered **broken**, regardless of whether encoding or decoding appears to succeed locally.



***

### Why "Northstar"

Northstars exist to prevent silent regression.

As Strata evolves, new features, layers, tooling, and integrations will be added. The Northstars act as fixed reference points that ensure:

* Core guarantees do not drift
* Determinism is never weakened
* Compatibility across languages remains intact
* Hashes never change unexpectedly

They are not optional goals.\
They are **hard invariants**.



***

### What a Northstar T is (and is not)

A Northstar T **is**:

* A precise behavioral contract
* A cross-language property
* Backed by runnable tests
* Enforced in CI
* Immutable once finalized

A Northstar T **is not**:

* A feature description
* A performance benchmark
* A convenience abstraction
* A suggestion or best practice

Northstars describe _what must always be true_, not _how it is implemented_.



***

### Structure of a Northstar T

Each Northstar T:

* Defines a specific scope of determinism
* Specifies exactly what crosses system boundaries
* Forbids hidden normalization or interpretation
* Is validated using independent implementations

All Northstars are layered. Higher Northstars assume the guarantees of lower ones.



***

### The Northstar hierarchy

Strata currently defines the following Northstars:

* **Northstar T1** – Wire determinism
* **Northstar T2** – Raw wire determinism
* **Northstar T3** – Framed streaming determinism

Each one increases environmental complexity while preserving the same core promise: **canonical bytes and hashes must not change**.



***

### Why Northstars are separate from the core spec

The Strata specification defines _what canonical encoding is_.

Northstars define _what must remain true when that encoding is used in reality_: across HTTP, streams, languages, runtimes, and tooling.

This separation ensures the spec remains clean, while guarantees remain enforceable.



***

### Immutability of Northstars

Once a Northstar T is finalized:

* Its guarantee cannot be weakened
* Existing tests must always pass
* Future extensions must layer on top, never rewrite history

New Northstars may be added. Existing ones are never redefined.

This is how Strata remains stable over time.
