# Northstar T2 - Raw Wire Determinism

Northstar T2 proves that Strata Core Binary (.scb) survives raw transport without envelopes.

A Strata payload encoded in Rust, transmitted as raw bytes over HTTP, must decode, re-encode, and re-hash identically in JavaScript.

No helpers.
No encoding layers.
No forgiveness.

If this passes, Strata is protocol-safe.
