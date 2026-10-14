<p align="center">
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/header/graph.svg?title=Cargo+Showcase&subtitle=Demonstração+prática+da+ferramenta+de+build+Cargo+do+Rust&align=left&font=geist-mono&mode=dark" /><img alt="Cargo Showcase" src="https://shieldcn.dev/header/graph.svg?title=Cargo+Showcase&subtitle=Demonstração+prática+da+ferramenta+de+build+Cargo+do+Rust&align=left&font=geist-mono&mode=light" /></picture>
</p>

<p align="center"><b>Português (BR)</b> · <a href="README.md">English</a></p>

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

# 📖 Sobre

O **Cargo Showcase** é um projeto modular e didático desenvolvido para exemplificar todas as capacidades da ferramenta de build e gerenciador de pacotes **Cargo** da linguagem **Rust**. Estruturado como um *workspace* monorepo, o projeto aborda desde funcionalidades básicas (compilação, execução, testes unitários e de integração) até recursos avançados de engenharia de software (scripts de compilação `build.rs`, compilação condicional via feature flags, microbenchmarks, documentação automática com *doc-tests*, otimizações de binário com LTO/stripping e subcomandos customizados da CLI). Além disso, o repositório inclui um relatório acadêmico completo em [SEMINARIO.md](SEMINARIO.md) e um roteiro de apresentação em [ROTEIRO_10MIN.md](ROTEIRO_10MIN.md).

# 📋 Motivo

Trabalho acadêmico desenvolvido para a PUC Minas (Pontifícia Universidade Católica de Minas Gerais) para exemplificar e demonstrar as capacidades da ferramenta de build Cargo da linguagem Rust.

# 💻 Como iniciar

### Requisitos

- [Rust & Cargo](https://www.rust-lang.org/tools/install) >= 1.70.0
- [Git](https://git-scm.com/)

### Instalação

1. Clone o repositório:

   ```sh
   git clone https://github.com/BGLuis/cargo-showcase.git
   ```

2. Entre no diretório do projeto:

   ```sh
   cd cargo-showcase
   ```

3. Instale as dependências e inicie:

   ```sh
   # Executar todos os testes (unitários, integração e doc-tests)
   cargo test --workspace

   # Executar a aplicação CLI de demonstração
   cargo run -p app_cli

   # Ou executar a demonstração guiada completa
   bash scripts/demo.sh
   ```

# 🤝 Contribuidores

<a href="https://github.com/BGLuis/cargo-showcase/graphs/contributors">
  <picture><source media="(prefers-color-scheme: dark)" srcset="https://shieldcn.dev/contributors/BGLuis/cargo-showcase.svg?title=false&preset=transparent&border=false&mode=dark" /><img alt="Contribuidores" src="https://shieldcn.dev/contributors/BGLuis/cargo-showcase.svg?title=false&preset=transparent&border=false&mode=light" /></picture>
</a>

# 📄 Licença

Distribuído sob a licença [MIT](LICENSE).
