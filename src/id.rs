#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct CtrlId {
    pub vid: u16,
    pub ssvid: u16,
    pub sn: [u8; 20],
    pub mn: [u8; 40],
    pub fr: [u8; 8],
    pub rab: u8,
    pub ieee: [u8; 3],
    pub cmic: u8,
    pub mdts: u8,
    pub cntlid: u16,
    pub ver: u32,
    pub rtd3r: u32,
    pub rtd3e: u32,
    pub oaes: u32,
    pub ctratt: u32,
    pub rrls: u16,
    _0: [u8; 9],
    pub cntrltype: u8,
    pub fguid: [u8; 16],
    pub crdt1: u16,
    pub crdt2: u16,
    pub crdt3: u16,
    _1: [u8; 106],
    pub oacs: u16,
    pub acl: u8,
    pub aerl: u8,
    pub frmw: u8,
    pub lpa: u8,
    pub elpe: u8,
    pub npss: u8,
    pub avscc: u8,
    pub apsta: u8,
    pub wctemp: u16,
    pub cctemp: u16,
    pub mtfa: u16,
    pub hmpre: u32,
    pub hmmin: u32,
    pub tnvmcap: [u8; 16],
    pub unvmcap: [u8; 16],
    pub rpmbs: u32,
    pub edstt: u16,
    pub dsto: u8,
    pub fwug: u8,
    pub kas: u16,
    pub hctma: u16,
    pub mntmt: u16,
    pub mxtmt: u16,
    pub sanicap: u32,
    pub hmminds: u32,
    pub hmmaxd: u16,
    pub nsetidmax: u16,
    pub endgidmax: u16,
    pub anatt: u8,
    pub anacap: u8,
    pub anagrpmax: u32,
    pub nanagrpid: u32,
    pub pels: u32,
    pub domainid: u16,
    _2: [u8; 10],
    pub megcap: [u8; 16],
    _3: [u8; 128],
    pub sqes: u8,
    pub cqes: u8,
    pub maxcmd: u16,
    pub nn: u32,
    pub oncs: u16,
    pub fuses: u16,
    pub fna: u8,
    pub vwc: u8,
    pub awun: u16,
    pub awupf: u16,
    pub nvscc: u8,
    pub nwpc: u8,
    pub acwu: u16,
    _4: u16,
    pub sgls: u32,
    pub mnan: u32,
    pub maxdna: [u8; 16],
    pub maxcna: u32,
    _5: [u8; 156],
    __iocs: [u8; 1344],
    pub psd: [PwrStDesc; 32],
    _vendor: [u8; 1024]
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PwrStDesc {
    pub mp: u16,
    _0: u8,
    pub flags: u8,
    pub enlat: u32,
    pub exlat: u32,
    pub rrt: u8,
    pub rrl: u8,
    pub rwt: u8,
    pub rwl: u8,
    pub idlp: u16,
    pub ips: u8,
    _1: u8,
    pub actp: u16,
    pub apw_aps: u8,
    _2: [u8; 9]
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NsId {
    pub nsze: u64,
    pub ncap: u64,
    pub nuse: u64,
    pub nsfeat: u8,
    pub nlbaf: u8,
    pub flbas: u8,
    pub mc: u8,
    pub dpc: u8,
    pub dps: u8,
    pub nmic: u8,
    pub rescap: u8,
    pub fpi: u8,
    pub dlfeat: u8,
    pub nawun: u16,
    pub nawupf: u16,
    pub nacwu: u16,
    pub nabsn: u16,
    pub nabo: u16,
    pub nabspf: u16,
    pub noiob: u16,
    pub nvmcap: [u8; 16],
    pub npwg: u16,
    pub npwa: u16,
    pub npdg: u16,
    pub npda: u16,
    pub nows: u16,
    pub mssrl: u16,
    pub mcl: u32,
    pub msrc: u8,
    _0: [u8; 11],
    pub anagrpid: u32,
    _1: [u8; 3],
    pub nsattr: u8,
    pub nvmsetid: u16,
    pub endgid: u16,
    pub nguid: [u8; 16],
    pub eui64: [u8; 8],
    pub lbaf: [LbaFormat; 16],
    _2: [u8; 192],
    _3: [u8; 2688],
    _vendor: [u8; 1024]
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LbaFormat {
    pub ms: u16,
    pub lbads: u8,
    pub rp: u8
}

impl CtrlId {
    pub fn serial(&self) -> &str {
        return core::str::from_utf8(&self.sn)
            .unwrap_or("").trim_end();
    }

    pub fn model(&self) -> &str {
        return core::str::from_utf8(&self.mn)
            .unwrap_or("").trim_end();
    }

    pub fn firm(&self) -> &str {
        return core::str::from_utf8(&self.fr)
            .unwrap_or("").trim_end();
    }

    pub fn max_xfer(&self, pg_size: usize) -> Option<usize> {
        if self.mdts == 0 {
            return None;
        }
        return Some(pg_size * (1 << self.mdts));
    }

    pub fn version(&self) -> (u8, u8, u8) {
        let major = ((self.ver >> 16) & 0xFF) as u8;
        let minor = ((self.ver >> 8) & 0xFF) as u8;
        let ter = (self.ver & 0xFF) as u8;
        return (major, minor, ter);
    }
}

impl NsId {
    pub fn lba_size(&self) -> usize {
        let fmt_idx = (self.flbas & 0x0F) as usize;
        if fmt_idx >= 16 {
            return 0;
        }

        let lbads = self.lbaf[fmt_idx].lbads;
        if lbads == 0 {
            return 0;
        }

        return 1 << lbads;
    }

    pub fn meta_size(&self) -> usize {
        let fmt_idx = (self.flbas & 0x0F) as usize;
        if fmt_idx >= 16 {
            return 0;
        }

        return self.lbaf[fmt_idx].ms as usize;
    }

    pub fn cap_bytes(&self) -> u64 {
        return self.ncap.saturating_mul(self.lba_size() as u64);
    }

    pub fn size_bytes(&self) -> u64 {
        return self.nsze.saturating_mul(self.lba_size() as u64);
    }

    pub fn is_thin(&self) -> bool {
        return (self.nsfeat & 0x01) != 0;
    }

    pub fn fmt_idx(&self) -> u8 {
        return self.flbas & 0x0F;
    }
}

impl LbaFormat {
    pub fn lba_size(&self) -> usize {
        return (1 << self.lbads) & !1;
    }

    pub fn meta_size(&self) -> usize {
        return self.ms as usize;
    }

    pub fn valid(&self) -> bool {
        return self.lbads != 0;
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LogErr {
    pub err_cnt: u64,
    pub sqid: u16,
    pub cmdid: u16,
    pub status: u16,
    pub prm_loc: u16,
    pub lba: u64,
    pub nsid: u32,
    pub vnd_spec: u8,
    pub trtype: u8,
    _0: [u8; 2],
    pub cs: u64,
    pub trtype_spec: u16,
    _1: [u8; 22]
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LogSmart {
    pub crit_warn: u8,
    pub temp: [u8; 2],
    pub avl_spr: u8,
    pub spr_thrs: u8,
    pub pct_used: u8,
    pub endur_cw: u8,
    _0: [u8; 25],
    pub data_rd: [u8; 16],
    pub data_wr: [u8; 16],
    pub host_rd: [u8; 16],
    pub host_wr: [u8; 16],
    pub busy_tm: [u8; 16],
    pub pwr_cyc: [u8; 16],
    pub pwr_hrs: [u8; 16],
    pub unsafe_sd: [u8; 16],
    pub med_err: [u8; 16],
    pub n_err_log: [u8; 16],
    pub warn_tmp: u32,
    pub crit_tmp: u32,
    pub tmp_sens: [u16; 8],
    pub tmp1_cnt: u32,
    pub tmp2_cnt: u32,
    pub tmp1_tm: u32,
    pub tmp2_tm: u32,
    _1: [u8; 280]
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LogPageFwSlot {
    pub afi: u8,
    _0: [u8; 7],
    pub frs: [[u8; 8]; 7],
    _1: [u8; 448]
}

pub const LOG_ERR: u8 = 0x01;
pub const LOG_SMART: u8 = 0x02;
pub const LOG_FW: u8 = 0x03;
pub const LOG_NS_CHG: u8 = 0x04;
pub const LOG_CMD_EFF: u8 = 0x05;

pub const FT_ARBITR: u8 = 0x01;
pub const FT_POWER: u8 = 0x02;
pub const FT_LBA_RNG: u8 = 0x03;
pub const FT_TEMP_TH: u8 = 0x04;
pub const FT_ERR_REC: u8 = 0x05;
pub const FT_VOL_WC: u8 = 0x06;
pub const FT_NQ: u8 = 0x07;
pub const FT_IRQ_COAL: u8 = 0x08;
pub const FT_IRQ_CFG: u8 = 0x09;
pub const FT_WR_ATOM: u8 = 0x0A;
pub const FT_ASYNC: u8 = 0x0B;
pub const FT_AUTO_PST: u8 = 0x0C;
pub const FT_HOST_MEM: u8 = 0x0D;
pub const FT_TSTAMP: u8 = 0x0E;
pub const FT_KEEPALV: u8 = 0x0F;
pub const FT_THERM: u8 = 0x10;
pub const FT_NOP_PS: u8 = 0x11;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncEventConfig {
    pub value: u32
}

impl AsyncEventConfig {
    pub fn new() -> Self {
        return Self { value: 0 };
    }

    pub fn en_smart_hlt(&mut self) -> &mut Self {
        self.value |= 1 << 0;
        return self;
    }

    pub fn en_ns_attr(&mut self) -> &mut Self {
        self.value |= 1 << 8;
        return self;
    }

    pub fn en_fw_actv(&mut self) -> &mut Self {
        self.value |= 1 << 9;
        return self;
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncEventInfo {
    pub dw0: u32
}

impl AsyncEventInfo {
    pub fn evt_type(&self) -> u8 {
        return ((self.dw0 >> 0) & 0x7) as u8;
    }

    pub fn evt_info(&self) -> u8 {
        return ((self.dw0 >> 8) & 0xFF) as u8;
    }

    pub fn log_page(&self) -> u8 {
        return ((self.dw0 >> 16) & 0xFF) as u8;
    }
}

pub const AER_TYPE_ERROR: u8 = 0;
pub const AER_TYPE_SMART: u8 = 1;
pub const AER_TYPE_NOTICE: u8 = 2;
pub const AER_TYPE_VENDOR: u8 = 7;
