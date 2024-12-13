//! Error types for DCMI functions.

use std::ffi::c_int;
use thiserror::Error;

/// Error type for DCMI function which gets data.
#[derive(Error, Debug)]
pub enum GetDataError {
    /// Invalid data
    #[error("Invalid data")]
    InvalidData,
    /// Error when reading data
    #[error("Data read error")]
    ReadError,
}

/// Error type for DCMI functions.
#[derive(Error, Debug)]
pub enum DCMIError {
    /// Error when parsing c char array return by DCMI
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    /// Error when getting data from DCMI c library
    #[error(transparent)]
    GetDataError(#[from] GetDataError),

    /// Invalid parameter
    #[error("Invalid parameter")]
    InvalidParameter,

    /// Memory operation failed
    #[error("Memory operation failed")]
    MemoryOperateFail,

    /// Operation not permitted
    #[error("Operation not permitted")]
    OperationNotPermitted,

    /// Secure function fail
    #[error("Secure function fail")]
    SecureFunctionFail,

    /// DCMI Inner error
    #[error("Inner error")]
    InnerError,

    /// time out
    #[error("time out")]
    CodeTimeOut,

    /// Invalid device ID
    #[error("Invalid device ID")]
    InvalidDeviceId,

    /// Device not exist
    #[error("Device not exist")]
    DeviceNotExist,

    /// Ioctl return fail
    #[error("Ioctl return fail")]
    IoctlFail,

    /// Send message fail
    #[error("Send message fail")]
    SendMessageFail,

    /// Receive message fail
    #[error("Receive message fail")]
    ReceiveMessageFail,

    /// Not ready
    #[error("Not ready")]
    NotReady,

    /// Not support in container
    #[error("Not support in container")]
    NotSupportInContainer,

    /// Reset fail
    #[error("Reset fail")]
    ResetFail,

    /// Reset operation abort
    #[error("Reset operation abort")]
    AbortOperation,

    /// Is upgrading
    #[error("Is upgrading")]
    IsUpgrading,

    /// Device Resource occupied
    #[error("Device resource occupied")]
    ResourceOccupied,

    /// Device id / function not support
    #[error("Device id / function not support")]
    NotSupport,

    /// Unknown error
    #[error("Unknown error, error code: {0}")]
    UnknownError(i32),
}

/// Result type for DCMI functions.
pub type DCMIResult<T> = Result<T, DCMIError>;

/// Converts DCMI error code into a `Result<(), DCMIError>`.
pub fn dcmi_try(code: c_int) -> Result<(), DCMIError> {
    use super::ffi;

    match code {
        0 => Ok(()),
        ffi::DCMI_ERR_CODE_INVALID_PARAMETER => Err(DCMIError::InvalidParameter),
        ffi::DCMI_ERR_CODE_OPER_NOT_PERMITTED => Err(DCMIError::OperationNotPermitted),
        ffi::DCMI_ERR_CODE_MEM_OPERATE_FAIL => Err(DCMIError::MemoryOperateFail),
        ffi::DCMI_ERR_CODE_SECURE_FUN_FAIL => Err(DCMIError::SecureFunctionFail),
        ffi::DCMI_ERR_CODE_INNER_ERR => Err(DCMIError::InnerError),
        ffi::DCMI_ERR_CODE_TIME_OUT => Err(DCMIError::CodeTimeOut),
        ffi::DCMI_ERR_CODE_INVALID_DEVICE_ID => Err(DCMIError::InvalidDeviceId),
        ffi::DCMI_ERR_CODE_DEVICE_NOT_EXIST => Err(DCMIError::DeviceNotExist),
        ffi::DCMI_ERR_CODE_IOCTL_FAIL => Err(DCMIError::IoctlFail),
        ffi::DCMI_ERR_CODE_SEND_MSG_FAIL => Err(DCMIError::SendMessageFail),
        ffi::DCMI_ERR_CODE_RECV_MSG_FAIL => Err(DCMIError::ReceiveMessageFail),
        ffi::DCMI_ERR_CODE_NOT_REDAY => Err(DCMIError::NotReady),
        ffi::DCMI_ERR_CODE_NOT_SUPPORT_IN_CONTAINER => Err(DCMIError::NotSupportInContainer),
        ffi::DCMI_ERR_CODE_RESET_FAIL => Err(DCMIError::ResetFail),
        ffi::DCMI_ERR_CODE_ABORT_OPERATE => Err(DCMIError::AbortOperation),
        ffi::DCMI_ERR_CODE_IS_UPGRADING => Err(DCMIError::IsUpgrading),
        ffi::DCMI_ERR_CODE_RESOURCE_OCCUPIED => Err(DCMIError::ResourceOccupied),
        ffi::DCMI_ERR_CODE_NOT_SUPPORT => Err(DCMIError::NotSupport),
        _ => Err(DCMIError::UnknownError(code.into())),
    }
}
