fn main() {
    let mut texto = String::new();
    let mut pilha_desfazer: Vec<String> = Vec::new();
    let mut pilha_refazer: Vec<String> = Vec::new();

    let digitado1 = String::from("Ola ");
    texto.push_str(&digitado1);
    pilha_desfazer.push(digitado1);

    let digitado2 = String::from("mundo");
    texto.push_str(&digitado2);
    pilha_desfazer.push(digitado2);

    let ultima_acao = pilha_desfazer.pop().unwrap();
    let novo_tamanho = texto.len() - ultima_acao.len();
    texto.truncate(novo_tamanho);
    pilha_refazer.push(ultima_acao);

    let refazer = pilha_refazer.pop().unwrap();
    texto.push_str(&refazer);
    pilha_desfazer.push(refazer);

    println!("Exercicio 7 — Desfazer e refazer");
    println!("Texto final: {}", texto);
}