# rust-exe: a Rust script runner

not complete

## Tasks:

- [ ] fix current tests
- [ ] run tests on CI
- [ ] keep trunk green
  - [ ] use bors-ns to merge
- [ ] collecting multiple files/modules
- [ ] use cached binary if (actual) file modification time is recent enough.
- [ ] reading timestamp annotations from file
- [ ] writing timestamp annotation on file
- [ ] reading annotations on top-level items
- [ ] writing annotations with top-level use statements
- [ ] check modification times in Git (if unmodified)

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
  ///[#!serde]: 0.1.0 (-default +std)
  ///[#!serde]: using ({ version = "1.2.4", path = "foo/bar" })
  use ::serde;
  ```
- Succinct syntax for explicitly specifying toolchain.
  ```rust
  //![#!rust]: nightly
  ```
  ...or freezing the timestamp...
  ```rust
  //![#!date]: ~2022-12-09-16:09:53
  ```
- `--lock` flag to generate an associated `.lock` file, and `--locked` to
  require it.
- `--save` flag to inline inferred versions for direct dependencies, and current
  mtime.
  - `--save-versions` or `--save-mtime` are available to just save one or the
    other.
- Or you can embed a full manifest in your crate docs, if you really want to do
  that.
- WASI support, FWIW. For sandboxing.
- If path dependencies are .rs files instead of directories with Cargo.toml,
  apply everything recursively? That might be a lot easier than managing a workspace.

## Installation

You can choose to use either the `rust` or `rust-exe` binary name.

```sh
cargo install rust-exe --bin rust
```

```sh
cargo install rust-exe --bin rust-exe
```
