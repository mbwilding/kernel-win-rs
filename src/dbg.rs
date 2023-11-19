use crate::types::*;

#[link(name = "ntoskrnl")]
extern "C" {
    #[allow(dead_code)]
    pub fn DbgPrint(format: PCSTR, ...) -> u32;
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($string: expr) => {
        unsafe {
            $crate::DbgPrint(concat!("[kernel_win_rs.sys] ", $string, '\0').as_ptr())
        }
    };

    ($string: expr, $($x:tt)*) => {
        unsafe {
            $crate::DbgPrint(concat!("[kernel_win_rs.sys] ", $string, '\0').as_ptr(), $($x)*)
        }
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($string: expr) => {};
    ($string: expr, $($x:tt)*) => {};
}
