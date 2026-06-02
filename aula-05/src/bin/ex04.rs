fn main() {
    let mut a = vec![1, 3, 5, 7];
    let b = vec![2, 4, 6, 8];

    a.extend(b);
    a.sort();

    println!("Exercicio 4 — Mescla ordenada");
    println!("Resultado: {:?}", a);
}