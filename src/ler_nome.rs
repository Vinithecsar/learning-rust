use std::io::{self, Write};

pub fn ler_nome() -> io::Result<()> {
    let mut nome = String::new();
    print!("Digite o seu nome: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut nome)?;
    println!("Olá, {}!", nome.trim());
    Ok(())
}
