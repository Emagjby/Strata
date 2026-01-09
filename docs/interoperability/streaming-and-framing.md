# Streaming & Framing

Streaming and framing are **outside** the Strata core.

Strata defines values. Streaming defines how multiple values move over time. Framing defines how boundaries between values are identified.

Strata deliberately separates these concerns.



***

### Core principle

**Strata Core Binary (SCB) is self-contained, but not self-delimiting.**

* A single SCB payload represents exactly one value
* SCB does not include length prefixes
* SCB does not include terminators
* SCB does not define stream boundaries

Anything beyond one value requires framing.



***

### Why Strata avoids built-in framing

Embedding framing into the core would:

* Bind Strata to a specific transport model
* Complicate canonical encoding
* Introduce ambiguity around boundaries
* Create hidden state across values

Instead, Strata enforces a clean rule:

> Framing exists outside the canonical value.



***

### What framing is allowed to do

A framing layer MAY:

* Prefix payloads with a length
* Chunk payloads across packets
* Multiplex multiple values on a stream
* Define message boundaries
* Handle backpressure or buffering

A framing layer MUST NOT:

* Inspect Strata values
* Normalize payloads
* Re-encode data
* Modify bytes
* Depend on Strata semantics

Framing handles bytes, not meaning.



***

### Northstar T3: framed determinism

Northstar T3 exists to prove one thing:

**Framing must be transparent.**

In T3:

* Values are encoded to SCB
* SCB is split into frames
* Frames are transmitted over a stream
* Frames are reassembled
* The reconstructed payload is decoded
* Re-encoded bytes must hash identically

If framing alters even one byte, T3 fails.



***

### Length-prefixed framing

The most common framing strategy:

* Prefix each payload with its byte length
* Length is outside SCB
* Length is not hashed
* Length is not interpreted by Strata

Example conceptually:

Code: \[length]\[scb-bytes]\[length]\[scb-bytes]...

Only the SCB bytes are decoded and hashed.



***

### Framing formats

Valid framing examples:

* Fixed-size length prefix (u32, u64)
* Varint-prefixed length
* Record-based streams
* HTTP chunking
* WebSocket binary frames
* TCP stream with explicit boundaries

Invalid framing examples:

* JSON-wrapped payloads
* Base64 inside streams without explicit agreement
* Line-delimited SCB
* Text-based delimiters

Text framing risks corruption.



***

### Streaming multiple values

Streaming allows multiple Strata values to be sent sequentially.

Rules:

* Each value must be independently framed
* Each frame must decode to exactly one value
* No shared state between values
* Failure of one value does not poison the stream

This enables:

* Event streams
* Logs
* State replication
* Incremental updates



***

### Partial delivery

Partial frames are allowed at the transport level.

However:

* A Strata value must be fully reconstructed before decoding
* Decoding partial SCB is invalid
* Truncated payloads must fail explicitly

Streaming does not imply partial decoding.



***

### Backpressure and flow control

Strata is agnostic to:

* Backpressure
* Windowing
* Rate limiting

These belong to the transport or framing layer.

Strata only sees completed byte sequences.



***

### Error handling in streams

Framing layers may:

* Drop invalid frames
* Close streams on errors
* Retry transmission
* Signal frame-level failures

Strata errors remain local to the payload.

No global stream state is implied.



***

### What is guaranteed

With correct framing:

* Canonical bytes survive transport
* Hashes remain stable
* Cross-language decoding succeeds
* Determinism holds per value



***

### What is not guaranteed

Strata does NOT guarantee:

* Ordering across frames
* Delivery guarantees
* Reliability
* Retry semantics
* Idempotency

Those belong elsewhere.



***

### Summary

Strata values are atomic. Streams are sequences of atoms. Framing draws the boundaries.

As long as framing is invisible to the bytes, determinism remains intact.

Break the boundary rules, and Northstar T3 will catch you.
