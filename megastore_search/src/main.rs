mod produto;
mod index;

use crate::produto::Produto;
use crate::index::Indexador;
use std::io::{self, Write};

fn carregar_produtos() -> Vec<Produto> {
    vec![
        Produto { id: 1, nome: "Smartphone Galaxy S21".into(), marca: "Samsung".into(), categoria: "Eletrônicos".into() },
        Produto { id: 2, nome: "Smartphone iPhone 13".into(), marca: "Apple".into(), categoria: "Eletrônicos".into() },
        Produto { id: 3, nome: "Notebook Dell Inspiron".into(), marca: "Dell".into(), categoria: "Eletrônicos".into() },
        Produto { id: 4, nome: "Notebook Samsung Book".into(), marca: "Samsung".into(), categoria: "Eletrônicos".into() },
        Produto { id: 5, nome: "Camiseta Adidas".into(), marca: "Adidas".into(), categoria: "Vestuário".into() },
        Produto { id: 6, nome: "Calça Adidas Sport".into(), marca: "Adidas".into(), categoria: "Vestuário".into() },
        Produto { id: 7, nome: "Camiseta Nike".into(), marca: "Nike".into(), categoria: "Vestuário".into() },
        Produto { id: 8, nome: "Tênis Nike Air Max".into(), marca: "Nike".into(), categoria: "Calçados".into() },
        Produto { id: 9, nome: "Tênis Adidas Run".into(), marca: "Adidas".into(), categoria: "Calçados".into() },
        Produto { id: 10, nome: "iPad Pro".into(), marca: "Apple".into(), categoria: "Eletrônicos".into() },
    ]
}

fn buscar_produtos(indexador: &Indexador, produtos: &Vec<Produto>) {
    loop {
        println!("\nDigite o termo de busca (ou 'sair' para encerrar): ");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).unwrap();
        let entrada = entrada.trim().to_lowercase();

        if entrada == "sair" {
            break;
        }

        let encontrados = indexador.buscar(&entrada);

        if encontrados.is_empty() {
            println!("Nenhum produto encontrado para '{}'", entrada);
        } else {
            println!("\n🔎 Produtos encontrados:");
            for id in &encontrados {
                if let Some(produto) = produtos.iter().find(|p| p.id == *id) {
                    println!("- [{}] {} ({}, {})", produto.id, produto.nome, produto.marca, produto.categoria);
                }
            }
        }
    }
}

fn main() {
    let produtos = carregar_produtos();
    let mut indexador = Indexador::new();

    for produto in &produtos {
        indexador.indexar(produto);
    }

    buscar_produtos(&indexador, &produtos);
}
