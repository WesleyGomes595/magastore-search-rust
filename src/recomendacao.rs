use std::collections::{HashMap, HashSet, VecDeque};

pub struct SistemaRecomendacao {
    grafo: HashMap<u32, Vec<u32>>,
}

impl SistemaRecomendacao {
    pub fn novo() -> Self {
        Self {
            grafo: HashMap::new(),
        }
    }

    pub fn adicionar_relacao(&mut self, origem: u32, destino: u32) {
        self.grafo.entry(origem).or_default().push(destino);
    }

    // 🔹 Recomendação direta
    pub fn recomendar(&self, produto_id: u32) -> Option<&Vec<u32>> {
        self.grafo.get(&produto_id)
    }

    // 🔥 Recomendação avançada usando BFS
    pub fn recomendar_bfs(&self, produto_id: u32) -> Vec<u32> {
        let mut visitados = HashSet::new();
        let mut fila = VecDeque::new();
        let mut recomendados = Vec::new();

        fila.push_back(produto_id);
        visitados.insert(produto_id);

        while let Some(atual) = fila.pop_front() {
            if let Some(vizinhos) = self.grafo.get(&atual) {
                for &vizinho in vizinhos {
                    if !visitados.contains(&vizinho) {
                        visitados.insert(vizinho);
                        fila.push_back(vizinho);
                        recomendados.push(vizinho);
                    }
                }
            }
        }

        recomendados
    }
}