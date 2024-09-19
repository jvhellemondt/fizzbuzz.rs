# FizzBuzz | Rust

## Run the project

- To build and run the project in one step, use:

```bash
cargo run
```
## Rust the project with Watch mode

Make sure to install `cargo-watch` globally by running:

```bash
cargo install cargo-watch
```

To run the project in watch mode, use:

```bash
cargo watch -x run
```

To use watch mode with Tests, run:

```bash
cargo watch -x test
```

## Rust Installation

### 1. Installing Rust

#### macOS and Linux

To install Rust, you can use the `rustup` toolchain installer, which will set up the Rust compiler and the Cargo
package manager.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

For Homebrew:

```bash
brew install rust
```

Follow the on-screen instructions to complete the installation. After installation, make sure to configure your
environment:

```bash
source $HOME/.cargo/env
```

### 2. Verifying Installation

Check the installation by confirming the versions of `rustc`, `cargo`, and `rustup`:

```bash
rustc --version
cargo --version
rustup --version
```
