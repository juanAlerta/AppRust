use std::fs::DirEntry;
use std::{fs, path};
use std::path::{Path, PathBuf};
use clap::Error;
use procfs::process::Process;

pub fn proc_data() {
    let proc_path = Path::new("/proc");
    for entry in fs::read_dir(proc_path).expect("Can't access to /proc.") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(pid) = path.file_name().unwrap().to_str().unwrap().parse::<i32>() {
                    if let Ok(process) = Process::new(pid) {
                        println!("PID: {}, Nombre: {}, Tamaño: {}", process.stat.pid, process.stat.comm, process.stat.size);
                    }
                }
            }
        }
    }
}


pub fn leer_proc_data(){
    //Recorre la carpeta
    let path_folder: &path = Path::new("/proc");
    
    for entry in fs::read_dir(path).expect("Can't access to /proc.") {
        let Ok(entry) = entry{
            let path_file = entry.path(),



        }
    }


}
