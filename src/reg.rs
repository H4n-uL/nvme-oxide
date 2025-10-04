pub const CAP: usize = 0x00;
pub const VS: usize = 0x08;
pub const INTMS: usize = 0x0C;
pub const INTMC: usize = 0x10;
pub const CC: usize = 0x14;
pub const CSTS: usize = 0x1C;
pub const NSSR: usize = 0x20;
pub const AQA: usize = 0x24;
pub const ASQ: usize = 0x28;
pub const ACQ: usize = 0x30;
pub const CMBLOC: usize = 0x38;
pub const CMBSZ: usize = 0x3C;
pub const BPINFO: usize = 0x40;
pub const BPRSEL: usize = 0x44;
pub const BPMBL: usize = 0x48;
pub const CMBMSC: usize = 0x50;
pub const CMBSTS: usize = 0x58;
pub const PMRCAP: usize = 0xE00;
pub const PMRCTL: usize = 0xE04;
pub const PMRSTS: usize = 0xE08;
pub const PMREBS: usize = 0xE0C;
pub const PMRSWTP: usize = 0xE10;
pub const PMRMSCL: usize = 0xE14;
pub const PMRMSCU: usize = 0xE18;

pub const CC_EN: u32 = 1 << 0;
pub const CC_CSS_NVM: u32 = 0 << 4;
pub const CC_MPS_SHIFT: u32 = 7;
pub const CC_AMS_RR: u32 = 0 << 11;
pub const CC_SHN_NONE: u32 = 0 << 14;
pub const CC_SHN_NORMAL: u32 = 1 << 14;
pub const CC_SHN_ABRUPT: u32 = 2 << 14;
pub const CC_IOSQES_SHIFT: u32 = 16;
pub const CC_IOCQES_SHIFT: u32 = 20;

pub const CSTS_RDY: u32 = 1 << 0;
pub const CSTS_CFS: u32 = 1 << 1;
pub const CSTS_SHST_NORMAL: u32 = 0 << 2;
pub const CSTS_SHST_OCCURRING: u32 = 1 << 2;
pub const CSTS_SHST_COMPLETE: u32 = 2 << 2;
pub const CSTS_NSSRO: u32 = 1 << 4;
pub const CSTS_PP: u32 = 1 << 5;

pub const NSSR_RESET: u32 = 0x4E564D65;

pub const CAP_MQES_MASK: u64 = 0xFFFF;
pub const CAP_CQR: u64 = 1 << 16;
pub const CAP_AMS_WRR: u64 = 1 << 17;
pub const CAP_TO_SHIFT: u64 = 24;
pub const CAP_TO_MASK: u64 = 0xFF;
pub const CAP_DSTRD_SHIFT: u64 = 32;
pub const CAP_DSTRD_MASK: u64 = 0xF;
pub const CAP_NSSRS: u64 = 1 << 36;
pub const CAP_CSS_NVM: u64 = 1 << 37;
pub const CAP_BPS: u64 = 1 << 45;
pub const CAP_MPSMIN_SHIFT: u64 = 48;
pub const CAP_MPSMIN_MASK: u64 = 0xF;
pub const CAP_MPSMAX_SHIFT: u64 = 52;
pub const CAP_MPSMAX_MASK: u64 = 0xF;
pub const CAP_PMRS: u64 = 1 << 56;
pub const CAP_CMBS: u64 = 1 << 57;

pub fn doorbell_sq(qid: u16, dstrd: u8) -> usize {
    0x1000 + (2 * qid as usize) * (4 << dstrd)
}

