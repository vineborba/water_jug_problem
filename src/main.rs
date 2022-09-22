use std::env;
use std::process;

use water_jug_problem::Config;

fn main() {
    let config = Config::new(env::args());

    println!("Lendo arquivo {}", config.input_file);

    if let Err(e) = water_jug_problem::run(&config) {
        eprintln!("Erro da aplicação: {}", e);
        process::exit(1);
    } else {
        println!("Resultados disponíveis em {}", config.output_file);
    }
}
