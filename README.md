# Os Três Jarros do Sheik

Este repositório contém o código referente a resolução de um problema dado como um trabalho de Algorítmos e Estrutura de Dados II.

----

O problema consiste em receber um conjunto de entradas através de um arquivo de texto, as quais representam o seguinte modelo:

- A primeira linha dita quais as capacidades dos jarros;
- A segunda linha dita a quantidade inicial de cada jarro;
- A terceira linha dita a quantidade desejada de água em cada jarro;

Para o desenvolvimento da solução, temos 3 regras a serem seguidas:

- É proibido jogar água fora;
- É proibido adicionar água nos jarros além do valor inicial dado;
- Pode-se somente esvaziar um jarro em outro ou completar um jarro até a borda;

Com a entrada de exemplo a seguir, pode-se resolver o problema com 2 movimentos:

```
6 10 15 -> capacidade de cada jarro
5  7  8 -> valor inicial de cada jarro
0  5 15 -> valor desejado em cada jarro
```

## Modelagem do problema

Para a solução do problema, foram modelados as seguintes estruturas de dados;
```rust
#[derive(Clone)]
struct Jug {
    capacity: i32,
    current: i32,
}

#[derive(Clone)]
struct Movement {
    state: Vec<Jug>,
    counter: i32,
}

struct Round {
    capacities: Vec<i32>,
    initial_volumes: Vec<i32>,
    desired_volumes: Vec<i32>,
    movements: i32,
}
```
- Jug é a representação de um jarro, guardado sua capacidade total e volume atual (inicializado como a quantidade de água inicial de cada um);

- Movement guarda o estado dos jarros atuais em um vetor e possui um contador para saber quantos movimentos foram realizados para chegar nesse estado;

- Round é a representação de uma entrada lida do arquivo, já que é possível ler várias entradas em sequência;

- Jug e Movement derivam a implementação do _trait_ clone, necessário para o funcionamento do algorítmo com uma fila (explicado abaixo);

- É também feita uma implementação para o _trait_ `Display` para a estrutura Round, para ser salva em um arquivo de texto ao final do processamento do mesmo;


## Desenvolvimento da solução

O algoritmo implementado funciona como uma "força bruta", onde é gerado todos possíveis movimentos a partir de um estado, testando os mesmos conforme a geração, e armazenando-os em uma fila.

Para não haver processamento repetido de valores, estados já testados são armazenados em um HashSet, para assim serem ignorados conforme forem sendo repetidos.

```rust
// Inicialização dos jarros a cada round de entrada
let jugs = initialize_jugs(&round);
let jugs_lenght = jugs.len();

// Valores já processados
let mut calculated_values: HashSet<Vec<i32>> = HashSet::new();

// Inicialização da fila com o estado inicial
let mut q: Queue<Movement> = queue![];
q.add(Movement {
    state: jugs,
    counter: 0,
})?;
```

O processamento é feito através da fila de movimentos gerados, rodando em um loop while enquanto a fila não estiver vazia ou o resultado não tenha sido atingido.

Para cada movimento executado, a referência do movimento é clonada para que seja possível reaproveitar o estado. Há dois loops for, onde há uma iteração do jarro origem (_from_) para o jarro destino (_to_). Caso a origem e destino sejam o mesmo, a iteração é ignorada.

Ao final de cada round, o resultado é escrito no arquivo de saída.
```rust
'counter: while q.size() != 0 {
    let current_movement = q.remove()?;
    for from in 0..jugs_lenght {
        for to in 0..jugs_lenght {
            // Ignorando a iteração caso origem e destino sejam o mesmo jarro
            if from == to {
                continue;
            }

            // Clonando a referência do movimento atual
            let mut new_movement = current_movement.clone();
            pour(&mut new_movement.state, from, to);

            // Incrementando o contador de movimento
            new_movement.counter += 1;

            // Criando um vetor com os volumes após movimentar a água
            let current_volumes: Vec<i32> =
                new_movement.state.iter().map(|j| j.current).collect();
            // Checando caso o volume já tenha sido processado
            if calculated_values.contains(&current_volumes) {
                continue;
            }

            // Comparando com o desejado
            if compare_to_desired(&current_volumes, &round.desired_volumes) {
                // Caso seja o valor desejado, é salvo o contado do movimento no estado do round
                // para que seja impresso no arquivo de saída
                round.movements = new_movement.counter;
                // Quebra do laço while
                break 'counter;
            }
            // Inserindo os valores processados no HashSet
            calculated_values.insert(current_volumes);
            // Inserindo o novo movimento gerado na fila para processamento futuro
            q.add(new_movement)?;
        }
    }
}
writeln!(&mut file, "{}", round)?;
```
---
## Como executar?

Para compilar e rodar local é necessário ter instalado a toolchain da linguagem, encontrada em seu [site oficial](https://www.rust-lang.org/pt-BR/tools/install)


Há um replit online onde o código pode ser executado, encontrado em [aqui](https://replit.com/@Esqu1l0/waterjugproblem)

O programa aceita 2 argumentos, o primeiro é o nome do arquivo de entrada e o segundo é o nome do arquivo de saída.

Por padrão, o programa está lendo um arquivo com nome `entrada_exemplo_T1.txt`, que pode ser encontado no repositório. Há um arquivo chamado `test-inputs.txt` que também pode ser utilizado, que possui mais casos de execução.

---
## Por que Rust?

Rust é uma linguagem poderosa, possuindo um sistema de gerenciamento de memória avançado sem que haja a necessidade de um _garbage colector_.

Isso possiblita que o desenvolvimento de algorítmos seja seguro, pois seu compilador possui checagens de má utilização de memória em tempo de compilação.

Além disso, assim que um valor sai do escopo de execução, ele é automaticamente desalocado da memória e o recurso é liberado. Sendo assim, o programa roda de forma muito eficiente tanto no consumo de recursos como também em velocidade.