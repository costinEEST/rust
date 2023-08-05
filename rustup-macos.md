- `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

```bash
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /Users/constantin.campean/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  /Users/constantin.campean/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /Users/constantin.campean/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /Users/constantin.campean/.profile
  /Users/constantin.campean/.bash_profile
  /Users/constantin.campean/.bashrc
  /Users/constantin.campean/.zshenv

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-apple-darwin
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>

info: profile set to 'default'
info: default host triple is x86_64-apple-darwin
info: syncing channel updates for 'stable-x86_64-apple-darwin'
info: latest update on 2023-08-03, rust version 1.71.1 (eb26296b5 2023-08-03)
info: downloading component 'cargo'
  5.6 MiB /   5.6 MiB (100 %)   2.5 MiB/s in  2s ETA:  0s
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 13.6 MiB /  13.6 MiB (100 %)   3.2 MiB/s in  5s ETA:  0s
info: downloading component 'rust-std'
 24.5 MiB /  24.5 MiB (100 %)   1.8 MiB/s in 10s ETA:  0s
info: downloading component 'rustc'
 55.3 MiB /  55.3 MiB (100 %)   2.6 MiB/s in 17s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.6 MiB /  13.6 MiB (100 %)   3.3 MiB/s in  3s ETA:  0s
info: installing component 'rust-std'
 24.5 MiB /  24.5 MiB (100 %)  12.7 MiB/s in  4s ETA:  0s
info: installing component 'rustc'
 55.3 MiB /  55.3 MiB (100 %)  16.2 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-apple-darwin'

  stable-x86_64-apple-darwin installed - rustc 1.71.1 (eb26296b5 2023-08-03)


Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

- `rustup --version`

```bash
rustup 1.26.0 (5af9b9484 2023-04-05)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.71.1 (eb26296b5 2023-08-03)`
```

- `rustup default nightly`

```bash
info: syncing channel updates for 'nightly-x86_64-apple-darwin'
info: latest update on 2023-08-05, rust version 1.73.0-nightly (e4c144684 2023-08-04)
info: downloading component 'cargo'
  6.2 MiB /   6.2 MiB (100 %)   3.3 MiB/s in  1s ETA:  0s
info: downloading component 'clippy'
  2.3 MiB /   2.3 MiB (100 %) 261.5 KiB/s in  2s ETA:  0s
info: downloading component 'rust-docs'
 13.8 MiB /  13.8 MiB (100 %)   3.2 MiB/s in  4s ETA:  0s
info: downloading component 'rust-std'
 24.9 MiB /  24.9 MiB (100 %)   4.2 MiB/s in  8s ETA:  0s
info: downloading component 'rustc'
 54.7 MiB /  54.7 MiB (100 %)   3.6 MiB/s in 19s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 13.8 MiB /  13.8 MiB (100 %)   3.4 MiB/s in  3s ETA:  0s
info: installing component 'rust-std'
 24.9 MiB /  24.9 MiB (100 %)  12.1 MiB/s in  3s ETA:  0s
info: installing component 'rustc'
 54.7 MiB /  54.7 MiB (100 %)  13.1 MiB/s in  4s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'nightly-x86_64-apple-darwin'

  nightly-x86_64-apple-darwin installed - rustc 1.73.0-nightly (e4c144684 2023-08-04)
```
