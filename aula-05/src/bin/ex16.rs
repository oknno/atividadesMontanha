use std::collections::VecDeque;

fn main() {
    let mut tarefas: VecDeque<String> = VecDeque::new();

    tarefas.push_back(String::from("Atualizar planilha"));
    tarefas.push_back(String::from("Responder email"));
    tarefas.push_front(String::from("Corrigir erro urgente"));

    println!("Exercicio 16 — Fila de tarefas");

    while tarefas.len() > 0 {
        let tarefa = tarefas.pop_front().unwrap();
        println!("Executando: {}", tarefa);
    }
}