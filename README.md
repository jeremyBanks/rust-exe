# rust-exe: the ultimate Rust script runner

It's the best!

## Features:

- Run Rust scripts with a shebang, `#!/usr/bin/env rust`, or directly,
  `rust run ./foo.rs`.
- Implicit dependencies based on `::rooted` paths.
- Implicit dependency versions based on script modification time.
  - Looks for a version compatible with the largest stable release at
    modification time.
  - If the file is in git and unmodified, we'll use the author timestamp of the
    last commit to modify it. Otherwise, we'll use the time from the filesystem.
  - Or a timestamp override can be specified in the file itself.
- Implicit script edition based on modification time.
- Multi-file scripts supported (i.e. `mod` statements work).
- Succinct syntax for explicit dependency version and feature specification.
  ```rust
  /// [@] "~1.0.0" (-default +derive)
  use ::serde;
  ```
- Succinct syntax for explicitly specifying toolchain.
  ```rust
  //! [@] nightly
  ```
  ...or freezing the timestamp...
  ```rust
  //! [@] 1654641060.136
  ```
- `--lock` flag to generate an associated `.lock` file, and `--locked` to
  require it.
- `--save` flag to inline inferred versions for direct dependencies, and current
  mtime.
  - `--save-versions` or `--save-mtime` are available to just save one or the
    other.
- Or you can embed a full manifest in your crate docs, if you really want to do
  that.
- WASI support, FWIW.

## Installation

You can choose to use either the `rust` or `rust-exe` binary name.

```sh
cargo install rust-exe --bin rust
```

```sh
cargo install rust-exe --bin rust-exe
```
