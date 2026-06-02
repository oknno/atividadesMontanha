fn main() {
    let testes = vec!["{[()]}", "([)]", "((("];

    println!("Exercicio 8 — Sequencias de simbolos");

    for expressao in testes {
        let mut pilha: Vec<char> = Vec::new();
        let mut balanceado = true;

        for c in expressao.chars() {
            if c == '(' || c == '[' || c == '{' {
                pilha.push(c);
            } else if c == ')' || c == ']' || c == '}' {
                if pilha.len() == 0 {
                    balanceado = false;
                    break;
                }

                let topo = pilha.pop().unwrap();

                if c == ')' && topo != '(' {
                    balanceado = false;
                } else if c == ']' && topo != '[' {
                    balanceado = false;
                } else if c == '}' && topo != '{' {
                    balanceado = false;
                }
            }
        }

        if pilha.len() > 0 {
            balanceado = false;
        }

        println!("{} -> {}", expressao, balanceado);
    }
}