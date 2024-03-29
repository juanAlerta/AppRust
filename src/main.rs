use std::{env, num::ParseIntError, time::Duration, thread};
use clap::{Arg, Command, arg};

mod info_process_proc;

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

    //let mut proc_data1: ProcData = ProcData::new(0, vec![0]);
    //proc_data1 = info_process_proc::process_list();

    let mut objeto_argumentos = Argumento {
        primary_type: Argument_primary_type::Notime,
        secundary_type: Argument_secundary_type::Zero,
        extra_type: Argument_secundary_type::Zero,
        time_atr: 0,
        dir_atr: default_dir,
    };

    let matches = Command::new("\n\n\n _____           _   _     \n|_   _|__   ___ | |_| |__  \n  | |/ _ \\ / _ \\| __| '_ \\ \n  | | (_) | (_) | |_| | | |\n  |_|\\___/ \\___/ \\__|_| |_|\n")
        .about("\nTooth is a forensic computer analysis wich notes the programs running on your operative system.\nDocumentation: https://github.com/juanAlerta/AppRust")
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
                .about("Print the log process."),
        )

        .subcommand(
            Command::new("path")
                .short_flag('p')
                .about("Change the log to different path"), 

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

            thread::sleep(Duration::from_secs(10)); //Espera en comprobacion, esto no debería ir aquí

        }

        Some(("notime", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            objeto_argumentos.primary_type = Argument_primary_type::Notime;
            println!("✍️ Starting registry with no time limit ✍️");

            thread::sleep(Duration::from_secs(10));
        }

        Some(("info", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            objeto_argumentos.primary_type = Argument_primary_type::Info;
            loop {
                //info_process_proc::process_list();
                  //info_process_proc::compare_proc_dir(info_process_proc::process_list().pid_list);
                  let new_process_list: Vec<i32> = info_process_proc::compare_proc_dir(info_process_proc::process_list().pid_list);
                  print!("\nProcesos diferentes{:?}",new_process_list);
                  info_process_proc::process_data(new_process_list);  
              }

        }

        Some(("pruebas", cosa)) => {
            loop {
              //info_process_proc::process_list();
                //info_process_proc::compare_proc_dir(info_process_proc::process_list().pid_list);
                let new_process_list: Vec<i32> = info_process_proc::compare_proc_dir(info_process_proc::process_list().pid_list);
                print!("\nProcesos diferentes{:?}",new_process_list);
                info_process_proc::process_data(new_process_list);  
            }
        }
        
        _ => unreachable!(), 
    }
}  

