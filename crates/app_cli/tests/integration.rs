//! Teste de integração de caixa preta executado pelo Cargo via pasta `tests/`.
//!
//! Para rodar apenas este teste:
//!   cargo test -p app_cli --test integration

#[test]
fn test_fluxo_integracao_calculos() {
    let a = 15;
    let b = 25;
    let soma = core_calc::somar(a, b);
    let mult = core_calc::multiplicar(soma, 2);

    assert_eq!(soma, 40);
    assert_eq!(mult, 80);
}

#[test]
fn test_fatorial_integrado() {
    assert_eq!(core_calc::fatorial(4), 24);
}
