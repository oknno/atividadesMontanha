struct StackMin {
    dados: Vec<i32>,
    minimos: Vec<i32>,
}

impl StackMin {
    fn new() -> Self {
        StackMin {
            dados: Vec::new(),
            minimos: Vec::new(),
        }
    }

    fn push(&mut self, valor: i32) {
        self.dados.push(valor);

        if self.minimos.len() == 0 {
            self.minimos.push(valor);
        } else {
            let menor_atual = self.minimos[self.minimos.len() - 1];

            if valor <= menor_atual {
                self.minimos.push(valor);
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.dados.len() == 0 {
            return None;
        }

        let valor = self.dados.pop().unwrap();

        if self.minimos.len() > 0 {
            let menor_atual = self.minimos[self.minimos.len() - 1];

            if valor == menor_atual {
                self.minimos.pop();
            }
        }

        Some(valor)
    }

    fn min(&self) -> Option<i32> {
        if self.minimos.len() == 0 {
            None
        } else {
            Some(self.minimos[self.minimos.len() - 1])
        }
    }
}

fn main() {
    let mut pilha = StackMin::new();

    pilha.push(5);
    pilha.push(3);
    pilha.push(7);
    pilha.push(2);

    println!("Exercicio 9 — Pilha com minimo");
    println!("Menor valor atual: {:?}", pilha.min());

    pilha.pop();

    println!("Menor valor depois do pop: {:?}", pilha.min());
}