//! Exemplo demonstrativo do suporte nativo do Cargo para a pasta `examples/`.
//!
//! Para rodar este exemplo:
//!   cargo run -p app_cli --example basic_usage

fn main() {
    println!("=== Exemplo Rápido: Consumindo core_calc via Cargo Examples ===");
    let soma = core_calc::somar(100, 250);
    let fat = core_calc::fatorial(6);

    println!("Resultado da soma: {}", soma);
    println!("Resultado do fatorial: {}", fat);
    println!("Recurso 'examples/' do Cargo permite documentar APIs com exemplos executáveis reais!");
}
