use std::process::Command;

fn executar(nome: &str) {
    println!("\nExecutando {}...\n", nome);

    let status = Command::new("cargo")
        .args(["run", "--bin", nome, "--release"])
        .status();

    match status {
        Ok(s) => {
            if !s.success() {
                println!("Erro ao executar {}", nome);
            }
        }
        Err(_) => {
            println!("Nao foi possivel executar {}", nome);
        }
    }
}

fn main() {
    println!("Aula 05 — TADs Lineares");

    executar("ex01");
    executar("ex02");
    executar("ex03");
    executar("ex04");
    executar("ex05");
    executar("ex06");
    executar("ex07");
    executar("ex08");
    executar("ex09");
    executar("ex10");
    executar("ex11");
    executar("ex12");
    executar("ex13");
    executar("ex14");
    executar("ex15");
    executar("ex16");
    executar("ex17");
    executar("ex18");
    executar("ex19");
    executar("ex20");
}