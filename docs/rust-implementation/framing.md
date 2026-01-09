# Framing

Framing in Strata is **explicit, optional, and non-semantic**.

Framing exists to move bytes safely. It does not change meaning. It does not participate in canonical encoding. It does not affect hashing.

Canonical truth always lives **inside** the payload.



***

### What framing is (and is not)

Framing is a transport convenience layer.

It exists to:

* Define message boundaries
* Enable streaming transports
* Support multiplexed or chunked delivery

Framing does **not**:

* Interpret data
* Normalize payloads
* Modify bytes
* Affect hashes
* Participate in canonical rules

If framing changes bytes, it is broken.



***

### Core principle

**Framing must be lossless.**

Removing framing must yield the exact original `.scb` bytes.

If this is not true, framing has violated Strata guarantees.



***

### Framing vs canonical encoding

Canonical encoding answers:

* What bytes represent this value?

Framing answers:

* Where does one payload start and end?

They are intentionally orthogonal.

Canonical encoding is frozen. Framing is replaceable.



***

### Reference framing format (Rust)

The Rust implementation provides a **reference framing format** for testing and tooling.

It is not mandatory. It is not special. It is just a concrete example.



***

### Framed encoding layout

The reference framing layout is:

```
STRATA_MAGIC (7 bytes) = "STRATA1" VERSION (1 byte) PAYLOAD (canonical .scb bytes)
```

The payload is identical to unframed `.scb`.



***

### Framing encoder

Framed encoding is implemented as a thin wrapper.

```
pub fn encode_framed(value: &Value) -> Vec
```

Internally:

* Canonical `.scb` bytes are produced first
* Framing bytes are prepended
* No transformation occurs

Encoding failures inside framing are canonical failures, not framing failures.



***

### Why framing is separate

Bundling framing into canonical encoding would:

* Couple transport with meaning
* Make hashing ambiguous
* Break content addressing
* Prevent raw-wire usage

Strata explicitly rejects this.

Canonical bytes must be usable:

* In files
* Over HTTP
* In streams
* In memory
* Inside other protocols

Without modification.



***

### Streaming and chunking

Framing is essential for streaming transports.

Example cases:

* TCP streams
* HTTP chunked transfer
* WebSockets
* Message queues

In these cases, framing defines boundaries. Payload bytes remain untouched.



***

### Northstar enforcement

Framing behavior is enforced by **Northstar T3**.

T3 guarantees:

* Payloads can be split arbitrarily
* Frames can arrive fragmented
* Boundaries do not affect hashes
* Re-encoded payloads remain identical

If framed transport breaks determinism, the implementation fails T3.



***

### Multiple framing layers

Multiple framing layers are allowed.

For example:

* Length-prefix framing
* Magic-header framing
* Application-level envelopes

Rules:

* Each layer must be removable
* Inner canonical bytes must remain identical
* Hashing must operate only on canonical bytes

If layers interfere, they are invalid.



***

### When to use framing

Use framing when:

* Transport is streaming
* Message boundaries are required
* Payloads are multiplexed
* Backpressure matters

Do not use framing when:

* Writing canonical `.scb` to disk
* Hashing content
* Comparing values
* Performing verification



***

### Framing and hashing

Hashing always operates on canonical payload bytes.

Never hash:

* Framing headers
* Length prefixes
* Transport metadata

Hashing framed bytes is a protocol bug.



***

### Cross-language expectations

Other language implementations may:

* Use different framing formats
* Use no framing at all

This is allowed.

What is not allowed:

* Framing that changes payload bytes
* Framing that alters decoding behavior
* Framing that affects hash output



***

### Summary

Framing in Strata is:

* Optional
* Explicit
* Transport-only
* Canonical-neutral

Canonical bytes are sacred.

Everything else is scaffolding.
