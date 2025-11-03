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

/* Exemplo de implementação em main.rs
use crate::analisador_lexico_encapsulado::Analisador;

mod analisador_lexico_encapsulado;

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
        "4 5 0 + 2 3",
        "1🐧  50"
    ];

    for entrada_atual in entradas {
        println!("Analisando a entrada {}", entrada_atual);
        let mut analisador = Analisador::novo(entrada_atual);
        let mut result: Result<(usize, &str), Option<usize>> = analisador.próximo();

        let mut caracateres_ja_contados: usize = 0;

        while let Ok((pos, conteudo)) = result {
            print!("(\"{}\", {}) ", conteudo, caracateres_ja_contados + pos);

            let caracteres_pulados = pos - 1;
            let caracteres_no_conteudo = conteudo.chars().count();
            let total_consumido_nesta_iteracao = caracteres_pulados + caracteres_no_conteudo;

            caracateres_ja_contados += total_consumido_nesta_iteracao;

            result = analisador.próximo();
        }

        if let Err(Some(pos)) = result {
            print!("Erro na posição {}", caracateres_ja_contados + pos);
        }

        println!();
    }
}
*/

pub struct Analisador<'a> {
    pos: usize,
    prox: &'a str,
}

impl<'a> Analisador<'a> {
    pub fn novo(entrada: &'a str) -> Self {
        Analisador {
            pos: 0,
            prox: entrada,
        }
    }

    pub fn próximo(&mut self) -> Result<(usize, &str), Option<usize>> {
        let operadores: [char; 5] = ['+', '-', '/', '*', '🐧'];
        let mut pos_char: usize = 0;
        let mut indice_byte_pos_inicial: Option<usize> = None;

        // Encontrando o ínicio do próximo símbolo
        for (indice_byte, caractere) in self.prox.char_indices() {
            pos_char += 1;

            if caractere.is_whitespace() || caractere == '🦀' {
                continue;
            }

            self.pos = pos_char;
            indice_byte_pos_inicial = Some(indice_byte);
            break;
        }

        let indice_byte_pos_inicial: usize = match indice_byte_pos_inicial {
            Some(index) => index,
            None => return Err(None),
        };

        // Identificando o símbolo
        let primeiro_char: char = self.prox[indice_byte_pos_inicial..].chars().next().unwrap();
        let primeiro_char_comprimento_bytes: usize = primeiro_char.len_utf8();

        if operadores.contains(&primeiro_char) {
            let conteudo: &str = &self.prox[indice_byte_pos_inicial .. indice_byte_pos_inicial + primeiro_char_comprimento_bytes];
            self.prox = &self.prox[indice_byte_pos_inicial + primeiro_char_comprimento_bytes..];
            return Ok((self.pos, conteudo));
        }

        if primeiro_char.is_numeric() {
            let indice_byte_pos_final: usize = indice_byte_pos_inicial + primeiro_char_comprimento_bytes;
            let mut ultimo_digito_pos_final: usize = indice_byte_pos_final;

            for (indice_byte_relativo, caractere) in self.prox[indice_byte_pos_final..].char_indices()
            {
                let indice_byte_absoluto = indice_byte_pos_final + indice_byte_relativo;

                if caractere.is_numeric() {
                    ultimo_digito_pos_final = indice_byte_absoluto + 1;
                } else if caractere.is_whitespace() || caractere == '🦀' {
                    continue;
                } else {
                    break;
                }
            }

            let conteudo: &str = &self.prox[indice_byte_pos_inicial..ultimo_digito_pos_final];
            self.prox = &self.prox[ultimo_digito_pos_final..];
            return Ok((self.pos, conteudo));
        }

        return Err(Some(self.pos));
    }
}
