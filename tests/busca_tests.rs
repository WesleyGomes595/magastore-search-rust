use megastore_search::produto::Produto;
use megastore_search::busca::SistemaBusca;

#[test]
fn deve_encontrar_produto_por_nome() {
    let mut sistema = SistemaBusca::novo();

    let p = Produto::novo(1, "Notebook", "Dell", "Eletronicos", 4500.0);
    sistema.indexar(p);

    let resultado = sistema.buscar_por_nome("Notebook");

    assert!(resultado.is_some());
}