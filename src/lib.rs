//! The build.rs automatically links to the correct library (libreadline on
//! Linux, libedit on Mac OS X)
#![feature(libc, std_misc)]
extern crate libc;
use self::libc::{c_char, c_void};
use self::libc::funcs::c95::stdlib;

use std::ffi::{CString, CStr};

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod ffi {
    use libc::c_char;
    extern "C" {
        /// Reads a line from the command line
        ///
        /// **Arguments**
        /// prompt: prompt to prompt the user with
        ///
        /// **Returns**
        /// returns a cstr if it succeeds, NULL if EOF
        pub fn readline(prompt: *const c_char) -> *const c_char;
        /// Allows a person to press the up arrow, getting their history
        ///
        /// Useful if you're making an interpreter, or a shell, for example
        ///
        /// **Arguments**
        /// line: the actual line to add to history
        pub fn add_history(line: *const c_char);
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn readline(prompt: &str) -> Option<String> {
    unsafe {
        // It doesn't matter if there is an interior null
        // It just won't prompt all the way
        let prompt = match CString::new(prompt.as_bytes()) {
            Ok(s) => s.as_ptr(),
            Err(_) => return None,
        };
        let line_ptr: *const c_char = ffi::readline(prompt);

        if line_ptr.is_null() {
            return None;
        }

        let ret =
            String::from_utf8_lossy(CStr::from_ptr(line_ptr)
                                    .to_bytes()).into_owned();
        stdlib::free(line_ptr as *mut c_void);
        Some(ret)
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn add_history(line: &str) {
    let l = match CString::new(line.as_bytes()) {
        Ok(s) => s.as_ptr(),
        Err(_) => return,
    };
    unsafe { ffi::add_history(l as *const c_char) };
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn readline(prompt: &str) -> Option<String> {
    use std::io::stdio::stdin;
    print!("{}", prompt);
    let line = stdin().read_line();
    match line {
        Ok(mut s) => {
            s.pop(); // take the last \n off the returned string
            Ok(s)
        },
        Err(_) => {
            None
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn add_history(_: *const c_char){}
