//! Only gets added if the OS is Linux or Mac.
//!
//! The build.rs automatically links to the correct library (libreadline on
//! Linux, libedit on Mac OS X)
//!
//! NOTE: to get a *const c_char from &str -> s.to_c_str_unchecked().as_ptr()
#![feature(libc)]
extern crate libc;
use self::libc::c_char;

#[cfg(any(target_os = "linux", target_os = "macos"))]
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

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn readline(prompt: *const c_char) -> *const c_char {
    use std::io::stdio::stdin;
    print!("{}", prompt);
    let line = stdin().read_line();
    match line {
        Ok(mut s) => {
            s.pop(); // take the last \n off the returned string
            s.as_slice().as_ptr() as *const c_char
        },
        Err(_) => {
            std::ptr::null()
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn add_history(_: *const c_char){}
