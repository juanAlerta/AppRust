mod utils;

use std::ffi::CString;

pub trait evec_wrapper {
    fn execv_wrapper(path: &CString, argv: &[CString]);
    fn execve_wrapper(path: &CString, argv: &[CString], envp: &[CString]);

}