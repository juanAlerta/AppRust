use std::fs;
use std::path::Path;
use procfs::process::Process;

fn proc_data() {
    let proc_path = Path::new("/proc");
    for entry in fs::read_dir(proc_path).expect("Fallo al leer el directorio /proc") {
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