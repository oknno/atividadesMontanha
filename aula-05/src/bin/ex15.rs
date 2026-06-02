use std::collections::VecDeque;

fn main() {
    let v = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;

    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut resultado: Vec<i32> = Vec::new();

    for i in 0..v.len() {
        while deque.len() > 0 && deque[0] + k <= i {
            deque.pop_front();
        }

        while deque.len() > 0 {
            let ultimo_indice = deque[deque.len() - 1];

            if v[ultimo_indice] <= v[i] {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back(i);

        if i + 1 >= k {
            let indice_maior = deque[0];
            resultado.push(v[indice_maior]);
        }
    }

    println!("Exercicio 15 — Janela deslizante maxima");
    println!("Vetor: {:?}", v);
    println!("k: {}", k);
    println!("Resultado: {:?}", resultado);
}