use std::collections::HashMap;

fn main() {
    let frase = "estrutura de dados";
    let letras: Vec<char> = frase.chars().collect();

    let mut contagem: HashMap<char, i32> = HashMap::new();

    for letra in &letras {
        if letra.is_alphabetic() {
            let letra_minuscula = letra.to_ascii_lowercase();

            if contagem.contains_key(&letra_minuscula) {
                let valor_atual = contagem.get(&letra_minuscula).unwrap();
                contagem.insert(letra_minuscula, valor_atual + 1);
            } else {
                contagem.insert(letra_minuscula, 1);
            }
        }
    }

    println!("Exercicio 2 — Contador de ocorrencias");
    println!("Frase: {}", frase);
    println!("Contagem: {:?}", contagem);
}