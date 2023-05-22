use std::fs::DirEntry;
use std::{fs, path};
use std::path::{Path, PathBuf};
use clap::Error;
use procfs::process::Process;

/* Estructura de datos ProcData                             */
/* Describe los datos recopilados del directorio /proc      */
/* en el que se almacena información sobre los procesos     */
/* que existen en tiempo de ejecución.                      */
struct ProcData {
    num_active_process: usize,
    pid_list: Vec<i32>, // Lista de procesos obtenidos de la lectura de /proc
}

impl ProcData {
    
    // Constructor
    fn new(num_active_process: usize, pid_list: Vec<i32>) -> Self {
        Self { //puede que sea ProcData
            num_active_process: 0,
            pid_list: Vec::new(),
        }
    }

    fn get_pid_list(&self) -> &Vec<i32> {
        &self.pid_list
    }

    fn get_num_active_process(&self) -> &usize {
        &self.num_active_process
    }

    fn set_pid_list(&mut self, new_pid_list: Vec<i32>) {
        self.pid_list = new_pid_list
    }
}

// Método que lista los procesos que hay activos, listando las caprtas con nombre un entero en /proc
pub fn process_list() -> ProcData {

    let mut proc_data: ProcData = ProcData::new(0, vec![0]);

    println!("🧙🧙🧙 {:?}",  proc_data.pid_list);

    let proc_path = Path::new("/proc");

    for entry in fs::read_dir(proc_path).expect("Can't access to /proc.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(pid) = path.file_name().unwrap().to_str().unwrap().parse::<i32>() {
                    proc_data.pid_list.insert(proc_data.num_active_process, pid);
                    proc_data.num_active_process += 1;
                }   
            } 
        }
    }

    println!("\n🧙🧙🧙 {:?}",  proc_data.num_active_process);
    println!("process_list: {:?}",  proc_data.pid_list);

    proc_data // return
}


pub fn compare_proc_dir(){

    
    //vector de procesos nuevos
    //vector de procesos viejos
    //se comparan los vectores y se saca el proceso diferente --> Se pasa el proceso a process_data() y se obtiene la info
}

// Método que lee el contenido de la carpeta proceso y saca información de él.
pub fn process_data(){
    // se saca la info del proceso nuevo obtenido de compare_proc_dir()
    // se formatea la info a json y se añade al log
}




