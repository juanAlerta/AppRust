use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // de momento 3 argumentos maximo
    parse_config(&args); 

    /* 
    println!("Buscando {}", query);
    println!("en el archivo {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Algo ha salido mal leyendo el fichero");

    println!("With text:\n{}", contents);
    */
}

// parse_config recoge lo argumentos introducidos
fn parse_config(args: &[String]) {
    let arg1 = &args[0];
    let arg2 = &args[1];
    let arg3 = &args[2];    

    match arg1 {         
        a => {
            println!("Argumento 1 a");
        },
        b => {
            println!("Argumento 1 b");
        },
        c => {
            println!("Argumento 1 c");
        },
        _ => {
            println!("No hay argumentos");
        },
    }
    match arg2 {         
        a => {
            println!("Argumento 2 a");
        },
        b => {
            println!("Argumento 2 b");
        },
        c => {
            println!("Argumento 2 c");
        }
    }
    match arg3 { 
        a => {
            println!("Argumento 3 a");
        },
        b => {
            println!("Argumento 3 b");
        },
        c => {
            println!("Argumento 3 c");
        }
    }
}

