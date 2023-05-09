use std::fs::DirEntry;
use std::{fs, path};
use std::path::{Path, PathBuf};
use clap::Error;
use procfs::process::Process;

pub struct ProcInfo {
    num_active_process: usize,
    process_list: Vec<i32>,
}

// Método que lista los procesos que hay actios, listando las caprtas con nombre un entero en /proc
pub fn process_list() {

    let mut proc_info =  ProcInfo {
        num_active_process: 0,
        process_list: Vec::new(),
    };

    proc_info.process_list.push(-1);
    let mut i = 0;

    let proc_path = Path::new("/proc");

    for entry in fs::read_dir(proc_path).expect("Can't access to /proc.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(pid) = path.file_name().unwrap().to_str().unwrap().parse::<i32>() {
                    if let Ok(process) = Process::new(pid) {

                        proc_info.process_list[proc_info.num_active_process] = pid;  
                        print!("tamman = {:?}\t",proc_info.process_list.len());
                        proc_info.num_active_process += 1;
                        
                        print!("{:?}\t", process.pid);
                    }
                }   
            }
        }
    }

    println!("ActiveProcess: {:?}",  proc_info.num_active_process);
    println!("process_list: {:?}",  proc_info.process_list);

}


pub fn compare_proc_dir(old_process: ProcInfo, new_process: ProcInfo){
    //vector de procesos nuevos
    //vector de procesos viejos
    //se comparan los vectores y se saca el proceso diferente --> Se pasa el proceso a process_data() y se obtiene la info
}

// Método que lee el contenido de la carpeta proceso y saca información de él.
pub fn process_data(){
    // se saca la info del proceso nuevo obtenido de compare_proc_dir()
    // se formatea la info a json y se añade al log
}




