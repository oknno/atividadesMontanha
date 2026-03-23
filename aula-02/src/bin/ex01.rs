use std::time::Instant;

fn busca_sequencial_simples_str(vetor: &[&str], alvo: &str) -> (Option<usize>, usize) {
    let mut operacoes: usize = 0;
    let mut resultado: Option<usize> = None;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i); // última ocorrência
        }
    }

    (resultado, operacoes)
}

fn busca_sequencial_interrompida_str(vetor: &[&str], alvo: &str) -> (Option<usize>, usize) {
    let mut operacoes: usize = 0;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes); // primeira ocorrência
        }
    }

    (None, operacoes)
}

fn main() {
    let vetor = vec!["ana", "bia", "cai", "bia", "dan"];
    let alvo = "bia";

    let inicio = Instant::now();
    let (r1, ops1) = busca_sequencial_simples_str(&vetor, alvo);
    let t1 = inicio.elapsed();

    let inicio = Instant::now();
    let (r2, ops2) = busca_sequencial_interrompida_str(&vetor, alvo);
    let t2 = inicio.elapsed();

    println!("Exercício 1 — Busca com strings");
    println!("Vetor: {:?}", vetor);
    println!("Alvo: {}", alvo);

    println!("\nBusca simples:");
    println!("Resultado: {:?} | Operações: {} | Tempo: {:?}", r1, ops1, t1);

    println!("\nBusca com interrupção:");
    println!("Resultado: {:?} | Operações: {} | Tempo: {:?}", r2, ops2, t2);
}