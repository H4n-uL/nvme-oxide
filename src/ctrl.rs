use crate::{cmd::Cmd, id::CtrlId, queue::{Cqe, Queue}, reg, Dma, LogErr, LogSmart, NVMeError, Result};
use core::{hint::spin_loop, sync::atomic::{AtomicBool, AtomicU16, Ordering}};
use alloc::{string::{String, ToString}, sync::Arc, collections::BTreeMap, vec::Vec};
use spin::Mutex;

pub struct CtrlData {
    pub serial: String,
    pub model: String,
    pub firm: String,
    pub mts: usize,
    pub mqe: u16,
    pub min_pg: usize
}

pub struct Ctrl<A: Dma> {
    mmio: usize,
    dstrd: u8,
    admin: Mutex<Option<Queue<A>>>,
    io: Mutex<BTreeMap<u16, Arc<Queue<A>>>>,
    data: Arc<CtrlData>,
    alloc: Arc<A>,
    active: AtomicBool,
    rr_cnt: AtomicU16
}

impl<A: Dma> Ctrl<A> {
    pub fn new(mmio: usize, alloc: A) -> Result<Self> {
        let mut ctrl = Self {
            mmio,
            dstrd: 0,
            admin: Mutex::new(None),
            io: Mutex::new(BTreeMap::new()),
            data: Arc::new(CtrlData {
                serial: String::new(),
                model: String::new(),
                firm: String::new(),
                mts: 0,
                mqe: 0,
                min_pg: 0
            }),
            alloc: Arc::new(alloc),
            active: AtomicBool::new(true),
            rr_cnt: AtomicU16::new(0)
        };

        ctrl.init()?;
        return Ok(ctrl);
    }

    fn init(&mut self) -> Result<()> {
        let cap: u64 = unsafe { self.read(reg::CAP) };
        self.dstrd = ((cap >> 32) & 0xF) as u8;

        unsafe {
            let cc: u32 = self.read(reg::CC);
            if (cc & reg::CC_EN) != 0 {
                self.write(reg::CC, 0u32);
                while self.read::<u32>(reg::CSTS) & reg::CSTS_RDY != 0 {
                    spin_loop();
                }
            }
        }

        let mqes = ((cap & 0xFFFF) + 1) as usize;
        let admin_size = mqes;

        let admin = Queue::new(0, admin_size, self.alloc.as_ref())?;

        unsafe {
            self.write(reg::ASQ, admin.sq_phys());
            self.write(reg::ACQ, admin.cq_phys());

            let aqa = ((admin_size - 1) << 16) | (admin_size - 1);
            self.write(reg::AQA, aqa as u32);

            let cc = (4 << 20) | (6 << 16) | 1;
            self.write(reg::CC, cc);

            while self.read::<u32>(reg::CSTS) & reg::CSTS_RDY == 0 {
                spin_loop();
            }
        }

        *self.admin.lock() = Some(admin);

        let id_buf_size = 4096;
        let id_buf = unsafe { self.alloc.alloc(id_buf_size) };
        if id_buf == 0 {
            return Err(NVMeError::OoRam);
        }
        unsafe { (id_buf as *mut u8).write_bytes(0, id_buf_size) };

        let id_buf_phys = self.alloc.virt_to_phys(id_buf) as u64;
        let cmd = Cmd::id_ctrl(id_buf_phys);
        self.admin_cmd(&cmd)?;

        let ctrl_id = unsafe { &*(id_buf as *const CtrlId) };

        let serial = ctrl_id.serial().to_string();
        let model = ctrl_id.model().to_string();
        let firm = ctrl_id.firm().to_string();

        let mps_bytes = ((cap >> 48) & 0xF) as usize;
        let min_pg = 1 << (12 + mps_bytes);

        let mts = ctrl_id.max_xfer(min_pg).unwrap_or(usize::MAX);

        self.data = Arc::new(CtrlData {
            serial,
            model,
            firm,
            mts,
            mqe: mqes as u16,
            min_pg
        });

        unsafe { self.alloc.free(id_buf, id_buf_size) };

        let io_size = mqes.min(256);
        self.new_ioq(io_size)?;
        return Ok(());
    }

    unsafe fn read<T: Copy>(&self, offset: usize) -> T {
        return unsafe { ((self.mmio + offset) as *const T).read_volatile() };
    }

    unsafe fn write<T: Copy>(&self, offset: usize, val: T) {
        return unsafe { ((self.mmio + offset) as *mut T).write_volatile(val); };
    }

    pub fn admin_cmd(&self, cmd: &Cmd) -> Result<()> {
        if let Some(ref admin) = *self.admin.lock() {
            admin.submit(cmd, self.mmio, self.dstrd)?;
            return Ok(());
        }
        return Err(NVMeError::InvQp);
    }

