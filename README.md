# Memory-mapped File IO

This library allows reading files by using [`mmap()`](https://en.wikipedia.org/wiki/Mmap) under the hood.

Documentation is hosted [here](https://br0kenpixel.github.io/mapped_fileio/mapped_fileio/struct.MappedFile.html).

### What is [`mmap()`](https://en.wikipedia.org/wiki/Mmap)?
`mmap()` is a POSIX-compliant system call, which allows mapping mapping files and devices into (virtual) memory.

### How does this library work?
When opening files the following operations will be executed:
1. [`open()`](https://man7.org/linux/man-pages/man2/open.2.html) is called to get a file descriptor.
2. [`fstat()`](https://linux.die.net/man/2/fstat) is called to get the file size. This is needed to let `mmap()` know how much memory to map. This number is also used for bondary checking.
2. Using [`mmap()`](https://en.wikipedia.org/wiki/Mmap) the file is mapped to memory.

The `MappedFile` structure will keep track of the current seek position.
It also implements the following traits:
- [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html)
- [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html)
- [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)
- [`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)

### Dependencies
- [`nix`](https://crates.io/crates/nix)
    - Provides friendly bindings to various *nix platform APIs
    - *Only the necessary features are enabled.*

### ⚠️ Limitations
This library only works on *nix-based systems (Linux, macOS). It __does NOT__ support Windows.

### Does this library use `unsafe` operations?
Yes. Unfortunately, there's no way around this.
- System calls are from Rust's perspective "unsafe", since they are external  `C` functions.
- Usage of raw pointers

### Will large files take up a lot of RAM?
This heavily depends on how the OS decides to map the file. Such mappings are managed by the kernel automatically, and you don't have any control over it.

Either way, most operating systems are smart enough to not automatically map a huge file directly into RAM. `mmap()` will almost always use virtual memory, instead of the physical RAM.

To put it simply, you don't really need to worry. Just trust your OS.

### Notes on seeking
The [`seek()`](https://doc.rust-lang.org/std/io/trait.Seek.html#tymethod.seek) function permits seeking beyond the end of the file, however since such an operation would very likely cause a segfault, this library does __not__ permit such operations. If a seek is attempted with an invalid offset, an error is returned.

### Closing files
When a `MappedFile` goes out of scope, the memory is automatically unmapped using `munmap()` and the file descriptor is closed.