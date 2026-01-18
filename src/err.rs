use core::result::Result as CoreResult;

/// NVMe driver error types.
#[derive(Debug, Clone, Copy)]
pub enum NVMeError {
    /// Operation timed out.
    Timeout,
    /// Out of memory for DMA allocation.
    OoRam,
    /// Invalid queue pair.
    InvQp,
    /// No available queue slots.
    FullQp,
    /// Command failed with status code.
    CmdFail(u16),
    /// I/O error.
    IoError,
    /// Invalid buffer (alignment or size).
    InvBuf,
    /// MMIO mapping failed.
    MapFail,
}

/// Result type for NVMe operations.
pub type Result<T> = CoreResult<T, NVMeError>;
