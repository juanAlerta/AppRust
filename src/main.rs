use std::{env, num::ParseIntError};
use clap::{Arg, Command, arg};
use info_process_proc::ProcData;

mod info_process_proc;
mod log;

#[derive(Debug)]
enum Argument_primary_type {
    Time,
    Notime,
    Info
}
#[derive(Debug)]
enum Argument_secundary_type {
    Dir,
    Show,
    Zero
}

#[derive(Debug)]
pub struct Argumento {
    primary_type: Argument_primary_type,
    secundary_type: Argument_secundary_type,
    extra_type: Argument_secundary_type,
    time_atr: u32,
    dir_atr: String,
}


fn main() {

    let mut default_dir = String::from("~/Desktop"); 

    let mut objeto_argumentos = Argumento {
        primary_type: Argument_primary_type::Notime,
        secundary_type: Argument_secundary_type::Zero,
        extra_type: Argument_secundary_type::Zero,
        time_atr: 0,
        dir_atr: default_dir,
    };

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
    
        /* TO DO:                                    */
        /*  - meter sub argumentos de alguna forma  */
        /*  - validar tiempo                        */
        

    match matches.subcommand() {
        Some(("time", time_matches)) => {
            let unidad_tiempo:u32 = time_matches.value_of("UNIDAD_TIEMPO").expect("required").parse().unwrap();
            
            // - validar tipo

            objeto_argumentos.primary_type = Argument_primary_type::Time;
            objeto_argumentos.time_atr = unidad_tiempo;
            println!("✍️ Starting registry for {} minutes along ✍️", time_matches.value_of("UNIDAD_TIEMPO").expect("required"));
            println!("🐜 DEBUGGING -->  primary_type: {:?} , time_atr: {:?} 🐜", objeto_argumentos.primary_type, objeto_argumentos.time_atr);
        }

        Some(("notime", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            objeto_argumentos.primary_type = Argument_primary_type::Notime;
            println!("✍️ Starting registry with no time limit ✍️");
        }

        Some(("info", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            objeto_argumentos.primary_type = Argument_primary_type::Info;
            println!("<INFORMACION GENERAL>");
        }

        Some(("pruebas", cosa)) => {
            info_process_proc::process_list();
            info_process_proc::compare_proc_dir(vec![1, 7, 8, 10, 181]);

            info_process_proc::process_data(1);
        }
        
        _ => unreachable!(), 
    }
}  

