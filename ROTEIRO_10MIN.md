# ⏱️ Roteiro da Apresentação de 10 Minutos: Seminário Cargo (Rust)

Este documento foi elaborado para garantir que o grupo apresente todos os tópicos exigidos no enunciado do seminário com precisão, clareza e controle rigoroso do tempo.

---

## 📊 Estrutura de Tempo (Visão Geral)

| Intervalo | Etapa | Foco | Ação na Tela |
| :---: | :--- | :--- | :--- |
| **00:00 - 02:00** | **1. Introdução & Arquitetura** | O que é o Cargo, filosofia, convenção sobre configuração e `Cargo.toml`. | Slide inicial / `cat Cargo.toml` |
| **02:00 - 04:00** | **2. Ciclo de Vida & Recompilação Mínima** | Dependências SemVer, determinismo (`Cargo.lock`), `cargo check` e DAG. | `cargo check` vs `cargo build` |
| **04:00 - 07:00** | **3. Live Demo (Execução Prática)** | Testes (unit/integ/doc-tests), Features, `build.rs` e análise de artefatos. | Executar `bash scripts/demo.sh` |
| **07:00 - 08:30** | **4. Recursos Avançados & Comparativo** | `cargo metadata`, Workspaces e Matriz (Cargo vs Maven/Gradle/Bazel). | Slide de comparação / `cargo tree` |
| **08:30 - 10:00** | **5. Vantagens, Limitações & Conclusão** | Prós e contras na indústria, tomada de decisão e abertura para perguntas. | Slide final / Q&A |

---

## 🎙️ Roteiro Minuto a Minuto (Falas Recomendadas)

### [00:00 - 02:00] Bloco 1: Introdução, Filosofia e Configuração
> **O que falar:**
> *"Boa noite a todos e professor Diego. Hoje vamos apresentar o **Cargo**, a ferramenta oficial de automação de build e gerenciador de pacotes da linguagem Rust.
>
> Enquanto ferramentas clássicas como o Make exigiam scripts imperativos complexos e o Maven utilizava arquivos XML extremamente verbosos, o Cargo adota o paradigma de **Convenção sobre Configuração**.
> 
> Ele utiliza um arquivo declarativo simples chamado `Cargo.toml`, onde definimos metadados, dependências e perfis. Se você organizar seus arquivos na pasta `src/`, `tests/`, `examples/` ou `benches/`, o Cargo infere automaticamente o tipo de artefato (binário executável ou biblioteca) sem que você precise escrever uma única regra manual de compilação."*

---

### [02:00 - 04:00] Bloco 2: Gestão de Dependências e Recompilação Mínima
> **O que falar:**
> *"No aspecto de gerenciamento de dependências, o Cargo integra-se nativamente ao repositório central `crates.io`. Ele resolve conflitos com base em Versionamento Semântico (SemVer) e gera o arquivo `Cargo.lock`.
> 
> O `Cargo.lock` garante que qualquer membro da equipe ou servidor de CI construa exatamente os mesmos bytes, com hashes criptográficos validados, eliminando o clássico problema do 'na minha máquina funciona'.
>
> Para a eficiência de compilação, o Cargo constrói um DAG (Grafo Acíclico Dirigido) de dependências e calcula *fingerprints* de cada arquivo. Além disso, ele possui o comando `cargo check`, que executa a análise sintática, verificação de tipos e o famoso *Borrow Checker* do Rust em milissegundos, sem gastar tempo gerando código de máquina no disco."*

---

