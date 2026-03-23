mod ex01_verificar_primeiro;
mod ex02_somar_lista;
mod ex03_busca_binaria;
mod ex04_pares_com_soma;
mod ex05_imprimir_pares_e_pares;
mod ex06_potencias_de_dois;
mod ex07_fibonacci_recursivo;
mod ex08_ordenacao_bolha;
mod ex09_produto_de_matrizes;
mod ex10_merge_sort;

fn main() {
    let lista = vec![10, 20, 30, 40];

    println!("Ex01: {:?}", ex01_verificar_primeiro::verificar_primeiro(&lista));
    println!("Ex02: {}", ex02_somar_lista::somar_lista(&lista));
    println!("Ex03: {:?}", ex03_busca_binaria::busca_binaria(&lista, 30));

    ex04_pares_com_soma::pares_com_soma(&lista, 50);

    ex05_imprimir_pares_e_pares::imprimir_pares_e_pares(&[1, 2, 3]);

    ex06_potencias_de_dois::potencias_de_dois(20);

    println!("Ex07: {}", ex07_fibonacci_recursivo::fibonacci_recursivo(6));

    let mut lista_bolha = vec![5, 3, 8, 1];
    ex08_ordenacao_bolha::ordenacao_bolha(&mut lista_bolha);
    println!("Ex08: {:?}", lista_bolha);

    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    let c = ex09_produto_de_matrizes::produto_de_matrizes(&a, &b);
    println!("Ex09: {:?}", c);

    let lista_merge = vec![9, 4, 7, 1, 3];
    let ordenada = ex10_merge_sort::merge_sort(lista_merge);
    println!("Ex10: {:?}", ordenada);
}