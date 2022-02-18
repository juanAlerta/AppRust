use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
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