### [04:00 - 07:00] Bloco 3: Demonstração Prática ao Vivo (Live Demo)
> **Ação:** No terminal, execute:
> ```bash
> bash scripts/demo.sh
> ```
> *(O script é pausado a cada passo com `[ENTER]`. Conduza os passos conforme abaixo:)*
>
> - **Passo 1 (Workspace):** Mostre o `Cargo.toml` raiz unificando múltiplos pacotes.
> - **Passo 2 (`cargo check`):** Mostre que a verificação leva menos de 0.1 segundo.
> - **Passo 3 (`cargo test`):** **Destaque os Doc-Tests!** Diga: *"Vejam que o Cargo não testa apenas arquivos de teste separados: ele compila os blocos de código que estão dentro dos comentários de documentação `///`, garantindo que a documentação nunca fique defasada."*
> - **Passo 4 (Feature Flags):** Execute com e sem `--features crypto` e mostre que código não solicitado não é embutido no executável.
> - **Passo 5 (`build.rs`):** Diga: *"O `build.rs` é o script de pré-compilação do Cargo. Ele executa código Rust antes do build para injetar metadados dinâmicos (como versão do git e timestamp) ou compilar bibliotecas C."*
> - **Passo 6 (Debug vs Release):** **Momento de grande impacto visual!** Mostre a saída do `ls -lh`:
>   - `debug`: ~4.3 MB (símbolos e checagens ativas).
>   - `release`: ~300 KB (otimização agressiva `opt-level=3`, LTO e *strip* de símbolos). Redução de mais de 90%!
> - **Passo 7 (`cargo tree` & `cargo metadata`):** Mostre a árvore de dependências e o motor JSON.
> - **Passo 8 (`cargo doc`):** Mostre a geração do portal de documentação HTML.
> - **Passo 9 (Extensibilidade):** Mostre o comando `cargo resumo` rodando nativamente como subcomando via binário do PATH.

---

### [07:00 - 08:30] Bloco 4: Comparativo com Outras Ferramentas do Enunciado
> **O que falar:**
> *"Comparando o Cargo com as outras ferramentas do nosso enunciado:
>
> 1. **Contra o Maven:** O Cargo evita o 'classpath hell' de dependências transitivas e adota TOML no lugar de XML.
> 2. **Contra o Gradle:** O Gradle oferece um DSL Turing-completo (Groovy/Kotlin) com muita flexibilidade, mas que pode tornar os builds imprevisíveis e lentos. O Cargo restringe a automação a ciclos padronizados.
> 3. **Contra o Bazel:** O Bazel é o padrão para monorepos poliglotas massivos e garante hermeticidade absoluta (sandbox). O Cargo é especializado em Rust; seu script `build.rs` permite grande flexibilidade com código C e ambiente host, mas sacrifica parte dessa hermeticidade estrita.
> 4. **Contra o Make:** O Cargo gerencia dependências remotas, testes e documentação nativamente, enquanto o Make gerencia apenas arquivos locais e timestamps."*

---

### [08:30 - 10:00] Bloco 5: Prós, Contras e Conclusão
> **O que falar:**
> *"Como vantagens decisivas para uso em projetos reais:
> - Ecossistema 100% unificado: build, testes, linters, documentação e auditoria usam a mesma ferramenta.
> - Garantia de reprodutibilidade via `Cargo.lock`.
> - Otimizações de ponta (LTO, PGO, Cross-compilation facilitada com `--target`).
>
> Como limitações:
> - Focado exclusivamente em Rust (para projetos com dezenas de linguagens, Bazel costuma ser adotado no topo).
> - Builds frios (*clean build*) podem demorar devido à intensidade da otimização do compilador `rustc`.
> - A pasta `target/` pode crescer significativamente se não for limpa periodicamente (`cargo clean`).
>
> Em conclusão, o Cargo é considerado hoje um dos maiores referenciais de engenharia de ferramentas de build da indústria moderna. Estamos abertos para perguntas. Obrigado!"*

---

## ❓ Possíveis Perguntas da Plateia / Professor e Como Responder

1. **"O que acontece se duas dependências usarem versões diferentes de uma mesma biblioteca?"**
   - *Resposta:* O Cargo permite compilar múltiplas versões da mesma biblioteca simultaneamente (ex: `v0.4` e `v0.5`), desde que não haja conflito de tipos públicos no mesmo escopo, evitando conflitos que seriam fatais no Java/Maven.
2. **"O Cargo faz cross-compilation (compilação cruzada)?"**
   - *Resposta:* Sim! Basta instalar a target triple com `rustup target add <alvo>` e rodar `cargo build --target <alvo>`.
3. **"O que são os Doc-tests e por que eles são importantes?"**
   - *Resposta:* São testes colocados nos comentários de documentação da biblioteca. O Cargo extrai os blocos markdown ` ``` `, compila e roda no `cargo test`. Se a documentação tiver um exemplo que não compila mais, o build/CI quebra imediatamente.
