# Table of contents

## Introduction

* [What is Strata](README.md)
* [When to use Strata](introduction/when-to-use-strata.md)
* [When NOT to use Strata](introduction/when-not-to-use-strata.md)
* [Design Philosophy](introduction/design-philosophy.md)
* [Terminology](introduction/terminology.md)

## Core Concepts

* [Canonical encoding](core-concepts/canonical-encoding.md)
* [Determinism & Hashing](core-concepts/determinism-and-hashing.md)
* [Value model](core-concepts/value-model.md)
* [Strata Text vs Strata Core Binary](core-concepts/strata-text-vs-strata-core-binary.md)
* [Encoding vs decoding](core-concepts/encoding-vs-decoding.md)
* [Strictness & Failure semantics](core-concepts/strictness-and-failure-semantics.md)

## Specification

* [Canonical rules](specification/canonical-rules.md)
* [Value types](specification/value-types.md)
* [Integer semantics](specification/integer-semantics.md)
* [String & UTF-8 rules](specification/string-and-utf-8-rules.md)
* [Map ordering](specification/map-ordering.md)
* [Binary layouts & Tags](specification/binary-layouts-and-tags.md)
* [Hashing contract](specification/hashing-contract.md)

## Northstar Guarantees

* [What is a Northstar T](northstar-guarantees/what-is-a-northstar-t.md)
* [Northstar T1 - Wire determinism](northstar-guarantees/northstar-t1-wire-determinism.md)
* [Northstar T2 - Raw wire determinism](northstar-guarantees/northstar-t2-raw-wire-determinism.md)
* [Northstar T3 - Framed streaming determinism](northstar-guarantees/northstar-t3-framed-streaming-determinism.md)
* [How Northstar Ts block regressions](northstar-guarantees/how-northstar-ts-block-regressions.md)

## Rust Implementation

* [Overview](rust-implementation/overview.md)
* [Value model (Rust)](rust-implementation/value-model-rust.md)
* [Encoding](rust-implementation/encoding.md)
* [Decoding](rust-implementation/decoding.md)
* [Hashing](rust-implementation/hashing.md)
* [Strata Text parser](rust-implementation/strata-text-parser.md)
* [Error model](rust-implementation/error-model.md)
* [Framing](rust-implementation/framing.md)
* [CLI](rust-implementation/cli.md)
* [Golden vectors (Rust)](rust-implementation/golden-vectors-rust.md)

## JavaScript Implementation

* [Overview](javascript-implementation/overview.md)
* [Value model (JS)](javascript-implementation/value-model-js.md)
* [BigInt & Integer rules](javascript-implementation/bigint-and-integer-rules.md)
* [Encoding](javascript-implementation/encoding.md)
* [Decoding](javascript-implementation/decoding.md)
* [Hashing](javascript-implementation/hashing.md)
* [Strata Text parse](javascript-implementation/strata-text-parse.md)
* [Error model](javascript-implementation/error-model.md)
* [CLI](javascript-implementation/cli.md)
* [Golden vectors (JS)](javascript-implementation/golden-vectors-js.md)

## Interoperability

* [Rust <-> JS parity](interoperability/rust-js-parity.md)
* [Wire formats](interoperability/wire-formats.md)
* [HTTP transport](interoperability/http-transport.md)
* [Streaming & Framing](interoperability/streaming-and-framing.md)
* [What is guaranteed cross-language](interoperability/what-is-guaranteed-cross-language.md)
* [What is NOT guaranteed](interoperability/what-is-not-guaranteed.md)

## Testing & Verification

* [Golden vectors](testing-and-verification/golden-vectors.md)
* [Positive vectors](testing-and-verification/positive-vectors.md)
* [Negative vectors](testing-and-verification/negative-vectors.md)
* [Semantic vectors](testing-and-verification/semantic-vectors.md)
* [CI enforcement](testing-and-verification/ci-enforcement.md)
* [What happens when a test fails](testing-and-verification/what-happens-when-a-test-fails.md)

## Tooling

* [CLI overview](tooling/cli-overview.md)
* [Compile .st -> .scb](tooling/compile-st-scb.md)
* [Decode .scb -> inspect](tooling/decode-scb-inspect.md)
* [Hash](tooling/hash.md)
* [Format](tooling/format.md)
* [Inspecting non-canonical data](tooling/inspecting-non-canonical-data.md)

## Versioning & Stability

* [Versioning policy](versioning-and-stability/versioning-policy.md)
* [Frozen guarantees](versioning-and-stability/frozen-guarantees.md)
* [What requires a new version](versioning-and-stability/what-requires-a-new-version.md)
* [Backward compatibility philosophy](versioning-and-stability/backward-compatibility-philosophy.md)
* [Upgrade strategy](versioning-and-stability/upgrade-strategy.md)

## Contributing

* [How to contribute](contributing/how-to-contribute.md)
* [What changes are acceptable](contributing/what-changes-are-acceptable.md)
* [Canonical change process](contributing/canonical-change-process.md)
* [Adding vectors](contributing/adding-vectors.md)
* [Code of conduct](contributing/code-of-conduct.md)

## Appendix

* [FAQ](appendix/faq.md)
* [Common misconceptions](appendix/common-misconceptions.md)
* [Comparison to other formats](appendix/comparison-to-other-formats.md)
* [Security conciderations](appendix/security-conciderations.md)
* [Glossary](appendix/glossary.md)
