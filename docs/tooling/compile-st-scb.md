# Compile .st -> .scb

The `compile` command converts **Strata Text (`.st`)** into **Strata Core Binary (`.scb`)**.

This is a **canonical compilation step**. The output bytes are the only valid binary representation of the input value.



***

### Purpose

`compile` exists to:

* Parse human-authored Strata Text
* Validate all syntax and semantics
* Produce canonical Strata Core Binary
* Reject any invalid or ambiguous input

It is the only supported way to produce `.scb` from `.st`.



***

### Command

```
strata-js compile <input.st> <output.scb>
```



***

### Compilation pipeline

The command performs the following steps, in order:

1. **Parse**
   * The `.st` file is parsed into a Strata value
   * All syntax rules are enforced
   * All integers are range-checked
   * All bytes and strings are validated
2. **Validate**
   * The value model is validated against Strata rules
   * Invalid constructs are rejected
   * No normalization or coercion is performed
3. **Encode**
   * The value is encoded into canonical Strata Core Binary
   * Map keys are ordered canonically
   * Integers are encoded using canonical varints
   * Strings are encoded as validated UTF-8
4. **Write**
   * The resulting bytes are written exactly as produced
   * No metadata or headers are added



***

### Canonical guarantee

For a given `.st` file:

* The output `.scb` is deterministic
* Recompiling produces byte-identical output
* Hashes computed over the output are stable

Any change in output bytes indicates:

* A change in input
* Or a regression in the implementation



***

### Error handling

Compilation fails if:

* The input contains invalid syntax
* An integer is out of range
* A string is malformed
* The input violates Strata rules

On failure:

* No output file is written
* The command exits with a non-zero code
* An error message is printed



***

### No partial success

`compile` is atomic.

Either:

* The entire file is valid and compiled
* Or nothing is produced

There is no partial output.



***

### Example

```
strata-js compile config.st config.scb
```

This:

* Reads `config.st`
* Produces `config.scb`
* Guarantees canonical encoding



***

### Relationship to hashing

Hashes are not computed during compilation.

However:

* The output `.scb` is hash-ready
* Hashing the result is guaranteed to be stable
* Hashing before or after transport yields the same result



***

### Summary

`compile` is the boundary between human input and canonical data.

Once a value is compiled:

* It is frozen
* It is unambiguous
* It is safe to hash, transmit, and store

All Strata pipelines start here.
