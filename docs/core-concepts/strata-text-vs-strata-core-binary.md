# Strata Text vs Strata Core Binary

Strata consists of **two distinct layers** with different purposes:

* **Strata Text (`.st`)** — human-facing authoring format
* **Strata Core Binary (`.scb`)** — canonical, machine-facing format

They are related, but they are not equal.



***

### Two-layer architecture

Strata is deliberately split into:

1. A **convenience layer** for humans
2. A **truth layer** for machines

Only one of these defines correctness.



***

### Strata Core Binary (`.scb`)

Strata Core Binary is the **source of truth**.

It is the only layer that:

* defines canonical encoding
* defines hashing input
* participates in determinism guarantees
* is used for storage and transport
* is covered by Northstar guarantees

If something is ambiguous in `.st` but unambiguous in `.scb`, `.scb` wins.



***

#### Properties of `.scb`

Strata Core Binary is:

* fully deterministic
* canonical
* unambiguous
* stable across languages
* stable across time
* hostile-input safe

Every valid Strata value has **exactly one** `.scb` representation.



***

#### What `.scb` is used for

`.scb` is used for:

* hashing
* signing
* verification
* transport
* storage
* cross-language exchange
* audit trails

Any system that relies on correctness must operate on `.scb`.



***

### Strata Text (`.st`)

Strata Text is a **human authoring format**.

It exists to make Strata usable by humans, not to define truth.



***

#### Properties of `.st`

Strata Text is:

* readable
* writable
* flexible
* ergonomic
* discardable

It is **not canonical**.



***

#### What `.st` is used for

`.st` is used for:

* configuration files
* fixtures
* test vectors
* hand-authored data
* inspection and debugging
* tooling input

`.st` exists to compile into `.scb`.



***

### Compilation boundary

The transition from `.st` to `.scb` is a **one-way boundary**:
