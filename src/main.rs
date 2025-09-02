use std::io;

// mod converter_string;
// mod ler_nome;
// mod print_random;
mod jogo_adivinhacao;

fn main() -> io::Result<()> {
    // print_random::print_random();
    // converter_string::converter_string();
    // ler_nome::ler_nome()?;

    jogo_adivinhacao::jogo_adivinhacao()?;

    Ok(())
}
