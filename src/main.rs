use std::io;

mod jogo_adivinhacao;
mod miscellaneous;

fn main() -> io::Result<()> {
    miscellaneous::print_random();
    miscellaneous::converter_string();
    miscellaneous::ler_nome()?;

    match jogo_adivinhacao::jogo_adivinhacao() {
        Ok(()) => Ok(()),
        Err(e) => {
            println!(
                "Ocorreu um erro, execute o programa novamente. Detalhes: {}",
                e
            );
            return Err(e);
        }
    }
}
