# Emagjby Strata Northstar v3

JS Parity & Cross-Language Law

⚠️ **Northstar v3 adds a JavaScript implementation and enforcement tooling without changing canonical truth.**  
Any change to canonical bytes, hashing, or semantics requires **Northstar v4**.

---

## Northstar v3 Goal

A Strata payload encoded in Rust must be identical to one encoded in JavaScript, and both must decode each other perfectly.

---

## Non-Goals (v3)

- No new value types
- No new syntax
- No schema layer
- No compression
- No canonical changes

---

## 1. Project Shape & Shared Assets

### Requirements

- Add `strata-js/` package (TypeScript-first)
- Shared vectors remain in `/vectors/`
- JS consumes vectors from repo root, not copied

### Tasks

- `strata-js/package.json` + build/test tooling
- Runtime target: Node 20+
- Add `vectors/README.md` defining the cross-language contract

**Exit condition:** JS project builds and can read `/vectors` files in tests.

---

## 2. JavaScript Value Model

### Rules

JS must represent the exact v1 Value model:

- Null
- Bool
- Int (BigInt)
- String
- Bytes (`Uint8Array`)
- List
- Map (`Map<string, Value>`)

### Tasks

- Define TS types for `Value`
- Enforce BigInt-only integers
- Ensure no precision loss

**Exit condition:** All vector structures can be represented losslessly in JS.

---

## 3. Varints in JS

### 3.1 ULEB128

**Tasks**

- Encode ULEB128 from `bigint`
- Decode with `{ value: bigint, nextOffset }`
- Reject overflow and invalid continuation

**Exit condition:** JS ULEB128 matches Rust output for all test values.

---

### 3.2 SLEB128

**Tasks**

- Encode/decode signed BigInt values
- Correct sign extension
- Reject overflow

**Exit condition:** JS SLEB128 matches Rust exactly for boundary cases.

---

## 4. Core Encoding (JS)

### Rules

- Emit exact v1 type tags
- UTF-8 string bytes
- Canonical map ordering by UTF-8 byte order

### Tasks

- `encodeValue(value: Value): Uint8Array`
- Recursive encoder
- Canonical key sorting by UTF-8 bytes

**Exit condition:** JS encoder produces byte-for-byte identical `.scb` files.

---

## 5. Core Decoding (JS)

### Rules

- Safe decoding with structured errors
- No canonical enforcement
- Duplicate keys allowed (debug-only)

### Tasks

- `decodeValue(bytes): Value`
- `DecodeError` with kind and offset
- UTF-8 validation

**Exit condition:** JS decoder can decode all valid `.scb` files safely.

---

## 6. Hashing (JS)

### Rules

- BLAKE3-256 over canonical bytes
- 32-byte output

### Tasks

- Integrate JS BLAKE3 library
- `hashValue(value)` implementation

**Exit condition:** JS hashes match Rust `.hash.hex` vectors exactly.

---

## 7. Golden Vector Enforcement

Golden vectors are law.

### 7.1 Positive Vectors

**Tasks**

- Load `.scb.hex` and `.hash.hex`
- Encode → hash → compare

**Exit condition:** JS passes all v1 and v2 positive vectors.

---

### 7.2 Negative Vectors

**Tasks**

- Decode malformed inputs
- Assert error kind and offset

**Exit condition:** JS failure behavior matches Rust exactly.

---

## 8. JavaScript Parser (.st)

### Rules

- Grammar identical to Rust
- Reject invalid integers
- Match comments, escapes, and bytes literals

### Tasks

- Implement lexer + parser in TS
- Port semantic tests
- Compile `.st` vectors to canonical `.scb`

**Exit condition:** JS parser produces correct `.scb` and hashes for vectors.

---

## 9. JavaScript CLI

### Tasks

- Implement `compile`, `hash`, `decode`, `fmt`
- Match Rust CLI exit codes and error output

**Exit condition:** JS CLI passes smoke tests and produces identical results.

---

## Northstar v3 Exit Criteria

Northstar v3 ships only if:

- JS encode/decode/hash matches Rust
- All positive vectors pass
- All negative vectors pass
- `.st` parsing works
- Cross-language CI enforces parity

**Final outcome:** A fully working, cross-language Strata implementation with enforced determinism.
