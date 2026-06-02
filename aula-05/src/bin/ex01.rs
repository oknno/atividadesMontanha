fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let mut invertido = Vec::new();

    while v.len() > 0 {
        let valor = v.pop().unwrap();
        invertido.push(valor);
    }

    println!("Exercicio 1 — Inversao com Vec");
    println!("Resultado: {:?}", invertido);
}