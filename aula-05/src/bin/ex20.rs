#[derive(Clone)]
struct Processo {
    id: i32,
    tempo_restante: i32,
}

struct FilaCircularProcessos {
    dados: Vec<Option<Processo>>,
    inicio: usize,
    fim: usize,
    tamanho: usize,
    capacidade: usize,
}

impl FilaCircularProcessos {
    fn new(capacidade: usize) -> Self {
        FilaCircularProcessos {
            dados: vec![None; capacidade],
            inicio: 0,
            fim: 0,
            tamanho: 0,
            capacidade,
        }
    }

    fn enqueue(&mut self, processo: Processo) {
        if self.tamanho < self.capacidade {
            self.dados[self.fim] = Some(processo);
            self.fim = (self.fim + 1) % self.capacidade;
            self.tamanho += 1;
        }
    }

    fn dequeue(&mut self) -> Option<Processo> {
        if self.tamanho == 0 {
            return None;
        }

        let processo = self.dados[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        self.tamanho -= 1;

        processo
    }

    fn vazia(&self) -> bool {
        self.tamanho == 0
    }
}

fn main() {
    let quantum = 3;
    let mut tempo_total = 0;

    let mut fila = FilaCircularProcessos::new(10);

    fila.enqueue(Processo {
        id: 1,
        tempo_restante: 10,
    });

    fila.enqueue(Processo {
        id: 2,
        tempo_restante: 5,
    });

    fila.enqueue(Processo {
        id: 3,
        tempo_restante: 8,
    });

    println!("Exercicio 20 — Round Robin");

    while !fila.vazia() {
        let mut processo = fila.dequeue().unwrap();

        if processo.tempo_restante > quantum {
            processo.tempo_restante -= quantum;
            tempo_total += quantum;

            println!(
                "Processo {} executou {} e ainda falta {}",
                processo.id, quantum, processo.tempo_restante
            );

            fila.enqueue(processo);
        } else {
            tempo_total += processo.tempo_restante;

            println!(
                "Processo {} terminou no tempo {}",
                processo.id, tempo_total
            );

            processo.tempo_restante = 0;
        }
    }
}