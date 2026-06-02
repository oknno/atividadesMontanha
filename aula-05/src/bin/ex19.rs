use std::collections::VecDeque;

fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    let mut numero_lote = 1;

    while fila.len() > 0 {
        let mut lote: Vec<i32> = Vec::new();

        for _ in 0..tamanho_lote {
            if fila.len() > 0 {
                let valor = fila.pop_front().unwrap();
                lote.push(valor);
            }
        }

        println!("Lote {}: {:?}", numero_lote, lote);
        numero_lote += 1;
    }
}

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();

    fila.push_back(1);
    fila.push_back(2);
    fila.push_back(3);
    fila.push_back(4);
    fila.push_back(5);
    fila.push_back(6);
    fila.push_back(7);

    println!("Exercicio 19 — Fila com iteracao controlada");

    processar_em_lotes(&mut fila, 3);
}