# ELF Symbols

[![Crates.io](https://img.shields.io/crates/v/elf-symbols)](https://crates.io/crates/elf-symbols)
[![docs.rs](https://img.shields.io/docsrs/elf-symbols)](https://docs.rs/elf-symbols)
[![CI](https://github.com/mkroening/elf-symbols/actions/workflows/ci.yml/badge.svg)](https://github.com/mkroening/elf-symbols/actions/workflows/ci.yml)

This crate provides and documents linker-defined ELF symbols.

For API documentation, see the [docs].

[docs]: https://docs.rs/elf-symbols

## Examples

```rust
println!("Executable start:  {:p}", elf_symbols::executable_start());
println!("ELF header:        {:p}", elf_symbols::elf_header());
println!("Text segment end:  {:p}", elf_symbols::text_end());
println!("Data segment end:  {:p}", elf_symbols::data_end());
println!("BSS segment start: {:p}", elf_symbols::bss_start());
println!("Executable end:    {:p}", elf_symbols::executable_end());
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
