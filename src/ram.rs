use crate::{NVMeError, Result};

pub trait Dma: Send + Sync {
    unsafe fn alloc(&self, size: usize) -> usize;
    unsafe fn free(&self, addr: usize, size: usize);
    fn virt_to_phys(&self, va: usize) -> usize;
}

pub struct PrpList {
    pub addr: usize,
    pub sz: usize
}

impl PrpList {
    pub fn free<A: Dma>(&self, alloc: &A) {
        unsafe { alloc.free(self.addr, self.sz); }
    }
}

pub fn build_prp<A: Dma>(
    alloc: &A,
    buf: usize,
    sz: usize
) -> Result<(u64, u64, Option<PrpList>)> {
    if buf & 0x3 != 0 {
        return Err(NVMeError::InvBuf);
    }

    let prp1 = alloc.virt_to_phys(buf) as u64;
    let off = buf & 0xFFF;
    let pages = (off + sz + 4095) / 4096;

    if pages == 1 {
        return Ok((prp1, 0, None));
    }

    if off != 0 {
        return Err(NVMeError::InvBuf);
    }

    let prp2_pa = alloc.virt_to_phys(buf + 4096);

    if pages == 2 {
        return Ok((prp1, prp2_pa as u64, None));
    }

    let list_sz = (pages - 1) * 8;
    let list_aligned = (list_sz + 4095) & !4095;
    let list_va = unsafe { alloc.alloc(list_aligned) };

    if list_va == 0 {
        return Err(NVMeError::OoRam);
    }

    let list_ptr = list_va as *mut u64;
    for i in 0..(pages - 1) {
        let page_pa = alloc.virt_to_phys(buf + (i + 1) * 4096);
        unsafe {
            *list_ptr.add(i) = page_pa as u64;
        }
    }

    let list_pa = alloc.virt_to_phys(list_va) as u64;

    return Ok((prp1, list_pa, Some(PrpList { addr: list_va, sz: list_aligned })));
}