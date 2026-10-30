//! Linker-defined ELF symbols.
//!
//! This crate provides and documents linker-defined ELF symbols.
//! They provide information about the currently running executable program.
//! Since these symbols are ELF-specific, this crate is not cross-platform.
//!
//! # Symbols
//!
//! The following symbols are provided:
//!
//! | Symbol               | Name                 |
//! | -------------------- | -------------------- |
//! | `__executable_start` | [`executable_start`] |
//! | `__ehdr_start`       | [`elf_header`]       |
//! | `_etext`             | [`text_end`]         |
//! | `_edata`             | [`data_end`]         |
//! | `_end`               | [`executable_end`]   |
//!
//! # Linkers
//!
//! The following linkers support all symbols exposed by this crate.
//! Each symbol documents the linker commit it has been introduced in.
//! All supported linkers are tested in CI.
//!
//! | Linker | Supported |
//! | ------ |:---------:|
//! | BFD    | ✅        |
//! | gold   | ✅        |
//! | LLD    | ✅        |
//! | mold   | ✅        |
//! | Wild   | ✅        |
//!
//! # Examples
//!
//! ```
//! println!("Executable start:  {:p}", elf_symbols::executable_start());
//! println!("ELF header:        {:p}", elf_symbols::elf_header());
//! println!("Text segment end:  {:p}", elf_symbols::text_end());
//! println!("Data segment end:  {:p}", elf_symbols::data_end());
//! println!("BSS segment start: {:p}", elf_symbols::bss_start());
//! println!("Executable end:    {:p}", elf_symbols::executable_end());
//! ```

#![no_std]

/// Start of the executable (`__executable_start`).
///
/// The address of `__executable_start` is the location of the first
/// loadable segment.
///
/// Apart from changelogs, it is not officially documented.
///
/// It is defined since:
///
/// - BFD: [binutils-gdb@`c6d3b05`]
/// - gold: [binutils-gdb@`f6ce93d`]
/// - LLD: [llvm/llvm-project@`0e454a9`]
/// - mold: [rui314/mold@`5f492fe`]
/// - Wild: [wild-linker/wild@`0a21948`]
///
/// [binutils-gdb@`c6d3b05`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=c6d3b05fe766fe33bb96b8850559c9ada7296dd4
/// [binutils-gdb@`f6ce93d`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=f6ce93d6e999d1a0c450c5e71c5b3468e6217f0a#patch4
/// [llvm/llvm-project@`0e454a9`]: https://github.com/llvm/llvm-project/commit/0e454a9837c312807e8791dfcd8607cbc18d4359
/// [rui314/mold@`5f492fe`]: https://github.com/rui314/mold/commit/5f492fea708029656ddaea8e9b53a8fc3b503b7a
/// [wild-linker/wild@`0a21948`]: https://github.com/wild-linker/wild/commit/0a219486590a3349c803377170beed9afe759210
#[doc(alias = "__executable_start")]
#[must_use]
#[inline]
pub fn executable_start() -> *mut () {
    unsafe extern "C" {
        static mut __executable_start: u8;
    }

    (&raw mut __executable_start).cast::<()>()
}

