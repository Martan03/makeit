# makeit - c template

makeit template for creating C project with `Makefile` capable of compiling
multiple files (note that the `Makefile` was stolen from my
[friend](https://github.com/BonnyAD9))

## Variables
- `args`: if defined, main will contain parameters for arguments
- `cc`: compiler to use
- `dflags`: flags to use when debugging
- `rflags`: flags to use for release builds

## Dependencies
- [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html)
