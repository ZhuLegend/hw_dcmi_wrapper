//! some useful macros

macro_rules! call_dcmi_function {
    ($func_name:ident, $dcmi:expr, $($arg:expr),*) => {
        dcmi_try(
            unsafe {
                #[cfg(feature = "load_dynamic")]
                {
                    $dcmi.$func_name($($arg),*)
                }
                #[cfg(not(feature = "load_dynamic"))]
                {
                    ffi::$func_name($($arg),*)
                }
            }
        )?
    };
}

macro_rules! check_value {
    ($value:expr) => {
        match $value {
            0x7ffd => Err(GetDataError::InvalidData),
            0x7fff => Err(GetDataError::ReadError),
            _ => Ok($value),
        }
    };
}
