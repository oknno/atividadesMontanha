pub fn verificar_primeiro(lista: &[i32]) -> Option<i32> {
    if lista.len() == 0 {
        return None;
    }

    Some(lista[0])
}
