mod jogo_da_velha;
use crate::jogo_da_velha::{converter_indice_para_coordenada, jogo_acabou, ler_entrada_usuario, mostrar_tabuleiro};

fn main() {
    let mut tabuleiro: [[&str; 3]; 3] = [[" ", " ", " "],
                                        [" ", " ", " ",],
                                        [" ", " ", " "]];
    let mut jogador: &str = "X";

    loop {
        mostrar_tabuleiro(&tabuleiro);
        'jogada_valida: loop {
            let entrada: usize = ler_entrada_usuario();
            let posicao: (usize, usize) = converter_indice_para_coordenada(entrada);
            if tabuleiro[posicao.0][posicao.1] != " " {
                println!("Posição já preenchida! Tente novamente.");
            } else {
                tabuleiro[posicao.0][posicao.1] = jogador;
                break 'jogada_valida;
            }
        }

        match jogo_acabou(&tabuleiro) {
            jogo_da_velha::EstadoJogo::EmAndamento => {
                jogador = if jogador == "X" { "O"} else { "X" };
                continue;
            },
            jogo_da_velha::EstadoJogo::Vitoria => {
                mostrar_tabuleiro(&tabuleiro);
                println!("Parabéns, você ganhou!");
                break;
            },
            jogo_da_velha::EstadoJogo::Empate => {
                mostrar_tabuleiro(&tabuleiro);
                println!("Jogo empatado");
                break;
            },
        }
    }
}
