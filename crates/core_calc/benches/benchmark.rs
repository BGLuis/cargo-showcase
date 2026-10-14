//! Microbenchmark simples demonstrando o suporte nativo do Cargo à pasta `benches/`.
//! Pode ser executado com `cargo bench -p core_calc`.

use std::time::Instant;

fn main() {
    println!("\n=== [BENCHMARK] Executando Microbenchmarks em core_calc ===");

    // Benchmark do Fatorial
    let inicio = Instant::now();
    let mut total = 0;
    for _ in 0..100_000 {
        total += core_calc::fatorial(15);
    }
    let duracao = inicio.elapsed();
    println!("⏱️  100.000 iterações de fatorial(15): {:?} (resultado acumulado: {})", duracao, total);

    // Benchmark de Multiplicação
    let inicio = Instant::now();
    let mut acc: i64 = 0;
    for i in 0..1_000_000 {
        acc = acc.wrapping_add(core_calc::multiplicar(i, 42));
    }
    let duracao = inicio.elapsed();
    println!("⏱️  1.000.000 multiplicações: {:?} (resultado acumulado: {})", duracao, acc);

    println!("=== [BENCHMARK] Concluído com sucesso! ===\n");
}
