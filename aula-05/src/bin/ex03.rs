fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut sem_pares = Vec::new();

    for numero in &v {
        if numero % 2 != 0 {
            sem_pares.push(*numero);
        }
    }

    println!("Exercicio 3 — Remocao condicional");
    println!("Vetor original: {:?}", v);
    println!("Sem numeros pares: {:?}", sem_pares);
}