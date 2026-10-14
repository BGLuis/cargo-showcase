<p align="center">
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/header/graph.svg?title=Cargo+Showcase&subtitle=Practical+demonstration+of+the+Rust+Cargo+build+tool&align=left&font=geist-mono&mode=dark" /><img alt="Cargo Showcase" src="https://shieldcn.dev/header/graph.svg?title=Cargo+Showcase&subtitle=Practical+demonstration+of+the+Rust+Cargo+build+tool&align=left&font=geist-mono&mode=light" /></picture>
</p>

<p align="center"><b>English</b> · <a href="README.pt-BR.md">Português (BR)</a></p>

<p align="center">
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/stars/BGLuis/cargo-showcase.svg?variant=secondary&size=sm&mode=dark" /><img alt="GitHub Stars" src="https://shieldcn.dev/github/stars/BGLuis/cargo-showcase.svg?variant=secondary&size=sm&mode=light" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/forks/BGLuis/cargo-showcase.svg?variant=secondary&size=sm&mode=dark" /><img alt="GitHub Forks" src="https://shieldcn.dev/github/forks/BGLuis/cargo-showcase.svg?variant=secondary&size=sm&mode=light" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/watchers/BGLuis/cargo-showcase.svg?variant=secondary&size=sm&mode=dark" /><img alt="Watchers" src="https://shieldcn.dev/github/watchers/BGLuis/cargo-showcase.svg?variant=secondary&size=sm&mode=light" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/contributors/BGLuis/cargo-showcase.svg?theme=emerald&size=sm&mode=dark" /><img alt="Contributors" src="https://shieldcn.dev/github/contributors/BGLuis/cargo-showcase.svg?theme=emerald&size=sm&mode=light" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/github/license/BGLuis/cargo-showcase.svg?variant=ghost&size=sm&mode=dark" /><img alt="License" src="https://shieldcn.dev/github/license/BGLuis/cargo-showcase.svg?variant=ghost&size=sm&mode=light" /></picture>
</p>

<p align="center">
  <img alt="Rust" src="https://shieldcn.dev/badge/Language-Rust-000000.svg?logo=rust&variant=branded&size=sm" />
  <img alt="Bash" src="https://shieldcn.dev/badge/Language-Bash-4EAA25.svg?logo=gnubash&variant=branded&size=sm" />
  <img alt="Linux" src="https://shieldcn.dev/badge/Platform-Linux-FCC624.svg?logo=linux&variant=branded&size=sm" />
</p>

# 📖 About

**Cargo Showcase** is a modular, educational project designed to demonstrate the full spectrum of capabilities of Rust's official build tool and package manager, **Cargo**. Structured as a multi-crate *workspace*, the project covers everything from fundamental workflows (compilation, execution, unit and integration tests) to advanced software engineering features (`build.rs` build scripts, conditional compilation via feature flags, microbenchmarks, automated documentation with *doc-tests*, binary optimization via LTO/stripping, and custom CLI subcommands). Additionally, the repository includes a comprehensive academic report in [SEMINARIO.md](SEMINARIO.md) and a presentation guide in [ROTEIRO_10MIN.md](ROTEIRO_10MIN.md).

# 📋 Motivation

Academic assignment developed for PUC Minas (Pontifícia Universidade Católica de Minas Gerais) to exemplify and demonstrate the capabilities of the Cargo build tool for the Rust programming language.

# 💻 Getting Started

### Requirements

- [Rust & Cargo](https://www.rust-lang.org/tools/install) >= 1.70.0
- [Git](https://git-scm.com/)

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/BGLuis/cargo-showcase.git
   ```

2. Enter the project directory:

   ```sh
   cd cargo-showcase
   ```

3. Install dependencies and run:

   ```sh
   # Run all tests (unit, integration, and doc-tests)
   cargo test --workspace

   # Run the CLI demonstration application
   cargo run -p app_cli

   # Run the complete guided interactive demo
   bash scripts/demo.sh
   ```

# 🤝 Contributors

<a href="https://github.com/BGLuis/cargo-showcase/graphs/contributors">
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/contributors/BGLuis/cargo-showcase.svg?title=false&preset=transparent&border=false&mode=dark" /><img alt="Contributors" src="https://shieldcn.dev/contributors/BGLuis/cargo-showcase.svg?title=false&preset=transparent&border=false&mode=light" /></picture>
</a>

# 📄 License

Distributed under the [MIT](LICENSE) license.
