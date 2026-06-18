fn main() {
    println!("Executable start:  {:p}", elf_symbols::executable_start());
    println!("ELF header:        {:p}", elf_symbols::elf_header());
    println!("Text segment end:  {:p}", elf_symbols::text_end());
    println!("Data segment end:  {:p}", elf_symbols::data_end());
    println!("BSS segment start: {:p}", elf_symbols::bss_start());
    println!("Executable end:    {:p}", elf_symbols::executable_end());
}
