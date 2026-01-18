//! Bare-metal lightweight NVMe driver for OS development.
//!
//! This crate provides functionality for interacting with NVMe storage devices
//! in environments without the standard library, such as kernels, bootloaders,
//! or embedded systems.
//!
//! # Features
//!
//! - NVMe controller initialisation and management
//! - Admin and I/O queue creation and management
//! - Namespace enumeration and block I/O operations
//! - SMART and error log retrieval
//! - Sanitise operations (block erase, crypto erase, overwrite)
//!
//! # Example
//!
//! ```ignore
//! // Initialize NVMe controller from PCI BAR0
//! let dev = NVMeDev::new(pci_bar0_addr, MyDma)?;
//!
//! // Access namespace 1
//! if let Some(ns) = dev.ns(1) {
//!     let mut buf = vec![0u8; ns.blk_sz()];
//!     ns.read(0, &mut buf)?;
//! }
//! ```
#![no_std]
#![deny(missing_docs)]

extern crate alloc;

mod cmd;
mod ctrl;
mod dev;
mod err;
mod id;
mod ns;
mod queue;
mod ram;
mod reg;

pub use crate::{
    ctrl::Ctrl,
    dev::NVMeDev,
    err::{NVMeError, Result},
    id::{CtrlId, LbaFormat, LogErr, LogSmart, NsId, PwrStDesc},
    ns::Ns,
    queue::{Cq, Sq},
    ram::Dma,
};
