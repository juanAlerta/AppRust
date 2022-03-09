use std::{env, num::ParseIntError};
use clap::{Arg, Command};
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
        .about("software activity register")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Tooth Development Team")
        // time
        .arg(Arg::with_name("time")
            .short('t')
            .long("time")
            .takes_value(false)
            .help("Define the time that Tooth will be working.")
            
        )  

        .arg(Arg::with_name("info")
            .short('i')
            .long("info")
            .help("Show info about the config.")
            
            
        )
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        
    .get_matches();
    
    match matches.subcommand() {
        Some(("time", time_matches)) => {

            let tiempo_activo = matches.value_of("time");
            match tiempo_activo {
                None => println!("☹️ No active time indicated ☹️"),
                Some(s) => {
                    match s.parse::<u16>() {
                        Ok(n) => println!("starting registry for {} along",n),
                        Err(_) => println!("☹️ No valid format. Expected Integer ☹️"),
                    }
                }
            }       
        }

        Some(("info", time_matches)) => {

            println!("Esta es la info 🧐🧐🧐🧐🧐🧐🧐🧐 ");
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
