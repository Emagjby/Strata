# How Northstar Ts block regressions

Northstar tests are not examples.\
They are **enforced invariants**.

A Northstar defines something that must **never become false**, no matter how the implementation evolves.



***

### The regression problem

Most regressions in data formats are subtle:

* A refactor changes byte ordering
* An optimization alters integer encoding
* A parser becomes more permissive
* A hash is computed over slightly different input
* A transport helper "fixes" malformed data

These changes often:

* Pass unit tests
* Appear logically equivalent
* Break determinism silently

Northstars exist to stop this class of failure.



***

### What a Northstar really is

A Northstar is:

* A **formal invariant**
* A **reproducible test**
* A **hard boundary** for change

It answers one question only:

> "If this fails, has Strata broken one of its promises?"

If the answer is yes, the change is rejected.



***

### How Northstars are enforced

Each Northstar:

* Is implemented as an executable test
* Runs in continuous integration
* Is language-agnostic
* Operates on canonical bytes and hashes

Northstars do not rely on:

* Shared code
* Shared helpers
* Shared ASTs
* Mocked behavior

Only observable outputs matter.



***

### Regression blocking in practice

When code changes:

1. The implementation is rebuilt
2. Northstar tests run automatically
3. Canonical bytes are produced
4. Hashes are computed
5. Outputs are compared exactly

If any byte differs, the build fails.

There is no tolerance window.



***

### Why golden vectors alone are not enough

Golden vectors catch local mistakes. Northstars catch systemic failures.

Golden vectors verify:

* Specific inputs
* Known edge cases

Northstars verify:

* Cross-language behavior
* Transport behavior
* Streaming behavior
* End-to-end determinism

They test the **system**, not the function.



***

### Preventing "harmless" changes

Many regressions are introduced by changes that seem safe:

* Replacing a data structure
* Optimizing map iteration
* Switching integer libraries
* Adjusting error handling
* Adding convenience APIs

Northstars make these changes explicit.

If a change is truly harmless, it passes. If it changes behavior, it fails loudly.



***

### Northstars as change gates

Any change that affects:

* Canonical encoding
* Hash input
* Decode acceptance
* Cross-language parity
* Transport semantics

Must either:

* Preserve all existing Northstars, or
* Introduce a **new Northstar** with a new version boundary

There is no third option.



***

### Long-term stability

Northstars scale over time.

As Strata evolves:

* Old Northstars remain
* New ones are added
* Guarantees accumulate

This creates a one-way ratchet toward correctness.

Once something is guaranteed, it stays guaranteed.



***

### The practical effect

For contributors and maintainers:

* Regressions cannot slip in unnoticed
* Refactors are safe but accountable
* Optimizations are measurable
* Guarantees are explicit

For users:

* Bytes stay stable
* Hashes stay stable
* Behavior stays predictable

Northstars turn promises into law.



***

### Summary

Northstar tests block regression by:

* Encoding guarantees as executable invariants
* Enforcing them continuously
* Rejecting any change that violates them

They are not documentation. They are the enforcement mechanism.

If a Northstar fails, Strata is broken.
