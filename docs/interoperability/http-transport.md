# HTTP transport

HTTP is a **transport**, not a data model.

Strata works over HTTP because Strata does not depend on HTTP semantics at all. The only thing that matters is what bytes arrive and in what order.



***

### Core rule

**HTTP must transport Strata bytes verbatim.**

* No rewriting
* No normalization
* No JSON wrapping
* No implicit encoding
* No compression unless explicitly agreed

If the bytes change, determinism is broken.



***

### Content-Type

When transporting Strata Core Binary over HTTP, the canonical content type is:

```
application/strata
```

This signals:

* The payload is raw Strata Core Binary
* The body represents exactly one Strata value
* The bytes are canonical and hashable

Clients must not assume JSON or text semantics.



***

### Request bodies

HTTP request bodies may contain:

* A single SCB payload
* Raw binary bytes
* No framing unless explicitly documented

Typical usage:

* POST with `application/strata`
* PUT with `application/strata`

The server must treat the body as opaque bytes and pass them directly to the decoder.



***

### Response bodies

HTTP responses may return:

* A single SCB payload
* Raw binary bytes
* The same content type

The response body must be decodable as a complete Strata value.

Partial values are invalid.



***

### No JSON roundtrips

This is forbidden:

* SCB → JSON → SCB
* SCB → AST → JSON → AST → SCB

Any intermediate representation risks:

* Key reordering
* Integer loss
* Precision loss
* Type coercion

If HTTP is used, SCB must stay SCB end to end.



***

### Length handling

HTTP provides its own framing via:

* Content-Length
* Chunked transfer encoding

Strata does not depend on either.

Decoders must operate on the received byte stream after HTTP reassembly.



***

### Chunked transfer encoding

Chunked encoding is allowed.

Rules:

* Chunks must be concatenated in order
* No chunk may mutate content
* Decoder sees a single contiguous byte stream

Chunking is invisible to Strata.



***

### Streaming responses

HTTP streaming is allowed only if:

* Each frame is explicitly framed outside SCB
* Framing is removed before decoding
* Each SCB payload is decoded independently

Example use cases:

* Server-sent streams
* Long-lived HTTP connections
* Event feeds

Strata itself does not define stream boundaries.



***

### Error signaling

HTTP status codes signal transport-level errors only.

Examples:

* 400: malformed request
* 413: payload too large
* 500: server error

Strata decode errors are not HTTP errors by default.

They may be surfaced as:

* HTTP 400 with error metadata
* Separate diagnostic endpoints
* Logged failures

But the wire format itself remains unchanged.



***

### Hash verification over HTTP

A common pattern:

1. Client sends SCB
2. Server decodes and re-encodes
3. Server hashes canonical bytes
4. Server compares expected hash

If hashes differ, the payload is rejected.

This works because HTTP preserves byte order and content.



***

### Security considerations

Strata over HTTP is:

* Not encrypted
* Not authenticated
* Not signed

Security must be provided by:

* TLS
* Auth headers
* Application-level signatures
* External envelopes

Strata does not embed security primitives.



***

### What HTTP must never do

HTTP infrastructure must not:

* Reorder bytes
* Convert encodings
* Interpret payload as text
* Auto-decompress without agreement
* Auto-serialize to JSON

If a proxy does this, the system is invalid.



***

### Summary

HTTP is just a pipe.

Strata over HTTP means:

* application/strata
* Raw bytes
* One value per payload
* No interpretation
* No forgiveness

If the bytes survive, determinism survives.
