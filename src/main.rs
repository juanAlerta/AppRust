use std::{env, num::ParseIntError};
use clap::{Arg, Command, arg};

// de momento no uso los enum
enum Argument_primary_type {
    Time,
    Notime,
    Info
}
enum Argument_secundary_type {
    Dir,
    Show,
    Zero
}

struct Argument {
    primary_type: Argument_primary_type,
    secundary_type: Argument_secundary_type,
    extra_type: Argument_secundary_type,
    time_atr: u16,
    dir_atr: String,
}

fn main() {

    

    let matches = Command::new("tooth")
        .about("Tooth is a forensic computer analysis wich notes the programs running on your operative system.\nDocumentation: https://github.com/juanAlerta/AppRust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            // time
            Command::new("time")
                .short_flag('t')
                .about("Define the time that the application will be doing the registry, or use <notime>")
                .arg(arg!(<UNIDAD_TIEMPO> "The time required"))
                .arg_required_else_help(true)
                /* .arg(
                Arg::new("dir")
                    .short('d')
                    .help("Define the file path.")
                    .arg(arg!(<RUTA> "Direction"))
                    .takes_value(true),
                ) */
        )
        .subcommand(
            Command::new("notime")
                .short_flag('n')
                .about("Start registry with no time limit"), 
        )
        .subcommand(
            Command::new("info")
                .short_flag('i')
                .about("show general info")
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("time", time_matches)) => {
            let unidad_tiempo:u16 = time_matches.value_of("UNIDAD_TIEMPO").expect("required").parse().unwrap();
            // validar tipo
            println!("✍️ Starting registry for {} seconds along ✍️", time_matches.value_of("UNIDAD_TIEMPO").expect("required"));

            /*
            if time_matches.is_present("dir") {
                let direccion: Vec<_> = time_matches.values_of("dir").unwrap().collect();
                let values = direccion.join(", ");
                println!("🤓 Saving data in {} 🤓", values);
                return;
            }*/ 
        }

        Some(("notime", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            println!("✍️ Starting registry with no time limit ✍️");
        }

        Some(("info", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            println!("<INFORMACION GENERAL>");
        }
        
        _ => unreachable!(), 
    }
}  

