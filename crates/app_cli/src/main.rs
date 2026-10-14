fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║       DEMONSTRAÇÃO DE CAPACIDADES DO CARGO (RUST)              ║");
    println!("║       Seminário de Ferramentas de Build - PUC Minas            ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    // Exibição dos metadados injetados em tempo de compilação pelo build.rs
    println!("\n📦 [METADADOS INJETADOS PELO BUILD.RS]:");
    println!("   • Target Triple : {}", env!("BUILD_TARGET"));
    println!("   • Host Triple   : {}", env!("BUILD_HOST"));
    println!("   • Perfil Build  : {}", env!("BUILD_PROFILE"));
    println!("   • Git Commit    : {}", env!("BUILD_GIT_HASH"));
    println!("   • Build Time (s): {}", env!("BUILD_TIMESTAMP_SECS"));

    // Exibição das features ativadas
    println!("\n⚙️  [FEATURES ATIVAS NO CRATE CORE_CALC]:");
    for feat in core_calc::features_ativas() {
        println!("   • {}", feat);
    }

    // Execução de operações da biblioteca do workspace
    println!("\n🧮 [EXECUÇÃO DE OPERAÇÕES DO CORE_CALC]:");
    let s = core_calc::somar(40, 2);
    let m = core_calc::multiplicar(6, 7);
    let f = core_calc::fatorial(5);
    println!("   • somar(40, 2)       = {}", s);
    println!("   • multiplicar(6, 7)  = {}", m);
    println!("   • fatorial(5)        = {}", f);

    #[cfg(feature = "crypto")]
    {
        let hash = core_calc::gerar_hash("Demonstração Cargo");
        println!("   • [Feature crypto] gerar_hash(\"Demonstração Cargo\") = 0x{:016X}", hash);
    }

    #[cfg(feature = "fastmath")]
    {
        let fast_sum = core_calc::soma_intervalo_rapida(100);
        println!("   • [Feature fastmath] soma_intervalo_rapida(100) = {}", fast_sum);
    }

    println!("\n✨ Execução concluída com sucesso via Cargo!\n");
}
