pub fn imprimir_pares_e_pares(lista: &[i32]) {
    for i in 0..lista.len() {
        println!("{}", lista[i]);
    }

    for i in 0..lista.len() {
        for j in 0..lista.len() {
            println!("({}, {})", lista[i], lista[j]);
        }
    }
}