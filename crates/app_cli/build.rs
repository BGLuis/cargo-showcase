//! Script de compilação customizado executado pelo Cargo antes de compilar o pacote `app_cli`.
//!
//! Demonstra o poder do `build.rs` para:
//! 1. Injetar variáveis de ambiente em tempo de compilação (`cargo::rustc-env=KEY=VALUE`).
//! 2. Controlar a invalidação de cache incremental (`cargo::rerun-if-changed=...`).
//! 3. Inspecionar o sistema operacional, arquitetura e estado do repositório Git.

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. Avisa o Cargo para só reexecutar este script se o próprio build.rs for alterado
    println!("cargo::rerun-if-changed=build.rs");

    // 2. Calcula timestamp do momento exato do build
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo::rustc-env=BUILD_TIMESTAMP_SECS={}", now);

    // 3. Captura informações de arquitetura e plataforma disponibilizadas pelo Cargo
    let target = std::env::var("TARGET").unwrap_or_else(|_| "desconhecido".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "desconhecido".to_string());
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());

    println!("cargo::rustc-env=BUILD_TARGET={}", target);
    println!("cargo::rustc-env=BUILD_HOST={}", host);
    println!("cargo::rustc-env=BUILD_PROFILE={}", profile);

    // 4. Tenta capturar o hash do commit git atual
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "git-desconhecido".to_string());

    println!("cargo::rustc-env=BUILD_GIT_HASH={}", git_hash);
}
