# Emagjby Strata Northstar v1

⚠️ **Northstar v1 is frozen.** Any breaking change becomes **Northstar v2**.

Strata is a deterministic data language with two layers:

- **Strata Text (`.st`)**: human-friendly authoring format
- **Strata Core (`.scb`)**: canonical binary encoding (source of truth for hashing, storage, transport)

**Guarantee:** Same data -> same canonical bytes -> same hash, across Rust and JavaScript.

**Non-goals**

- Not a general-purpose programming language
- Not schema-first (schemas may exist later as a layer, not in core)
- Not magical (no implicit typing tricks)

---

## 1. Data Model

Strata represents values as:

```
Value =
  Null
| Bool
| Int
| String
| Bytes
| List<Value>
| Map<String, Value>
```

### Type constraints

- **Int** is signed 64-bit integer in range `[-2^63, 2^63-1]`.
- **String** is UTF-8 text.
- **Bytes** is raw byte array (hashes, blobs, packed payloads).
- **Map keys** are **Strings only**.

---

## 2. Strata Text (`.st`)

Text is sugar that compiles into the Value model.

### 2.1 Lexical rules

- Whitespace is irrelevant except inside strings.
- Comments:
  - `//` line comment
  - `#` line comment
- Identifiers: `[A-Za-z_][A-Za-z0-9_]*` (ASCII only, v1)
- Strings: double-quoted with escapes:
  - `\"` `\\` `\n` `\r` `\t`
  - `\uXXXX` (exactly 4 hex digits)

### 2.2 Literals

- `null`
- `true` / `false`
- Integers: `-?[0-9]+` (no floats in v1)
- Strings: `"hello"`
- Bytes literal (v1): `0x` followed by even hex digits

### 2.3 Containers

- List: `[value, value, ...]` (trailing comma allowed)
- Map (object): `{ key: value, ... }` (trailing comma allowed)
- Map shorthand: `name { ... }` is sugar for `{ name: { ... } }`

### 2.4 Example

```st
user {
  id: 42
  name: "Gencho"
  active: true
  skills: ["rust", "svelte", "systems"]
  avatar_hash: 0x9f86d081884c7d659a2feaa0c55ad015
}
```

---

## 3. Canonicalization Rules

### 3.1 Strings

- Strings are stored as UTF-8 bytes exactly.
- No Unicode normalization is applied in v1.

### 3.2 Maps

- Entries are sorted by UTF-8 byte order.
- Duplicate keys are invalid.

### 3.3 Integers

- Integers are canonical as signed i64.

---

## 4. Strata Core Binary (`.scb`)

### 4.1 Byte order

- Fixed-width integers are little-endian.
- Varints use ULEB128 and SLEB128.

### 4.2 Type tags

- `0x00` Null
- `0x01` False
- `0x02` True
- `0x10` Int
- `0x20` String
- `0x21` Bytes
- `0x30` List
- `0x40` Map

---

## 5. Hashing

- BLAKE3-256 over canonical Value bytes.

---

## 6. Cross-language rules

- JS must use BigInt for Int.
- Rust uses i64.

---

## 7. CLI

- `strata compile`
- `strata decode`
- `strata hash`
- `strata fmt`
