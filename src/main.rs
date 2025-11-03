/*  Exemplos
    10 + 20
    +
    ├ 10
    └ 20
    Resultado: Some(30)

    10 / 0
    /
    ├ 10
    └ 0
    Resultado: None

    (10 + 20) * 30
    *
    ├ +
    │ ├ 10
    │ └ 20
    └ 30
    Resultado: Some(900)

    10 + 20 * 30
    +
    ├ 10
    └ *
      ├ 20
      └ 30
    Resultado: Some(610)

    (-(10 + 20) + 30 + 40 + (50 + 60)) * -5
    *
    ├ +
    │ ├ +
    │ │ ├ +
    │ │ │ ├ -
    │ │ │ │ └ +
    │ │ │ │ ├ 10
    │ │ │ │ └ 20
    │ │ │ └ 30
    │ │ └ 40
    │ └ +
    │ ├ 50
    │ └ 60
    └ -
    └ 5
    Resultado: Some(-750)
*/

use crate::arvore_sintatica::{parse_expressao, tokenizar};

mod arvore_sintatica;

fn main() {
    let entradas = [
        "10 + 20",
        "10 / 0",
        "(10 + 20) * 30",
        "10 + 20 * 30",
        "(-(10 + 20) + 30 + 40 + (50 + 60)) * -5",
        "-5 + 3",
        "10 * (20 / (5 + 5))",
        "-(-(-10))",
        "100 - 50 * 2 + 10 / 5",
        "(1 + 2) * (3 + 4)",
        "((1 + 2) * 3) + 4",
        "-(3 + 4) * 2",
        "5 + (-3) * 2",
        "((10))",
    ];

    for entrada_atual in entradas {
        println!("Analisando a entrada {}", entrada_atual);
        let tokens = tokenizar(entrada_atual);

        match tokens {
            Some(value) => match parse_expressao(&value) {
                Some(expressao) => {
                    print!("Expressão a partir dos valores do Enum: ");
                    expressao.0.imprimir();

                    println!("Árvore sintática da expressão:");
                    expressao.0.imprimir_árvore();

                    println!("Resultado: {:?}", expressao.0.avaliar());
                }
                None => {
                    println!("Erro ao fazer o parse de {entrada_atual}");
                }
            },
            None => {
                println!("Erro ao tokenizar {entrada_atual}");
            }
        }
        println!();
    }
}
