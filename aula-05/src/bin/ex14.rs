use std::collections::VecDeque;

fn main() {
    let texto = "A man a plan a canal Panama";
    let mut deque: VecDeque<char> = VecDeque::new();

    for c in texto.chars() {
        if c != ' ' {
            deque.push_back(c.to_ascii_lowercase());
        }
    }

    let mut palindromo = true;

    while deque.len() > 1 {
        let inicio = deque.pop_front().unwrap();
        let fim = deque.pop_back().unwrap();

        if inicio != fim {
            palindromo = false;
            break;
        }
    }

    println!("Exercicio 14 — Palindromo com Deque");
    println!("Texto: {}", texto);
    println!("E palindromo? {}", palindromo);
}