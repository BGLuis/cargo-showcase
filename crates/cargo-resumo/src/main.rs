//! Subcomando customizado para o Cargo.
//!
//! O Cargo possui uma arquitetura de plugins transparente:
//! Se existir um executável no seu PATH chamado `cargo-algo`, executar `cargo algo`
//! fará com que o Cargo repasse a chamada diretamente para esse executável!

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // Se invocado via `cargo resumo`, o Cargo passa "resumo" como primeiro argumento
    if args.len() > 1 && args[1] == "resumo" {
        args.remove(1);
    }

    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║       SUBCOMANDO EXTENSÍVEL DO CARGO: 'cargo resumo'                 ║");
    println!("║       Demonstração de Extensibilidade e Arquitetura de Plugins       ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");
    println!("\n🔍 Inspecionando o workspace atual via Cargo Plugin...\n");

    if Path::new("Cargo.toml").exists() {
        println!("✅ Cargo.toml raiz encontrado!");
        if let Ok(conteudo) = fs::read_to_string("Cargo.toml") {
            println!("\n📄 Membros do Workspace identificados:");
            for linha in conteudo.lines() {
                if linha.trim().starts_with('"') && linha.contains("crates/") {
                    println!("   👉 {}", linha.trim().trim_matches(','));
                }
            }
        }
    } else {
        println!("⚠️ Nenhum Cargo.toml encontrado no diretório atual.");
    }

    println!("\n💡 Como funciona este recurso?");
    println!("   1. O Cargo procura no PATH por qualquer binário com prefixo 'cargo-'");
    println!("   2. 'cargo <comando>' chama transparentemente 'cargo-<comando>'");
    println!("   3. Isso permitiu à comunidade criar ferramentas consagradas como:");
    println!("      • cargo-clippy   (Linter oficial)");
    println!("      • cargo-llvm-cov (Relatórios de cobertura)");
    println!("      • cargo-deny     (Auditoria de segurança e licenças)");
    println!("      • cargo-watch    (Hot-reloading para desenvolvimento)");
    println!("      • cargo-resumo   (Este nosso plugin customizado!)\n");
}
