//! NVMe Submission and Completion Queue management.

use crate::{
    Dma, NVMeError, Result,
    cmd::{Cmd, Sqe},
    reg,
};

use core::{
    marker::PhantomData,
    sync::atomic::{AtomicU8, AtomicU16, Ordering},
};

/// Completion Queue Entry.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Cqe {
    /// Command Specific DWord 0.
    pub dw0: u32,
    /// Command Specific DWord 1.
    pub dw1: u32,
    /// SQ Head Pointer.
    pub sqhd: u16,
    /// SQ Identifier.
    pub sqid: u16,
    /// Command Identifier.
    pub cid: u16,
    /// Status Field.
    pub sf: u16,
}

impl Cqe {
    /// Returns the phase tag bit.
    pub fn phase(&self) -> bool {
        return (self.sf & 1) != 0;
    }

    /// Returns the status code.
    pub fn status(&self) -> u16 {
        return (self.sf >> 1) & 0x7FF;
    }

    /// Returns true if the command completed successfully.
    pub fn ok(&self) -> bool {
        return self.status() == 0;
    }
}

/// Submission Queue.
pub struct Sq<A: Dma> {
    qid: u16,
    addr: usize,
    phys: u64,
    size: usize,
    #[allow(dead_code)]
    bytes: usize,
    #[allow(dead_code)]
    align: usize,
    tail: AtomicU16,
    cid: AtomicU16,
    pub(crate) pending: AtomicU16,
    _alloc: PhantomData<A>,
}

impl<A: Dma> Sq<A> {
    /// Creates a new Submission Queue.
    pub fn new(qid: u16, size: usize, align: usize, alloc: &A) -> Result<Self> {
        let bytes = size * core::mem::size_of::<Sqe>();
        let addr = unsafe { alloc.alloc(bytes, align) };
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
            bytes,
            align,
            tail: AtomicU16::new(0),
            cid: AtomicU16::new(0),
            pending: AtomicU16::new(0),
            _alloc: PhantomData,
        });
    }

    /// Returns the physical address of the queue.
    pub fn phys(&self) -> u64 {
        return self.phys;
    }

    /// Returns the queue size in entries.
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Returns the next command identifier.
    pub fn next_cid(&self) -> u16 {
        return self.cid.fetch_add(1, Ordering::Relaxed);
    }

    /// Submits a command to the queue.
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

    /// Returns true if no commands are pending.
    pub fn is_idle(&self) -> bool {
        return self.pending.load(Ordering::SeqCst) == 0;
    }
}

/// Completion Queue.
pub struct Cq<A: Dma> {
    qid: u16,
    addr: usize,
    phys: u64,
    size: usize,
    #[allow(dead_code)]
    bytes: usize,
    #[allow(dead_code)]
    align: usize,
    head: AtomicU16,
    phase: AtomicU8,
    _alloc: PhantomData<A>,
}

impl<A: Dma> Cq<A> {
    /// Creates a new Completion Queue.
    pub fn new(qid: u16, size: usize, align: usize, alloc: &A) -> Result<Self> {
        let bytes = size * core::mem::size_of::<Cqe>();
        let addr = unsafe { alloc.alloc(bytes, align) };
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
            bytes,
            align,
            head: AtomicU16::new(0),
            phase: AtomicU8::new(1),
            _alloc: PhantomData,
        });
    }

    /// Returns the physical address of the queue.
    pub fn phys(&self) -> u64 {
        return self.phys;
    }

    /// Returns the queue size in entries.
    pub fn size(&self) -> usize {
        return self.size;
    }

    /// Polls for completion of a command with the given CID.
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
            self.phase
                .store(if phase != 0 { 0 } else { 1 }, Ordering::Release);
        }

        self.head.store(next, Ordering::Release);

        let db = mmio + reg::doorbell_cq(self.qid, dstrd);
        unsafe {
            (db as *mut u32).write_volatile(next as u32);
        }

        if !cqe.ok() {
            return Err(NVMeError::CmdFail(cqe.status()));
        }

        return Ok(cqe);
    }
}

/// Combined Submission and Completion Queue pair.
pub struct Queue<A: Dma> {
    #[allow(dead_code)]
    qid: u16,
    sq: Sq<A>,
    cq: Cq<A>,
}

impl<A: Dma> Queue<A> {
    /// Creates a new Queue pair.
    pub fn new(qid: u16, size: usize, align: usize, alloc: &A) -> Result<Self> {
        return Ok(Self {
            qid,
            sq: Sq::new(qid, size, align, alloc)?,
            cq: Cq::new(qid, size, align, alloc)?,
        });
    }

    /// Returns the queue identifier.
    #[allow(dead_code)]
    pub fn qid(&self) -> u16 {
        return self.qid;
    }

    /// Returns the physical address of the Submission Queue.
    pub fn sq_phys(&self) -> u64 {
        return self.sq.phys();
    }

    /// Returns the physical address of the Completion Queue.
    pub fn cq_phys(&self) -> u64 {
        return self.cq.phys();
    }

    /// Returns the combined size of both queues.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        return self.sq.size() + self.cq.size();
    }

    /// Submits a command and waits for completion.
    pub fn submit(&self, cmd: &Cmd, mmio: usize, dstrd: u8) -> Result<Cqe> {
        let cid = self.sq.next_cid();
        let sqe = cmd.to_sqe(cid);
        self.sq.submit(&sqe, mmio, dstrd);
        let result = self.cq.poll(cid, mmio, dstrd);

        self.sq.pending.fetch_sub(1, Ordering::SeqCst);

        return result;
    }

    /// Returns true if no commands are pending.
    pub fn is_idle(&self) -> bool {
        return self.sq.is_idle();
    }
}
