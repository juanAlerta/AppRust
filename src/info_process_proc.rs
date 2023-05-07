use std::fs::DirEntry;
use std::{fs, path};
use std::path::{Path, PathBuf};
use clap::Error;
use procfs::process::Process;

struct Proc_info {
    procesos_activos: i32,
    
}

// Método que lista los procesos que hay actios, listando las caprtas con nombre un entero en /proc
pub fn process_list() {

    let mut proc_info =  Proc_info {
        procesos_activos: 0,
        
    };
   
    let proc_path = Path::new("/proc");
    for entry in fs::read_dir(proc_path).expect("Can't access to /proc.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(pid) = path.file_name().unwrap().to_str().unwrap().parse::<i32>() {
                    if let Ok(process) = Process::new(pid) {
                        proc_info.procesos_activos += 1;
                        print!("{:?}\t", process.pid);
                        
                    }
                }
            }
        }
    }

    println!("ActiveProcess: {:?}",  proc_info.procesos_activos);

}

// Método que lee el contenido de la carpeta proceso y saca información de él.
pub fn process_data(){
    // se saca la info del proceso nuevo obtenido de compare_proc_dir()
    // se formatea la info a json y se añade al log
}

pub fn compare_proc_dir(){
    //vector de procesos nuevos
    //vector de procesos viejos
    //se comparan los vectores y se saca el proceso diferente --> Se pasa el proceso a process_data() y se obtiene la info
}


