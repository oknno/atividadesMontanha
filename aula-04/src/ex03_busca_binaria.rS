pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    if lista.len() == 0 {
        return None;
    }

    let mut esquerda: isize = 0;
    let mut direita: isize = lista.len() as isize - 1;

    while esquerda <= direita {
        let meio = (esquerda + direita) / 2;
        let indice = meio as usize;

        if lista[indice] == alvo {
            return Some(indice);
        } else if lista[indice] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }

    None
}