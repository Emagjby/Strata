# Northstar T3 - Framed Wire Determinism

Northstar T3 verifies that Strata Core Binary (.scb) remains canonical and hash-stable when transmitted as multiple framed messages over a streaming transport.

This test introduces framing only to define message boundaries.
Framing MUST NOT interpret, normalize, or mutate Strata payload bytes.
