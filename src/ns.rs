use crate::{cmd::Cmd, id::NsId, ram::build_prp, Ctrl, Dma, NVMeError, Result};
use alloc::sync::Arc;

pub struct Ns<A: Dma> {
    ctrl: Arc<Ctrl<A>>,
    nsid: u32,
    blk_sz: usize,
    blk_cnt: u64
}

impl<A: Dma> Ns<A> {
    pub fn new(ctrl: Arc<Ctrl<A>>, nsid: u32) -> Result<Self> {
        let buffer = unsafe { ctrl.alloc().alloc(4096) };
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

            ctrl.alloc().free(buffer, 4096);

            return Ok(Self {
                ctrl,
                nsid,
                blk_sz,
                blk_cnt
            });
        }
    }

    pub fn id(&self) -> u32 {
        return self.nsid;
    }

    pub fn blk_sz(&self) -> usize {
        return self.blk_sz;
    }

    pub fn blk_cnt(&self) -> u64 {
        return self.blk_cnt;
    }

    pub fn read(&self, lba: u64, buf: &mut [u8]) -> Result<()> {
        let nlb = (buf.len() / self.blk_sz) as u16;
        let (prp1, prp2, prp_list) = build_prp(
            self.ctrl.alloc(),
            buf.as_ptr() as usize,
            buf.len()
        )?;

        let cmd = Cmd::read(self.nsid, lba, nlb, prp1, prp2);
        let res = self.ctrl.io_cmd(&cmd);

        if let Some(list) = prp_list {
            list.free(self.ctrl.alloc());
        }

        return res;
    }

    pub fn write(&self, lba: u64, buf: &[u8]) -> Result<()> {
        let nlb = (buf.len() / self.blk_sz) as u16;
        let (prp1, prp2, prp_list) = build_prp(
            self.ctrl.alloc(),
            buf.as_ptr() as usize,
            buf.len()
        )?;

        let cmd = Cmd::write(self.nsid, lba, nlb, prp1, prp2);
        let res = self.ctrl.io_cmd(&cmd);

        if let Some(list) = prp_list {
            list.free(self.ctrl.alloc());
        }

        return res;
    }

    pub fn flush(&self) -> Result<()> {
        let cmd = Cmd::flush(self.nsid);
        return self.ctrl.io_cmd(&cmd);
    }

    pub fn trim(&self, lba: u64, blocks: u64) -> Result<()> {
        #[repr(C, packed)]
        struct DsmRange {
            context_attr: u32,
            length: u32,
            slba: u64
        }

        let range_buf = unsafe { self.ctrl.alloc().alloc(16) };
        if range_buf == 0 {
            return Err(NVMeError::OoRam);
        }

        let range = DsmRange {
            context_attr: 0,
            length: blocks as u32,
            slba: lba
        };

        unsafe {
            (range_buf as *mut DsmRange).write_volatile(range);
        }

        let range_phys = self.ctrl.alloc().virt_to_phys(range_buf) as u64;
        let cmd = Cmd::dset_mgmt(self.nsid, 0, range_phys, 0x4);
        let res = self.ctrl.io_cmd(&cmd);

        unsafe { self.ctrl.alloc().free(range_buf, 16); }
        return res;
    }

    pub fn write_zeroes(&self, lba: u64, blocks: u16) -> Result<()> {
        let cmd = Cmd::wr_zero(self.nsid, lba, blocks);
        return self.ctrl.io_cmd(&cmd);
    }

    pub fn verify(&self, lba: u64, blocks: u16) -> Result<()> {
        let cmd = Cmd::verify(self.nsid, lba, blocks);
        return self.ctrl.io_cmd(&cmd);
    }

    pub fn compare(&self, lba: u64, buf: &[u8]) -> Result<()> {
        let nlb = (buf.len() / self.blk_sz) as u16;
        let (prp1, prp2, prp_list) = build_prp(
            self.ctrl.alloc(),
            buf.as_ptr() as usize,
            buf.len()
        )?;

        let cmd = Cmd::cmp(self.nsid, lba, nlb, prp1, prp2);
        let res = self.ctrl.io_cmd(&cmd);

        if let Some(list) = prp_list {
            list.free(self.ctrl.alloc());
        }

        return res;
    }
}
