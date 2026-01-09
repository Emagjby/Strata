# BigInt & Integer rules

Strata uses **JavaScript `bigint`** to represent integers. This is not an optimization. It is a correctness requirement.

JavaScript `number` is explicitly rejected.



***

### Why BigInt is mandatory

JavaScript `number` is IEEE-754 double precision.

This means:

* Integers above 2^53 − 1 lose precision
* Equality becomes unreliable
* Canonical hashing becomes impossible

Strata operates in a domain where:

* Exact integers matter
* Hashes must be stable
* Equality must be absolute

Therefore: **All integers in Strata JS are `bigint`.**

No exceptions.



***

### Integer domain

Strata integers are defined as:

* Signed 64-bit
* Two’s complement semantics
* Range:
  * Minimum: −2^63
  * Maximum: 2^63 − 1

In JavaScript terms:

```
I64_MIN = -(1n << 63n) I64_MAX = (1n << 63n) - 1n
```

Any value outside this range is rejected.



***

### Construction rules

Integers are constructed via the value factory.

```
V.int(42n)
```

Rules:

* Argument MUST be a `bigint`
* Passing a `number` throws immediately
* Range is validated at runtime

This fails fast by design.



***

### Parsing behavior

When parsing Strata Text:

* Integer literals are parsed into `bigint`
* Decimal representation only
* Leading `-` is allowed
* No suffixes
* No scientific notation

```
42 -7 9007199254740993
```

Rejected: 42.0 1e6 0xFF



***

### Encoding semantics

During encoding:

* `bigint` is encoded as signed SLEB128
* Encoding is minimal and canonical
* No padding bytes
* No alternative encodings

```
Int(1) → 0x10 0x01
```

If two `bigint` values are equal, their encodings are identical.



***

### Decoding semantics

During decoding:

* SLEB128 is decoded into `bigint`
* Overflow beyond 64 bits is rejected
* Invalid varints raise explicit errors
* No silent truncation

Decoded integers are always exact.



***

### Arithmetic is external

Strata does **not** define arithmetic semantics.

* No addition
* No multiplication
* No coercions

Strata stores integers. It does not compute with them.

Any arithmetic happens outside the format.



***

### Comparison semantics

Equality is structural:

* Two `bigint` values are equal if their numeric value is equal
* Encoding equality implies value equality
* Hash equality implies value equality

There is no fuzzy comparison. There is no tolerance.



***

### Interop with JavaScript code

When integrating with JS code:

* Convert `bigint` explicitly if needed
* Do not downcast silently
* Be aware JSON cannot represent `bigint`

Strata values are not JSON. Do not treat them as such.



***

### Why not BigInt everywhere?

Only integers require BigInt.

Other types:

* Strings are UTF-8 text
* Bytes are Uint8Array
* Lists and maps are structural

BigInt is used surgically, not globally.



***

### Future evolution

If future Strata versions introduce new numeric types:

* They will not change BigInt semantics
* They will not redefine Int
* They will exist as new value kinds

Canonical integers are frozen.



***

### Summary

BigInt in Strata JS is:

* Mandatory
* Bounded
* Canonical
* Non-negotiable

If a value cannot be represented exactly as a 64-bit integer, it does not belong in Strata.

Precision beats convenience. Every time.
