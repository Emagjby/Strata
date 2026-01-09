# Northstar T3 - Framed streaming determinism

**Northstar T3** guarantees that Strata Core Binary remains canonical, deterministic, and hash-stable when transmitted as **multiple framed messages over a streaming transport**.

Framing may define message boundaries.\
Framing must never interpret, normalize, or mutate Strata payload bytes.

If this guarantee fails, Strata is not stream-safe.



***

### What Northstar T3 guarantees

Northstar T3 asserts the following invariant:

> Canonical Strata Core Binary payloads, when split into framed messages and transmitted over a stream, must decode, re-encode, and re-hash to identical results.

Each frame contains:

* A length prefix
* Exactly one canonical Strata payload

The payload bytes themselves are never altered.



***

### Why framing matters

Many real systems do not send single payloads.

They stream:

* Multiple messages over one connection
* Chunked responses
* WebSocket frames
* TCP streams
* Async message queues

Framing is unavoidable in these environments.

Northstar T3 ensures that **adding framing does not change meaning**.



***

### What framing is allowed to do

Framing is allowed to:

* Define message boundaries
* Prefix payloads with length metadata
* Segment streams safely
* Enable multiplexing or batching

Framing is not allowed to:

* Inspect Strata contents
* Modify payload bytes
* Reorder payloads internally
* Normalize or validate data
* Add semantic meaning

Framing is mechanical, not logical.



***

### What Northstar T3 explicitly forbids

Northstar T3 forbids:

* Frame-level normalization
* Implicit buffering assumptions
* Zero-length frames
* Partial payload acceptance
* Silent truncation
* Hashing framed data instead of payload data

Only the payload bytes are canonical.



***

### Streaming failure modes T3 prevents

Without T3, systems often fail due to:

* Frame boundary misalignment
* Partial reads being accepted
* Accidental concatenation of payloads
* Hashing framed data instead of content
* Transport chunking affecting semantics

Northstar T3 exists to block these failures permanently.



***

### Failure meaning

If Northstar T3 fails, at least one of the following is true:

* Payload boundaries are ambiguous
* Framing leaks into canonical encoding
* Streaming alters byte-level meaning
* Hash stability depends on transport layout
* Decoding tolerates malformed streams

Any of these breaks Strata’s determinism guarantees.



***

### Relationship to other Northstars

Northstar T3 builds on earlier guarantees:

* T1 ensures cross-language determinism
* T2 ensures raw-wire safety
* T3 ensures stream safety with framing

All three must hold simultaneously.



***

### Stability guarantee

Once Northstar T3 is finalized:

* Its framing rules are frozen
* All future implementations must pass it
* Any new transport layer must preserve it

If a change breaks T3, it requires a new major version and an explicit opt-in.

Northstar T3 defines Strata’s ceiling for safe streaming transport.
