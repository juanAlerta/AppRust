use std::fs::DirEntry;
use std::{fs, path};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
//use procfs::process::Process;

/* Estructura de datos ProcData                             */
/* Describe los datos recopilados del directorio /proc      */
/* en el que se almacena información sobre los procesos     */
/* que existen en tiempo de ejecución.                      */
pub struct ProcData {
    num_active_process: usize,
    pid_list: Vec<i32>, // Lista de procesos obtenidos de la lectura de /proc
}

impl ProcData {
    
    // Constructor
    pub fn new(num_active_process: usize, pid_list: Vec<i32>) -> Self {
        Self { //puede que sea ProcData
            num_active_process: num_active_process,
            pid_list: pid_list,
        }
    }
    /* 
    fn get_pid_list(&self) -> &Vec<i32> {
        &self.pid_list
    }

    fn get_num_active_process(&self) -> &usize {
        &self.num_active_process
    }
    */
    fn set_pid_list(&mut self, new_pid_list: Vec<i32>) {
        self.pid_list = new_pid_list
    }
    
}

struct ProcessInfo {
    id: i32, // pid
    cmd_line: String, // comando con el que se ha lanzado
    environ: String, // variables de entorno
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


pub fn compare_proc_dir(old_vector: Vec<i32>) -> Vec<Option<i32>>{

    let old_proc_data_vec = old_vector; // procesos de un momento anterior (hace 3'')
    let new_proc_data_vec: Vec<i32> = process_list().pid_list; // procesos en este momento exacto

    //let mut value: i32;

    // thread::sleep(Duration::from_secs(3)); //Espera en comrobacion, esto no debería ir aquí

    let diff_process: Vec<Option<i32>> = old_proc_data_vec.iter().map(|&x| {
        if new_proc_data_vec.contains(&x) {
            None
        } else {
            Some(x)
            

        }
    }).collect();
    
    println!("🧝 Procesos diferentes: {:?}",  diff_process);
    
    diff_process
}

// Método que lee el contenido de la carpeta proceso y saca información de él.
// Devuelve un objeto con las caracterísicas importantes
pub fn process_data(pid: i32) {

    let proc_path = Path::new("/proc").join(pid.to_string());

    if proc_path.exists() && proc_path.is_dir() {
        // Acceder a los archivos relevantes en el directorio del proceso
        let cmdline_path = proc_path.join("cmdline");
        let status_path = proc_path.join("status");
        let environ_path = proc_path.join("environ");

        // Leer el contenido de los archivos
        if let Ok(cmdline_content) = fs::read_to_string(cmdline_path) {
            println!("🤠 Command line: {}", cmdline_content);
        }

        if let Ok(status_content) = fs::read_to_string(status_path) {
            println!("🤠 Status: {}", status_content);
        }

        if let Ok(environ_content) = fs::read_to_string(environ_path) {
            println!("🤠 Environ: {}", environ_content);
        }

        } else {
            println!("No se encontró el proceso con PID {}", pid);
        }
    // se saca la info del proceso nuevo obtenido de compare_proc_dir()
    // se formatea la info a json y se añade al log
}





