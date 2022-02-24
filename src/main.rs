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

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
   
    parse_config(&args); 
}

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

