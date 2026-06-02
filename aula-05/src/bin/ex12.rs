use std::collections::VecDeque;

fn main() {
    let capacidade = 3;
    let mut buffer: VecDeque<String> = VecDeque::new();

    let mensagens = vec!["msg 1", "msg 2", "msg 3", "msg 4", "msg 5"];

    for msg in mensagens {
        if buffer.len() == capacidade {
            buffer.pop_front();
        }

        buffer.push_back(String::from(msg));
    }

    println!("Exercicio 12 — Buffer de mensagens");
    println!("Buffer final: {:?}", buffer);
}