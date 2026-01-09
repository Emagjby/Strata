# Format

The `format` (`fmt`) command parses Strata Text and renders it into a **normalized, inspection-friendly form**.

It exists to help humans **see what Strata thinks the value is**, without changing semantics or producing binary output.



***

### Purpose

`format` is a **developer experience tool**, not a canonical transformation.

It is used to:

* Validate Strata Text syntax
* Inspect parsed structure
* Debug parsing issues
* Normalize layout for readability
* Confirm what will be encoded

It does **not** produce Strata Core Binary.



***

### Command

```
strata-js fmt <input.st>
```



***

### What it does

The command performs the following steps:

1. Reads the `.st` file as UTF-8 text
2. Parses it using the Strata Text grammar
3. Converts the parsed value into an inspectable representation
4. Prints formatted JSON to stdout

The output reflects the **exact parsed value**, not a reinterpreted one.



***

### Output format

The output is:

* JSON
* Pretty-printed
* Deterministic
* Intended for humans

Rules:

* Integers are rendered as strings
* Bytes are rendered as byte arrays
* Maps preserve logical structure
* Lists preserve order

This format is **not** Strata Text and **not** Strata Core Binary.



***

### Example

Input:

```
config { 
    enabled: true 
    retries: 3 
    name: "strata" 
    empty: null 
}
```

Output:

```
{ 
    "config": { 
        "enabled": true, 
        "retries": "3", 
        "name": "strata", 
        "empty": null 
    } 
}
```



***

### Canonical guarantees

`format` does not guarantee:

* Canonical byte layout
* Canonical key ordering
* Hash stability

Those guarantees only apply to `.scb` encoding.

However:

* If `fmt` output changes, the parser behavior changed
* If parser behavior changes, encoding and hashing may change
* Therefore, `fmt` is a **diagnostic window into determinism**



***

### Failure modes

Formatting fails if:

* The input is not valid Strata Text
* Integers exceed the allowed range
* Strings contain invalid escapes
* Bytes literals are malformed

Errors include precise location information (line, column, offset).



***

### What `format` does NOT do

The command does not:

* Encode to `.scb`
* Hash values
* Validate canonical ordering
* Normalize semantic meaning
* Accept non-canonical binary input

It operates strictly at the **Strata Text layer**.



***

### Relationship to other commands

* Use `fmt` to inspect
* Use `compile` to encode
* Use `hash` to fingerprint
* Use `decode` to inspect binary data

Each command has a single responsibility.



***

### Summary

`format` answers one question:

**“What value does this Strata Text actually describe?”**

It is a visibility tool. It is not a canonical transformation. It exists to keep humans honest while Strata stays exact.