    pub fn io_cmd(&self, cmd: &Cmd) -> Result<()> {
        let io = self.io.lock();
        if io.is_empty() {
            return Err(NVMeError::InvQp);
        }

        let cnt = self.rr_cnt.fetch_add(1, Ordering::Relaxed) as usize;
        let keys = io.keys().cloned().collect::<Vec<u16>>();
        let qid = keys[cnt % keys.len()];

        let queue = io.get(&qid).ok_or(NVMeError::InvQp)?.clone();
        drop(io);
        queue.submit(cmd, self.mmio, self.dstrd)?;
        return Ok(());
    }

    pub fn new_ioq(&self, size: usize) -> Result<()> {
        let mut qid = 0;
        for i in 1..=self.data.mqe {
            if !self.io.lock().contains_key(&i) {
                qid = i;
                break;
            }
        }
        if qid == 0 || size == 0 {
            return Err(NVMeError::FullQp);
        }

        let io = Queue::new(qid, size, self.alloc.as_ref())?;

        let cmd = Cmd::cq_create(qid, size as u16, io.cq_phys());
        self.admin_cmd(&cmd)?;

        let cmd = Cmd::sq_create(qid, size as u16, qid, io.sq_phys());
        self.admin_cmd(&cmd)?;

        self.io.lock().insert(qid, Arc::new(io));
        return Ok(());
    }

    pub fn rm_ioq(&self, qid: u16) -> Result<()> {
        if qid == 0 {
            return Err(NVMeError::InvQp);
        }

        let mut io = self.io.lock();
        if !io.contains_key(&qid) {
            return Err(NVMeError::InvQp);
        }

        loop {
            if io.get(&qid).map(|q| q.is_idle()).unwrap_or(true) {
                break;
            }
            drop(io);
            spin_loop();
            io = self.io.lock();
        }

        drop(io);

        let cmd = Cmd::sq_del(qid);
        self.admin_cmd(&cmd)?;

        let cmd = Cmd::cq_del(qid);
        self.admin_cmd(&cmd)?;

        self.io.lock().remove(&qid);
        return Ok(());
    }

    pub fn shutdown(&self) -> Result<()> {
        self.active.store(false, Ordering::SeqCst);

        loop {
            let io = self.io.lock();
            let all_idle = io.values().all(|q| q.is_idle());
            drop(io);

            if all_idle {
                break;
            }
            spin_loop();
        }

        loop {
            let admin = self.admin.lock();
            if admin.as_ref().map(|q| q.is_idle()).unwrap_or(true) {
                break;
            }
            drop(admin);
            spin_loop();
        }

        unsafe {
            let mut cc: u32 = self.read(reg::CC);
            cc = (cc & !(0x3 << 14)) | reg::CC_SHN_NORMAL;
            self.write(reg::CC, cc);

            loop {
                let csts: u32 = self.read(reg::CSTS);
                let shst = (csts >> 2) & 0x3;
                if shst == 2 {
                    break;
                }
                spin_loop();
            }

            cc &= !reg::CC_EN;
            self.write(reg::CC, cc);

            loop {
                let csts: u32 = self.read(reg::CSTS);
                if (csts & reg::CSTS_RDY) == 0 {
                    break;
                }
                spin_loop();
            }
        }

        return Ok(());
    }

    pub fn resume(&self) -> Result<()> {
        unsafe {
            let mut cc: u32 = self.read(reg::CC);
            cc |= reg::CC_EN;
            self.write(reg::CC, cc);

            loop {
                let csts: u32 = self.read(reg::CSTS);
                if (csts & reg::CSTS_RDY) != 0 {
                    break;
                }
                spin_loop();
            }
        }

        self.active.store(true, Ordering::SeqCst);
        return Ok(());
    }

    pub fn alloc(&self) -> &A {
        return &self.alloc;
    }

    pub fn data(&self) -> &CtrlData {
        return &self.data;
    }

    pub fn reg_nss(&self) -> Result<Vec<u32>> {
        let buf = unsafe { self.alloc.alloc(4096) };
        if buf == 0 {
            return Err(NVMeError::OoRam);
        }

        unsafe {
            (buf as *mut u8).write_bytes(0, 4096);
        }

        let buf_phys = self.alloc.virt_to_phys(buf) as u64;
        let cmd = Cmd::id_nss(buf_phys);
        self.admin_cmd(&cmd)?;

        let mut ns_list = Vec::new();
        unsafe {
            let ids = buf as *const u32;
            for i in 0..1024 {
                let nsid = ids.add(i).read_volatile();
                if nsid == 0 {
                    break;
                }
                ns_list.push(nsid);
            }
        }

        unsafe { self.alloc.free(buf, 4096); }
        return Ok(ns_list);
    }

    pub fn log_page(&self, lid: u8, buf: &mut [u8]) -> Result<()> {
        if buf.len() < 4 || buf.len() % 4 != 0 {
            return Err(NVMeError::InvBuf);
        }

        let buf_phys = self.alloc.virt_to_phys(buf.as_ptr() as usize) as u64;
        let numdl = ((buf.len() / 4) - 1) as u16;

        let cmd = Cmd::get_log(lid, numdl, buf_phys, 0);
        self.admin_cmd(&cmd)?;
        return Ok(());
    }

