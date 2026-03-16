# usize-conv

![crates.io](https://img.shields.io/crates/v/usize-conv)
![docs.rs](https://img.shields.io/docsrs/usize-conv)
![MSRV](https://img.shields.io/badge/MSRV-1.85-blue)
![no_std](https://img.shields.io/badge/no__std-compatible-blue)

Infallible integer conversions to and from `usize` and `isize` under explicit
portability guarantees.

- `#![no_std]`
- zero dependencies
- conversions verified at compile time

```rust
use usize_conv::ToUsize;

let n: usize = 42_u32.to_usize();
```

## Why this crate exists

Rust provides many safe integer conversions through `From`, `Into`, and
`TryFrom`. However, conversions involving the machine-sized integers
`usize` and `isize` can be awkward in portable code.

Some integer conversions are *infallible on common architectures*, but Rust
only provides infallible conversions that assume `usize ≥ 16 bits`, the
minimum width guaranteed by the language (for example `usize::from(42_u8)`).
As a result, conversions that are perfectly safe on 32-bit or 64-bit systems
may still require `TryFrom` or explicit casts.

Using `as` casts is concise, but they may silently *truncate values or lose
sign information*, which can hide bugs.

`usize-conv` provides explicit, infallible conversions to and from `usize`
and `isize` through extension traits such as `ToUsize` and `ToIsize`. These
conversions are only available when they are guaranteed to be safe under a
chosen *portability contract*.

For example:

```rust
use usize_conv::ToUsize;

let n: usize = 42_u32.to_usize();
```

This conversion is available when the crate is compiled with the
`min-usize-32` feature, which guarantees `usize ≥ 32 bits`.

Conversions that would *truncate values or lose sign information* on
supported targets are not implemented.

## Portability contracts

The crate exposes conversions according to explicit portability floors:

| Feature        | Guarantee                                                                               |
|----------------|-----------------------------------------------------------------------------------------|
| `min-usize-32` | Code must run on targets where `usize ≥ 32 bits`                                        |
| `min-usize-64` | Code must run on targets where `usize ≥ 64 bits`                                        |
| `from_usize`   | Enable widening conversions from `usize` / `isize` that are valid on all 64-bit systems |



`min-usize-64` implies `min-usize-32`. If both features are enabled (for
example through [Cargo feature unification][1]), the larger portability floor
applies.

If no portability feature is enabled, the crate compiles but does not expose
any conversion traits. Conversions become available once a portability
contract is selected.

### Conversions from `usize` / `isize`

When the `from_usize` feature is enabled, widening conversions are provided
through the traits `ToU64`, `ToU128`, `ToI64`, and `ToI128`.

The following conversions are available:

- `usize → u64`
- `usize → u128`
- `usize → i128`
- `isize → i64`
- `isize → i128`

Conversions into narrower integer types (such as `usize → u32`) are not
provided. While these may be infallible on smaller targets, they would
break portability on common 64-bit systems.

## When to use this crate

Use `usize-conv` when:

- Your code needs succinct **infallible conversions involving `usize` or `isize`**.
- The conversion is safe under your **portability assumptions** (for example,
  code that only runs on 32-bit or 64-bit systems).
- You want to avoid `TryFrom` in cases where failure **cannot occur on the
  targets you support**.

Typical examples include:

- embedded firmware targeting 32-bit microcontrollers or backend code
  deployed on 64-bit servers
- generic code that needs conversions such as `T → usize`
- macros or libraries where `TryFrom` is inconvenient

## When not to use this crate

You **do not need this crate** if:

- The standard library already provides the conversion (`From` / `Into`)
- The conversion may fail on your supported targets (use `TryFrom`)
- Your code must run on **all Rust targets where `usize ≥ 16 bits`**

## For library authors

`usize-conv` is particularly useful in generic code where integer types are not
known ahead of time.

The standard library provides the same set of infallible integer conversions on
all targets, because they must remain valid under Rust’s guarantee that
`usize ≥ 16 bits`. As a result, conversions that are perfectly safe on common
32-bit or 64-bit systems may still require `TryFrom`.

`usize-conv` exposes additional infallible conversions when they are guaranteed
to be safe under a stronger portability contract.

For example:

```rust
use usize_conv::ToUsize;

fn index<T: ToUsize>(i: T) -> usize {
    i.to_usize()
}
```

The available implementations depend on the portability contract selected by
the final application. This allows libraries to remain portable while still
benefiting from additional infallible conversions on platforms where they are
guaranteed to be valid.

## Implementation

All conversions are validated at compile time.

Each implementation includes a compile-time assertion that verifies the
conversion is indeed infallible (i.e. it cannot truncate values or lose sign
information). These assertions exist to ensure the correctness of the crate’s
implementations.

The set of available conversions is determined entirely by the selected
feature flags and is fixed at compile time.

Additionally, the `min-usize-*` features include `compile_error!` checks to
ensure the selected portability contract is compatible with the compilation
target. For example, enabling `min-usize-64` on a 32-bit target will cause the
build to fail.

[1]: https://doc.rust-lang.org/cargo/reference/features.html#feature-unification
