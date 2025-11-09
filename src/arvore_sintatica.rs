pub enum Expressão {
    Valor(i64),
    OperacaoBinaria {
        operacao: char, // Representa as operações de adição, subtração, multiplicação, divisão, resto de divisão e negação
        valor_esquerdo: Box<Expressão>, 
        valor_direito: Box<Expressão>,
    },
    OperacaoUnaria(Box<Expressão>), // Representa a operação de subtração
}

impl Expressão {
    pub fn avaliar(&self) -> Option<i64> {
        match self {
            Expressão::Valor(v) => Some(*v),
            Expressão::OperacaoBinaria {
                operacao,
                valor_esquerdo,
                valor_direito,
            } => {
                let esq = valor_esquerdo.avaliar()?;
                let dir = valor_direito.avaliar()?;

                match operacao {
                    '+' => Some(esq + dir),
                    '-' => Some(esq - dir),
                    '*' => Some(esq * dir),
                    '/' => {
                        if dir == 0 {
                            None
                        } else {
                            Some(esq / dir)
                        }
                    }
                    '%' => {
                        if dir == 0 {
                            None
                        } else {
                            Some(esq % dir)
                        }
                    }
                    _ => None,
                }
            }
            Expressão::OperacaoUnaria(expr) => {
                let v = expr.avaliar()?;
                Some(-v)
            }
        }
    }

    pub fn imprimir(&self) {
        fn imprimir_aux(expr: &Expressão, prec_pai: u8) -> String {
            match expr {
                Expressão::Valor(v) => v.to_string(),

                Expressão::OperacaoUnaria(interno) => match &**interno {
                    Expressão::OperacaoUnaria(filho) => imprimir_aux(filho, 3),
                    _ => {
                        let conteudo = imprimir_aux(interno, 3);
                        format!("-{}", conteudo)
                    }
                },

                Expressão::OperacaoBinaria {
                    operacao,
                    valor_esquerdo,
                    valor_direito,
                } => {
                    let prec_atual = precedencia(*operacao);
                    let esq = imprimir_aux(&valor_esquerdo, prec_atual);
                    let dir = imprimir_aux(&valor_direito, prec_atual);

                    let texto = format!("{esq} {operacao} {dir}");
                    if prec_atual < prec_pai {
                        format!("({texto})")
                    } else {
                        texto
                    }
                }
            }
        }

        let resultado = imprimir_aux(self, 0);
        println!("{resultado}");
    }

    pub fn imprimir_árvore(&self) {
        fn imprimir_árvore_aux(expr: &Expressão, prefixo: &str, eh_ultimo: bool) {
            let conector = if eh_ultimo { "└" } else { "├" };

            match expr {
                Expressão::Valor(v) => {
                    println!("{prefixo}{conector} {v}");
                }

                Expressão::OperacaoUnaria(interno) => {
                    println!("{prefixo}{conector} -");
                    let novo_prefixo = format!("{}{}", prefixo, if eh_ultimo { "  " } else { "│ " });
                    imprimir_árvore_aux(&interno, &novo_prefixo, true);
                }

                Expressão::OperacaoBinaria {
                    operacao,
                    valor_esquerdo,
                    valor_direito,
                } => {
                    println!("{prefixo}{conector} {operacao}");
                    let novo_prefixo = format!("{}{}", prefixo, if eh_ultimo { "  " } else { "│ " });

                    imprimir_árvore_aux(&valor_esquerdo, &novo_prefixo, false);
                    imprimir_árvore_aux(&valor_direito, &novo_prefixo, true);
                }
            }
        }

        match self {
            Expressão::Valor(v) => println!("{v}"),
            Expressão::OperacaoUnaria(interno) => {
                println!("-");
                imprimir_árvore_aux(&interno, "", true);
            }
            Expressão::OperacaoBinaria {
                operacao,
                valor_esquerdo,
                valor_direito,
            } => {
                println!("{operacao}");
                imprimir_árvore_aux(&valor_esquerdo, "", false);
                imprimir_árvore_aux(&valor_direito, "", true);
            }
        }
    }
}

fn precedencia(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        _ => 0,
    }
}

pub fn tokenizar(s: &str) -> Option<Vec<String>> {
    let mut tokens = Vec::new();
    let mut atual = String::new();

    for c in s.chars() {
        match c {
            ' ' => {
                if !atual.is_empty() {
                    tokens.push(atual.clone());
                    atual.clear();
                }
            }
            '(' | ')' | '+' | '-' | '*' | '/' | '%' => {
                if !atual.is_empty() {
                    tokens.push(atual.clone());
                    atual.clear();
                }
                tokens.push(c.to_string());
            }
            d if d.is_ascii_digit() => atual.push(d),
            _ => return None,
        }
    }

    if !atual.is_empty() {
        tokens.push(atual);
    }

    Some(tokens)
}

pub fn parse_expressao(tokens: &[String]) -> Option<(Expressão, &[String])> {
    let (mut esquerda, mut resto) = parse_termo(tokens)?;
    while let Some(op) = resto.first() {
        if op == "+" || op == "-" {
            let op_bin = if op == "+" { '+' } else { '-' };
            let (direita, resto2) = parse_termo(&resto[1..])?;
            esquerda = Expressão::OperacaoBinaria {
                operacao: op_bin,
                valor_esquerdo: Box::new(esquerda),
                valor_direito: Box::new(direita),
            };
            resto = resto2;
        } else {
            break;
        }
    }
    Some((esquerda, resto))
}

pub fn parse_termo(tokens: &[String]) -> Option<(Expressão, &[String])> {
    let (mut esquerda, mut resto) = parse_fator(tokens)?;
    while let Some(op) = resto.first() {
        if op == "*" || op == "/" || op == "%" {
            let op_bin = if op == "*" { '*' } else if op == "/" { '/' } else { '%' };
            let (direita, resto2) = parse_fator(&resto[1..])?;
            esquerda = Expressão::OperacaoBinaria {
                operacao: op_bin,
                valor_esquerdo: Box::new(esquerda),
                valor_direito: Box::new(direita),
            };
            resto = resto2;
        } else {
            break;
        }
    }
    Some((esquerda, resto))
}

pub fn parse_fator(tokens: &[String]) -> Option<(Expressão, &[String])> {
    let primeiro = tokens.first()?;

    match primeiro.as_str() {
        "(" => {
            let (expr, resto) = parse_expressao(&tokens[1..])?;
            if resto.first()? == ")" {
                Some((expr, &resto[1..]))
            } else {
                None
            }
        }
        "-" => {
            let (expr, resto) = parse_fator(&tokens[1..])?;
            Some((Expressão::OperacaoUnaria(Box::new(expr)), resto))
        }
        _ => {
            let valor = primeiro.parse::<i64>().ok()?;
            Some((Expressão::Valor(valor), &tokens[1..]))
        }
    }
}
