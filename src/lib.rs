#![no_std]

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
    id::{CtrlId, LbaFormat, NsId, PwrStDesc, LogSmart, LogErr},
    ns::Ns,
    queue::{Cq, Sq},
    ram::Dma
};
