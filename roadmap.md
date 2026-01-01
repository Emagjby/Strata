Emagjby Strata — Northstar v1 Implementation Roadmap

⚠️ This roadmap follows Northstar v1 exactly.
Any breaking change requires Northstar v2.

---

4. Strata Core Binary (.scb) — Encoding

Goal

Transform a parsed Value into canonical, byte-identical binary output.

The encoder is the source of truth for hashing, storage, and transport.

---

4.1 Byte Order & Varints

Requirements
• Fixed-width integers: little-endian (not used in v1 core)
• Variable-length integers:
• ULEB128(u64) for lengths and counts
• SLEB128(i64) for Int values
• Encoding must be:
• minimal
• canonical
• deterministic

Tasks
• Implement encode_uleb128(u64, &mut Vec<u8>)
• Implement encode_sleb128(i64, &mut Vec<u8>)
• Add unit tests for:
• small values
• boundary values
• negative integers

---

4.2 Type Tags & Value Encoding

Requirements
Each Value starts with a 1-byte tag, followed by type-specific payload.

Type Tag Payload
Null 0x00 none
False 0x01 none
True 0x02 none
Int 0x10 SLEB128
String 0x20 ULEB128(len) + UTF-8 bytes
Bytes 0x21 ULEB128(len) + raw bytes
List 0x30 ULEB128(count) + values
Map 0x40 ULEB128(count) + entries

Map entries:
• key encoded exactly like String
• entries emitted in canonical order

Tasks
• Implement encode_value(&Value) -> Vec<u8>
• Implement recursive encode_into
• Ensure Map iteration uses canonical order
• Add encoder unit tests for:
• primitives
• lists
• maps
• nested structures

⸻

4.3 Optional File Framing

Requirements
A standalone .scb file may include:
• Magic bytes: "STRATA1"
• Version byte: 0x01
• Followed by exactly one encoded Value

Tasks
• Decide whether framing is included in canonical hash
• Implement optional framing helper
• Add test for framed vs unframed output

⸻

5. Hashing

Goal

Provide a stable, content-addressable identity for Strata data.

⸻

5.1 Canonical Hash

Requirements
• Hash algorithm: BLAKE3-256
• Input: canonical Value bytes
• Output: 32 raw bytes (hex for display)

Tasks
• Add hash_value(&Value) -> [u8; 32]
• Hash only canonical bytes (no framing unless specified)
• Add unit tests:
• same value → same hash
• different value → different hash

⸻

5.2 Golden Vector Verification

Requirements
Golden vectors are law.

Each vector includes:
• .st source
• expected .scb.hex
• expected .hash.hex

Tasks
• Load vectors from vectors/
• Parse → encode → hash
• Assert byte-for-byte and hash-for-hash equality
• Fail hard on mismatch

⸻

6. Cross-language Contract

Goal

Guarantee identical behavior between Rust and JavaScript implementations.

⸻

6.1 Integer Semantics

Rules
• Rust: i64
• JavaScript: BigInt only
• No floats
• No implicit coercions

Tasks (Rust side)
• Document integer rules clearly
• Ensure encoder rejects non-i64 values (already enforced)

Tasks (JS side, future)
• Decode Int → BigInt
• Encode only from BigInt
• Match varint behavior exactly

⸻

6.2 Ordering & Canonicalization

Rules
• Map keys sorted by UTF-8 byte order
• Duplicate keys invalid

Tasks
• Add test asserting unordered input → ordered output
• Decide duplicate-key behavior (reject or last-write)

⸻

7. CLI

Goal

Provide a thin, honest interface over the core library.

No hidden behavior. No magic.

⸻

7.1 Commands

strata compile <in.st> <out.scb>
• Parse Strata Text
• Encode canonical bytes
• Write .scb

strata hash <in.st | in.scb>
• Produce BLAKE3 hex hash
• Accept text or binary input

strata decode <in.scb>
• Decode binary to debug-friendly structure (non-canonical)
• For inspection only

strata fmt <in.st>
• Pretty-print Strata Text
• Optional, non-blocking

⸻

Tasks
• Implement minimal CLI with clap
• Wire commands to library functions
• Add smoke tests for CLI paths

⸻

Northstar v1 Completion Criteria

Northstar v1 is complete when:
• Encoder produces canonical .scb
• Hashing matches golden vectors
• CLI can compile and hash data
• Rust implementation matches spec exactly
• Spec is frozen

After this:
• JS implementation can begin
• Strata becomes a real interchange format
• Northstar v2 discussions may start

⸻

If you want, next we can:
• split this into GitHub issues
• generate TODOs in code
• or jump straight into encode.rs step-by-step

Just tell me the next move.
