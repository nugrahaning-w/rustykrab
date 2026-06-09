# RustyKrab

RustyKrab is a native-first cross-platform mobile framework written in Rust.

The framework compiles Rust UI DSL into native platform source code:

* SwiftUI (iOS)
* Jetpack Compose (Android)

RustyKrab does not include a rendering engine. All generated output is native platform code.

---

# Prerequisites

Before working on RustyKrab, make sure the following tools are installed:

| Tool    | Version            |
| ------- | ------------------ |
| Git     | Latest             |
| Rust    | Stable             |
| Cargo   | Included with Rust |
| rustfmt | Latest             |
| Clippy  | Latest             |
| Make    | macOS/Linux        |

---

# Verify Installation

## Git

```bash
git --version
```

Expected:

```text
git version x.x.x
```

## Rust

```bash
rustc --version
```

Expected:

```text
rustc x.x.x
```

## Cargo

```bash
cargo --version
```

Expected:

```text
cargo x.x.x
```

## Rustfmt

```bash
cargo fmt --version
```

Expected:

```text
rustfmt x.x.x
```

## Clippy

```bash
cargo clippy --version
```

Expected:

```text
clippy x.x.x
```

---

# Clone Repository

```bash
git clone <repository-url>
cd rustykrab
```

---

# Workspace Structure

```text
rustykrab/
├── Cargo.toml
├── rustfmt.toml
├── clippy.toml
├── .editorconfig
├── Makefile
│
├── crates/
│   ├── rustykrab-core
│   ├── rustykrab-widget
│   ├── rustykrab-ast
│   ├── rustykrab-parser
│   ├── rustykrab-generator
│   ├── rustykrab-swiftui
│   ├── rustykrab-compose
│   └── rustykrab-cli
│
├── docs/
├── examples/
├── templates/
└── tests/
```

---

# Install Rust Components

Install formatting and linting tools:

```bash
rustup component add rustfmt
rustup component add clippy
```

Verify:

```bash
cargo fmt --version
cargo clippy --version
```

---

# Build Project

Check workspace integrity:

```bash
cargo check
```

Expected:

```text
Finished dev profile
```

---

# Run Formatter

Format all Rust source files:

```bash
cargo fmt --all
```

Check formatting without modifying files:

```bash
cargo fmt --all --check
```

---

# Run Clippy

Run static analysis:

```bash
cargo clippy --workspace
```

Run with warnings treated as errors:

```bash
cargo clippy --workspace -- -D warnings
```

---

# Run Tests

Execute all tests:

```bash
cargo test --workspace
```

---

# Makefile Commands

RustyKrab provides several helper commands.

## Format Source Code

```bash
make fmt
```

## Run Linter

```bash
make lint
```

## Run Tests

```bash
make test
```

## Run Full Validation

```bash
make check
```

This command executes:

```bash
cargo fmt --all --check
cargo clippy --workspace
cargo test --workspace
```

---

# Development Workflow

Before creating a Pull Request:

```bash
make check
```

Ensure:

* All code is formatted
* No Clippy warnings
* All tests pass

---

# Coding Standards

## Formatting

Rust source code must follow the project rustfmt configuration.

```bash
cargo fmt --all
```

## Linting

All code must pass Clippy checks.

```bash
cargo clippy --workspace
```

## Unsafe Code

Unsafe Rust is forbidden.

Workspace configuration:

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
```

---

# Continuous Integration

Every Pull Request executes:

```text
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

Pull Requests that fail any of these checks cannot be merged.

---

# Troubleshooting

## cargo command not found

Reload Cargo environment:

```bash
source $HOME/.cargo/env
```

Or restart your terminal.

## rustfmt not found

Install rustfmt:

```bash
rustup component add rustfmt
```

## clippy not found

Install Clippy:

```bash
rustup component add clippy
```

---