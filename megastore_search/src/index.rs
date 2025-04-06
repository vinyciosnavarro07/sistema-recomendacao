use std::collections::HashMap;
use crate::produto::Produto;
use deunicode::deunicode; // <- Importa o deunicode

pub struct Indexador {
    indice: HashMap<String, Vec<u64>>,
}

impl Indexador {
    pub fn new() -> Self {
        Self { indice: HashMap::new() }
    }

    pub fn indexar(&mut self, produto: &Produto) {
        let texto = format!("{} {} {}", produto.nome, produto.marca, produto.categoria);
        let texto_normalizado = deunicode(&texto).to_lowercase();

        for palavra in texto_normalizado.split_whitespace() {
            self.indice
                .entry(palavra.to_string())
                .or_default()
                .push(produto.id);
        }
    }

    pub fn buscar(&self, termo: &str) -> Vec<u64> {
        let termo_normalizado = deunicode(termo).to_lowercase();
        let palavras = termo_normalizado.split_whitespace();

        let mut resultados = Vec::new();

        for palavra in palavras {
            if let Some(lista) = self.indice.get(palavra) {
                resultados.extend(lista);
            }
        }

        resultados
    }
}