    pub fn smart_log(&self) -> Result<crate::id::LogSmart> {
        let buf = unsafe { self.alloc.alloc(512) };
        if buf == 0 {
            return Err(NVMeError::OoRam);
        }

        unsafe {
            (buf as *mut u8).write_bytes(0, 512);
        }

        let buf_phys = self.alloc.virt_to_phys(buf) as u64;
        let cmd = Cmd::get_log(crate::id::LOG_SMART, 127, buf_phys, 0);
        self.admin_cmd(&cmd)?;

        let smart = unsafe { (buf as *const LogSmart).read_volatile() };
        unsafe { self.alloc.free(buf, 512); }

        return Ok(smart);
    }

    pub fn error_log(&self, entries: usize) -> Result<Vec<LogErr>> {
        let buf_size = entries * size_of::<LogErr>();
        let buf = unsafe { self.alloc.alloc(buf_size) };
        if buf == 0 {
            return Err(NVMeError::OoRam);
        }

        unsafe {
            (buf as *mut u8).write_bytes(0, buf_size);
        }

        let buf_phys = self.alloc.virt_to_phys(buf) as u64;
        let numdl = ((buf_size / 4) - 1) as u16;
        let cmd = Cmd::get_log(crate::id::LOG_ERR, numdl, buf_phys, 0);
        self.admin_cmd(&cmd)?;

        let mut errors = Vec::new();
        unsafe {
            let ptr = buf as *const LogErr;
            for i in 0..entries {
                let entry = ptr.add(i).read_volatile();
                if entry.err_cnt == 0 {
                    break;
                }
                errors.push(entry);
            }
            self.alloc.free(buf, buf_size);
        }

        return Ok(errors);
    }

    pub fn set_feat(&self, fid: u8, value: u32) -> Result<()> {
        let cmd = Cmd::set_feat(fid, value);
        self.admin_cmd(&cmd)?;
        return Ok(());
    }

    pub fn get_feat(&self, fid: u8) -> Result<u32> {
        let cmd = Cmd::get_feat(fid);
        let cqe = self.adm_cmd_res(&cmd)?;
        return Ok(cqe.dw0);
    }

    fn adm_cmd_res(&self, cmd: &Cmd) -> Result<Cqe> {
        if let Some(ref admin) = *self.admin.lock() {
            return admin.submit(cmd, self.mmio, self.dstrd);
        }
        return Err(NVMeError::InvQp);
    }

    pub fn set_qs_n(&self, nsq: u16, ncq: u16) -> Result<(u16, u16)> {
        let value = (((ncq - 1) as u32) << 16) | ((nsq - 1) as u32);
        let cmd = Cmd::set_feat(crate::id::FT_NQ, value);
        let cqe = self.adm_cmd_res(&cmd)?;

        let allocd_nsq = ((cqe.dw0 & 0xFFFF) + 1) as u16;
        let allocd_ncq = (((cqe.dw0 >> 16) & 0xFFFF) + 1) as u16;

        return Ok((allocd_nsq, allocd_ncq));
    }

    pub fn set_ioq_cnt(&self, count: u16) -> Result<u16> {
        if count == 0 {
            return Ok(0);
        }

        let (allocated, _) = self.set_qs_n(count, count)?;
        let target = allocated.min(count);

        let cur_cnt = self.io.lock().len() as u16;

        if target > cur_cnt {
            let io_size = self.data.mqe as usize;
            let io_size = io_size.min(256);

            for _ in cur_cnt..target {
                self.new_ioq(io_size)?;
            }
        } else if target < cur_cnt {
            let to_remove: Vec<u16> = self.io.lock()
                .keys()
                .filter(|&&qid| qid > target)
                .copied()
                .collect();

            for qid in to_remove {
                self.rm_ioq(qid)?;
            }
        }

        return Ok(target);
    }

    pub fn en_async_ev(&self) -> Result<()> {
        let mut aec = crate::id::AsyncEventConfig::new();
        aec.en_smart_hlt()
           .en_ns_attr()
           .en_fw_actv();

        return self.set_feat(crate::id::FT_ASYNC, aec.value);
    }

    pub fn block_erase(&self) -> Result<()> {
        let cmd = Cmd::sanitise(0x02, false, 0, false, false);
        return self.admin_cmd(&cmd);
    }

    pub fn overwrite(&self, passes: u8, invert: bool) -> Result<()> {
        let cmd = Cmd::sanitise(0x03, false, passes, invert, false);
        return self.admin_cmd(&cmd);
    }

    pub fn crypto_erase(&self) -> Result<()> {
        let cmd = Cmd::sanitise(0x04, false, 0, false, false);
        return self.admin_cmd(&cmd);
    }
}

impl<A: Dma> Drop for Ctrl<A> {
    fn drop(&mut self) {
        let _ = self.shutdown();
    }
}
