//! Collection of functions that help control the terminal
//!
//! These are built to work at least on these platforms:
//! Windows, Linux, and Mac, but are likely to work on more

#[allow(unused)]
pub mod unix;

#[allow(unused)]
pub mod windows;

/// Generate a function to run code based on the os neutuino is being compiled for.
///
/// # Examples
///
/// Automatically generate function from the name
/// ```
/// generate_os_function!(pub fn enable_ansi() -> io::Result<()>);
/// ```
///
/// Generate a function from the functions in os specific code
/// ```
/// generate_os_function!(pub fn enable_ansi() -> io::Result<()>, (os::unix::enable_ansi), (os::windows::enable_ansi));
/// ```
#[macro_export]
macro_rules! generate_os_function {
    ($vis:vis fn $func_name:ident($($args_name:ident: $args_type:ty$(,)*)*)$( -> $return_type:ty)*, $unix_func:tt, $win_func:tt) => {
        $vis fn $func_name($($args_name: $args_type, )*)$( -> $return_type)* {
            #[cfg(unix)]
            return $unix_func($($args_name)*);
            #[cfg(windows)]
            return $win_func($($args_name)*)
        }
    };

    ($vis:vis fn $func_name:ident($($args_name:ident: $args_type:ty$(,)*)*)$( -> $return_type:ty)*) => {
        $vis fn $func_name($($args_name: $args_type, )*)$( -> $return_type)* {
            #[cfg(unix)]
            use crate::os::unix::$func_name;
            #[cfg(windows)]
            use crate::os::windows::$func_name;
            return $func_name($($args_name)*);
        }
    };
}

/// Generate a function to run code based on the os neutuino is being compiled for.
///
/// # Examples
///
/// Automatically generate struct from the name
/// ```
/// generate_os_struct!(pub struct RawTerminal);
/// ```
///
/// Generate a struct from the structs in os specific code
/// ```
/// generate_os_struct!(pub struct RawTerminal, (os::unix::RawTerminal), (os::windows::RawTerminal));
/// ```
#[macro_export]
macro_rules! generate_os_struct {
    ($vis:vis struct $struct_name:ident, $unix_struct:tt, $win_struct:tt) => {
        #[cfg(unix)]
        $vis use $unix_struct;
        #[cfg(windows)]
        $vis use $win_struct;
    };

    ($vis:vis struct $struct_name:ident) => {
        #[cfg(unix)]
        $vis use os::unix::$struct_name;
        #[cfg(windows)]
        $vis use os::windows::$struct_name;
    };
}
