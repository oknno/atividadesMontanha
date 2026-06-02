# Aula 05 — TADs Lineares: Vec, Pilha, Fila e Deque

Disciplina: Estruturas de Dados e Análise de Algoritmos

---

## Exercício 1

**Complexidade:** O(n)  
**Justificativa:**  
Percorre o vetor removendo os elementos com `pop` e inserindo em outro vetor com `push`. Depende da quantidade de elementos.

---

## Exercício 2

**Complexidade:** O(n)  
**Justificativa:**  
Percorre o `Vec<char>` uma vez e atualiza a contagem de cada letra em um `HashMap`.

---

## Exercício 3

**Complexidade:** O(n)  
**Justificativa:**  
Percorre todos os números do vetor e copia apenas os ímpares para outro vetor. Usa espaço auxiliar.

---

## Exercício 4

**Complexidade:** O(n log n)  
**Justificativa:**  
Junta os dois vetores com `extend` e depois ordena tudo com `sort`, que tem custo maior que uma mescla manual.

---

## Exercício 5

**Complexidade:** O(n)  
**Justificativa:**  
Percorre cada token da expressão RPN uma vez. Os números são empilhados e os operadores removem elementos da pilha.

---

## Exercício 6

**Complexidade:** O(1) por operação  
**Justificativa:**  
As operações de voltar e avançar usam `push` e `pop` em pilhas, que são operações de tempo constante.

---

## Exercício 7

**Complexidade:** O(1) para desfazer/refazer simples  
**Justificativa:**  
Cada ação é guardada em uma pilha. Para desfazer ou refazer, apenas move a última ação entre as pilhas.

---

## Exercício 8

**Complexidade:** O(n)  
**Justificativa:**  
Percorre a expressão uma vez. Os símbolos de abertura são empilhados e os de fechamento são comparados com o topo da pilha.

---

## Exercício 9

**Complexidade:** O(1) para `push`, `pop` e `min`  
**Justificativa:**  
Usa uma pilha principal e uma pilha auxiliar de mínimos. Assim, o menor valor atual fica sempre disponível no topo da pilha auxiliar.

---

## Exercício 10

**Complexidade:** O(n)  
**Justificativa:**  
Cada cliente entra na fila e é atendido uma única vez. O tempo médio de espera é calculado durante o processamento.

---

## Exercício 11

**Complexidade:** O(n)  
**Justificativa:**  
Cada trabalho de impressão é inserido na fila e removido uma vez, seguindo a ordem de chegada.

---

## Exercício 12

**Complexidade:** O(1) por inserção  
**Justificativa:**  
Quando o buffer está cheio, remove a mensagem mais antiga da frente e insere a nova no final. As operações do `VecDeque` nas pontas são constantes.

---

## Exercício 13

**Complexidade:** O(n) para remover  
**Justificativa:**  
A fila de prioridade foi feita com busca linear. Para remover, percorre os itens procurando a maior prioridade. Em caso de empate, mantém a ordem de chegada.

---

## Exercício 14

**Complexidade:** O(n)  
**Justificativa:**  
Percorre a string para montar o `VecDeque` e depois compara os caracteres do começo e do fim.

---

## Exercício 15

**Complexidade:** O(n)  
**Justificativa:**  
Usa um `VecDeque` para manter os possíveis máximos da janela. Cada elemento entra e sai do deque no máximo uma vez.

---

## Exercício 16

**Complexidade:** O(1) por operação  
**Justificativa:**  
Tarefas urgentes entram pela frente, tarefas normais entram pelo fundo e todas saem pela frente. Essas operações são eficientes com `VecDeque`.

---

## Exercício 17

**Complexidade:** Vec ingênua O(n²), VecDeque O(n), Fila Circular O(n)  
**Justificativa:**  
A fila com `Vec` usa `remove(0)`, que desloca os elementos e custa O(n) por remoção. Já `VecDeque` e fila circular removem da frente em O(1).

---

## Exercício 18

**Complexidade:** Análise conceitual  
**Justificativa:**  
O exercício compara qual TAD usar em cada situação. A escolha depende do comportamento necessário, como LIFO para pilha, FIFO para fila e acesso pelas duas pontas para deque.

---

## Exercício 19

**Complexidade:** O(n)  
**Justificativa:**  
Cada elemento da fila é removido e processado uma única vez. Os lotes apenas agrupam os elementos durante a execução.

---

## Exercício 20

**Complexidade:** O(r)  
**Justificativa:**  
No Round Robin, cada processo pode voltar várias vezes para a fila até terminar. O custo depende da quantidade total de rodadas necessárias.
