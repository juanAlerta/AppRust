use std::fs::OpenOptions;
use std::io::Write;


//mod info_process_proc;

pub fn write_to_log(output: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("thoots.log")?; // Nombre del archivo de registro

    writeln!(file, "{}", output)?; // Escribir la salida en el archivo
    Ok(())
}