fn busca_todas_posicoes(vetor: &[i32], alvo: i32) -> (Vec<usize>, usize) {
    let mut operacoes: usize = 0;
    let mut posicoes: Vec<usize> = Vec::new();

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            posicoes.push(i);
        }
    }

    (posicoes, operacoes)
}

fn main() {
    let vetor = vec![1, 2, 5, 3, 5, 5, 4];
    let alvo = 5;

    let (pos, ops) = busca_todas_posicoes(&vetor, alvo);

    println!("Exercício 4 — Todas as posições");
    println!("Vetor: {:?}", vetor);
    println!("Alvo: {}", alvo);
    println!("Posições encontradas: {:?}", pos);
    println!("Operações: {}", ops);
}