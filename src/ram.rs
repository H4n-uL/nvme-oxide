use crate::{NVMeError, Result};

/// DMA memory allocation trait for NVMe operations.
///
/// Implement this trait to provide physically contiguous memory allocation
/// and MMIO mapping for the NVMe driver.
///
/// # Safety
///
/// Implementations must ensure that:
/// - `alloc` returns physically contiguous memory with the requested alignment
/// - `virt_to_phys` correctly translates virtual addresses to physical addresses
/// - Memory remains valid until `free` is called
pub trait Dma: Send + Sync {
    /// Allocates physically contiguous memory.
    ///
    /// Returns the virtual address of the allocated memory, or `None` on failure.
    unsafe fn alloc(&self, size: usize, align: usize) -> Option<usize>;

    /// Frees previously allocated memory.
    unsafe fn free(&self, addr: usize, size: usize, align: usize);

    /// Maps MMIO region into virtual address space.
    ///
    /// Returns the virtual address of the mapped region, or `None` on failure.
    unsafe fn map_mmio(&self, phys: usize, size: usize) -> Option<usize>;

    /// Unmaps a previously mapped MMIO region.
    unsafe fn unmap_mmio(&self, virt: usize, size: usize);

    /// Translates a virtual address to its physical address.
    fn virt_to_phys(&self, va: usize) -> usize;

    /// Returns the system page size (must be >= 4096).
    ///
    /// The driver uses `max(page_size(), CAP.MPSMIN)` for alignment.
    fn page_size(&self) -> usize;
}

pub struct PrpList {
    pub addr: usize,
    pub sz: usize,
    pub align: usize,
}

impl PrpList {
    pub fn free<A: Dma>(&self, alloc: &A) {
        unsafe {
            alloc.free(self.addr, self.sz, self.align);
        }
    }
}

pub fn build_prp<A: Dma>(
    alloc: &A,
    buf: usize,
    sz: usize,
    page_size: usize,
) -> Result<(u64, u64, Option<PrpList>)> {
    if buf & 0x3 != 0 {
        return Err(NVMeError::InvBuf);
    }

    let page_mask = page_size - 1;

    let prp1 = alloc.virt_to_phys(buf) as u64;
    let off = buf & page_mask;
    let pages = (off + sz + page_mask) / page_size;

    if pages == 1 {
        return Ok((prp1, 0, None));
    }

    if off != 0 {
        return Err(NVMeError::InvBuf);
    }

    let prp2_pa = alloc.virt_to_phys(buf + page_size);

    if pages == 2 {
        return Ok((prp1, prp2_pa as u64, None));
    }

    let list_sz = (pages - 1) * 8;
    let list_aligned = (list_sz + page_mask) & !page_mask;

    let list_va = unsafe {
        alloc.alloc(list_aligned, page_size)
    }.ok_or(NVMeError::OoRam)?;

    let list_ptr = list_va as *mut u64;
    for i in 0..(pages - 1) {
        let page_pa = alloc.virt_to_phys(buf + (i + 1) * page_size);
        unsafe {
            *list_ptr.add(i) = page_pa as u64;
        }
    }

    let list_pa = alloc.virt_to_phys(list_va) as u64;

    return Ok((
        prp1,
        list_pa,
        Some(PrpList {
            addr: list_va,
            sz: list_aligned,
            align: page_size,
        }),
    ));
}
