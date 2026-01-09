# What happens when a test fails

In Strata, a failing test is not a warning.\
It is a **hard stop**.

No exception. No workaround. No interpretation.



***

### Immediate outcome

When any test fails:

* The CI pipeline fails
* The pull request is blocked
* The commit is rejected from main

The system assumes the implementation is wrong until proven otherwise.



***

### Why failure is treated this way

Strata’s guarantees are absolute:

* Same data must produce the same bytes
* Same bytes must produce the same hash
* Same invalid input must fail the same way

A single failing test means at least one of these guarantees is broken.

There is no partial correctness in determinism.



***

### Common failure categories

#### Golden vector mismatch

If a golden vector test fails, one of the following is true:

* Canonical encoding changed
* Map ordering is incorrect
* Integer or byte encoding drifted
* Hashing behavior changed

Resolution rule:

* **Never change the vector**
* Fix the implementation

Vectors are law.



***

#### Negative vector failure

If a negative test fails, it means:

* Invalid input was accepted
* The wrong error kind was produced
* The error offset is incorrect

This is a spec violation.

Silent acceptance is considered corruption.



***

#### Semantic vector failure

Semantic failures indicate:

* Parser ambiguity
* Incorrect shorthand resolution
* Ordering normalization bugs
* Duplicate key behavior changed

These are dangerous failures because they alter meaning without changing bytes.



***

#### Northstar test failure

A Northstar failure means:

* Cross-language parity broke
* Wire transport mutated data
* Framing altered payload boundaries
* Streaming corrupted canonical bytes

Northstar failures are treated as **critical regressions**.



***

### What you must do when CI fails

1. Read the failing test output
2. Identify which guarantee was violated
3. Fix the implementation
4. Re-run tests locally
5. Push again

You do not negotiate with CI.



***

### What you must NOT do

When a test fails, you must not:

* Edit golden vectors to match output
* Relax comparisons
* Add conditionals for “compatibility”
* Skip or ignore tests
* Mark failures as expected

Any of these permanently breaks trust in the system.



***

### When vectors legitimately change

Vectors may only change when **all** are true:

* A new Northstar version is introduced
* The change is explicitly documented
* Old vectors remain valid under their version
* CI enforces version separation

If these conditions are not met, vector changes are forbidden.



***

### Debugging philosophy

Strata debugging follows a simple rule:

> If two implementations disagree, one of them is wrong.

There is no tie-breaker. There is no majority vote. There is no heuristic fallback.

Determinism demands exactness.



***

### The final authority

CI is the final arbiter.

* Human intuition is secondary
* Performance arguments are irrelevant
* Convenience does not matter

If CI fails, the change is invalid.



***

### Summary

A failing test means:

* A Strata guarantee was violated
* The change cannot ship
* The implementation must be fixed

This is not harsh.\
This is how correctness survives at scale.

Strata would rather reject a thousand commits\
than accept one incorrect byte.
