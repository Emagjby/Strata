# Emagjby Strata Northstar v4

DX Macros, Factory Ergonomics, and Integration Reference (Strata v0.4.0)

⚠️ **Northstar v4 improves developer experience and documentation without changing canonical truth.**  
Any change to canonical bytes, hashing, error meaning, or value semantics requires **Northstar v5**.

---

## Northstar v4 Goal

**Make constructing Strata Values fast, obvious, and hard to misuse in both Rust and JavaScript** while preserving all existing behavior.

v4 is an ergonomics release:

- Reduce boilerplate when building Values programmatically.
- Preserve the strict, explicit Value model (no implicit coercions).
- Improve official documentation so humans and AI models can integrate correctly.

---

## Non-Goals (v4)

- No new value types
- No new `.st` syntax
- No canonical `.scb` changes
- No hashing changes
- No schema layer
- No compression
- No backward-incompatible API changes

---

## 1. Rust DX: Macro for Every Value Kind

### Rules

- Macros MUST be **purely additive**. Existing APIs remain valid.
- Macros MUST construct the same `Value` structures a user could build manually.
- Map keys in macros MUST be **string literals** for clarity.
- Duplicate key behavior MUST match Strata map semantics: **last-write-wins**.
- Macros MUST not change canonical encoding or hashing results.

### Tasks

Provide a minimal, consistent macro surface that maps 1:1 to the value model:

- `null!()` -> `Value::Null`
- `bool!(true | false)` -> `Value::Bool(..)`
- `int!(...)` -> `Value::Int(..)`
- `string!(...)` -> `Value::String(..)`
- `bytes!(...)` -> `Value::Bytes(..)`
- `list![ ... ]` -> `Value::List(..)`
- `map!{ "k" => v, ... }` -> `Value::Map(..)`

Recommended macro design constraints:

- `map!{ ... }` MUST accept nested values (including other macros).
- `list![ ... ]` MUST accept zero or more items.
- `bytes!(...)` MUST support a byte-oriented ergonomic input (implementation choice), without altering canonical bytes.

Optional DX improvements (allowed if they do not change semantics):

- Add `From` conversions to reduce `.into()` noise while keeping types explicit.

**Exit condition:** Rust users can build nested values in a few lines without losing clarity or correctness.

---

## 2. Rust Enforcement: Unit Tests for Each Macro

### Rules

- **Every new macro MUST ship with unit tests.**
- Tests MUST cover:
  - correct construction
  - nesting
  - duplicate key semantics
- Existing tests MUST NOT regress.

### Tasks

Add unit tests that assert at least:

- Each macro constructs the expected `Value` variant.
- Nested construction works (`map!` containing `list!`, etc.).
- Duplicate keys in `map!` are last-write-wins.
- Byte-for-byte stability: encoding and hashing of macro-built values match manually-built equivalents.

**Exit condition:** Macro behavior is enforced by tests and cannot silently regress.

---

## 3. JavaScript DX: Value Factory Upgrades (Additive Only)

### Rules

- The factory MUST be exported under a clear, descriptive primary name.
  - Preferred export name: `Value`.
  - `V` may remain as a supported alias, but it MUST NOT be the primary documented name.
- Existing factory calls MUST remain supported (no breaks):
  - `V.null()`, `V.bool(x)`, `V.int(1n)`, `V.string(s)`, `V.bytes(u8)`, `V.list(array)`, `V.map(entries)`
- Improvements MUST be additive and MUST NOT change encoding/hashing outcomes.
- Integers MUST remain BigInt-only (JS `number` is rejected).
- Duplicate key behavior MUST be consistent with current Strata semantics:
  - Where maps are constructed from repeated keys, the observable map MUST reflect **last-write-wins**.

### Tasks

Upgrade the JS value factory to improve DX without breaking existing code:

- Add ergonomics helpers for common construction patterns (implementation choice):
  - object-style map construction
  - variadic list construction
  - bytes helper(s) for common developer inputs
- Keep the factory as the recommended construction path.

**Exit condition:** JS users can build Values ergonomically, with strict type safety preserved.

---

## 4. JavaScript Enforcement: Unit Tests for Each Factory Upgrade

### Rules

- **Every factory improvement MUST ship with unit tests.**
- Existing vector enforcement tests MUST continue to pass unchanged.
- Old factory usage MUST have regression coverage.

### Tasks

- Add tests proving the old API still works.
- Add tests for each new helper.
- Add tests ensuring invalid uses still fail (e.g. `V.int(42)` throws).

**Exit condition:** JS DX improvements are enforced and cannot regress.

---

