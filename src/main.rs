use std::{env, num::ParseIntError};
use clap::{Arg, Command, arg};
//use std::fs;
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
        .about("Informacion del programa")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("time")
                .short_flag('t')
                .about("Define the time that the application will be doing the registry, or use <notime>")
                .arg(arg!(<UNIDAD_TIEMPO> "The time required"))
                .arg_required_else_help(true),
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
            println!("Starting registry for {} seconds along ✍️", time_matches.value_of("UNIDAD_TIEMPO").expect("required"));

                
        }

      
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }

}  

/* 
fn parse_config(args: &[String]) -> Argument {

    let ruta = String::from("C:/Users/Juan/Projects/Tooth/AppRust");

    let primary_type = Argument_primary_type::Time;
    let secundary_type = Argument_secundary_type::Zero;
    let extra_type = Argument_secundary_type::Zero;
    let time_atr = 0;
    let dir_atr = ruta;

    let mut arguments = Argument{
        primary_type: primary_type,
        secundary_type: secundary_type,
        extra_type: extra_type,
        time_atr: time_atr,
        dir_atr: dir_atr,
    };

    println!("VECTOR: {:?} ",args);

    for i in args {
        if i.as_str() == "time" {
            arguments.primary_type = Argument_primary_type::Time;
            println!("argumento time 🤠");
        }
        if i.as_str() == "notime" {
            arguments.primary_type = Argument_primary_type::Notime;
            println!("argumento notime 🤑");
        }
        if i.as_str() == "info" {
            arguments.primary_type = Argument_primary_type::Info;
            println!("argumento time 📝");
        }
        
        
    }
    

    arguments
    
}
*/
