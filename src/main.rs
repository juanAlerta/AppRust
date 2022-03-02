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
        // Query subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("time")
                .short_flag('t')
                .long_flag("time")
                .about("Define the time that Tooth will be working.")
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                       //.conflicts_with("info")
                        .takes_value(true)
                        .multiple_values(true)
                        .help("specifies the registry file direction"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        //.conflicts_with("info")
                        //.takes_value(false)
                        //.multiple_values(true)
                        .help("show the process in real time"),
                )
        )
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        .subcommand(
            Command::new("notime")
                .short_flag('n')
                .long_flag("notime")
                .about("Init the anaylis without time limit.")
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                       //.conflicts_with("info")
                        .takes_value(true)
                        //.multiple_values(true)
                        .help("specifies the registry file direction"),
                )
                .arg(
                    Arg::new("show")
                        .short('s')
                        .long("show")
                        //.conflicts_with("info")
                        //.takes_value(false)
                        //.multiple_values(true)
                        .help("show the process in real time"),
                )
        )

        .subcommand(
            Command::new("info")
                .short_flag('i')
                .long_flag("info")
                .about("Show general info about the configuration.")
        )
    .get_matches();
    
    match matches.subcommand() {
        Some(("time", time_matches)) => {
            // argumentos secundarios
            if time_matches.is_present("dir") {
                let direction: String = time_matches.value_of("dir").unwrap().to_string(); //cambiar porque solo es una direccion
                println!("Making registry in {} ✍️ ", direction);
                return;
            }
            if time_matches.is_present("show") {
                println!("Showing process ...");
            }

           // argumento principal
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
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.values_of("info") {
                let comma_sep = packages.collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {}...", comma_sep);
            } else if let Some(queries) = query_matches.values_of("search") {
                let comma_sep = queries.collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {}...", comma_sep);
            } else {
                println!("Displaying all locally installed packages...");
            }
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
