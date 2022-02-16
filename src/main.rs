use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("Buscando {}", query);
    println!("en el archivo {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Algo ha salido mal leyendo el fichero");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
