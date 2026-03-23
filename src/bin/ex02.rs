fn contar_ocorrencias(vetor: &[i32], alvo: i32) -> (usize, usize) {
    let mut operacoes: usize = 0;
    let mut contagem: usize = 0;

    for &valor in vetor {
        operacoes += 1;
        if valor == alvo {
            contagem += 1;
        }
    }

    (contagem, operacoes)
}

fn main() {
    let vetor = vec![1, 2, 5, 3, 5, 5, 4];
    let alvo = 5;

    let (qtd, ops) = contar_ocorrencias(&vetor, alvo);

    println!("Exercício 2 — Contar ocorrências");
    println!("Vetor: {:?}", vetor);
    println!("Alvo: {}", alvo);
    println!("Quantidade de ocorrências: {}", qtd);
    println!("Operações: {}", ops);
}