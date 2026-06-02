use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<(String, u32, u32)> = VecDeque::new();

    fila.push_back((String::from("Cliente 1"), 0, 4));
    fila.push_back((String::from("Cliente 2"), 2, 3));
    fila.push_back((String::from("Cliente 3"), 5, 2));
    fila.push_back((String::from("Cliente 4"), 6, 5));

    let mut tempo_atual = 0;
    let mut soma_espera = 0;
    let mut total = 0;

    println!("Exercicio 10 — Simulador de fila de banco");

    while fila.len() > 0 {
        let cliente = fila.pop_front().unwrap();

        let nome = cliente.0;
        let chegada = cliente.1;
        let duracao = cliente.2;

        if tempo_atual < chegada {
            tempo_atual = chegada;
        }

        let espera = tempo_atual - chegada;
        soma_espera += espera;
        total += 1;

        println!("{} esperou {} minutos", nome, espera);

        tempo_atual += duracao;
    }

    let media = soma_espera as f64 / total as f64;
    println!("Tempo medio de espera: {:.2}", media);
}