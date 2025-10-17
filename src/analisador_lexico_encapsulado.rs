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
