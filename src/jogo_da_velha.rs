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