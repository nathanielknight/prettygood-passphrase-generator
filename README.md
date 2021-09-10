# Prettygood Passphrase Generator

This is a passphrase generator library and CLI tool. It **hasn't been reviewed
by a security expert or audited for problems**, but it should be pretty good.

What's the basis for that assertion?

* It uses the 'formal' word list for Christopher Wellons's
  [Pokerware](https://github.com/skeeto/pokerware), which is a pretty good
  word list.
* It uses an (intended) cryptographically secure random number
  generator from [`rand`](https://crates.io/crates/rand), which is
  a pretty good random number generator.
* It draws five words from a ~5300 word list, so your passphrase should be
  one in 4197759730304385024, which is a pretty good number of possibilities
  to be pulling from.
* You can use it from the CLI or as a library (including with WASM).


# CLI

## Usage

When called with no arguments, PPG prints a five word passphrase to standard
output.

## Installation
If you have `cargo` installed you can install it with

```
cargo install ppg
```

