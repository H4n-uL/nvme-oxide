use crate::{Ctrl, Dma, Ns, Result};

use alloc::{sync::Arc, vec::Vec};

/// High-level NVMe device handle.
///
/// Provides access to the controller and all attached namespaces.
///
/// # Example
///
/// ```ignore
/// let dev = NVMeDev::new(pci_bar0_addr, MyDma)?;
///
/// // Read from namespace 1
/// if let Some(ns) = dev.ns(1) {
///     let mut buf = vec![0u8; ns.blk_sz()];
///     ns.read(0, &mut buf)?;
/// }
/// ```
pub struct NVMeDev<A: Dma> {
    ctrl: Arc<Ctrl<A>>,
    nss: Vec<Arc<Ns<A>>>,
}

impl<A: Dma> NVMeDev<A> {
    /// Creates a new NVMe device from a PCI BAR0 physical address.
    ///
    /// Initialises the controller, enumerates namespaces, and creates
    /// a default I/O queue.
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

    /// Returns the controller handle.
    pub fn ctrl(&self) -> Arc<Ctrl<A>> {
        return self.ctrl.clone();
    }

    /// Returns a namespace by ID, or `None` if not found.
    pub fn ns(&self, nsid: u32) -> Option<Arc<Ns<A>>> {
        for ns in &self.nss {
            if ns.id() == nsid {
                return Some(ns.clone());
            }
        }
        return None;
    }

    /// Returns all namespaces.
    pub fn ns_list(&self) -> &[Arc<Ns<A>>] {
        return &self.nss;
    }
}
