use std::env;
//use std::fs;

enum Argument_primary_type {
    Time,
    Notime,
    Info
}

enum Argument_secundary_type {
    Dir,
    Show,
}

struct Arguments {
    primary_type: Argument_primary_type,
    secundary_type: Argument_secundary_type,
    extra_type: Argument_secundary_type,
    time_atr: u16,
    dir_atr: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    args.remove(0); 
    parse_config(&args); 

    /* 
    let contents = fs::read_to_string(filename)
        .expect("Algo ha salido mal leyendo el fichero");
    */
}

// parse_config recoge lo argumentos introducidos
fn parse_config(args: &[String]) {

    //parametros
    let arg_time = String::from("time");
    let arg_notime = String::from("notime");


    for i in args {
        println!("{}",i);
        println!("{}",i.eq(&arg_time));
    }

    /* 
    let arg1 = &args[0];
    let arg2 = &args[1];
    let arg3 = &args[2];    
    */

    
    
}