/// ELF file header (`__ehdr_start`).
///
/// This symbol is provided if there is a load segment that contains the file
/// and program headers. Programs can use this to examine themselves. This
/// pointer can be cast to a `Elf64_Ehdr` definition of your choice, for
/// example [`elf::Elf64_Ehdr`].
///
/// Apart from changelogs, it is not officially documented.
///
/// It is defined since:
///
/// - BFD: [binutils-gdb@`62655c7`]
/// - gold: [binutils-gdb@`eabc84f`]
/// - LLD: [llvm/llvm-project@`4f7a5c3`]
/// - mold: [rui314/mold@`ccc7f83`]
/// - Wild: [wild-linker/wild@`fb7da78`]
///
/// [`elf::Elf64_Ehdr`]: https://docs.rs/elf/0.8.0/elf/file/struct.Elf64_Ehdr.html
/// [binutils-gdb@`62655c7`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=62655c7b8bfc33e6c12694f439ff8f7e8da3005a
/// [binutils-gdb@`eabc84f`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=eabc84f4848311a68f65df04e428c8b53a92f1c0
/// [llvm/llvm-project@`4f7a5c3`]: https://github.com/llvm/llvm-project/commit/4f7a5c3bb429c945ff4e456478b5532f828d1143
/// [rui314/mold@`ccc7f83`]: https://github.com/rui314/mold/commit/ccc7f83cde954048424fa488464be41f210e60d1
/// [wild-linker/wild@`fb7da78`]: https://github.com/wild-linker/wild/commit/fb7da7841ad9e64e2cd1128a62d23d488dae921d#diff-54c31ef8bdfe7c6822f3a28461f7c2b70169d704a60baf0adf27f29ddd44812a
#[doc(alias = "__ehdr_start")]
#[must_use]
#[inline]
pub fn elf_header() -> *mut () {
    unsafe extern "C" {
        static mut __ehdr_start: u8;
    }

    (&raw mut __ehdr_start).cast::<()>()
}

/// End of the text segment (`_etext`).
///
/// The address of `_etext` is the first location after the last read-only
/// loadable segment. For details, see [etext(3C)].
///
/// It is defined since:
///
/// - BFD: [binutils-gdb@`252b513`]
/// - gold: [binutils-gdb@`f6ce93d`]
/// - LLD: [llvm/llvm-project@`9e85939`]
/// - mold: [rui314/mold@`9917372`]
/// - Wild: [wild-linker/wild@`7d9b5c0`]
///
/// [etext(3C)]: https://docs.oracle.com/cd/E86824_01/html/E54766/etext-3c.html
/// [binutils-gdb@`252b513`]: https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=ld/scripttempl/elf.sc;h=e1fea97d791a692b267a267a9a70463289eeb998;hb=252b5132c753830d5fd56823373aed85f2a0db63
/// [binutils-gdb@`f6ce93d`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=f6ce93d6e999d1a0c450c5e71c5b3468e6217f0a#patch4
/// [llvm/llvm-project@`9e85939`]: https://github.com/llvm/llvm-project/commit/9e8593949d6d2df38f9d354e99c683fb4f014710
/// [rui314/mold@`9917372`]: https://github.com/rui314/mold/commit/991737256916ec86d1f5a94be35b30e9849ca11a
/// [wild-linker/wild@`7d9b5c0`]: https://github.com/wild-linker/wild/commit/7d9b5c0cbe0788ad0df2cba928421539d1fe935b
#[doc(alias = "_etext")]
#[must_use]
#[inline]
pub fn text_end() -> *mut () {
    unsafe extern "C" {
        static mut _etext: u8;
    }

    (&raw mut _etext).cast::<()>()
}

/// End of the data segment (`_edata`).
///
/// The address of `_edata` is the first location after the last read-write
/// loadable segment. For details, see [etext(3C)].
///
/// It is defined since:
///
/// - BFD: [binutils-gdb@`252b513`]
/// - gold: [binutils-gdb@`f6ce93d`]
/// - LLD: [llvm/llvm-project@`9e85939`]
/// - mold: [rui314/mold@`9917372`]
/// - Wild: [wild-linker/wild@`7d9b5c0`]
///
/// [etext(3C)]: https://docs.oracle.com/cd/E86824_01/html/E54766/etext-3c.html
/// [binutils-gdb@`252b513`]: https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=ld/scripttempl/elf.sc;h=e1fea97d791a692b267a267a9a70463289eeb998;hb=252b5132c753830d5fd56823373aed85f2a0db63
/// [binutils-gdb@`f6ce93d`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=f6ce93d6e999d1a0c450c5e71c5b3468e6217f0a#patch4
/// [llvm/llvm-project@`9e85939`]: https://github.com/llvm/llvm-project/commit/9e8593949d6d2df38f9d354e99c683fb4f014710
/// [rui314/mold@`9917372`]: https://github.com/rui314/mold/commit/991737256916ec86d1f5a94be35b30e9849ca11a
/// [wild-linker/wild@`7d9b5c0`]: https://github.com/wild-linker/wild/commit/7d9b5c0cbe0788ad0df2cba928421539d1fe935b
#[doc(alias = "_edata")]
#[must_use]
#[inline]
pub fn data_end() -> *mut () {
    unsafe extern "C" {
        static mut _edata: u8;
    }

    (&raw mut _edata).cast::<()>()
}

