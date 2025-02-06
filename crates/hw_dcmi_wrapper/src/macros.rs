//! some useful macros

macro_rules! call_dcmi_function {
    ($func_name:ident, $dcmi:expr $(, $arg:expr)*) => {
        crate::dcmi_try(
            unsafe {
                #[cfg(feature = "load_dynamic")]
                {
                    $dcmi.$func_name($($arg),*)
                }
                #[cfg(not(feature = "load_dynamic"))]
                {
                    crate::ffi::$func_name($($arg),*)
                }
            }
        )?
    };
}

macro_rules! check_value {
    ($value:expr) => {
        match $value {
            0x7ffd => Err(crate::error::GetDataError::InvalidData),
            0x7fff => Err(crate::error::GetDataError::ReadError),
            _ => Ok($value),
        }
    };
}
