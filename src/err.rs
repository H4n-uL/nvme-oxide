use core::result::Result as CoreResult;

#[derive(Debug, Clone, Copy)]
pub enum NVMeError {
    Timeout,
    OoRam,
    InvQp,
    FullQp,
    CmdFail(u16),
    IoError,
    InvBuf
}

pub type Result<T> = CoreResult<T, NVMeError>;
