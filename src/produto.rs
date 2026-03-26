#[derive(Clone, Debug, PartialEq)]
pub struct Produto {
    id: u32,
    nome: String,
    marca: String,
    categoria: String,
    preco: f64,
}

impl Produto {
    pub fn novo(
        id: u32,
        nome: &str,
        marca: &str,
        categoria: &str,
        preco: f64,
    ) -> Self {
        Self {
            id,
            nome: nome.to_string(),
            marca: marca.to_string(),
            categoria: categoria.to_string(),
            preco,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn nome(&self) -> &str {
        &self.nome
    }

    pub fn marca(&self) -> &str {
        &self.marca
    }

    pub fn categoria(&self) -> &str {
        &self.categoria
    }

    pub fn preco(&self) -> f64 {
        self.preco
    }
}