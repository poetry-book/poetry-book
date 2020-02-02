# Poetry book
Rust library that allows you to create a poetry book starting from plain text.

The output is a latex file which by default produces a nice pdf, but you can also edit it to suit your needs.

For example you can change font size and paper format by editing:

```
\documentclass[11pt, a4paper]{article}
```

## Features
- Table of contents
- Preface
- Automatic poem alignment
  - You can center the poems according to the average verse length or the
    longest verse. See `poetry-book::CenteredVerse`
- Localization
  - You can see list of the languages built into your LaTeX system every
    time the compiler is started in the `.log` file

## Example
See [tests/latex_book.rs](./tests/latex_book.rs).
