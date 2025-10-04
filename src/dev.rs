use crate::{Ctrl, Dma, Ns, Result};
use alloc::{vec::Vec, sync::Arc};

pub struct NVMeDev<A: Dma> {
    ctrl: Arc<Ctrl<A>>,
    nss: Vec<Arc<Ns<A>>>
}

impl<A: Dma> NVMeDev<A> {
    pub fn new(mmio: usize, alloc: A) -> Result<Arc<Self>> {
        let ctrl = Arc::new(Ctrl::new(mmio, alloc)?);

        let mut nss = Vec::new();
        for nsid in ctrl.reg_nss()? {
            if let Ok(ns) = Ns::new(ctrl.clone(), nsid) {
                nss.push(Arc::new(ns));
            }
        }

        return Ok(Arc::new(Self { ctrl, nss }));
    }

    pub fn ctrl(&self) -> Arc<Ctrl<A>> {
        return self.ctrl.clone();
    }

    pub fn ns(&self, nsid: u32) -> Option<Arc<Ns<A>>> {
        for ns in &self.nss {
            if ns.id() == nsid {
                return Some(ns.clone());
            }
        }
        return None;
    }

    pub fn ns_list(&self) -> &[Arc<Ns<A>>] {
        return &self.nss;
    }
}
