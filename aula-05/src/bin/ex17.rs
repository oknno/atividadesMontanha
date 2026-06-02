use std::collections::VecDeque;
use std::time::Instant;

struct FilaCircular {
    dados: Vec<Option<i32>>,
    inicio: usize,
    fim: usize,
    tamanho: usize,
    capacidade: usize,
}

impl FilaCircular {
    fn new(capacidade: usize) -> Self {
        FilaCircular {
            dados: vec![None; capacidade],
            inicio: 0,
            fim: 0,
            tamanho: 0,
            capacidade,
        }
    }

    fn enqueue(&mut self, valor: i32) {
        if self.tamanho < self.capacidade {
            self.dados[self.fim] = Some(valor);
            self.fim = (self.fim + 1) % self.capacidade;
            self.tamanho += 1;
        }
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.tamanho == 0 {
            return None;
        }

        let valor = self.dados[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        self.tamanho -= 1;

        valor
    }
}

fn main() {
    let quantidade = 10_000;

    let inicio_vec = Instant::now();
    let mut fila_vec: Vec<i32> = Vec::new();

    for i in 0..quantidade {
        fila_vec.push(i);
    }

    while fila_vec.len() > 0 {
        fila_vec.remove(0);
    }

    let tempo_vec = inicio_vec.elapsed();

    let inicio_deque = Instant::now();
    let mut fila_deque: VecDeque<i32> = VecDeque::new();

    for i in 0..quantidade {
        fila_deque.push_back(i);
    }

    while fila_deque.len() > 0 {
        fila_deque.pop_front();
    }

    let tempo_deque = inicio_deque.elapsed();

    let inicio_circular = Instant::now();
    let mut fila_circular = FilaCircular::new(quantidade as usize);

    for i in 0..quantidade {
        fila_circular.enqueue(i);
    }

    while fila_circular.dequeue().is_some() {}

    let tempo_circular = inicio_circular.elapsed();

    println!("Exercicio 17 — Comparacao de desempenho");
    println!("Quantidade de elementos: {}", quantidade);
    println!("Vec com remove(0): {:?}", tempo_vec);
    println!("VecDeque: {:?}", tempo_deque);
    println!("Fila circular: {:?}", tempo_circular);
}