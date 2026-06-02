fn main() {
    let mut pagina_atual = String::from("google.com");
    let mut historico_back: Vec<String> = Vec::new();
    let mut historico_forward: Vec<String> = Vec::new();

    historico_back.push(pagina_atual);
    pagina_atual = String::from("rust-lang.org");

    historico_back.push(pagina_atual);
    pagina_atual = String::from("github.com");

    let anterior = historico_back.pop().unwrap();
    historico_forward.push(pagina_atual);
    pagina_atual = anterior;

    let proxima = historico_forward.pop().unwrap();
    historico_back.push(pagina_atual);
    pagina_atual = proxima;

    println!("Exercicio 6 — Historico de navegacao");
    println!("Pagina atual: {}", pagina_atual);
}