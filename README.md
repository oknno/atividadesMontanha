Exercícios de Estruturas de Dados e Análise de Algoritmos
Identificação do Estudante

Nome completo: Matheus Okano
Turma: Noite
Disciplina: Estruturas de Dados e Análise de Algoritmos

Descrição dos Exercícios Entregues

Este repositório contém os exercícios práticos desenvolvidos ao longo do semestre na disciplina de Estruturas de Dados e Análise de Algoritmos.

As atividades foram organizadas com o objetivo de regularizar as entregas pendentes e consolidar os principais conceitos estudados durante a disciplina.

Os exercícios abordam temas como:

estruturas de dados lineares e não lineares;
listas, pilhas, filas e conjuntos;
árvores;
algoritmos de busca;
algoritmos de ordenação;
recursão;
análise de complexidade;
aplicação da notação Big-O, quando solicitada.
Organização do Repositório

O repositório está organizado em pastas, separando os exercícios por aula, lista ou atividade.

Cada pasta contém os arquivos relacionados aos exercícios correspondentes, incluindo:

código-fonte;
comentários explicativos, quando necessário;
indicação da estratégia ou estrutura de dados utilizada;
análise de complexidade, quando aplicável;
um README.md próprio com orientações específicas de execução.

Exemplo de organização:

estrutura-de-dados-rust/
│
├── README.md
│
├── aula-01/
│   ├── README.md
│   ├── src/
│   └── Cargo.toml
│
├── aula-02/
│   ├── README.md
│   ├── src/
│   └── Cargo.toml
│
└── aula-03/
    ├── README.md
    ├── src/
    └── Cargo.toml
Instruções Gerais para Execução dos Códigos

Os exercícios foram desenvolvidos em Rust e podem ser executados utilizando o Cargo.

Antes de executar os códigos, verifique se o Rust e o Cargo estão instalados:

rustc --version
cargo --version

Para clonar o repositório:

git clone https://github.com/oknno/estrutura-de-dados-rust.git

Acesse a pasta do repositório:

cd estrutura-de-dados-rust

Depois, entre na pasta da atividade desejada:

cd [nome-da-pasta]

Cada pasta possui um arquivo README.md próprio com os comandos específicos para execução dos exercícios daquela atividade.

Exemplo de Execução

Em algumas pastas, a execução pode seguir o seguinte padrão:

cargo run --release

Para executar exercícios específicos:

cargo run --bin ex01 --release
cargo run --bin ex02 --release
cargo run --bin ex03 --release
cargo run --bin ex04 --release

O comando pode variar conforme a organização de cada atividade. Por isso, recomenda-se consultar o README.md da pasta correspondente antes da execução.
