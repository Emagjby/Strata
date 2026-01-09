# Encoding vs decoding

Encoding and decoding in Strata are **not symmetrical operations**.

They serve different purposes, enforce different rules, and carry different guarantees.

Understanding this distinction is critical to using Strata correctly.



***

### Encoding

**Encoding** transforms a structured value into **canonical Strata Core Binary (`.scb`)**.

It is a **strict**, **opinionated**, and **enforcing** operation.

```
Value -> encode -> .scb
```



***

### What encoding guarantees

Encoding guarantees that:

* the output is canonical
* the output is unambiguous
* the output is deterministic
* the output is valid Strata Core Binary
* the output has exactly one possible byte representation

If encoding succeeds, the bytes are correct by definition.



***

### What encoding enforces

Encoding enforces:

* canonical map ordering
* canonical integer encoding
* canonical length encoding
* canonical tag usage
* rejection of invalid values<br>

Encoding does **not** preserve:

* source formatting
* original key order
* comments
* syntactic sugar
* author intent<br>

Only semantics survive.



***

### Encoding failure

Encoding fails if:

* the value is invalid
* a rule would be violated
* canonical representation cannot be produced

Encoding never guesses.\
Encoding never coerces.\
Encoding never normalizes silently.

Failure is explicit.



***

### Decoding

**Decoding** transforms raw bytes into a structured value.

```
.scb -> decode -> Value
```

Decoding is **interpretive**, not enforcing.



***

#### What decoding guarantees

Decoding guarantees that:

* the bytes are structurally valid
* the structure can be interpreted
* the value can be reconstructed

Decoding does **not** guarantee:

* canonical origin
* uniqueness of representation
* semantic correctness beyond structure

Decoding answers one question only:

> "Do these bytes represent a valid Strata value?"



***

#### Strict decoding

Strata decoding is still **strict**:

* invalid tags are rejected
* truncated inputs are rejected
* malformed varints are rejected
* invalid UTF-8 is rejected
* trailing bytes are rejected

Invalid data does not round-trip.



***

#### What decoding does not enforce

Decoding does **not** enforce:

* canonical map ordering
* canonical source structure
* minimal representations

Those guarantees exist only at encoding time.



***

### Canonical boundary

The canonical boundary is **encoding**, not decoding.

```
anything -> encode -> canonical
```

```
bytes -> decode -> interpretation
```

Decoding reveals. Encoding defines.



***

### Hashing relationship

Hashing operates on **encoded bytes**, never on decoded values.

```
Value -> encode -> hash
```

Never:

```
decode -> normalize -> hash
```

If hashing depended on decoding behavior, determinism would be lost.



***

### Round-tripping

A valid round-trip looks like this:

```
Value -> encode -> decode -> Value
```

The reverse is not guaranteed:

```
bytes -> decode -> encode
```

Encoding after decoding may produce **different bytes** if the original input was non-canonical.

This is intentional.



***

### Why this asymmetry exists

This asymmetry exists to:

* eliminate ambiguity
* centralize correctness
* simplify reasoning
* protect hashing and signatures
* allow safe decoding of hostile input

Encoding is law. Decoding is observation.



***

### Common mistake

> "If decoding accepts it, encoding should reproduce it."

No.

If decoding accepted it but encoding changes it, the input was not canonical.



***

### Summary

* encoding enforces truth
* decoding interprets data
* canonical form exists only after encoding
* hashing depends on encoding, not decoding
* asymmetry is a feature, not a bug

In Strata, **encoding is authority**.
