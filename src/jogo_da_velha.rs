/* Exemplo de implementação em main.rs
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
*/

use std::{io::{self, Write}};

pub fn mostrar_tabuleiro(tabuleiro: &[[&str; 3]; 3]) {
  println!(" {} | {} | {}", tabuleiro[0][0], tabuleiro[0][1], tabuleiro[0][2]);
  println!("-----------");
  println!(" {} | {} | {}", tabuleiro[1][0], tabuleiro[1][1], tabuleiro[1][2]);
  println!("-----------");
  println!(" {} | {} | {}", tabuleiro[2][0], tabuleiro[2][1], tabuleiro[2][2]);
}

pub fn converter_indice_para_coordenada(indice: usize) -> (usize, usize) {
  let indice_ajustado: usize = (indice - 1) as usize;
  let linha  = indice_ajustado / 3;
  let coluna = indice_ajustado % 3;

  return (linha, coluna);
}

pub fn ler_entrada_usuario() -> usize {
    loop {
        print!("Próxima jogada: ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        if let Ok(num) = entrada.trim().parse::<usize>() {
            if (1..=9).contains(&num) {
              return num;
            }
        }
    }
}

pub enum EstadoJogo {
  EmAndamento,
  Vitoria,
  Empate
}

pub fn jogo_acabou(tabuleiro: &[[&str; 3]; 3]) -> EstadoJogo {

  // Linhas e colunas
  for i in 0..3 {
    if tabuleiro[i][0] != " " 
        && tabuleiro[i][0] == tabuleiro[i][1]
        && tabuleiro[i][1] == tabuleiro[i][2] {
        return EstadoJogo::Vitoria;
    }

    if tabuleiro[0][i] != " "
        && tabuleiro[0][i] == tabuleiro[1][i]
        && tabuleiro[1][i] == tabuleiro[2][i] {
        return EstadoJogo::Vitoria;
    }
  }

  // Diagonais
  if tabuleiro[0][0] != " "
      && tabuleiro[0][0] == tabuleiro[1][1]
      && tabuleiro[1][1] == tabuleiro[2][2] {
      return EstadoJogo::Vitoria;
  }

  if tabuleiro[0][2] != " "
      && tabuleiro[0][2] == tabuleiro[1][1]
      && tabuleiro[1][1] == tabuleiro[2][0] {
      return EstadoJogo::Vitoria;
  }

  // Sem espaços vazios
  if tabuleiro.iter().flatten().all(|&casa| casa != " ") {
    return EstadoJogo::Empate;
  }


  return EstadoJogo::EmAndamento;
}