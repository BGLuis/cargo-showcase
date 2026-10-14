# Seminário: Ferramentas de Build — Cargo (Rust)

**Disciplina:** Seminários / Configuração e Gerenciamento de Ambientes de Software  
**Instituição:** Pontifícia Universidade Católica de Minas Gerais (PUC Minas)  
**Professor:** Diego Augusto de Faria Barros  
**Tema:** Ferramenta de Build **Cargo** (Ecossistema Rust)  
**Site Oficial:** [https://doc.rust-lang.org/cargo/](https://doc.rust-lang.org/cargo/)

---

## 📑 Sumário Executivo
O **Cargo** é a ferramenta oficial de automação de build, gerenciador de pacotes e orquestrador de ciclo de vida da linguagem de programação **Rust**. Este repositório foi projetado como um projeto de demonstração integral e didático, cobrindo desde as capacidades mais fundamentais até recursos avançados de engenharia de software utilizados em sistemas de missão crítica.

---

## 1. As 7 Características Essenciais de um Sistema de Build

Conforme preconizado na Seção 1.1 do enunciado acadêmico, o Cargo atende e expande cada uma das sete características fundamentais:

### 1.1. Geração do Script de Construção
O Cargo implementa o princípio de **Convenção sobre Configuração** (*Convention over Configuration*):
* **Inicialização Automatizada:** Os comandos `cargo new <nome>` ou `cargo init` criam instantaneamente o esqueleto do projeto com um manifesto declarativo `Cargo.toml`.
* **Zero Boilerplate:** O desenvolvedor não precisa escrever regras de compilação para cada arquivo fonte. O Cargo infere alvos automaticamente:
  - `src/main.rs` $\rightarrow$ Binário executável padrão.
  - `src/lib.rs` $\rightarrow$ Biblioteca (`rlib`).
  - `src/bin/*.rs` $\rightarrow$ Múltiplos binários no mesmo pacote.
  - `tests/*.rs` $\rightarrow$ Testes de integração de caixa preta.
  - `examples/*.rs` $\rightarrow$ Aplicações de exemplo executáveis.
  - `benches/*.rs` $\rightarrow$ Suítes de microbenchmark.
* **Workspaces Monorepo:** O arquivo `Cargo.toml` raiz centraliza a orquestração de múltiplos pacotes interdependentes através de `[workspace.dependencies]` e `[workspace.package]`.

### 1.2. Integração com o Sistema de Controle de Versão (VCS)
* **Geração Automática de Ignore:** Ao inicializar um projeto, o Cargo gera automaticamente o arquivo `.gitignore`, blindando o repositório contra artefatos intermediários (`/target`).
* **Determinismo Absoluto com `Cargo.lock`:** Enquanto o `Cargo.toml` expressa intenções e restrições de versão (SemVer, ex: `^1.0`), o `Cargo.lock` registra as versões exatas e os hashes criptográficos SHA-256 de todas as dependências diretas e transitivas resolvidas. Isso elimina qualquer discrepância entre o ambiente local do desenvolvedor e o pipeline de CI/CD.
* **Fontes Flexíveis de Dependências:** O Cargo suporta dependências publicadas no registro central (`crates.io`), repositórios Git remotos (`git = "https://..."`), branches específicos, tags, ou caminhos locais (`path = "../core_calc"`).

### 1.3. Recompilação Mínima e Eficiência
* **Grafo Acíclico Dirigido (DAG):** O Cargo analisa a árvore de dependências e compila pacotes independentes em paralelo, respeitando estritamente a topologia do grafo.
* **Fingerprinting Preciso:** O Cargo armazena hashes dos arquivos-fonte, flags do compilador, variáveis de ambiente e metadados em `target/`. Se um arquivo não sofreu modificação, sua compilação é pulada imediatamente.
* **Validação Instantânea com `cargo check`:** O Cargo permite rodar toda a análise léxica, sintática, verificação de tipos e o *Borrow Checker* sem gerar código de máquina (codegen LLVM). Isso reduz o ciclo de feedback de dezenas de segundos para poucos milissegundos.

### 1.4. Criação do Sistema Executável
* **Múltiplos Formatos de Artefatos:** Gera binários executáveis nativos (ELF no Linux, Mach-O no macOS, PE/COFF no Windows), bibliotecas dinâmicas C-compatíveis (`cdylib`), bibliotecas estáticas (`staticlib`) e bibliotecas nativas Rust (`rlib`).
* **Perfis Diferenciados (`dev` vs `release`):**
  - `dev` (Debug): Prioriza velocidade de compilação e inclui símbolos de depuração completos (`opt-level = 0`).
  - `release`: Aplica otimizações extremas via LLVM, Link-Time Optimization (**LTO**), compilação com unidade única de geração de código (`codegen-units = 1`), terminação imediata em caso de pânico (`panic = "abort"`) e remoção de símbolos de debug (**strip**).
  - *Demonstração prática neste repositório:* O binário passa de **4.3 MB** em modo debug para apenas **300 KB** em modo release (uma redução de mais de 93% no tamanho do executável).

### 1.5. Automação dos Testes
O comando `cargo test` unifica nativamente três modalidades de testes com execução paralela:
1. **Testes Unitários:** Declarados dentro dos próprios arquivos fonte através do módulo `#[cfg(test)] mod tests`. Possuem acesso a funções privadas.
2. **Testes de Integração:** Localizados na pasta `tests/`. O Cargo compila cada arquivo `.rs` como uma crate separada que consome a biblioteca externamente, testando sua API pública.
3. **Doc-Tests (Testes de Documentação):** O Cargo extrai os blocos de código presentes nos comentários de documentação `///` e `//!`, compila-os e os executa. Se um exemplo da documentação quebrar, o build falha imediatamente.

### 1.6. Emissão de Relatórios
* **Diagnósticos Ricos do Compilador:** O Cargo e o `rustc` são referências mundiais em mensagens de erro com cores, contexto de código, sugestões de correção automáticas (`cargo fix`) e códigos explicativos (`--explain E0382`).
* **Linter Oficial (`cargo clippy`):** Detecta antipatterns, código não idiomático e oportunidades de otimização de desempenho.
* **Auditoria de Dependências e Licenças (`cargo deny`):** Audita a árvore de dependências em busca de vulnerabilidades de segurança (RUSTSEC advisories), conformidade de licenças corporativas (ex: rejeitar GPL em projetos fechados) e bloqueio de dependências duplicadas.
* **Cobertura de Código (`cargo llvm-cov`):** Geração de relatórios de cobertura de código linha a linha.
* **Microbenchmarks (`cargo bench`):** Medição de performance em nanosegundos com isolamento de cache.

### 1.7. Geração de Documentação
* O comando `cargo doc --workspace --no-deps` compila toda a documentação da base de código em um portal estático HTML/CSS/JS autocontido com mecanismo de busca offline integrado.
* Suporte nativo completo a Markdown, renderização de código com destaque de sintaxe e hiperlinks automáticos entre tipos e métodos de crates distintas.

---

## 2. Aspectos Cruciais do Cargo (Seção 3 do Enunciado)

### 2.1. Facilidade de Configuração
Ao contrário do Apache Maven (que exige arquivos XML volumosos) ou do GNU Make (que exige regras imperativas de compilação manuais), o Cargo adota arquivos `Cargo.toml`. O formato TOML (*Tom's Obvious Minimal Language*) é legível, conciso e fortemente tipado.

### 2.2. Eficiência de Compilação
A compilação do Rust é intensiva devido às garantias estáticas de *memory safety* em tempo de compilação. Para mitigar o custo computacional:
- O Cargo utiliza paralelização em nível de pacote e compilação incremental no nível de funções do `rustc`.
- Suporta integração transparente com caches remotos/locais como o `sccache` (desenvolvido pela Mozilla).
- Fornece o comando `cargo check` para iterações rápidas durante o desenvolvimento.

### 2.3. Gerenciamento de Dependências
- **Ecossistema Crates.io:** Repositório oficial que centraliza mais de 160.000 pacotes de código aberto.
- **Resolução SemVer Estrita:** Garante que atualizações menores não introduzam incompatibilidades de API.
- **Feature Flags:** O Cargo permite ativar ou desativar partes do código de uma dependência através de *features condicionais*, evitando carregar dependências desnecessárias (ex: suporte a JSON, criptografia, multithreading).
- **Inspectability (`cargo tree` e `cargo metadata`):** Permite inspecionar a árvore de dependências em texto ou em formato JSON estruturado para ferramentas automatizadas e IDEs.

### 2.4. Suporte a Multiplataformas (Cross-Compilation)
O Cargo e o compilador Rust foram desenhados nativamente com suporte a *Target Triples* no formato `<arquitetura>-<fabricante>-<sistema_operacional>-<abi>` (ex: `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc`, `wasm32-unknown-unknown`).
- Compilar para outra plataforma exige apenas indicar o alvo: `cargo build --target <triple>`.
- Permite uso de diretivas condicionais no código: `#[cfg(target_os = "windows")]` ou `#[cfg(target_arch = "aarch64")]`.

### 2.5. Automação de Tarefas e Scripts de Compilação (`build.rs`)
Para tarefas customizadas de compilação, o Cargo permite criar um arquivo `build.rs` na raiz da crate:
- É compilado e executado pelo Cargo *antes* do pacote principal.
- Comunica-se com o Cargo via comandos emitidos na saída padrão (`stdout`), tais como `cargo::rustc-env=NOME=VALOR` ou `cargo::rerun-if-changed=caminho`.
- Comumente utilizado para conectar código C/C++ legado via FFI, gerar código fonte dinâmico (Protocol Buffers, schemas SQL) ou injetar metadados de compilação (hash git, data/hora).

### 2.6. Extensibilidade via Subcomandos da CLI
O Cargo possui uma arquitetura de plugins transparente baseada no sistema operacional:
- Qualquer executável presente no `PATH` que comece com o prefixo `cargo-<nome>` pode ser invocado diretamente pelo usuário como `cargo <nome>`.
- Isso permitiu que a comunidade e a Rust Foundation expandissem o Cargo sem inflar a ferramenta básica:
  - `cargo-clippy`: Linter oficial de código.
  - `cargo-deny`: Auditoria de segurança e licenças.
  - `cargo-watch`: Monitoramento de arquivos com recompilação automática (*hot-reload*).
  - `cargo-llvm-cov`: Relatórios de cobertura de testes.
  - `cargo-resumo`: Subcomando customizado criado neste trabalho.

### 2.7. Comunidade e Suporte
- **Documentação Oficial:** *The Cargo Book*, documentação canônica de referência mantida pela Rust Foundation.
- **Docs.rs:** Servidor central mantido pela comunidade que compila e hospeda automaticamente a documentação HTML de absolutamente qualquer crate publicada no crates.io.
- **Fóruns:** Comunidade altamente ativa no Rust Users Forum, Discord e Reddit (`r/rust`).

---

## 3. Matriz Comparativa: Cargo vs. Outras Ferramentas de Build

| Critério de Avaliação | Cargo (Rust) | Apache Maven (Java) | Gradle (JVM/Kotlin) | Bazel (Google/Polyglot) | GNU Make (C/C++) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Linguagens Suportadas** | Especializado em Rust (com C/C++ via FFI) | Primariamente Java | Java, Kotlin, Groovy, C++, Android | Poliglota universal (C++, Java, Rust, Go, Python) | Agnóstico a linguagens (foco em C/C++) |
| **Formato de Configuração** | `Cargo.toml` (TOML declarativo e limpo) | `pom.xml` (XML verboso e hierárquico) | `build.gradle.kts` (DSL em Kotlin/Groovy) | `BUILD` (Starlark, dialeto declarativo de Python) | `Makefile` (Imperativo com sintaxe rígida de tabs) |
| **Resolução de Dependências** | Determinística com `Cargo.lock` e SemVer estrito | `Maven Central`, propenso a conflitos transitivos | Flexível com version catalogs e dynamic constraints | Explícita e isolada via regras herméticas | Não possui nativamente (depende do SO/pacotes externos) |
| **Controle de Hermeticidade** | Alto no código Rust; dependente do host em `build.rs` | Médio (depende da JVM e do repositório local `~/.m2`) | Médio/Alto (depende do isolamento de scripts) | **Extremo** (Execução 100% isolada em sandbox de filesystem) | Baixo (vaza variáveis de ambiente do host facilmente) |
| **Testes e Documentação** | Integrados nativamente (`cargo test`, `cargo doc`, doc-tests) | Via plugins (`maven-surefire`, `maven-javadoc`) | Via tarefas configuradas no build script | Via regras de teste declaradas explicitamente | Depende de ferramentas e receitas externas |
| **Extensibilidade** | Scripts `build.rs` + Subcomandos no `PATH` (`cargo-*`) | Plugins Java com ciclo de vida amarrado ao POM | Scripts e plugins Groovy/Kotlin de alta complexidade | Regras customizadas em Starlark (`rules_*`) | Shell scripts e chamadas a comandos de terminal |

---

## 4. Vantagens e Desvantagens do Cargo

### Pontos Positivos (Vantagens)
1. **Padronização Universal:** Praticamente 100% dos projetos em Rust no mundo utilizam Cargo. Não há fragmentação do ecossistema.
2. **Determinismo e Segurança de Dependências:** O `Cargo.lock` associado a hashes SHA-256 e ao SemVer garante reprodutibilidade estrita.
3. **Doc-Tests Revolucionários:** Os exemplos presentes na documentação são testados no pipeline de testes, eliminando o problema de documentação desatualizada.
4. **Otimizações Transparentes:** Facilidade de alternar entre modo de desenvolvimento ágil e binários de produção ultra-otimizados (`release` com LTO).
5. **Arquitetura de Plugins Leve:** Extensão sem atritos através de executáveis com prefixo `cargo-`.

### Pontos Negativos (Desvantagens)
1. **Foco Monolíngue:** Não foi projetado para ser um sistema de compilação poliglota corporativo único (empresas como Google e Meta utilizam Bazel ou Buck2 para orquestrar múltiplos repositórios com Rust, C++ e Java).
2. **Tempo de Compilação Inicial (*Cold Build*):** Projetos com muitas dependências externas podem demorar no primeiro build devido à compilação e otimização exaustiva das bibliotecas pelo `rustc`.
3. **Volume da Pasta `target/`:** A compilação incremental e a preservação de múltiplos perfis consomem volumes significativos de disco, exigindo limpezas periódicas (`cargo clean`).
4. **Hermeticidade Imperfeita com `build.rs`:** Como o script `build.rs` executa código arbitrário no host, ele pode ler arquivos não declarados ou depender de utilitários locais do sistema operacional.

---

## 5. Como Executar e Demonstrar o Projeto

### Pré-requisitos
- Compilador Rust e Cargo instalados (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`).

### 5.1. Demonstração Rápida (Script Interativo para o Seminário)
Para reproduzir a demonstração completa de 10 minutos com pausas explicativas a cada etapa:
```bash
bash scripts/demo.sh
```
*(Ou use `bash scripts/demo.sh --auto` para execução sequencial automática sem pausas).*

### 5.2. Comandos Individuais para Apresentação em Aula

1. **Verificação Rápida de Tipos (sem codegen):**
   ```bash
   cargo check --workspace
   ```

2. **Execução de Todos os Testes (Unitários, Integração e Doc-Tests):**
   ```bash
   cargo test --workspace
   ```

3. **Execução da Aplicação CLI com Features Condicionais:**
   ```bash
   # Modo padrão
   cargo run -p app_cli
   
   # Modo com Features (Criptografia e Matemática Acelerada)
   cargo run -p app_cli --features "crypto,fastmath"
   ```

4. **Comparação de Tamanho dos Binários (Debug vs Release com LTO/Strip):**
   ```bash
   cargo build -p app_cli --release
   ls -lh target/debug/app_cli target/release/app_cli
   ```

5. **Inspeção do Grafo de Dependências e AST JSON:**
   ```bash
   cargo tree
   cargo metadata --format-version 1 --no-deps
   ```

6. **Geração do Portal de Documentação HTML:**
   ```bash
   cargo doc --workspace --no-deps --open
   ```

7. **Execução do Subcomando Customizado:**
   ```bash
   PATH="$PWD/target/debug:$PATH" cargo resumo
   ```

8. **Auditoria de Licenças e Governança:**
   ```bash
   cargo deny check licenses bans
   ```

---

## 6. Estrutura de Arquivos do Repositório

```
build/
├── .cargo/
│   └── config.toml             # Aliases (cargo demo, fast, etc.) e flags locais
├── Cargo.toml                  # Workspace Root (gerencia membros e perfis compartilhados)
├── Cargo.lock                  # Lockfile com hashes exatos de todas as dependências
├── deny.toml                   # Políticas de auditoria de licenças e segurança (cargo-deny)
├── .gitignore                  # Ignora artefatos temporários e a pasta target/
│
├── crates/
│   ├── core_calc/              # [LIB] Biblioteca central
│   │   ├── Cargo.toml          # Declaração de feature flags e metadados herdados
│   │   ├── benches/
│   │   │   └── benchmark.rs    # Microbenchmark de operações matemáticas
│   │   └── src/
│   │       └── lib.rs          # Funções com doc-tests executáveis e testes unitários
│   │
│   ├── app_cli/                # [BIN] Aplicação de linha de comando
│   │   ├── Cargo.toml          # Dependência local e ativação de build.rs
│   │   ├── build.rs            # Script de build que injeta timestamp e git hash em compilação
│   │   ├── examples/
│   │   │   └── basic_usage.rs  # Exemplo executável via cargo run --example
│   │   ├── tests/
│   │   │   └── integration.rs  # Testes de integração de caixa-preta
│   │   └── src/
│   │       └── main.rs         # Consome o build.rs e a biblioteca core_calc
│   │
│   └── cargo-resumo/           # [SUBCOMANDO] Plugin customizado
│       ├── Cargo.toml
│       └── src/
│           └── main.rs         # Binário invocado como 'cargo resumo' via PATH
│
├── scripts/
│   └── demo.sh                 # Script Bash interativo com cores para a live demo de 10 min
├── ROTEIRO_10MIN.md            # Roteiro minuto a minuto com sugestão de fala para a apresentação
└── README.md                   # Este relatório técnico completo
```
