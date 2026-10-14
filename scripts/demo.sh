#!/usr/bin/env bash
# ==============================================================================
# Script de Demonstração ao Vivo do Cargo (Rust)
# Seminário de Ferramentas de Build - PUC Minas
# Professor: Diego Augusto de Faria Barros
# ==============================================================================

set -e

# Cores para terminal
BOLD="\033[1m"
GREEN="\033[1;32m"
BLUE="\033[1;34m"
CYAN="\033[1;36m"
YELLOW="\033[1;33m"
MAGENTA="\033[1;35m"
RESET="\033[0m"

AUTO_MODE=false
if [[ "$1" == "--auto" ]]; then
    AUTO_MODE=true
fi

pause() {
    if [ "$AUTO_MODE" = false ]; then
        echo -e "\n${YELLOW}▶ Pressione [ENTER] para avançar ao próximo passo...${RESET}"
        read -r
    else
        echo ""
        sleep 1
    fi
}

header() {
    clear 2>/dev/null || true
    echo -e "${MAGENTA}╔════════════════════════════════════════════════════════════════════════╗${RESET}"
    echo -e "${MAGENTA}║      SEMINÁRIO: FERRAMENTAS DE BUILD - CARGO (LINGUAGEM RUST)         ║${RESET}"
    echo -e "${MAGENTA}║      Demonstração Prática ao Vivo (Tempo Estimado: 10 Minutos)        ║${RESET}"
    echo -e "${MAGENTA}╚════════════════════════════════════════════════════════════════════════╝${RESET}"
    echo ""
}

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 1/9] Estrutura Declarativa do Cargo (Convenção sobre Configuração)${RESET}"
echo -e "Diferente de Makefiles complexos ou XMLs verbosos (Maven), o Cargo adota:"
echo -e "  • ${BOLD}Cargo.toml${RESET}: manifesto declarativo limpo em TOML."
echo -e "  • ${BOLD}Workspaces${RESET}: monorepos modulares (bibliotecas, binários, plugins)."
echo -e "  • ${BOLD}Cargo.lock${RESET}: garantia matemática de builds reproduzíveis."
echo ""
echo -e "${BLUE}$ cat Cargo.toml${RESET}"
cat Cargo.toml
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 2/9] Recompilação Mínima e Verificação com 'cargo check'${RESET}"
echo -e "O Cargo implementa análise de tipos e borrow checking instantânea sem"
echo -e "precisar gastar tempo gerando código de máquina (codegen LLVM)."
echo ""
echo -e "${BLUE}$ cargo check --workspace${RESET}"
cargo check --workspace
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 3/9] Automação Completa de Testes (Unitários, Integração e Doc-Tests)${RESET}"
echo -e "O comando ${BOLD}cargo test${RESET} executa 3 níveis de teste em paralelo nativamente:"
echo -e "  1. Testes Unitários internos (em src/lib.rs)"
echo -e "  2. Testes de Integração de caixa-preta (na pasta tests/)"
echo -e "  3. ${BOLD}Doc-Tests:${RESET} exemplos de código dentro de comentários '///' da documentação!"
echo ""
echo -e "${BLUE}$ cargo test --workspace${RESET}"
cargo test --workspace
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 4/9] Compilação Condicional por Feature Flags${RESET}"
echo -e "O Cargo permite compilar apenas o que for necessário, reduzindo consumo de memória."
echo -e "Vamos executar o app CLI com e sem a feature 'crypto' e 'fastmath':"
echo ""
echo -e "${YELLOW}>> 1. Executando em modo básico padrão:${RESET}"
echo -e "${BLUE}$ cargo run -p app_cli --quiet${RESET}"
cargo run -p app_cli --quiet

echo -e "\n${YELLOW}>> 2. Executando com features adicionais ativadas dinamicamente:${RESET}"
echo -e "${BLUE}$ cargo run -p app_cli --features \"crypto,fastmath\" --quiet${RESET}"
cargo run -p app_cli --features "crypto,fastmath" --quiet
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 5/9] Script de Build Avançado (build.rs)${RESET}"
echo -e "O arquivo ${BOLD}build.rs${RESET} permite executar código Rust arbitrário em tempo de compilação,"
echo -e "injetando variáveis de ambiente, detectando o sistema host ou compilando C/C++ FFI."
echo ""
echo -e "${BLUE}$ cat crates/app_cli/build.rs | head -n 25${RESET}"
head -n 25 crates/app_cli/build.rs
echo -e "\n${GREEN}Observe como as variáveis BUILD_TARGET e BUILD_TIMESTAMP foram capturadas!${RESET}"
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 6/9] Perfis de Compilação e Otimização Extrema (Debug vs Release)${RESET}"
echo -e "O Cargo possui perfis com propósitos distintos:"
echo -e "  • ${BOLD}dev (debug):${RESET} compilação rápida, símbolos de debug, sem otimização."
echo -e "  • ${BOLD}release:${RESET} LTO (Link-Time Optimization), codegen-units=1 e strip de símbolos."
echo ""
echo -e "${BLUE}$ ls -lh target/debug/app_cli target/release/app_cli${RESET}"
ls -lh target/debug/app_cli target/release/app_cli
echo ""
echo -e "${GREEN}👉 Redução dramática no tamanho do binário final (~4.3MB -> ~300KB)!${RESET}"
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 7/9] Inspectability com 'cargo tree' e 'cargo metadata'${RESET}"
echo -e "O Cargo expõe o grafo completo de dependências tanto visualmente quanto em JSON:"
echo ""
echo -e "${YELLOW}>> Visualização da Árvore de Dependências:${RESET}"
echo -e "${BLUE}$ cargo tree${RESET}"
cargo tree

echo -e "\n${YELLOW}>> Inspecionando o motor de AST/JSON do Workspace via 'cargo metadata':${RESET}"
echo -e "${BLUE}$ cargo metadata --format-version 1 --no-deps | cut -c 1-140${RESET}"
cargo metadata --format-version 1 --no-deps | cut -c 1-140
echo -e "\n${GREEN}IDEs como o Rust-Analyzer utilizam essa API interna para fornecer autocomplete e análise em tempo real.${RESET}"
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 8/9] Geração Automática de Documentação Estática (cargo doc)${RESET}"
echo -e "O Cargo gera um portal de documentação HTML completo a partir do código:"
echo ""
echo -e "${BLUE}$ cargo doc --workspace --no-deps${RESET}"
cargo doc --workspace --no-deps
echo ""
echo -e "${GREEN}✅ Documentação gerada em: target/doc/core_calc/index.html${RESET}"
pause

# ------------------------------------------------------------------------------
header
echo -e "${CYAN}${BOLD}[PASSO 9/9] Extensibilidade do Cargo via Subcomandos Próprios${RESET}"
echo -e "Qualquer binário no PATH com nome ${BOLD}cargo-<nome>${RESET} torna-se um subcomando oficial!"
echo -e "Demonstração do nosso plugin customizado compilado no workspace:"
echo ""
echo -e "${BLUE}$ PATH=\"\$PWD/target/debug:\$PATH\" cargo resumo${RESET}"
PATH="$PWD/target/debug:$PATH" cargo resumo
echo ""
echo -e "${GREEN}${BOLD}🎉 Fim da Demonstração Prática do Cargo!${RESET}"
echo -e "O repositório cobriu as 7 características essenciais e todos os aspectos do seminário."
