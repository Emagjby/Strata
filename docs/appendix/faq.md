# FAQ

This section answers common questions about Strata’s design, guarantees, and intended use. If something feels strict, that is usually intentional.



***

### What problem does Strata solve?

Strata solves **byte-level determinism**.

If two systems encode the same logical value, they must produce:

* identical bytes
* identical hashes

This matters for:

* content addressing
* cryptographic hashing
* signatures
* distributed consensus
* reproducible builds
* audit trails

If bytes differ, Strata considers that a bug.



***

### How is Strata different from JSON, Protobuf, or MessagePack?

Those formats prioritize flexibility and evolution.

Strata prioritizes correctness and determinism.

Key differences:

* One valid encoding per value
* No field reordering ambiguity
* No multiple integer encodings
* No permissive decoding
* Canonical bytes are the contract

Strata trades convenience for guarantees.



***

### Is Strata human-readable?

No.

Strata Core Binary (.scb) is intentionally not human-readable.

Strata Text (.st) exists as a **human authoring format**, not a transport format. It always compiles into canonical binary.

Humans read and write `.st`. Machines exchange `.scb`.



***

### Why doesn’t Strata support floats?

Floating-point formats introduce ambiguity:

* platform differences
* rounding behavior
* multiple binary representations for similar values

Strata avoids this entirely at the core layer.

Future versions may introduce deterministic numeric extensions, but the v0.3.x core intentionally excludes floats.



***

### Can I add schemas or validation on top of Strata?

Yes, but **outside** the core.

Strata intentionally does not define:

* schemas
* optional fields
* defaults
* validation rules

These can be layered on top without affecting canonical encoding.

The core format stays small and frozen.



***

### Is Strata fast?

Strata is designed to be:

* simple to encode
* simple to decode
* easy to verify

Performance is predictable and stable. There are no hidden normalization steps or heuristics.

Correctness is prioritized over micro-optimizations.



***

### Can Strata be streamed?

Yes.

Strata itself is framing-agnostic. Northstar T3 proves that canonical bytes survive arbitrary framing.

Framing is a transport concern, not a format concern.



***

### What happens if decoding fails?

Decoding fails explicitly.

Strata decoders:

* reject malformed input
* reject non-canonical encodings
* report precise error offsets and kinds
* never silently recover

If decoding succeeds, the value is canonical.



***

### What are Northstars?

Northstars are **invariants enforced by tests**.

Each Northstar proves a specific guarantee, such as:

* cross-language determinism
* raw wire safety
* framed streaming safety

If a Northstar fails, the change is rejected.

Northstars block regressions by design.



***

### Is Strata stable?

Yes, within its defined scope.

As of v0.3.x, the following are frozen:

* canonical encoding
* hashing semantics
* value model
* decode behavior

Future versions may add layers, but existing guarantees will not be weakened.



***

### Who is Strata for?

Strata is for systems that need:

* correctness over convenience
* reproducibility over flexibility
* byte-level guarantees

If you need rapid schema evolution or human-readable transport, Strata is probably not the right tool.

That is an intentional tradeoff.



***

### Where is the source of truth?

Golden vectors.

* Rust must match them
* JavaScript must match them
* CI enforces them

If code disagrees with vectors, the code is wrong.

Always.



***

### Still unsure?

A good rule of thumb:

If your system cares about **exact bytes**, Strata fits. If it doesn’t, use something else.

Strata is opinionated by design. That is its strength.
