# nvme-oxide

Bare-metal lightweight NVMe driver for OS development.

## Features

- NVMe controller initialisation and shutdown
- Admin queue and I/O queue management
- Namespace enumeration and block I/O (read/write/flush)
- SMART and error log retrieval
- Trim (dataset management), write zeroes, compare, verify
- Firmware download and commit
- Sanitise operations (block erase, crypto erase, overwrite)
- Dynamic page size support via `CAP.MPSMIN`

## Integration

Implement the `Dma` trait to provide DMA allocation and MMIO mapping:

```rust
impl Dma for MyDma {
    unsafe fn alloc(&self, size: usize, align: usize) -> usize { /* physically contiguous */ }
    unsafe fn free(&self, addr: usize, size: usize, align: usize) { }
    unsafe fn map_mmio(&self, phys: usize, size: usize) -> usize { }
    unsafe fn unmap_mmio(&self, virt: usize, size: usize) { }
    fn virt_to_phys(&self, va: usize) -> usize { }
    fn page_size(&self) -> usize { /* >= 4096, must be >= NVMe CAP.MPSMIN */ }
}
```

Then initialise from PCI BAR0:

```rust
let dev = NVMeDev::new(pci_bar0_addr, MyDma)?;

// Access controller info
let ctrl = dev.ctrl();
println!("Model: {}", ctrl.data().model);
println!("Serial: {}", ctrl.data().serial);

// Read from namespace 1
if let Some(ns) = dev.ns(1) {
    let mut buf = vec![0u8; ns.blk_sz()];
    ns.read(0, &mut buf)?;
}
```

## Alignment Requirements

The `alloc` function receives alignment requirements per allocation. NVMe requires page-aligned buffers for most operations:

| Structure | Align |
|-----------|-------|
| Submission Queue | Page |
| Completion Queue | Page |
| PRP List | Page |
| Data buffers | Page |

The driver automatically uses `max(Dma::page_size(), CAP.MPSMIN)` for alignment.

## API Overview

### `NVMeDev<A: Dma>`
- `new(mmio: usize, alloc: A)` - Initialise controller
- `ctrl()` - Get controller handle
- `ns(nsid: u32)` - Get namespace by ID
- `ns_list()` - List all namespaces

### `Ctrl<A: Dma>`
- `data()` - Controller info (serial, model, firmware, max transfer size)
- `new_ioq(size)` / `rm_ioq(qid)` - Manage I/O queues
- `set_ioq_cnt(count)` - Set number of I/O queues
- `smart_log()` / `error_log(entries)` - Retrieve logs
- `shutdown()` / `resume()` - Power management
- `block_erase()` / `crypto_erase()` / `overwrite(passes, invert)` - Sanitise

### `Ns<A: Dma>`
- `id()` / `blk_sz()` / `blk_cnt()` - Namespace info
- `read(lba, buf)` / `write(lba, buf)` - Block I/O
- `flush()` - Flush volatile write cache
- `trim(lba, blocks)` - Deallocate blocks
- `write_zeroes(lba, blocks)` - Zero blocks without transfer
- `compare(lba, buf)` / `verify(lba, blocks)` - Data verification
