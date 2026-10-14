//! # Core Calc
//!
//! Biblioteca de lógica matemática e processamento de dados criada para
//! demonstrar os recursos do sistema de compilação **Cargo**.
//!
//! ## Recursos Demonstrados:
//! - **Doc-Tests Integrados:** Exemplos na documentação que o Cargo compila e testa.
//! - **Compilação Condicional:** Lógica ativada seletivamente por `features`.
//! - **Zero Dependências Externas Obrigatórias:** Rápido, determinístico e auditável.

/// Soma dois números inteiros de 64 bits.
///
/// # Doc-Test (Executado pelo `cargo test`)
/// ```
/// use core_calc::somar;
///
/// let resultado = somar(10, 20);
/// assert_eq!(resultado, 30);
/// ```
pub fn somar(a: i64, b: i64) -> i64 {
    a + b
}

/// Multiplica dois números inteiros de 64 bits.
///
/// # Doc-Test
/// ```
/// use core_calc::multiplicar;
///
/// assert_eq!(multiplicar(4, 5), 20);
/// assert_eq!(multiplicar(-2, 3), -6);
/// ```
pub fn multiplicar(a: i64, b: i64) -> i64 {
    a * b
}

/// Calcula o fatorial de um número natural.
///
/// # Doc-Test
/// ```
/// use core_calc::fatorial;
///
/// assert_eq!(fatorial(0), 1);
/// assert_eq!(fatorial(5), 120);
/// ```
pub fn fatorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => (2..=n as u64).product(),
    }
}

/// Retorna uma lista de strings descrevendo as features ativadas nesta compilação.
///
/// Isso ilustra o uso de compilação condicional via diretiva `#[cfg(feature = "...")]`.
pub fn features_ativas() -> Vec<&'static str> {
    let mut ativas = Vec::new();

    #[cfg(feature = "crypto")]
    ativas.push("crypto (Hashing FNV-1a habilitado)");

    #[cfg(feature = "fastmath")]
    ativas.push("fastmath (Aritmética acelerada de Gauss habilitada)");

    if ativas.is_empty() {
        ativas.push("nenhuma feature opcional ativa (modo padrão básico)");
    }

    ativas
}

#[cfg(feature = "crypto")]
/// Gera um hash de 64 bits (algoritmo FNV-1a) para os dados fornecidos.
///
/// Disponível apenas quando a feature `crypto` for ativada no `Cargo.toml`
/// ou via linha de comando (`cargo run --features crypto`).
///
/// # Doc-Test (Executado apenas com a feature crypto)
/// ```
/// use core_calc::gerar_hash;
///
/// let hash = gerar_hash("Cargo é incrível");
/// assert_ne!(hash, 0);
/// ```
pub fn gerar_hash(texto: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in texto.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(feature = "fastmath")]
/// Soma todos os inteiros de 1 até `n` usando a fórmula de Gauss O(1).
///
/// Disponível apenas quando a feature `fastmath` for ativada.
///
/// # Doc-Test
/// ```
/// use core_calc::soma_intervalo_rapida;
///
/// assert_eq!(soma_intervalo_rapida(100), 5050);
/// ```
pub fn soma_intervalo_rapida(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soma_basica() {
        assert_eq!(somar(2, 2), 4);
        assert_eq!(somar(-5, 5), 0);
    }

    #[test]
    fn test_multiplicacao_basica() {
        assert_eq!(multiplicar(3, 7), 21);
        assert_eq!(multiplicar(0, 100), 0);
    }

    #[test]
    fn test_fatorial_valores_conhecidos() {
        assert_eq!(fatorial(0), 1);
        assert_eq!(fatorial(1), 1);
        assert_eq!(fatorial(3), 6);
        assert_eq!(fatorial(6), 720);
    }

    #[cfg(feature = "crypto")]
    #[test]
    fn test_hashing_fnv1a() {
        let h1 = gerar_hash("teste");
        let h2 = gerar_hash("teste");
        let h3 = gerar_hash("outro");
        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }

    #[cfg(feature = "fastmath")]
    #[test]
    fn test_gauss_vs_iterativo() {
        let n = 1000;
        let iterativo: u64 = (1..=n).sum();
        assert_eq!(soma_intervalo_rapida(n), iterativo);
    }
}
