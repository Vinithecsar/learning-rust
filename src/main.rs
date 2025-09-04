use std::io;

mod jogo_adivinhacao;

fn main() -> io::Result<()> {
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