## 5. Documentation Updates (Official Docs in `/docs`)

### Rules

- Docs MUST reflect implementation exactly.
- Docs MUST not claim canonical, hashing, or semantic changes.
- Docs MUST highlight the strictness model and common footguns.

### Tasks

- Update Rust docs to teach the new macro surface.
- Update JS docs to teach the upgraded value factory.
- Explain duplicate key behavior clearly (last-write-wins) and where it applies.

**Exit condition:** Docs contain copy-pasteable, correct construction examples for Rust and JS.

---

## 6. New Docs Page: API Reference / Integration Reference (AI-Friendly)

### Rules

- Provide a single, canonical integration page.
- Must be structured for fast scanning and embedding into AI context.
- Must clearly separate:
  - the Value model
  - encoding vs decoding boundaries
  - hashing contract
  - what is canonical vs representational

### Tasks

Create a new docs page (name flexible, recommended: "API Reference" or "Integration Reference") that includes:

- A conceptual pipeline diagram (text-based) of:
  - Value -> encode -> `.scb` -> hash
  - `.scb` -> decode -> Value
  - `.st` -> parse -> Value -> encode -> `.scb`
- Rust integration surface:
  - `Value` and construction macros
  - public encode/decode/hash entry points
- JS integration surface:
  - `Value` union
  - `ValueFactory`/`V`
  - public encode/decode/hash/parse exports
- Minimal copy-paste examples in both languages.

**Exit condition:** A new doc page exists, linked in the table of contents, that enables correct integration without guesswork.

---

## 7. Phased Delivery & Test Gating (v4 Law)

### Rules

- Work MUST be delivered in phases.
- **Each phase MUST add unit tests in the same change as the implementation.**
- A phase is not complete unless:
  - its unit tests exist, and
  - all pre-existing tests still pass (no regressions).
- No phase may rely on "we will test later".

### Tasks

#### Phase 1 — Rust: `map!`

- Implement `map!{ "k" => v, ... }`.
- Add unit tests, including last-write-wins duplicate key behavior.

**Exit condition:** New tests pass and all existing Rust tests pass.

#### Phase 2 — Rust: macros for all other Value kinds

- Implement remaining macros (`null!`, `bool!`, `int!`, `string!`, `bytes!`, `list!`).
- Add unit tests for each macro and nested combinations.

**Exit condition:** New tests pass and all existing Rust tests pass.

#### Phase 3 — JavaScript: factory upgrades

- Add additive helpers.
- Add unit tests for new helpers and regression tests for old usage.

**Exit condition:** New tests pass and all existing JS tests (including vectors) pass.

#### Phase 4 — Docs: construction + API Reference

- Update Rust and JS docs.
- Add the new API Reference / Integration Reference page and link it.

**Exit condition:** Docs match the implementation, and all code tests remain green.

---

## Northstar v4 Exit Criteria

v4 ships only if:

- Rust has a macro surface covering every Value kind
- Rust macros match Strata semantics (including last-write-wins for duplicate keys)
- JS factory improvements are additive and preserve strictness
- Unit tests exist for every new macro and every new JS factory helper
- All existing tests remain green (no regressions)
- Docs are updated and a new API Reference / Integration Reference page is published and linked

---

## Examples (Non-Normative)

These examples illustrate the intended ergonomics only. They are not new semantics.

### Rust (macros)

```rust
use strata_rs::value::Value;

let v: Value = map!{
    "null" => null!(),
    "bool" => bool!(true),
    "int" => int!(42),
    "string" => string!("strata"),
    "bytes" => bytes!([0xde, 0xad, 0xbe, 0xef]),
    "list" => list![int!(1), int!(2), int!(3)],
    "nested" => map!{
        "a" => int!(1),
        // duplicate keys are last-write-wins
        "a" => int!(2),
    },
};
```

### JavaScript (value factory)

```ts
import { Value, encodeValue, hashValue, decodeValue } from "@emagjby/strata-js";

const v = Value.map([
  ["null", Value.null()],
  ["bool", Value.bool(true)],
  ["int", Value.int(42n)],
  ["string", Value.string("strata")],
  ["bytes", Value.bytes(new Uint8Array([0xde, 0xad, 0xbe, 0xef]))],
  ["list", Value.list([Value.int(1n), Value.int(2n), Value.int(3n)])],
]);

const bytes = encodeValue(v);
const hash = hashValue(bytes);
const roundtrip = decodeValue(bytes);
```

### Canonical pipeline (shared mental model)

```text
.st  -> parse  -> Value -> encode -> .scb -> hash
.scb -> decode -> Value
```
