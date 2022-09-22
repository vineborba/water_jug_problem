extern crate queues;

use core::fmt;
use queues::*;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;

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

impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let capacities_string = vec_to_string(&self.capacities);
        let initials_string = vec_to_string(&self.initial_volumes);
        let desireds_string = vec_to_string(&self.desired_volumes);
        write!(
            f,
            "{}\n{}\n{}\nMovimentos: {}",
            capacities_string, initials_string, desireds_string, self.movements
        )
    }
}

pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Config {
        args.next();

        let input_file = match args.next() {
            Some(arg) => arg,
            None => {
                println!("Nome de arquivo de entrada não informado, lendo arquivo padrão");
                "entrada_exemplo_T1.txt".to_string()
            }
        };

        let output_file = match args.next() {
            Some(arg) => arg,
            None => {
                println!("Nome de arquivo de saída não informado, escrevendo no arquivo padrão");
                "saida_exemplo_T1.txt".to_string()
            }
        };

        Config {
            input_file,
            output_file,
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.input_file)?;
    let test_rounds = parse_input_content(&contents);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&config.output_file)?;

    for mut round in test_rounds {
        let jugs: Vec<Jug> = initialize_jugs(&round);
        let jugs_lenght = jugs.len();

        let mut calculated_values: HashSet<Vec<i32>> = HashSet::new();
        let mut q: Queue<Movement> = queue![];
        q.add(Movement {
            state: jugs,
            counter: 0,
        })?;

        'counter: while q.size() != 0 {
            let current_movement = q.remove()?;
            for i in 0..jugs_lenght {
                for j in 0..jugs_lenght {
                    if i == j {
                        continue;
                    }

                    let mut new_movement = current_movement.clone();
                    pour(&mut new_movement.state, i, j);
                    new_movement.counter += 1;

                    let current_volumes: Vec<i32> =
                        new_movement.state.iter().map(|j| j.current).collect();
                    if calculated_values.contains(&current_volumes) {
                        continue;
                    }
                    if compare_to_desired(&current_volumes, &round.desired_volumes) {
                        round.movements = new_movement.counter;
                        break 'counter;
                    }
                    calculated_values.insert(current_volumes);
                    q.add(new_movement)?;
                }
            }
        }
        if round.movements > 0 {
            writeln!(&mut file, "{}", round)?;
        } else {
            println!("Round inválido");
        }
    }
    Ok(())
}

fn parse_input_content(contents: &String) -> Vec<Round> {
    let mut lines = contents.lines();
    let mut test_rounds = vec![];
    let mut next_line = lines.next();
    while next_line != None {
        let mut capacities: Vec<i32> = vec![];
        let mut initial_volumes: Vec<i32> = vec![];
        let mut desired_volumes: Vec<i32> = vec![];
        for index in 0..4 {
            let line = next_line.unwrap();
            let mod_line = index % 4;
            if mod_line == 0 {
                capacities = parse_input_line(line);
            } else if mod_line == 1 {
                initial_volumes = parse_input_line(line);
            } else if mod_line == 2 {
                desired_volumes = parse_input_line(line);
            }
            next_line = lines.next();
        }
        test_rounds.push(Round {
            capacities,
            initial_volumes,
            desired_volumes,
            movements: 0,
        });
    }
    test_rounds
}

fn parse_input_line(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(|c| c.parse::<i32>().expect("Valor de entrada inválido"))
        .collect()
}

fn initialize_jugs(round: &Round) -> Vec<Jug> {
    let mut jugs: Vec<Jug> = vec![];
    for i in 0..3 {
        let capacity = round.capacities[i];
        let current = round.initial_volumes[i];
        jugs.push(Jug { current, capacity })
    }
    jugs
}

fn pour(jugs: &mut Vec<Jug>, i: usize, j: usize) {
    let mut new_current = jugs[i].current + jugs[j].current;
    let mut leftover = 0;
    if new_current > jugs[j].capacity {
        leftover = new_current - jugs[j].capacity;
        new_current = jugs[j].capacity;
    }
    jugs[i].current = leftover;
    jugs[j].current = new_current;
}

fn vec_to_string(vec: &Vec<i32>) -> String {
    vec.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn compare_to_desired(current_volumes: &Vec<i32>, desired_volumes: &Vec<i32>) -> bool {
    for i in 0..3 {
        if current_volumes[i] != desired_volumes[i] {
            return false;
        }
    }
    return true;
}
