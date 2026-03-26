use std::collections::HashMap;
use crate::produto::Produto;

pub struct SistemaBusca {
    indice_nome: HashMap<String, Vec<Produto>>,
    indice_marca: HashMap<String, Vec<Produto>>,
    indice_categoria: HashMap<String, Vec<Produto>>,
}

impl SistemaBusca {
    pub fn novo() -> Self {
        Self {
            indice_nome: HashMap::new(),
            indice_marca: HashMap::new(),
            indice_categoria: HashMap::new(),
        }
    }

    pub fn indexar(&mut self, produto: Produto) {
        self.indice_nome
            .entry(produto.nome().to_string())
            .or_default()
            .push(produto.clone());

        self.indice_marca
            .entry(produto.marca().to_string())
            .or_default()
            .push(produto.clone());

        self.indice_categoria
            .entry(produto.categoria().to_string())
            .or_default()
            .push(produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Vec<Produto>> {
        self.indice_nome.get(nome)
    }

    pub fn buscar_por_marca(&self, marca: &str) -> Option<&Vec<Produto>> {
        self.indice_marca.get(marca)
    }

    pub fn buscar_por_categoria(&self, categoria: &str) -> Option<&Vec<Produto>> {
        self.indice_categoria.get(categoria)
    }
}