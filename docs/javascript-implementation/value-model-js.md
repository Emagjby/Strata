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

All values are constructed using the `Value` factory.

`V` remains supported as an alias for backwards compatibility.

```
Value.int(42n)
Value.string("strata")
Value.bytes(new Uint8Array([0xde, 0xad]))
Value.list([...])
Value.map([...])
```

The factory enforces:

* Correct types
* Runtime validation
* Early failure

Direct object construction is discouraged.

#### Core factory methods

* `Value.null()`
* `Value.bool(boolean)`
* `Value.int(bigint)` (BigInt only)
* `Value.string(string)`
* `Value.bytes(Uint8Array)`
* `Value.list(Value[])`
* `Value.map(Iterable<[string, Value]>)`

#### Additive helpers

* `Value.listOf(...Value)`
* `Value.mapObj({ [key: string]: Value })`
* `Value.mapOf(...[string, Value])`
* `Value.bytesFrom(Uint8Array | ArrayBuffer | number[] | Iterable<number>)`
* `Value.bytesHex(hexString)` (strict hex, even length, no prefix)

#### Examples

**Preferred: `Value` (nested construction)**

```
const v = Value.mapObj({
    user: Value.mapObj({
        id: Value.int(42n),
        name: Value.string("Ada"),
    }),
    tags: Value.listOf(Value.string("dx"), Value.string("v4")),
    data: Value.bytesHex("deadbeef"),
});
```

**`V` is an alias (still supported)**

```
const v = V.int(42n);
```

**Duplicate keys overwrite (last-write-wins)**

```
const m = Value.mapOf(
    ["k", Value.int(1n)],
    ["k", Value.int(2n)],
);

// last-write-wins: "k" is 2n
```

**Bytes helpers (`Uint8Array` at the core)**

```
const a = Value.bytesFrom([0xde, 0xad, 0xbe, 0xef]);
const b = Value.bytesHex("deadbeef");
```

***

### Strictness & Footguns

#### Integers are strict (`bigint` only)

Strata integers are signed 64-bit values.

In JS, that means `Value.int(...)` only accepts `bigint`.

Passing a JS `number` throws. This prevents silent precision loss.

#### Bytes are strict (`Uint8Array` at the core)

`Value.bytes(...)` requires a `Uint8Array`.

Helpers like `bytesFrom` and `bytesHex` exist only to reduce inputs into a canonical `Uint8Array`.

They do not change encoding, hashing, or semantics.

#### Duplicate keys overwrite instead of erroring

JS maps cannot represent duplicate keys as distinct entries.

So `Value.map(...)` / `Value.mapOf(...)` use **last-write-wins** when duplicates occur.

This matches Strata’s parsing behavior for non-canonical inputs.

#### Object-style maps cannot express duplicates

`Value.mapObj(...)` takes a plain object.

JS objects cannot contain duplicate keys, so you cannot express duplicates with `mapObj`.

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
