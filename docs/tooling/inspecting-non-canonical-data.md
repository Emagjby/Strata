# Inspecting non-canonical data

Strata deliberately separates **inspection** from **canonical truth**.

This allows tools to **observe reality** even when the data is malformed, non-canonical, or hostile, without silently fixing it.



***

### What is non-canonical data?

Non-canonical data is any Strata Core Binary (`.scb`) that:

* Uses valid tags but non-canonical ordering
* Contains duplicate map keys
* Preserves unexpected structure ordering
* Was produced by a non-canonical encoder
* Originated from untrusted or legacy systems

Non-canonical does **not** necessarily mean invalid.



***

### Why inspection matters

Inspection exists so that:

* Debugging is possible
* Auditing is possible
* Migration is possible
* Security analysis is possible
* Canonical violations are visible, not hidden

Strata refuses to "fix" data silently.



***

### Decode vs Encode philosophy

Decoding answers: **"What bytes did I actually receive?"**

Encoding answers: **"What bytes should exist?"**

These are intentionally different questions.



***

### Decoder behavior

The decoder:

* Accepts structurally valid input
* Preserves observed ordering where possible
* Reports exact failure offsets
* Does not reorder maps
* Does not normalize values
* Does not drop duplicate keys during decode

This allows inspection of wire-level reality.



***

### Inspection workflow

Typical inspection flow:

1. Receive raw `.scb` bytes
2. Decode using `decode`
3. Inspect decoded structure
4. Decide what to do next

Possible next steps:

* Reject
* Re-encode canonically
* Log and continue
* Migrate
* Audit



***

### CLI inspection

To inspect non-canonical binary data:

```
strata decode input.scb
```

This prints a structured, readable representation of the decoded value.

No re-encoding occurs.



***

### Example: non-canonical map order

Binary input may encode map keys out of order.

Decoding preserves the observed structure.

Re-encoding the decoded value will produce **canonical ordering**, which may change bytes and hashes.

This is intentional.



***

### Duplicate keys

During decoding:

* Duplicate keys are preserved at decode time
* Inspection tools may surface last-write behavior
* No implicit error is thrown unless required by the spec

Encoding enforces canonical rules.



***

### Hashing implications

Hashing **must never** be performed on non-canonical bytes if stability is required.

Rules:

* Hash raw bytes only if you trust the producer
* Hash values only after canonical encoding
* Never assume decoded structure equals canonical structure



***

### Security implications

Inspection without normalization prevents:

* Hash confusion attacks
* Canonical smuggling
* Structure laundering
* Silent data mutation

Strata treats visibility as a security feature.



***

### What inspection is NOT

Inspection is not:

* Validation
* Canonicalization
* Repair
* Forgiveness
* Schema enforcement

Those steps are explicit and opt-in.



***

### Summary

Strata allows you to **see broken reality clearly**.

* Decode to observe
* Inspect to understand
* Encode to enforce truth
* Hash only canonical bytes

Nothing is hidden. Nothing is silently corrected. Reality comes first.
