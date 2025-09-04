use rand::Rng;
use std::io::{self, Write};
use std::str::FromStr;

pub fn print_random() {
    let r = rand::rng().random_range(1..=100);
    println!("Hello, world! {r}");
}

pub fn converter_string() {
    let x = "321";
    let a: i32 = x.parse().unwrap();
    let b = x.parse::<i32>().unwrap();
    let c = i32::from_str(x).unwrap();

    println!("{a} {b} {c}");
}

pub fn ler_nome() -> io::Result<()> {
    let mut nome = String::new();
    print!("Digite o seu nome: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut nome)?;
    println!("Olá, {}!", nome.trim());
    Ok(())
}
