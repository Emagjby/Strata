# Wire formats

Strata defines a **single canonical wire format**: **Strata Core Binary**.

Everything else is either a transport concern or a convenience layer.

If bytes are identical, the meaning is identical.\
If bytes differ, the meaning differs.



***

### Strata Core Binary (SCB)

Strata Core Binary (`.scb`) is the authoritative wire format.

Properties:

* Fully deterministic
* Self-delimiting
* Unambiguous
* Language-neutral
* Hash-stable

SCB is what gets hashed, stored, transmitted, and verified.



***

### What SCB represents

SCB is a serialized form of the Strata value model.

Every encoded stream represents exactly one value.

There is:

* No schema
* No version field inside the value
* No implicit framing
* No implicit defaults

Decoding consumes exactly one value and must end at EOF.



***

### Tag-based encoding

Each value begins with a **single-byte tag**.

Tags define the type of the value and how the payload is interpreted.

Examples:

* `0x00` → null
* `0x01` → false
* `0x02` → true
* `0x10` → int (SLEB128 payload)
* `0x20` → string (ULEB128 length + UTF-8 bytes)
* `0x21` → bytes (ULEB128 length + raw bytes)
* `0x30` → list (ULEB128 count + values)
* `0x40` → map (ULEB128 count + key/value pairs)

The tag is part of the canonical data and is included in hashing.



***

### Length-prefixed structure

Variable-sized values use explicit lengths.

* Strings are length-prefixed
* Bytes are length-prefixed
* Lists are count-prefixed
* Maps are count-prefixed

There are no terminators and no sentinel values.

This makes the format safe for streaming and slicing.



***

### Canonical ordering on the wire

Maps are encoded in canonical order.

* Keys are strings only
* Keys are sorted by UTF-8 byte order
* Sorting happens before encoding

This ensures that semantically identical maps produce identical bytes.



***

### No implicit framing

SCB itself has **no framing**.

It represents exactly one value.

If you need to:

* Send multiple values
* Embed values in a stream
* Attach metadata
* Version a transport

You must add framing outside of SCB.



***

### Framed wire format

Strata defines an optional **framed format** for transport.

The framed format is:

* Magic bytes: `STRATA1`
* Version byte
* Canonical SCB payload

```
STRATA1 <canonical_scb_bytes>
```

Framing is not hashed and not part of the value.



***

### When to use framing

Use framing when:

* Streaming values over a socket
* Sending values over HTTP
* Embedding SCB in files or logs
* Distinguishing Strata data from other binary formats

Do not use framing when:

* Hashing
* Comparing values
* Storing canonical content-addressed data



***

### Transport neutrality

SCB can be transported over:

* Files
* TCP
* UDP
* HTTP
* Message queues
* Shared memory

Strata does not define or constrain transport behavior.

Only the bytes matter.



***

### Text is not a wire format

Strata Text (`.st`) is **not** a wire format.

It is:

* Human-authored
* Whitespace-sensitive
* Parser-dependent
* Not hash-stable

Text must always be parsed and encoded before transmission.



***

### Error visibility on the wire

Wire-level errors are explicit.

Decoders must detect:

* Truncated payloads
* Invalid tags
* Invalid varints
* Invalid UTF-8
* Trailing bytes

Silent recovery is forbidden.

Code

***

### Summary

Strata has exactly one canonical wire _&#x66;_&#x6F;rmat: **SCB**.

* SCB is what is hashed
* SCB is what is compared
* SCB is what guarantees determinism

Everything else is transport, tooling, or ergonomics layered on top.
