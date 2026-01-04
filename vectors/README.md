# Strata Golden Vectors

These files are the source of truth for Strata behavior.

- Rust implementations MUST match these vectors.
- JavaScript implementations MUST match these vectors.
- If implementation output differs, the implementation is wrong.

Rules:

- Vectors are never edited to satisfy code.
- New vectors may be added only via a new Northstar version.
- All languages consume these files directly from this directory.

Golden vectors define:

- Canonical `.scb` bytes
- Canonical hashes
- Required failure modes (error kind + offset)