/// Start of the BSS segment (`__bss_start`).
///
/// The address of `__bss_start` is the location of the BSS segment.
///
/// Apart from changelogs, it is not officially documented.
///
/// It is defined by:
///
/// - BFD: [binutils-gdb@`252b513`]
/// - gold: [binutils-gdb@`f6ce93d`]
/// - LLD: [llvm/llvm-project@`e6c5d38`]
/// - mold: [rui314/mold@`ccc7f83`]
/// - Wild: TODO
///
/// [binutils-gdb@`252b513`]: https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=ld/scripttempl/elf.sc;h=e1fea97d791a692b267a267a9a70463289eeb998;hb=252b5132c753830d5fd56823373aed85f2a0db63
/// [binutils-gdb@`f6ce93d`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=f6ce93d6e999d1a0c450c5e71c5b3468e6217f0a#patch4
/// [llvm/llvm-project@`e6c5d38`]: https://github.com/llvm/llvm-project/commit/e6c5d3862d123f84bf400078172fd93680e15c5b
/// [rui314/mold@`ccc7f83`]: https://github.com/rui314/mold/commit/ccc7f83cde954048424fa488464be41f210e60d1
#[doc(alias = "__bss_start")]
#[must_use]
#[inline]
pub fn bss_start() -> *mut () {
    unsafe extern "C" {
        static mut __bss_start: u8;
    }

    (&raw mut __bss_start).cast::<()>()
}

/// End of the executable (`_end`).
///
/// The address of `_end` is the first location after the last loadable
/// segment. For details, see [etext(3C)].
///
/// It is defined since:
///
/// - BFD: [binutils-gdb@`252b513`]
/// - gold: [binutils-gdb@`ead1e42`]
/// - LLD: [llvm/llvm-project@`b044af5`]
/// - mold: [rui314/mold@`9917372`]
/// - Wild: [wild-linker/wild@`fb7da78`]
///
/// [etext(3C)]: https://docs.oracle.com/cd/E86824_01/html/E54766/etext-3c.html
/// [binutils-gdb@`252b513`]: https://sourceware.org/git/?p=binutils-gdb.git;a=blob;f=ld/scripttempl/elf.sc;h=e1fea97d791a692b267a267a9a70463289eeb998;hb=252b5132c753830d5fd56823373aed85f2a0db63
/// [binutils-gdb@`ead1e42`]: https://sourceware.org/git/?p=binutils-gdb.git;a=commitdiff;h=ead1e4244a55707685d105c662a9a1faf5d122fe#patch8
/// [llvm/llvm-project@`b044af5`]: https://github.com/llvm/llvm-project/commit/b044af50f28209ff4eeed8fa4614e78969d8df74
/// [rui314/mold@`9917372`]: https://github.com/rui314/mold/commit/991737256916ec86d1f5a94be35b30e9849ca11a
/// [wild-linker/wild@`fb7da78`]: https://github.com/wild-linker/wild/commit/fb7da7841ad9e64e2cd1128a62d23d488dae921d#diff-54c31ef8bdfe7c6822f3a28461f7c2b70169d704a60baf0adf27f29ddd44812a
#[doc(alias = "_end")]
#[must_use]
#[inline]
pub fn executable_end() -> *mut () {
    unsafe extern "C" {
        static mut _end: u8;
    }

    (&raw mut _end).cast::<()>()
}
