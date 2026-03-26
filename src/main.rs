use megastore_search::produto::Produto;
use megastore_search::busca::SistemaBusca;
use megastore_search::metricas::medir_tempo;
use megastore_search::recomendacao::SistemaRecomendacao;

fn main() {
    println!("===== SIMULAÇÃO MEGASTORE =====\n");

    let mut sistema = SistemaBusca::novo();

    // 🔹 Inserção de 10.000 produtos
    let (_, tempo_insercao) = medir_tempo(|| {
        for i in 0..10_000 {
            let produto = Produto::novo(
                i,
                &format!("Produto{}", i),
                "MarcaX",
                "CategoriaY",
                99.90,
            );
            sistema.indexar(produto);
        }
    });

    println!("Tempo para inserir 10.000 produtos: {:?}", tempo_insercao);

    // 🔹 Busca por nome
    let (_, tempo_nome) = medir_tempo(|| {
        sistema.buscar_por_nome("Produto9999");
    });

    // 🔹 Busca por marca
    let (_, tempo_marca) = medir_tempo(|| {
        sistema.buscar_por_marca("MarcaX");
    });

    // 🔹 Busca por categoria
    let (_, tempo_categoria) = medir_tempo(|| {
        sistema.buscar_por_categoria("CategoriaY");
    });

    println!("Tempo busca por nome: {:?}", tempo_nome);
    println!("Tempo busca por marca: {:?}", tempo_marca);
    println!("Tempo busca por categoria: {:?}", tempo_categoria);

    // ===============================
    // 🔥 SISTEMA DE RECOMENDAÇÃO BFS
    // ===============================

    println!("\n===== TESTE DO SISTEMA DE RECOMENDAÇÃO =====\n");

    let mut sistema_rec = SistemaRecomendacao::novo();

    // Criando relações entre produtos (grafo)
    sistema_rec.adicionar_relacao(1, 2);
    sistema_rec.adicionar_relacao(2, 3);
    sistema_rec.adicionar_relacao(3, 4);
    sistema_rec.adicionar_relacao(4, 5);

    // Recomendação direta
    if let Some(diretas) = sistema_rec.recomendar(1) {
        println!("Recomendações diretas do produto 1: {:?}", diretas);
    }

    // Recomendação avançada com BFS
    let recomendacoes_bfs = sistema_rec.recomendar_bfs(1);
    println!("Recomendações BFS a partir do produto 1: {:?}", recomendacoes_bfs);

    println!("\nSimulação finalizada com sucesso!");
}