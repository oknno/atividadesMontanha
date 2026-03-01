fn busca_sequencial_simples(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0usize;
    let mut resultado = None;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            resultado = Some(i);
        }
    }

    (resultado, operacoes)
}

fn busca_sequencial_interrompida(vetor: &[i32], alvo: i32) -> (Option<usize>, usize) {
    let mut operacoes = 0usize;

    for i in 0..vetor.len() {
        operacoes += 1;
        if vetor[i] == alvo {
            return (Some(i), operacoes);
        }
    }

    (None, operacoes)
}

fn gerar_vetor(tamanho: usize) -> Vec<i32> {
    (1..=tamanho as i32).collect()
}

fn main() {
    let tamanhos = [10usize, 100, 1_000, 10_000, 100_000, 1_000_000];

    // Melhor caso para interrupção (alvo no início) e pior caso (inexistente)
    let alvo_inicio = 1;
    let alvo_inexistente = -1;

    println!("tamanho\tops_simples_inicio\tops_interrupcao_inicio\tops_simples_inexistente\tops_interrupcao_inexistente");

    for &t in &tamanhos {
        let v = gerar_vetor(t);

        let (_r1, ops_s_i) = busca_sequencial_simples(&v, alvo_inicio);
        let (_r2, ops_i_i) = busca_sequencial_interrompida(&v, alvo_inicio);

        let (_r3, ops_s_n) = busca_sequencial_simples(&v, alvo_inexistente);
        let (_r4, ops_i_n) = busca_sequencial_interrompida(&v, alvo_inexistente);

        println!("{}\t{}\t{}\t{}\t{}", t, ops_s_i, ops_i_i, ops_s_n, ops_i_n);
    }

    println!("\nComo usar na planilha:");
    println!("1) Copie tudo (incluindo o cabeçalho) e cole no Excel/Google Sheets.");
    println!("2) Insira um gráfico de linhas usando a coluna 'tamanho' como eixo X.");
}