use std::fs::DirEntry;
use std::{fs, path};
use std::path::{Path, PathBuf};
use clap::Error;
use procfs::process::Process;

pub struct ProcInfo {
    num_active_process: usize,
    pid_list: Vec<i32>, // Lista de procesos obtenidos de la lectura de /proc
}

// Método que lista los procesos que hay actios, listando las caprtas con nombre un entero en /proc
pub fn process_list() {

    let mut proc_info =  ProcInfo {
        num_active_process: 0,
        pid_list: Vec::new()
    };

    proc_info.pid_list.push(-1); // Cuando entra al for el proceso -1 es sustituido
    println!("🧙🧙🧙 {:?}",  proc_info.pid_list);

    let proc_path = Path::new("/proc");

    for entry in fs::read_dir(proc_path).expect("Can't access to /proc.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(pid) = path.file_name().unwrap().to_str().unwrap().parse::<i32>() {
                    //if let Ok(process) = Process::new(pid) {

                        proc_info.pid_list.insert(proc_info.num_active_process, pid);
                        proc_info.num_active_process += 1;
                        
                        // print!("{:?}\t", process.pid);
                    
                }   
            } 
        }
    }

    println!("\n🧙🧙🧙 {:?}",  proc_info.num_active_process);
    println!("process_list: {:?}",  proc_info.pid_list);

}


pub fn compare_proc_dir(){

    let aux_pid_list:Vec<i32> = Vec::new();

    let proc_path = Path::new("/proc");

    for entry in fs::read_dir(proc_path).expect("Can't access to /proc.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(pid) = path.file_name().unwrap().to_str().unwrap().parse::<i32>() {
                    //if let Ok(process) = Process::new(pid) {

                        proc_info.pid_list.insert(proc_info.num_active_process, pid);
                        proc_info.num_active_process += 1;
                        
                        // print!("{:?}\t", process.pid);
                    
                }       
            } 
        }
    }
    //vector de procesos nuevos
    //vector de procesos viejos
    //se comparan los vectores y se saca el proceso diferente --> Se pasa el proceso a process_data() y se obtiene la info
}

// Método que lee el contenido de la carpeta proceso y saca información de él.
pub fn process_data(){
    // se saca la info del proceso nuevo obtenido de compare_proc_dir()
    // se formatea la info a json y se añade al log
}




