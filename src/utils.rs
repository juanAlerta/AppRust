use std::ffi::CString;
use chrono::{self, DateTime, Utc};
use std::fs;
use log::{LevelFilter, trace};

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