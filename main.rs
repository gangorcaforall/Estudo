use std::io;

fn main() {
    let mut nome = String::new();

    println!("Digite seu nome:");

    io::stdin()
        .read_line(&mut nome)
        .expect("Erro ao ler");

    let nome = nome.trim();

    println!("Olá, {}!", nome);

    let tamanho = nome.len();

    println!("Seu nome tem {} letras.", tamanho);

    if tamanho >= 5 {
        println!("Seu nome é grande.");
    } else {
        println!("Seu nome é pequeno.");
    }
}