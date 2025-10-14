use std::io::{self, Write};

use rand::Rng;

/* Exemplo de utilizacação
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
*/

// 1. Sortear um número n entre 1 e 100
// 2, Ler palpite do jogador
// 3. Três possibilidades
//  a. O jogador acertou: anunciar que o jogador ganhou e acabar o jogo
//  b. O palpite é menor que n: anunciar que o número certo é maior
//  c. O palpite é maior que n: anunciar que o número é menor
// 4. Ir para o passo 2

pub fn jogo_adivinhacao() -> io::Result<()> {
    let sorteado: i32 = rand::rng().random_range(1..=100);

    println!("Número entre 1 e 100 sorteado!");

    let mut palpite = String::new();

    loop {
        palpite.clear();

        print!("Digite o seu palpite: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut palpite)?;

        let palpite_num: i32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número válido.");
                continue;
            }
        };

        if sorteado == palpite_num {
            println!("Parabéns! Você acertou o número sorteado.");
            break;
        } else if palpite_num < sorteado {
            println!("Tente novamente! O número certo é maior.");
        } else if palpite_num > sorteado {
            println!("Tente novamente! O número certo é menor.");
        }
    }

    Ok(())
}
