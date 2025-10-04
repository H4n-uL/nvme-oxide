use crate::{cmd::{Cmd, Sqe}, reg, Dma, NVMeError, Result};
use core::{marker::PhantomData, sync::atomic::{AtomicU16, AtomicU8, Ordering}};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Cqe {
    pub dw0: u32,
    pub dw1: u32,
    pub sqhd: u16,
    pub sqid: u16,
    pub cid: u16,
    pub sf: u16
}

impl Cqe {
    pub fn phase(&self) -> bool {
        return (self.sf & 1) != 0;
    }

    pub fn status(&self) -> u16 {
        return (self.sf >> 1) & 0x7FF;
    }

    pub fn ok(&self) -> bool {
        return self.status() == 0;
    }
}

pub struct Sq<A: Dma> {
    qid: u16,
    addr: usize,
    phys: u64,
    size: usize,
    tail: AtomicU16,
    cid: AtomicU16,
    pending: AtomicU16,
    _alloc: PhantomData<A>
}

impl<A: Dma> Sq<A> {
    pub fn new(qid: u16, size: usize, alloc: &A) -> Result<Self> {
        let bytes = size * 64;
        let addr = unsafe { alloc.alloc(bytes) };
        if addr == 0 {
            return Err(NVMeError::OoRam);
        }

        unsafe {
            (addr as *mut u8).write_bytes(0, bytes);
        }

        let phys = alloc.virt_to_phys(addr) as u64;

        return Ok(Self {
            qid,
            addr,
            phys,
            size,
            tail: AtomicU16::new(0),
            cid: AtomicU16::new(0),
            pending: AtomicU16::new(0),
            _alloc: PhantomData
        });
    }

    pub fn phys(&self) -> u64 {
        return self.phys;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn next_cid(&self) -> u16 {
        return self.cid.fetch_add(1, Ordering::Relaxed);
    }

    pub fn submit(&self, sqe: &Sqe, mmio: usize, dstrd: u8) {
        self.pending.fetch_add(1, Ordering::SeqCst);

        let tail = self.tail.load(Ordering::Acquire);
        let next = (tail + 1) % (self.size as u16);

        unsafe {
            let ptr = (self.addr + tail as usize * 64) as *mut Sqe;
            ptr.write_volatile(*sqe);

            let db = mmio + reg::doorbell_sq(self.qid, dstrd);
            (db as *mut u32).write_volatile(next as u32);
        }

        self.tail.store(next, Ordering::Release);
    }

    pub fn is_idle(&self) -> bool {
        return self.pending.load(Ordering::SeqCst) == 0;
    }
}

pub struct Cq<A: Dma> {
    qid: u16,
    addr: usize,
    phys: u64,
    size: usize,
    head: AtomicU16,
    phase: AtomicU8,
    _alloc: PhantomData<A>
}

impl<A: Dma> Cq<A> {
    pub fn new(qid: u16, size: usize, alloc: &A) -> Result<Self> {
        let bytes = size * 16;
        let addr = unsafe { alloc.alloc(bytes) };
        if addr == 0 {
            return Err(NVMeError::OoRam);
        }

        unsafe {
            (addr as *mut u8).write_bytes(0, bytes);
        }

        let phys = alloc.virt_to_phys(addr) as u64;

        return Ok(Self {
            qid,
            addr,
            phys,
            size,
            head: AtomicU16::new(0),
            phase: AtomicU8::new(1),
            _alloc: PhantomData
        });
    }

    pub fn phys(&self) -> u64 {
        return self.phys;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn poll(&self, cid: u16, mmio: usize, dstrd: u8) -> Result<Cqe> {
        let phase = self.phase.load(Ordering::Acquire);

        let mut head;
        let mut cqe;

        loop {
            head = self.head.load(Ordering::Acquire);
            let ptr = (self.addr + head as usize * 16) as *const Cqe;
            cqe = unsafe { ptr.read_volatile() };
            if cqe.phase() == (phase != 0) && cqe.cid == cid {
                break;
            }
        }
        let next = (head + 1) % (self.size as u16);

        if next == 0 {
            self.phase.store(if phase != 0 { 0 } else { 1 }, Ordering::Release);
        }

        self.head.store(next, Ordering::Release);

        let db = mmio + reg::doorbell_cq(self.qid, dstrd);
        unsafe { (db as *mut u32).write_volatile(next as u32); }

        if !cqe.ok() {
            return Err(NVMeError::CmdFail(cqe.status()));
        }

        return Ok(cqe);
    }
}

pub struct Queue<A: Dma> {
    qid: u16,
    sq: Sq<A>,
    cq: Cq<A>
}

impl<A: Dma> Queue<A> {
    pub fn new(qid: u16, size: usize, alloc: &A) -> Result<Self> {
        return Ok(Self {
            qid,
            sq: Sq::new(qid, size, alloc)?,
            cq: Cq::new(qid, size, alloc)?
        });
    }

    pub fn qid(&self) -> u16 {
        return self.qid;
    }

    pub fn sq_phys(&self) -> u64 {
        return self.sq.phys();
    }

    pub fn cq_phys(&self) -> u64 {
        return self.cq.phys();
    }

    pub fn size(&self) -> usize {
        return self.sq.size() + self.cq.size();
    }

    pub fn submit(&self, cmd: &Cmd, mmio: usize, dstrd: u8) -> Result<Cqe> {
        let cid = self.sq.next_cid();
        let sqe = cmd.to_sqe(cid);
        self.sq.submit(&sqe, mmio, dstrd);
        let result = self.cq.poll(cid, mmio, dstrd);

        self.sq.pending.fetch_sub(1, Ordering::SeqCst);

        return result;
    }

    pub fn is_idle(&self) -> bool {
        return self.sq.is_idle();
    }
}
