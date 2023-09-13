use std::ffi::CString;
use chrono::{self, DateTime, Utc};
use std::fs;
use log::{LevelFilter, trace};

use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
struct log_form {
    name: CString,
    argv: Vec<CString>,
    envp: Vec<CString>,
    date: Utc
}

impl log_form {
    fn save_data(filename: &CString, argv: &[CString], envp: &[CString]) -> bool{
        let path= String::from("~/Desktop"); 
        true
    }
}

fn log(log_form: log_form) -> bool {

    
    true
}


//mod info_process_proc;


pub fn write_to_log(output: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("thoots.log")?; // Nombre del archivo de registro

    writeln!(file, "{}", output)?; // Escribir la salida en el archivo
    Ok(())
}