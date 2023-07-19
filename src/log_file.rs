use json::{self, JsonValue};
use std::fs::OpenOptions;
use std::io::Write;

/* Funcion de parseo a json */
/* me devuelve un json que será el que se añada el log */
fn json_parser(data: String){ // revisar referencia del parámetro 
    
}

pub fn write_to_log(output: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("~/Desktop/thoots.log")?; // Nombre del archivo de registro

    writeln!(file, "{}", output)?; // Escribir la salida en el archivo
    Ok(())
}