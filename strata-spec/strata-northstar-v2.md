# Emagjby Strata Northstar v2

⚠️ **Northstar v2 introduces new capabilities but does not alter v1 canonical truth.**  
Any change affecting canonical encoding, hashing, or meaning requires **Northstar v3**.

Strata v2 expands Strata from a write-only canonical format into a fully inspectable, round-trippable system, while preserving v1 guarantees.

---

## Northstar v2 Goal

**Anything encoded by Strata can be decoded, inspected, and reasoned about without ambiguity.**

v2 is about visibility, tooling, and safety, not new semantics.

---

## Non-Goals (v2)

- No changes to canonical bytes
- No new value types
- No schema system
- No compression
- No backward-incompatible CLI changes

---

## 1. Decode Capability (Core)

### 1.1 Strata Core Decode (.scb → Value)

A decoder MUST exist that reconstructs a Value from .scb bytes.

Rules:

- Reads type tags exactly as defined in v1
- Uses ULEB128 / SLEB128 decoding
- Supports all v1 Value types
- Does not enforce canonical ordering
- Does not normalize data

**Exit condition:** Any valid .scb decodes into a Value without panic.

---

### 1.2 Decode Safety Rules

- Invalid tag → error
- Truncated payload → error
- Varint overflow → error
- Duplicate keys allowed in decode output (debug-only)

**Exit condition:** Decoder fails fast and safely on malformed input.

---

## 2. CLI Decode Command

### 2.1 `strata decode`

```
strata decode <input.scb>
```

- Produces human-readable output
- No hashing, no canonicalization

**Exit condition:** Any compiled .scb can be inspected safely.

---

## 3. Encode–Decode Round-Trip Guarantees

### 3.1 Structural Round-Trip

```
.st → parse → Value → encode → .scb → decode → Value'
```

Guarantees:

- Structural equivalence
- Ordering differences allowed
- No semantic loss

---

### 3.2 Canonical Re-Encode Stability

```
.scb → decode → Value → encode → .scb'
```

Guarantee:

- .scb' == .scb
- Hash stability preserved

---

## 4. Formatter

### 4.1 `strata fmt`

- Pretty-prints Strata Text
- Cosmetic only
- No semantic guarantees

---

## 5. Canonical Validation

- Duplicate keys rejected
- Invalid integers rejected
- UTF-8 enforced

---

## 6. Golden Vector Expansion

Vectors include:

- .st
- .scb.hex
- .hash.hex
- Optional decoded structure

---

## 7. Cross-Language Readiness

Rust decoder is reference implementation.

JS decoder must:

- Decode identical bytes
- Use BigInt
- Match behavior exactly

---

## Northstar v2 Exit Criteria

v2 ships only if:

- v1 unchanged
- Decoder implemented
- CLI decode works
- Round-trip proven
- CI passes clean

---

Northstar v3 will introduce JS parity and shared vector enforcement.
