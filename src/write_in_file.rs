use std::fs::OpenOptions;
use std::io::Write;

pub fn write_to_log(output: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("~/Desktop/thoots.log")?; // Nombre del archivo de registro

    writeln!(file, "{}", output)?; // Escribir la salida en el archivo
    Ok(())
}