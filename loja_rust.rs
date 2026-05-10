use std::io;

fn main() {
    let mut itens: Vec<String> = Vec::new();

    loop {
        println!("\nLista de Compras");
        println!("1 - Adicionar item");
        println!("2 - Ver itens");
        println!("3 - Remover item");
        println!("4 - Sair");
        println!("Escolha uma opção:");

        let mut escolha = String::new();

        io::stdin()
            .read_line(&mut escolha)
            .expect("Erro ao ler entrada");

        match escolha.trim() {
            "1" => {
                adicionar_item(&mut itens);
            }

            "2" => {
                listar_itens(&itens);
            }

            "3" => {
                remover_item(&mut itens);
            }

            "4" => {
                println!("Saindo...");
                break;
            }
            
            _ => {
                println!("Opção inválida!"),
            }
        }
    }
}

fn adicionar_item(itens: &mut Vec<String>) {
    println!("Digite o nome do item:");

    let mut item = String::new();

    io::stdin()
        .read_line(&mut item)
        .expect("Erro ao ler item");

    let item = item.trim().to_string();

    if item.is_empty() {
        println!("Item vazio não pode ser adicionado.");
        return;
    }

    itens.push(item);

    println!("Item adicionado com sucesso!");
}

fn listar_itens(itens: &Vec<String>) {
    if itens.is_empty() {
        println!("A lista está vazia.");
        return;
    }

    println!("\nItens da Lista");

    for (i, item) in itens.iter().enumerate() {
        println!("{} - {}", i + 1, item);
    }
}

fn remover_item(itens: &mut Vec<String>) {
    if itens.is_empty() {
        println!("Não há itens para remover.");
        return;
    }

    listar_itens(itens);

    println!("Digite o número do item para remover:");

    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    let indice: usize = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Número inválido.");
            return;
        }
    };

    if indice == 0 || indice > itens.len() {
        println!("Item inexistente.");
        return;
    }

    let removido = itens.remove(indice - 1);

    println!("Item '{}' removido com sucesso!", removido);
}