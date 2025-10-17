pub fn próximo(entrada: &str) -> Result<(usize, &str, &str), Option<usize>> {
    let operadores: [char; 5] = ['+', '-', '/', '*', '🐧'];
    let mut pos_char: usize = 0;
    let mut pos_inicial_char: usize = 0;
    let mut indice_byte_pos_inicial: Option<usize> = None;

    // Encontrando o ínicio do próximo símbolo
    for (indice_byte, caractere) in entrada.char_indices() {
        pos_char += 1;

        if caractere.is_whitespace() || caractere == '🦀' {
            continue;
        }

        pos_inicial_char = pos_char;
        indice_byte_pos_inicial = Some(indice_byte);
        break;
    }

    let indice_byte_pos_inicial: usize = match indice_byte_pos_inicial {
        Some(index) => index,
        None => return Err(None),
    };

    // Identificando o símbolo
    let primeiro_char: char = entrada[indice_byte_pos_inicial..].chars().next().unwrap();
    let primeiro_char_comprimento_bytes: usize = primeiro_char.len_utf8();

    if operadores.contains(&primeiro_char) {
        let conteudo: &str = &entrada[indice_byte_pos_inicial .. indice_byte_pos_inicial + primeiro_char_comprimento_bytes];
        let restante: &str = &entrada[indice_byte_pos_inicial + primeiro_char_comprimento_bytes ..];
        return Ok((pos_inicial_char, conteudo, restante));
    }

    if primeiro_char.is_numeric() {
        let indice_byte_pos_final: usize = indice_byte_pos_inicial + primeiro_char_comprimento_bytes;
        let mut ultimo_digito_pos_final: usize = indice_byte_pos_final;

        for (indice_byte_relativo, caractere) in entrada[indice_byte_pos_final..].char_indices() {
            
            let indice_byte_absoluto = indice_byte_pos_final + indice_byte_relativo;
            
            if caractere.is_numeric() {
                ultimo_digito_pos_final = indice_byte_absoluto + 1;
            } else if caractere.is_whitespace() || caractere == '🦀' {
                continue;
            } else {
                break;
            }
        }


        let conteudo: &str = &entrada[indice_byte_pos_inicial .. ultimo_digito_pos_final];
        let restante: &str = &entrada[ultimo_digito_pos_final ..];
        return Ok((pos_inicial_char, conteudo, restante));
    }

    return Err(Some(pos_inicial_char));
}
