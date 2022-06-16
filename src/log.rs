

fn log_syscall_execv(filename: &CString, argv: [&CString]) {

    let envp: [&CString] = {Null};
    log_syscall_exec(filename, argv, envp);
}

fn log_syscall_execve(filename: &CString, argv: [&CString], envp: [&CString]) {
    
    log_syscall_exec(filename, argv, envp);
}

fn log_syscall_exec(filename: &CString, argv: [&CString], envp: [&CString]) {

    let mut logMessage: &CString;
    // let configuracion: Objeto Configuracion
    

    
}