use std::collections::HashMap;
use crate::produto::Produto;

pub struct Recomendador {
    grafo: HashMap<u64, Vec<u64>>,
}

impl Recomendador {
    pub fn new() -> Self {
        Self { grafo: HashMap::new() }
    }

    pub fn adicionar_produto(&mut self, produto: &Produto) {
        self.grafo.entry(produto.id).or_default();
    }

    pub fn construir_relacoes(&mut self) {
        let ids: Vec<u64> = self.grafo.keys().copied().collect();

        for (i, &id1) in ids.iter().enumerate() {
            for &id2 in ids.iter().skip(i + 1) {
                self.grafo.entry(id1).or_default().push(id2);
                self.grafo.entry(id2).or_default().push(id1);
            }
        }
    }

    pub fn recomendar(&self, id: u64) -> Vec<u64> {
        self.grafo.get(&id).cloned().unwrap_or_default()
    }
}
