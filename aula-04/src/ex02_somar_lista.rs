pub fn somar_lista(lista: &[i32]) -> i32 {
    let mut total = 0;

    for elemento in lista {
        total += elemento;
    }

    total
}
