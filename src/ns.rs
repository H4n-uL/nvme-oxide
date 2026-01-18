use crate::{Ctrl, Dma, NVMeError, Result, cmd::Cmd, id::NsId, ram::build_prp};

use alloc::sync::Arc;

/// NVMe namespace handle for block I/O operations.
///
/// # Example
///
/// ```ignore
/// // Read 1 block from LBA 0
/// let mut buf = vec![0u8; ns.blk_sz()];
/// ns.read(0, &mut buf)?;
///
/// // Write data to LBA 100
/// ns.write(100, &data)?;
/// ns.flush()?;
/// ```
pub struct Ns<A: Dma> {
    ctrl: Arc<Ctrl<A>>,
    nsid: u32,
    blk_sz: usize,
    blk_cnt: u64,
}

impl<A: Dma> Ns<A> {
    /// Creates a new namespace handle by querying the controller.
    pub fn new(ctrl: Arc<Ctrl<A>>, nsid: u32) -> Result<Self> {
        let page_size = ctrl.data().min_pg;
        let buffer = unsafe { ctrl.alloc().alloc(4096, page_size) };
        if buffer == 0 {
            return Err(NVMeError::OoRam);
        }

        unsafe {
            (buffer as *mut u8).write_bytes(0, 4096);
        }

        let buffer_phys = ctrl.alloc().virt_to_phys(buffer) as u64;
        let cmd = Cmd::id_ns(nsid, buffer_phys);
        ctrl.admin_cmd(&cmd)?;

        unsafe {
            let ns_id = &*(buffer as *const NsId);

            let blk_sz = ns_id.lba_size();
            let blk_cnt = ns_id.nsze;

            ctrl.alloc().free(buffer, 4096, page_size);

            return Ok(Self {
                ctrl,
                nsid,
                blk_sz,
                blk_cnt,
            });
        }
    }

    /// Returns the namespace ID.
    pub fn id(&self) -> u32 {
        return self.nsid;
    }

    /// Returns the block size in bytes.
    pub fn blk_sz(&self) -> usize {
        return self.blk_sz;
    }

    /// Returns the total number of blocks.
    pub fn blk_cnt(&self) -> u64 {
        return self.blk_cnt;
    }

    /// Reads blocks starting at the given LBA into the buffer.
    ///
    /// The buffer length must be a multiple of the block size.
    pub fn read(&self, lba: u64, buf: &mut [u8]) -> Result<()> {
        let page_size = self.ctrl.data().min_pg;
        let nlb = (buf.len() / self.blk_sz) as u16;
        let (prp1, prp2, prp_list) =
            build_prp(self.ctrl.alloc(), buf.as_ptr() as usize, buf.len(), page_size)?;

        let cmd = Cmd::read(self.nsid, lba, nlb, prp1, prp2);
        let res = self.ctrl.io_cmd(&cmd);

        if let Some(list) = prp_list {
            list.free(self.ctrl.alloc());
        }

        return res;
    }

    /// Writes blocks starting at the given LBA from the buffer.
    ///
    /// The buffer length must be a multiple of the block size.
    pub fn write(&self, lba: u64, buf: &[u8]) -> Result<()> {
        let page_size = self.ctrl.data().min_pg;
        let nlb = (buf.len() / self.blk_sz) as u16;
        let (prp1, prp2, prp_list) =
            build_prp(self.ctrl.alloc(), buf.as_ptr() as usize, buf.len(), page_size)?;

        let cmd = Cmd::write(self.nsid, lba, nlb, prp1, prp2);
        let res = self.ctrl.io_cmd(&cmd);

        if let Some(list) = prp_list {
            list.free(self.ctrl.alloc());
        }

        return res;
    }

    /// Flushes volatile write cache to non-volatile storage.
    pub fn flush(&self) -> Result<()> {
        let cmd = Cmd::flush(self.nsid);
        return self.ctrl.io_cmd(&cmd);
    }

    /// Deallocates (trims) blocks starting at the given LBA.
    pub fn trim(&self, lba: u64, blocks: u64) -> Result<()> {
        #[repr(C, packed)]
        struct DsmRange {
            context_attr: u32,
            length: u32,
            slba: u64,
        }

        let page_size = self.ctrl.data().min_pg;
        let range_buf = unsafe { self.ctrl.alloc().alloc(16, page_size) };
        if range_buf == 0 {
            return Err(NVMeError::OoRam);
        }

        let range = DsmRange {
            context_attr: 0,
            length: blocks as u32,
            slba: lba,
        };

        unsafe {
            (range_buf as *mut DsmRange).write_volatile(range);
        }

        let range_phys = self.ctrl.alloc().virt_to_phys(range_buf) as u64;
        let cmd = Cmd::dset_mgmt(self.nsid, 0, range_phys, 0x4);
        let res = self.ctrl.io_cmd(&cmd);

        unsafe {
            self.ctrl.alloc().free(range_buf, 16, page_size);
        }
        return res;
    }

    /// Writes zeroes to blocks without data transfer.
    pub fn write_zeroes(&self, lba: u64, blocks: u16) -> Result<()> {
        let cmd = Cmd::wr_zero(self.nsid, lba, blocks);
        return self.ctrl.io_cmd(&cmd);
    }

    /// Verifies data integrity of blocks on the device.
    pub fn verify(&self, lba: u64, blocks: u16) -> Result<()> {
        let cmd = Cmd::verify(self.nsid, lba, blocks);
        return self.ctrl.io_cmd(&cmd);
    }

    /// Compares buffer contents with data on the device.
    pub fn compare(&self, lba: u64, buf: &[u8]) -> Result<()> {
        let page_size = self.ctrl.data().min_pg;
        let nlb = (buf.len() / self.blk_sz) as u16;
        let (prp1, prp2, prp_list) =
            build_prp(self.ctrl.alloc(), buf.as_ptr() as usize, buf.len(), page_size)?;

        let cmd = Cmd::cmp(self.nsid, lba, nlb, prp1, prp2);
        let res = self.ctrl.io_cmd(&cmd);

        if let Some(list) = prp_list {
            list.free(self.ctrl.alloc());
        }

        return res;
    }
}