pub fn doorbell_cq(qid: u16, dstrd: u8) -> usize {
    0x1000 + (2 * qid as usize + 1) * (4 << dstrd)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CapReg {
    value: u64
}

impl CapReg {
    pub fn from_raw(value: u64) -> Self {
        return Self { value };
    }

    pub fn mqes(&self) -> u16 {
        return (self.value & CAP_MQES_MASK) as u16;
    }

    pub fn cqr(&self) -> bool {
        return (self.value & CAP_CQR) != 0;
    }

    pub fn ams_wrr(&self) -> bool {
        return (self.value & CAP_AMS_WRR) != 0;
    }

    pub fn timeout(&self) -> u8 {
        return ((self.value >> CAP_TO_SHIFT) & CAP_TO_MASK) as u8;
    }

    pub fn dstrd(&self) -> u8 {
        return ((self.value >> CAP_DSTRD_SHIFT) & CAP_DSTRD_MASK) as u8;
    }

    pub fn nssrs(&self) -> bool {
        return (self.value & CAP_NSSRS) != 0;
    }

    pub fn css_nvm(&self) -> bool {
        return (self.value & CAP_CSS_NVM) != 0;
    }

    pub fn bps(&self) -> bool {
        return (self.value & CAP_BPS) != 0;
    }

    pub fn mpsmin(&self) -> u8 {
        return ((self.value >> CAP_MPSMIN_SHIFT) & CAP_MPSMIN_MASK) as u8;
    }

    pub fn mpsmax(&self) -> u8 {
        return ((self.value >> CAP_MPSMAX_SHIFT) & CAP_MPSMAX_MASK) as u8;
    }

    pub fn pmrs(&self) -> bool {
        return (self.value & CAP_PMRS) != 0;
    }

    pub fn cmbs(&self) -> bool {
        return (self.value & CAP_CMBS) != 0;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CcReg {
    value: u32
}

impl CcReg {
    pub fn new() -> Self {
        return Self { value: 0 };
    }

    pub fn from_raw(value: u32) -> Self {
        return Self { value };
    }

    pub fn raw(&self) -> u32 {
        return self.value;
    }

    pub fn enable(&mut self) -> &mut Self {
        self.value |= CC_EN;
        return self;
    }

    pub fn disable(&mut self) -> &mut Self {
        self.value &= !CC_EN;
        return self;
    }

    pub fn is_enabled(&self) -> bool {
        return (self.value & CC_EN) != 0;
    }

    pub fn set_css(&mut self, css: u32) -> &mut Self {
        self.value &= !(0x7 << 4);
        self.value |= (css & 0x7) << 4;
        return self;
    }

    pub fn set_mps(&mut self, mps: u8) -> &mut Self {
        self.value &= !(0xF << CC_MPS_SHIFT);
        self.value |= ((mps as u32) & 0xF) << CC_MPS_SHIFT;
        return self;
    }

    pub fn set_iosqes(&mut self, val: u8) -> &mut Self {
        self.value &= !(0xF << CC_IOSQES_SHIFT);
        self.value |= ((val as u32) & 0xF) << CC_IOSQES_SHIFT;
        return self;
    }

    pub fn set_iocqes(&mut self, val: u8) -> &mut Self {
        self.value &= !(0xF << CC_IOCQES_SHIFT);
        self.value |= ((val as u32) & 0xF) << CC_IOCQES_SHIFT;
        return self;
    }

    pub fn set_shdn(&mut self, shn: u32) -> &mut Self {
        self.value &= !(0x3 << 14);
        self.value |= (shn & 0x3) << 14;
        return self;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CstsReg {
    value: u32
}

impl CstsReg {
    pub fn from_raw(value: u32) -> Self {
        return Self { value };
    }

    pub fn is_ready(&self) -> bool {
        return (self.value & CSTS_RDY) != 0;
    }

    pub fn is_fatal(&self) -> bool {
        return (self.value & CSTS_CFS) != 0;
    }

    pub fn shdn_status(&self) -> u8 {
        return ((self.value >> 2) & 0x3) as u8;
    }

    pub fn nssro(&self) -> bool {
        return (self.value & CSTS_NSSRO) != 0;
    }

    pub fn proc_paused(&self) -> bool {
        return (self.value & CSTS_PP) != 0;
    }
}
