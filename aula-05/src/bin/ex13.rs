fn main() {
    let mut fila: Vec<(String, i32, i32)> = Vec::new();

    fila.push((String::from("normal 1"), 1, 0));
    fila.push((String::from("urgente"), 5, 1));
    fila.push((String::from("normal 2"), 1, 2));
    fila.push((String::from("media"), 3, 3));

    println!("Exercicio 13 — Fila de prioridade manual");

    while fila.len() > 0 {
        let mut melhor_indice = 0;

        for i in 1..fila.len() {
            let prioridade_atual = fila[i].1;
            let prioridade_melhor = fila[melhor_indice].1;

            let ordem_atual = fila[i].2;
            let ordem_melhor = fila[melhor_indice].2;

            if prioridade_atual > prioridade_melhor {
                melhor_indice = i;
            } else if prioridade_atual == prioridade_melhor && ordem_atual < ordem_melhor {
                melhor_indice = i;
            }
        }

        let item = fila.remove(melhor_indice);

        println!("Saiu: {} | prioridade {}", item.0, item.1);
    }
}