# Value model (JS)

The JavaScript value model defines the **in-memory representation** of Strata data in JS.

It mirrors the Strata Core Value Model exactly, with one intentional deviation: **JavaScript uses `bigint` for integers** to preserve correctness.

This model is immutable, explicit, and hostile to implicit coercions.



***

### Core principle

A Strata value in JavaScript must satisfy:

* Lossless representation of canonical data
* Deterministic round-tripping
* No reliance on JS `number`
* Structural parity with Rust `Value`

If a value cannot be represented safely, it is rejected.



***

### Value union

The full value union is:

```
Value = | Null | Bool | Int | String | Bytes | List | Map
```

Each variant is a **tagged object** with a fixed shape.



***

### Null

```
{ kind: "null" }
```

Represents the absence of a value.

Rules:

* No payload
* Always encoded as canonical null
* No alternative spellings or aliases



***

### Bool

```
{ kind: "bool", value: boolean }
```

Represents boolean truth values.

Rules:

* Only `true` or `false`
* Encoded as distinct tags
* No numeric coercion allowed



***

### Int (BigInt)

```
{ kind: "int", value: bigint }
```

Represents a signed 64-bit integer.

Rules:

* Value MUST be a `bigint`
* JS `number` is rejected at construction time
* Range enforced to i64 bounds
* Encoded using canonical SLEB128

This is non-negotiable.

JavaScript numbers are unsafe. Strata refuses them.



***

### String

```
{ kind: "string", value: string }
```

Represents UTF-8 text.

Rules:

* Internally stored as JS `string`
* Encoded as UTF-8 bytes
* Invalid UTF-8 is rejected during decoding
* No normalization or rewriting

What you write is what you hash.



***

### Bytes

```
{ kind: "bytes", value: Uint8Array }
```

Represents raw binary data.

Rules:

* Must be a `Uint8Array`
* Length encoded explicitly
* Bytes preserved verbatim
* No encoding assumptions

Bytes are bytes. Not strings. Not base64. Not hex.



***

### List

```
{ kind: "list", value: readonly Value[] }
```

Represents an ordered sequence of values.

Rules:

* Order is significant
* Length encoded explicitly
* Nested values allowed
* Empty lists are valid

Lists are structural. Reordering changes hashes.



***

### Map

```
{ kind: "map", value: ReadonlyMap<string, Value> }
```

Represents a key-value mapping.

Rules:

* Keys MUST be strings
* Values are arbitrary Strata values
* In-memory order is irrelevant
* Encoding enforces canonical UTF-8 key ordering

Maps are unordered semantically, ordered canonically.



***

### Construction via Value Factory

All values are constructed using the `V` factory.

```
 V.int(42n) 
 V.string("strata") 
 V.bytes(new Uint8Array([0xde, 0xad])) 
 V.list([...]) 
 V.map([...])
```

The factory enforces:

* Correct types
* Runtime validation
* Early failure

Direct object construction is discouraged.



***

### Immutability model

Values are treated as immutable by convention.

* No setters
* No mutation during encode/decode
* New values allocated on transformation

Mutation breaks determinism. Do not do it.



***

### Parity with Rust

JS Value Model ↔ Rust Value Model:

* Null ↔ Null
* Bool ↔ Bool
* Int(bigint) ↔ Int(i64)
* String ↔ String
* Bytes ↔ Bytes
* List ↔ List
* Map ↔ Map (BTreeMap / sorted)

Every variant has a one-to-one correspondence.



***

### What is intentionally excluded

The JS Value Model does not include:

* Floats
* Dates
* Objects
* Undefined
* Symbols
* NaN / Infinity

If it is not in the model, it does not exist.



***

### Summary

The JavaScript Value Model is:

* Strict
* Minimal
* Deterministic
* Parity-driven
* BigInt-first

It exists to make JavaScript behave like a systems language.

And for Strata, that is the only acceptable behavior.
