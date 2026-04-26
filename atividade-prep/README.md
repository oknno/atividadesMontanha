# Estudo de Caso A1 — O Problema da Entrega Inteligente

# Questão 1

## a)

O problema da FastBite pode ser considerado NP-Completo, porque ele é parecido com o problema do TSP e VRP.

Problemas da classe P conseguem ser resolvidos rapidamente. Já os NP são problemas onde conseguimos verificar a solução rapidamente.

No caso da FastBite existem muitas combinações possíveis de entregas e rotas, então encontrar a melhor solução é muito difícil.

---

## b)

O problema pode ser reduzido ao TSP porque os restaurantes e clientes podem ser vistos como cidades.

O entregador teria que passar pelos pontos tentando encontrar a menor rota possível.

Então:

- restaurantes = cidades
- clientes = cidades
- rota = caminho do caixeiro viajante

---

## c)

A força bruta é inviável porque o número de combinações aumenta muito rápido.

Com 8 pedidos:

```text
8! = 40320
```

E ainda existem as combinações entre os entregadores.

Então o sistema demoraria muito para calcular tudo.

A complexidade fica:

```text
O(n!)
```

---

# Questão 2

## a)

O algoritmo escolhe o entregador mais próximo do restaurante.

Depois disso ele continua escolhendo sempre o próximo ponto mais próximo.

Assim ele consegue tomar decisões rápidas.

---

## b)

Ele é considerado guloso porque sempre pega a melhor opção naquele momento.

Ele não pensa no resultado final, apenas na melhor escolha local.

---

## c)

Pode acontecer do algoritmo escolher um entregador que está perto no começo, mas depois ele fique muito longe das próximas entregas.

Outro entregador poderia fazer uma rota melhor, mas o algoritmo não percebe isso.

---

## d)

A complexidade pode ser:

```text
O(n * m)
```

Porque cada pedido precisa verificar os entregadores disponíveis.

---

# Questão 3

## a)

A Programação Dinâmica pode ser usada no roteamento de um entregador.

Ela evita repetir cálculos já feitos anteriormente.

Mas conforme aumenta o número de pedidos, o processamento fica muito pesado.

Então acaba não sendo muito viável em tempo real.

---

## b)

Divisão e Conquista poderia dividir a cidade em regiões.

Assim cada região teria seus próprios pedidos e entregadores.

Isso ajuda na velocidade do sistema.

O problema são os pedidos perto das fronteiras das regiões.

---

# Questão 4

A abordagem Greedy possui uma qualidade média de solução, porque ela toma decisões rápidas e locais. O lado positivo é que ela possui baixo custo de processamento e consegue funcionar bem em tempo real.

A Programação Dinâmica possui uma qualidade melhor, porque consegue encontrar soluções mais próximas do ideal. Porém, ela possui um custo muito alto de processamento e memória, principalmente quando aumenta a quantidade de pedidos.

Já a Divisão e Conquista consegue melhorar o desempenho dividindo o problema em partes menores, como regiões da cidade. Isso ajuda na velocidade, mas ainda pode apresentar problemas em pedidos próximos das fronteiras entre regiões.

No caso da FastBite, a abordagem gulosa parece ser a mais adequada, porque o sistema precisa responder muito rápido. Mesmo que ela não encontre sempre a melhor solução possível, ela consegue entregar resultados bons em pouco tempo.

---

# Questão 5

## a)

Heurística é uma solução aproximada.

Ela não encontra sempre a melhor resposta, mas encontra uma resposta boa rapidamente.

---

## b)

Uma solução real poderia:

1. dividir os pedidos por região
2. usar um algoritmo guloso
3. melhorar algumas rotas
4. parar o processamento em até 2 segundos

---

## c)

Vale buscar a solução ótima quando existem poucos pedidos.

Por exemplo em entregas especiais ou horários com pouco movimento.

---

# Questão 6

Nem sempre a melhor solução matemática é a melhor solução prática.

Em sistemas grandes o mais importante é responder rápido e funcionar bem.

Por isso muitas empresas preferem soluções aproximadas em vez da solução perfeita.

```

```
