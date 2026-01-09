# Adding vectors

Golden vectors are the **source of truth** in Strata.

They define what is correct behavior across all implementations. Code exists to satisfy vectors, never the other way around.

This page explains when, why, and how to add vectors.



***

### What vectors are

A vector is a **fully specified test case** that defines:

* Input representation (`.st` or raw bytes)
* Canonical binary output (`.scb.hex`)
* Canonical hash (`.hash.hex`)
* Or a required failure (`.error.json`)

Vectors are language-agnostic. They are consumed directly by Rust, JavaScript, and any future implementations.



***

### When vectors must be added

Vectors MUST be added in the following situations:

* Introducing a new Northstar
* Adding a new invariant
* Extending canonical behavior
* Adding new error cases
* Locking down edge cases
* Preventing a regression

If behavior matters, it must be vectorized.



***

### When vectors must NOT be added

Vectors must NOT be added to:

* Patch over a broken implementation
* “Fix” failing tests without an invariant
* Encode undocumented behavior
* Introduce ambiguity
* Change existing truth silently

If a vector contradicts existing vectors, the change requires a new version and a new Northstar.



***

### Vector categories

Strata vectors are grouped by purpose.

#### Positive vectors

Define valid inputs and their canonical outputs.

They include:

* `.st` source
* `.scb.hex` canonical binary
* `.hash.hex` canonical hash

These vectors prove correct behavior.



***

#### Negative vectors

Define invalid inputs and required failures.

They include:

* `.hex` raw input bytes
* `.error.json` expected error kind and offset

Negative vectors prove strictness.



***

#### Semantic vectors

Define higher-level invariants such as:

* Decode → encode round-trips
* Non-canonical input normalization
* Structural equivalence
* Cross-version behavior

These vectors prove meaning, not just bytes.



***

### Directory structure

Vectors are versioned and never overwritten.

Example:

```
vectors/
├── v1/
│   ├── 01-basic.st
│   ├── 01-basic.scb.hex
│   ├── 01-basic.hash.hex
│   └── …
├── v2/
│   └── …
├── v2.1/
│   ├── neg-01-invalid-tag.hex
│   ├── neg-01-invalid-tag.error.json
│   └── …
```

Rules:

* New vectors go into a new version directory
* Existing vectors are immutable
* Version directories correspond to Northstars



***

### Naming rules

Vector names must be:

* Stable
* Descriptive
* Ordered when possible

Examples:

* `01-basic`
* `02-map-order`
* `03-bigint-bytes`
* `neg-01-invalid-tag`
* `neg-02-truncated-string`

Do not rename vectors once published.



***

### Writing a positive vector

A positive vector must include:

* A clear `.st` source
* The exact canonical `.scb.hex`
* The exact canonical `.hash.hex`

All three must agree.

The `.st` file is human-facing. The `.scb.hex` and `.hash.hex` are law.



***

### Writing a negative vector

A negative vector must include:

* A `.hex` file with raw bytes
* A `.error.json` file describing:
  * Error kind
  * Error offset

Errors must be precise. Offsets matter.

Example `.error.json`:

```
{
    “kind”: “InvalidUtf8”,
    “offset”: 2
}
```



***

### Validation requirements

Every added vector MUST:

* Be consumed by Rust tests
* Be consumed by JavaScript tests
* Fail loudly if behavior diverges
* Run in CI

A vector that is not enforced does not exist.<br>

***

### Never modify existing vectors

This rule is absolute.

* Do not edit
* Do not reformat
* Do not regenerate
* Do not “fix” old vectors

If existing vectors are wrong, the fix is:

* A new version
* New vectors
* A new Northstar

History is immutable.



***

### Vectors over opinions

If there is disagreement about behavior:

* Write a vector
* Encode the invariant
* Let CI decide

Discussions end where vectors begin.



***

### Summary

Adding vectors means:

* Encoding truth, not convenience
* Freezing behavior permanently
* Enforcing invariants across languages

If something matters, add a vector. If it does not have a vector, it is not guaranteed.
