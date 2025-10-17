use crate::analisador_lexico::próximo;

mod analisador_lexico;

/*  Exemplos
    450 + 20
    ("450", 1) ("+", 5) ("20", 7)
    450     +     20
    ("450", 1) ("+", 9) ("20", 15)
    450+20
    ("450", 1) ("+", 4) ("20", 5)
    0+-0
    ("0", 1) ("+", 2) ("-", 3) ("0", 4)
    0 +++
    ("0", 1) ("+", 3) ("+", 4) ("+", 5)
    10+a
    ("10", 1) ("+", 3) Erro na posição 4
    10 + 20a
    ("10", 1) ("+", 4) ("20", 6) Erro na posição 8
    🦀🦀🦀30🦀🦀*    25  🐧/-+*30
    ("30", 4) ("*", 8) ("25", 13) ("🐧", 17) ("/", 18) ("-", 19) ("+", 20) ("*", 21) ("30", 22)
    4 5 0 + 2 3
    ("4 5 0", 1) ("+", 7) ("2 3", 9)
*/

fn main() {
    let entradas = [
        "450 + 20",
        "450     +     20",
        "450+20",
        "0+-0",
        "0 +++",
        "10+a",
        "10 + 20a",
        "🦀🦀🦀30🦀🦀*    25  🐧/-+*30",
        "4 5 0 + 2 3"
    ];

    for mut entrada_atual in entradas {
        println!("Analisando a entrada {}", entrada_atual);
        let mut result: Result<(usize, &str, &str), Option<usize>> = próximo(entrada_atual);

        let mut caracateres_ja_contados: usize = 0;

        while let Ok((pos, conteudo, restante)) = result {
            print!("(\"{}\", {}) ", conteudo, caracateres_ja_contados + pos);

            let caracteres_pulados = pos - 1;
            let caracteres_no_conteudo = conteudo.chars().count();
            let total_consumido_nesta_iteracao = caracteres_pulados + caracteres_no_conteudo;

            caracateres_ja_contados += total_consumido_nesta_iteracao;

            entrada_atual = restante;
            result = próximo(entrada_atual);
        }

        if let Err(Some(pos)) = result {
            print!("Erro na posição {}", caracateres_ja_contados + pos);
        }

        println!();
    }
}
