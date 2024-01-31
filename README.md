# rdrop
<!-- [![Crates.io](https://img.shields.io/crates/v/rdrop.svg)](https://crates.io/crates/rdrop) -->
<!-- [![Docs.rs](https://docs.rs/rdrop/badge.svg)](https://docs.rs/rdrop) -->
[![CI](https://github.com//rdrop/workflows/CI/badge.svg)](https://github.com//rdrop/actions)
## notes
pause 
- reimplementing this in rust: https://github.com/noctuid/tdrop
- for fun, and because it doesn't work with Code and Zotero
  - and I'd rather not debug bash 
- as of 2024-01-30, wrote the cli base   

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install rdrop`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## rdrop notes

```console
$ rdrop
<!-- Hello World! -->

$ rdrop
<!-- Hello Ferris! -->
```

<!-- You can also test for command failures and pass in environment variables:

```console
$ rdrop 
? 1
Must supply exactly one argument.

$ GOODBYE=true rdrop
Goodbye World!
``` -->

<!-- Sometimes, your test might include output that is generated at runtime. When that's the case, you
can
use variables to replace those values. In our `tests/trycmd.rs`, we've defined a
variable `[REPLACEMENT]` such that whenever the value `runtime-value` appears, it will be
replaced with `[REPLACEMENT]`:

```console
$ rdrop "blah blah runtime-value blah"
Hello blah blah [REPLACEMENT] blah!

$ rdrop "blah blah runtime-value blah"
Hello blah blah runtime-value blah!

``` -->
