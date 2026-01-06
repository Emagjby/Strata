# Northstar T1 – Wire Determinism

This test verifies that Strata data produced on a Rust backend can be decoded,
re-encoded, and re-hashed on a JavaScript frontend with **bit-for-bit identical
canonical hashes**.

If this test fails, Strata is not wire-safe.

This test does not:

- Share code between frontend and backend
- Transmit structured data (JSON AST)
- Perform schema-based reconstruction

Only canonical bytes cross the boundary.
