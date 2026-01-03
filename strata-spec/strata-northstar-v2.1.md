# Emagjby Strata Northstar v2.1

⚠️ **Northstar v2.1 introduces explicit error semantics and a refined CLI interface.**  
It does **not** modify canonical encoding, hashing, or v1/v2 guarantees.

Strata v2.1 formalizes how failures are represented, surfaced, and reasoned about, making Strata production-safe.

---

## Northstar v2.1 Goal

**Failures must be explicit, structured, and inspectable.**

- No panics on user input
- Deterministic, explainable errors
- CLI behaves like a real system tool

---

## Non-Goals (v2.1)

- No new value types
- No canonical byte changes
- No hashing changes
- No JS implementation yet
- No schemas

---

## 1. Error Model (Core)

### 1.1 Unified Error Type

All fallible operations MUST return structured errors.

```rust
pub enum StrataError {
    Parse(ParseError),
    Encode(EncodeError),
    Decode(DecodeError),
    Io(std::io::Error),
}
```

Rules:

- No `panic!` on input
- Errors are values, not strings
- No silent recovery

**Exit condition:** Public APIs are panic-free.

---

### 1.2 Decode Errors

Decoder MUST surface precise failures:

- InvalidTag(u8)
- UnexpectedEOF
- InvalidVarint
- InvalidUtf8
- TrailingBytes

Errors must identify malformed vs truncated data.

---

### 1.3 Parse Errors

Parser MUST:

- report failure location
- distinguish syntax vs semantic errors

Examples:

- unexpected token
- integer out of range
- malformed bytes literal

---

## 2. CLI Error Semantics

### 2.1 Exit Codes

| Scenario       | Exit Code |
| -------------- | --------- |
| Success        | 0         |
| Invalid input  | 1         |
| I/O failure    | 2         |
| Internal error | 100       |

CLI MUST never panic.

---

### 2.2 Error Output

Errors go to stderr:

```
error: decode failed
reason: invalid tag 0x99
offset: 12
```

No stack traces. Human-readable.

---

## 3. CLI Interface Refinement

Commands:

- compile
- decode
- hash
- fmt

Uniform syntax:

```
strata <command> <input> [options]
```

---

## 4. Validation Behavior

Encoding rejects:

- duplicate keys
- invalid integers
- invalid UTF-8

Decoding allows:

- duplicate keys
- non-canonical ordering

> Encoding enforces truth. Decoding reveals reality.

---

## 5. Golden Vector Error Coverage

Negative vectors MUST include:

- invalid tag
- truncated payload
- varint overflow
- invalid UTF-8

Each vector defines:

- input bytes
- expected error
- failure point

---

## 6. Documentation Guarantees

Docs MUST explain:

- what fails
- why it fails
- how to detect it

---

## Northstar v2.1 Exit Criteria

- v1 & v2 unchanged
- Structured errors everywhere
- CLI stable and panic-free
- Negative vectors enforced
- CI green
