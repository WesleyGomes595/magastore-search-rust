use std::time::{Duration, Instant};

pub fn medir_tempo<F, T>(funcao: F) -> (T, Duration)
where
    F: FnOnce() -> T,
{
    let inicio = Instant::now();
    let resultado = funcao();
    let duracao = inicio.elapsed();
    (resultado, duracao)
}