use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<(String, i32)> = VecDeque::new();

    fila.push_back((String::from("Relatorio"), 10));
    fila.push_back((String::from("Contrato"), 4));
    fila.push_back((String::from("Apresentacao"), 20));

    println!("Exercicio 11 — Impressora compartilhada");

    while fila.len() > 0 {
        let trabalho = fila.pop_front().unwrap();

        println!(
            "Imprimindo {} com {} paginas",
            trabalho.0, trabalho.1
        );
    }
}