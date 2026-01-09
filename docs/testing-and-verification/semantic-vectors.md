# Semantic vectors

Semantic vectors define **meaning**, not just bytes.

They assert that a given Strata Text input represents a **specific logical value**, independent of formatting, ordering, or surface syntax.

If two implementations interpret the same Strata Text differently, at least one of them is wrong.



***

### What a semantic vector is

A semantic vector starts from **Strata Text (`.st`)**, not binary.

It verifies that:

* Parsing produces the correct logical structure
* Semantic meaning is preserved
* Canonical encoding follows from that meaning
* Hashes reflect structure, not syntax

Semantic vectors test _understanding_, not serialization mechanics.



***

### Purpose of semantic vectors

Semantic vectors exist to lock down:

* Parsing rules
* Identifier handling
* Shorthand syntax behavior
* Structural equivalence
* Ordering normalization

They ensure that **Strata Text is not ambiguous**.



***

### What semantic vectors guarantee

Given a semantic vector:

* The parsed value must match the expected structure
* Equivalent inputs must produce identical values
* Encoding that value must produce canonical bytes
* Hashing that value must be deterministic

Syntax is disposable. Meaning is not.



***

### Examples of semantic guarantees

Semantic vectors cover cases such as:

* Maps with keys defined in different orders
* Nested map shorthand syntax
* Lists with equivalent structure
* Repeated keys and overwrite semantics
* Mixed formatting, whitespace, and comments

All of these must resolve to the same value model.



***

### Canonicalization boundary

Semantic vectors define the boundary between:

* Parsing
* Encoding

Parsing produces a **logical value**.\
Encoding produces **canonical bytes**.

Semantic vectors ensure this boundary is stable.



***

### Repeated keys and overwrite rules

Semantic vectors define how repeated keys behave.

If a key appears multiple times:

* The last occurrence wins
* Earlier values are discarded
* No error is raised

This rule is fixed and tested.



***

### Shorthand syntax semantics

Semantic vectors verify shorthand map syntax.

For example:

* `user { id: 1 }`
* `{ user: { id: 1 } }`

These must produce the **same logical value**.

If they do not, the parser is incorrect.



***

### Independence from encoding

Semantic vectors do not depend on:

* Tag values
* Binary layout
* Varint encoding
* Hash algorithms

They operate entirely at the **value model level**.

This allows the same vectors to remain valid across versions, as long as semantics do not change.



***

### Cross-language requirements

All implementations must:

* Parse semantic vectors identically
* Produce the same value structure
* Encode to identical canonical bytes
* Hash to identical outputs

There is no language-specific interpretation.



***

### Relationship to golden vectors

Semantic vectors complement golden vectors.

* Golden vectors assert canonical bytes and hashes
* Semantic vectors assert canonical meaning

Both are required.



***

### Adding new semantic vectors

New semantic vectors may be added when:

* New syntax is introduced
* New parsing rules are defined
* Ambiguities are resolved explicitly

Semantic vectors must not redefine existing meaning.



***

### Summary

Semantic vectors ensure that:

* Meaning is fixed
* Syntax is flexible
* Structure is deterministic

If two inputs mean the same thing, Strata treats them as the same.

Anything else is a bug.
