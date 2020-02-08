# Poetry book

[![Crates.io](https://img.shields.io/crates/v/poetry-book.svg)](https://crates.io/crates/poetry-book)
[![CI](https://github.com/poetry-book/poetry-book/workflows/Rust/badge.svg)](https://github.com/poetry-book/poetry-book/actions)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Rust library that allows you to create a poetry book starting from plain text.

The output is a latex file which by default produces a nice pdf, but you can also edit it to suit your needs.

For example you can change font size and paper format by editing:

```
\documentclass[11pt, a4paper]{article}
```

## Features

* Table of contents
* Preface
* Automatic poem alignment
  * You can center the poems according to the average verse length or the
    longest verse. See `poetry-book::CenteredVerse`
* Localization
  * You can see list of the languages built into your LaTeX system every
    time the compiler is started in the `.log` file

## Example

See [tests/latex_book.rs](./tests/latex_book.rs).

## Frontends

This project is a library, i.e. it can be used by other rust projects.
Users may create their books using one of the following frontends:

* [https://crates.io/crates/poetry-book-cli](poetry-book-cli): command line interface
* [https://crates.io/crates/poetry-book-web](poetry-book-web): web interface (TODO)

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
