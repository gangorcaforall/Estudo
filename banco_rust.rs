use std::io;

fn depositar(saldo: &mut f32) {
    let mut entrada = String::new();

    println!("Digite o valor do deposito:");

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao digitar");

    let valor: f32 = entrada.trim().parse().unwrap();

    *saldo += valor;

    println!("Deposito realizado!");
}

fn sacar(saldo: &mut f32) {
    let mut entrada = String::new();

    println!("Digite o valor do saque:");

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler");

    let valor: f32 = entrada.trim().parse().unwrap();

    if valor > *saldo {
        println!("Saldo insuficiente!");
    } else {
        *saldo -= valor;
        println!("Saque realizado!");
    }
}

fn main() {
    let mut saldo: f32 = 1000.0;
    let mut opcao = 0;
    let mut nome = String::new();
    
    print!("Digite seu nome: ");
    
    io::stdin()
        .read_line(&mut nome)
        .expect("Nome inválido");
        
    nome = nome.trim().to_string();
        
    while nome == "André" {

        print!("Não pode ser André. Tente novamente: ");

        io::stdin()
        .read_line(&mut nome)
        .expect("Nome inválido");
    }

    while opcao != 4 {
        println!("\n Banco");
        println!("1 - Ver saldo");
        println!("2 - Depositar");
        println!("3 - Sacar");
        println!("4 - Sair");

        let mut entrada = String::new();

        println!("Escolha uma opcao:");

        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler");

        opcao = entrada.trim().parse().unwrap();

        match opcao {
            1 => {
                println!("Saldo atual: {}", saldo);
            }

            2 => {
                depositar(&mut saldo);
            }

            3 => {
                sacar(&mut saldo);
            }

            4 => {
                println!("Até a próxima!");
            }

            _ => {
                println!("Opcao invalida!");
            }
        }
    }
}