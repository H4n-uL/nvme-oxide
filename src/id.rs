//! NVMe Identify structures and constants.

/// Controller Identify data structure (CNS 01h).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct CtrlId {
    /// PCI Vendor ID.
    pub vid: u16,
    /// PCI Subsystem Vendor ID.
    pub ssvid: u16,
    /// Serial Number.
    pub sn: [u8; 20],
    /// Model Number.
    pub mn: [u8; 40],
    /// Firmware Revision.
    pub fr: [u8; 8],
    /// Recommended Arbitration Burst.
    pub rab: u8,
    /// IEEE OUI Identifier.
    pub ieee: [u8; 3],
    /// Controller Multi-Path I/O and Namespace Sharing Capabilities.
    pub cmic: u8,
    /// Maximum Data Transfer Size.
    pub mdts: u8,
    /// Controller ID.
    pub cntlid: u16,
    /// Version.
    pub ver: u32,
    /// RTD3 Resume Latency.
    pub rtd3r: u32,
    /// RTD3 Entry Latency.
    pub rtd3e: u32,
    /// Optional Asynchronous Events Supported.
    pub oaes: u32,
    /// Controller Attributes.
    pub ctratt: u32,
    /// Read Recovery Levels Supported.
    pub rrls: u16,
    _0: [u8; 9],
    /// Controller Type.
    pub cntrltype: u8,
    /// FRU Globally Unique Identifier.
    pub fguid: [u8; 16],
    /// Command Retry Delay Time 1.
    pub crdt1: u16,
    /// Command Retry Delay Time 2.
    pub crdt2: u16,
    /// Command Retry Delay Time 3.
    pub crdt3: u16,
    _1: [u8; 106],
    /// Optional Admin Command Support.
    pub oacs: u16,
    /// Abort Command Limit.
    pub acl: u8,
    /// Asynchronous Event Request Limit.
    pub aerl: u8,
    /// Firmware Updates.
    pub frmw: u8,
    /// Log Page Attributes.
    pub lpa: u8,
    /// Error Log Page Entries.
    pub elpe: u8,
    /// Number of Power States Support.
    pub npss: u8,
    /// Admin Vendor Specific Command Configuration.
    pub avscc: u8,
    /// Autonomous Power State Transition Attributes.
    pub apsta: u8,
    /// Warning Composite Temperature Threshold.
    pub wctemp: u16,
    /// Critical Composite Temperature Threshold.
    pub cctemp: u16,
    /// Maximum Time for Firmware Activation.
    pub mtfa: u16,
    /// Host Memory Buffer Preferred Size.
    pub hmpre: u32,
    /// Host Memory Buffer Minimum Size.
    pub hmmin: u32,
    /// Total NVM Capacity.
    pub tnvmcap: [u8; 16],
    /// Unallocated NVM Capacity.
    pub unvmcap: [u8; 16],
    /// Replay Protected Memory Block Support.
    pub rpmbs: u32,
    /// Extended Device Self-test Time.
    pub edstt: u16,
    /// Device Self-test Options.
    pub dsto: u8,
    /// Firmware Update Granularity.
    pub fwug: u8,
    /// Keep Alive Support.
    pub kas: u16,
    /// Host Controlled Thermal Management Attributes.
    pub hctma: u16,
    /// Minimum Thermal Management Temperature.
    pub mntmt: u16,
    /// Maximum Thermal Management Temperature.
    pub mxtmt: u16,
    /// Sanitize Capabilities.
    pub sanicap: u32,
    /// Host Memory Buffer Minimum Descriptor Entry Size.
    pub hmminds: u32,
    /// Host Memory Maximum Descriptors Entries.
    pub hmmaxd: u16,
    /// NVM Set Identifier Maximum.
    pub nsetidmax: u16,
    /// Endurance Group Identifier Maximum.
    pub endgidmax: u16,
    /// ANA Transition Time.
    pub anatt: u8,
    /// Asymmetric Namespace Access Capabilities.
    pub anacap: u8,
    /// ANA Group Identifier Maximum.
    pub anagrpmax: u32,
    /// Number of ANA Group Identifiers.
    pub nanagrpid: u32,
    /// Persistent Event Log Size.
    pub pels: u32,
    /// Domain Identifier.
    pub domainid: u16,
    _2: [u8; 10],
    /// Max Endurance Group Capacity.
    pub megcap: [u8; 16],
    _3: [u8; 128],
    /// Submission Queue Entry Size.
    pub sqes: u8,
    /// Completion Queue Entry Size.
    pub cqes: u8,
    /// Maximum Outstanding Commands.
    pub maxcmd: u16,
    /// Number of Namespaces.
    pub nn: u32,
    /// Optional NVM Command Support.
    pub oncs: u16,
    /// Fused Operation Support.
    pub fuses: u16,
    /// Format NVM Attributes.
    pub fna: u8,
    /// Volatile Write Cache.
    pub vwc: u8,
    /// Atomic Write Unit Normal.
    pub awun: u16,
    /// Atomic Write Unit Power Fail.
    pub awupf: u16,
    /// NVM Vendor Specific Command Configuration.
    pub nvscc: u8,
    /// Namespace Write Protection Capabilities.
    pub nwpc: u8,
    /// Atomic Compare & Write Unit.
    pub acwu: u16,
    _4: u16,
    /// SGL Support.
    pub sgls: u32,
    /// Maximum Number of Allowed Namespaces.
    pub mnan: u32,
    /// Maximum Domain Namespace Attachments.
    pub maxdna: [u8; 16],
    /// Maximum I/O Controller Namespace Attachments.
    pub maxcna: u32,
    _5: [u8; 156],
    __iocs: [u8; 1344],
    /// Power State Descriptors.
    pub psd: [PwrStDesc; 32],
    _vendor: [u8; 1024],
}

/// Power State Descriptor.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PwrStDesc {
    /// Maximum Power.
    pub mp: u16,
    _0: u8,
    /// Flags.
    pub flags: u8,
    /// Entry Latency.
    pub enlat: u32,
    /// Exit Latency.
    pub exlat: u32,
    /// Relative Read Throughput.
    pub rrt: u8,
    /// Relative Read Latency.
    pub rrl: u8,
    /// Relative Write Throughput.
    pub rwt: u8,
    /// Relative Write Latency.
    pub rwl: u8,
    /// Idle Power.
    pub idlp: u16,
    /// Idle Power Scale.
    pub ips: u8,
    _1: u8,
    /// Active Power.
    pub actp: u16,
    /// Active Power Workload / Active Power Scale.
    pub apw_aps: u8,
    _2: [u8; 9],
}

/// Namespace Identify data structure (CNS 00h).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct NsId {
    /// Namespace Size.
    pub nsze: u64,
    /// Namespace Capacity.
    pub ncap: u64,
    /// Namespace Utilization.
    pub nuse: u64,
    /// Namespace Features.
    pub nsfeat: u8,
    /// Number of LBA Formats.
    pub nlbaf: u8,
    /// Formatted LBA Size.
    pub flbas: u8,
    /// Metadata Capabilities.
    pub mc: u8,
    /// End-to-end Data Protection Capabilities.
    pub dpc: u8,
    /// End-to-end Data Protection Type Settings.
    pub dps: u8,
    /// Namespace Multi-path I/O and Namespace Sharing Capabilities.
    pub nmic: u8,
    /// Reservation Capabilities.
    pub rescap: u8,
    /// Format Progress Indicator.
    pub fpi: u8,
    /// Deallocate Logical Block Features.
    pub dlfeat: u8,
    /// Namespace Atomic Write Unit Normal.
    pub nawun: u16,
    /// Namespace Atomic Write Unit Power Fail.
    pub nawupf: u16,
    /// Namespace Atomic Compare & Write Unit.
    pub nacwu: u16,
    /// Namespace Atomic Boundary Size Normal.
    pub nabsn: u16,
    /// Namespace Atomic Boundary Offset.
    pub nabo: u16,
    /// Namespace Atomic Boundary Size Power Fail.
    pub nabspf: u16,
    /// Namespace Optimal I/O Boundary.
    pub noiob: u16,
    /// NVM Capacity.
    pub nvmcap: [u8; 16],
    /// Namespace Preferred Write Granularity.
    pub npwg: u16,
    /// Namespace Preferred Write Alignment.
    pub npwa: u16,
    /// Namespace Preferred Deallocate Granularity.
    pub npdg: u16,
    /// Namespace Preferred Deallocate Alignment.
    pub npda: u16,
    /// Namespace Optimal Write Size.
    pub nows: u16,
    /// Maximum Single Source Range Length.
    pub mssrl: u16,
    /// Maximum Copy Length.
    pub mcl: u32,
    /// Maximum Source Range Count.
    pub msrc: u8,
    _0: [u8; 11],
    /// ANA Group Identifier.
    pub anagrpid: u32,
    _1: [u8; 3],
    /// Namespace Attributes.
    pub nsattr: u8,
    /// NVM Set Identifier.
    pub nvmsetid: u16,
    /// Endurance Group Identifier.
    pub endgid: u16,
    /// Namespace Globally Unique Identifier.
    pub nguid: [u8; 16],
    /// IEEE Extended Unique Identifier.
    pub eui64: [u8; 8],
    /// LBA Format Support.
    pub lbaf: [LbaFormat; 16],
    _2: [u8; 192],
    _3: [u8; 2688],
    _vendor: [u8; 1024],
}

/// LBA Format data structure.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LbaFormat {
    /// Metadata Size.
    pub ms: u16,
    /// LBA Data Size (power of 2).
    pub lbads: u8,
    /// Relative Performance.
    pub rp: u8,
}

impl CtrlId {
    /// Returns the serial number as a string.
    pub fn serial(&self) -> &str {
        return core::str::from_utf8(&self.sn).unwrap_or("").trim_end();
    }

    /// Returns the model name as a string.
    pub fn model(&self) -> &str {
        return core::str::from_utf8(&self.mn).unwrap_or("").trim_end();
    }

    /// Returns the firmware revision as a string.
    pub fn firm(&self) -> &str {
        return core::str::from_utf8(&self.fr).unwrap_or("").trim_end();
    }

    /// Returns the maximum data transfer size in bytes.
    pub fn max_xfer(&self, pg_size: usize) -> Option<usize> {
        if self.mdts == 0 {
            return None;
        }
        return Some(pg_size * (1 << self.mdts));
    }

    /// Returns the NVMe version as (major, minor, tertiary).
    pub fn version(&self) -> (u8, u8, u8) {
        let major = ((self.ver >> 16) & 0xFF) as u8;
        let minor = ((self.ver >> 8) & 0xFF) as u8;
        let ter = (self.ver & 0xFF) as u8;
        return (major, minor, ter);
    }
}

impl NsId {
    /// Returns the LBA data size in bytes.
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

    /// Returns the metadata size in bytes.
    pub fn meta_size(&self) -> usize {
        let fmt_idx = (self.flbas & 0x0F) as usize;
        if fmt_idx >= 16 {
            return 0;
        }

        return self.lbaf[fmt_idx].ms as usize;
    }

    /// Returns the namespace capacity in bytes.
    pub fn cap_bytes(&self) -> u64 {
        return self.ncap.saturating_mul(self.lba_size() as u64);
    }

    /// Returns the namespace size in bytes.
    pub fn size_bytes(&self) -> u64 {
        return self.nsze.saturating_mul(self.lba_size() as u64);
    }

    /// Returns true if the namespace supports thin provisioning.
    pub fn is_thin(&self) -> bool {
        return (self.nsfeat & 0x01) != 0;
    }

    /// Returns the current LBA format index.
    pub fn fmt_idx(&self) -> u8 {
        return self.flbas & 0x0F;
    }
}

impl LbaFormat {
    /// Returns the LBA data size in bytes.
    pub fn lba_size(&self) -> usize {
        return (1 << self.lbads) & !1;
    }

    /// Returns the metadata size in bytes.
    pub fn meta_size(&self) -> usize {
        return self.ms as usize;
    }

    /// Returns true if this format is valid.
    pub fn valid(&self) -> bool {
        return self.lbads != 0;
    }
}

/// Error Information Log Entry.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LogErr {
    /// Error Count.
    pub err_cnt: u64,
    /// Submission Queue ID.
    pub sqid: u16,
    /// Command ID.
    pub cmdid: u16,
    /// Status Field.
    pub status: u16,
    /// Parameter Error Location.
    pub prm_loc: u16,
    /// LBA.
    pub lba: u64,
    /// Namespace ID.
    pub nsid: u32,
    /// Vendor Specific Information Available.
    pub vnd_spec: u8,
    /// Transport Type.
    pub trtype: u8,
    _0: [u8; 2],
    /// Command Specific Information.
    pub cs: u64,
    /// Transport Type Specific Information.
    pub trtype_spec: u16,
    _1: [u8; 22],
}

/// SMART / Health Information Log.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LogSmart {
    /// Critical Warning.
    pub crit_warn: u8,
    /// Composite Temperature.
    pub temp: [u8; 2],
    /// Available Spare.
    pub avl_spr: u8,
    /// Available Spare Threshold.
    pub spr_thrs: u8,
    /// Percentage Used.
    pub pct_used: u8,
    /// Endurance Group Critical Warning Summary.
    pub endur_cw: u8,
    _0: [u8; 25],
    /// Data Units Read.
    pub data_rd: [u8; 16],
    /// Data Units Written.
    pub data_wr: [u8; 16],
    /// Host Read Commands.
    pub host_rd: [u8; 16],
    /// Host Write Commands.
    pub host_wr: [u8; 16],
    /// Controller Busy Time.
    pub busy_tm: [u8; 16],
    /// Power Cycles.
    pub pwr_cyc: [u8; 16],
    /// Power On Hours.
    pub pwr_hrs: [u8; 16],
    /// Unsafe Shutdowns.
    pub unsafe_sd: [u8; 16],
    /// Media and Data Integrity Errors.
    pub med_err: [u8; 16],
    /// Number of Error Information Log Entries.
    pub n_err_log: [u8; 16],
    /// Warning Composite Temperature Time.
    pub warn_tmp: u32,
    /// Critical Composite Temperature Time.
    pub crit_tmp: u32,
    /// Temperature Sensors.
    pub tmp_sens: [u16; 8],
    /// Thermal Management Temperature 1 Transition Count.
    pub tmp1_cnt: u32,
    /// Thermal Management Temperature 2 Transition Count.
    pub tmp2_cnt: u32,
    /// Total Time For Thermal Management Temperature 1.
    pub tmp1_tm: u32,
    /// Total Time For Thermal Management Temperature 2.
    pub tmp2_tm: u32,
    _1: [u8; 280],
}

/// Firmware Slot Information Log.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct LogPageFwSlot {
    /// Active Firmware Info.
    pub afi: u8,
    _0: [u8; 7],
    /// Firmware Revision for Slots 1-7.
    pub frs: [[u8; 8]; 7],
    _1: [u8; 448],
}

/// Log Page ID: Error Information.
pub const LOG_ERR: u8 = 0x01;
/// Log Page ID: SMART / Health Information.
pub const LOG_SMART: u8 = 0x02;
/// Log Page ID: Firmware Slot Information.
pub const LOG_FW: u8 = 0x03;
/// Log Page ID: Changed Namespace List.
pub const LOG_NS_CHG: u8 = 0x04;
/// Log Page ID: Commands Supported and Effects.
pub const LOG_CMD_EFF: u8 = 0x05;

/// Feature ID: Arbitration.
pub const FT_ARBITR: u8 = 0x01;
/// Feature ID: Power Management.
pub const FT_POWER: u8 = 0x02;
/// Feature ID: LBA Range Type.
pub const FT_LBA_RNG: u8 = 0x03;
/// Feature ID: Temperature Threshold.
pub const FT_TEMP_TH: u8 = 0x04;
/// Feature ID: Error Recovery.
pub const FT_ERR_REC: u8 = 0x05;
/// Feature ID: Volatile Write Cache.
pub const FT_VOL_WC: u8 = 0x06;
/// Feature ID: Number of Queues.
pub const FT_NQ: u8 = 0x07;
/// Feature ID: Interrupt Coalescing.
pub const FT_IRQ_COAL: u8 = 0x08;
/// Feature ID: Interrupt Vector Configuration.
pub const FT_IRQ_CFG: u8 = 0x09;
/// Feature ID: Write Atomicity Normal.
pub const FT_WR_ATOM: u8 = 0x0A;
/// Feature ID: Asynchronous Event Configuration.
pub const FT_ASYNC: u8 = 0x0B;
/// Feature ID: Autonomous Power State Transition.
pub const FT_AUTO_PST: u8 = 0x0C;
/// Feature ID: Host Memory Buffer.
pub const FT_HOST_MEM: u8 = 0x0D;
/// Feature ID: Timestamp.
pub const FT_TSTAMP: u8 = 0x0E;
/// Feature ID: Keep Alive Timer.
pub const FT_KEEPALV: u8 = 0x0F;
/// Feature ID: Host Controlled Thermal Management.
pub const FT_THERM: u8 = 0x10;
/// Feature ID: Non-Operational Power State Config.
pub const FT_NOP_PS: u8 = 0x11;

/// Asynchronous Event Configuration builder.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncEventConfig {
    /// Raw configuration value.
    pub value: u32,
}

impl AsyncEventConfig {
    /// Creates a new empty configuration.
    pub fn new() -> Self {
        return Self { value: 0 };
    }

    /// Enables SMART / Health critical warnings.
    pub fn en_smart_hlt(&mut self) -> &mut Self {
        self.value |= 1 << 0;
        return self;
    }

    /// Enables namespace attribute notices.
    pub fn en_ns_attr(&mut self) -> &mut Self {
        self.value |= 1 << 8;
        return self;
    }

    /// Enables firmware activation notices.
    pub fn en_fw_actv(&mut self) -> &mut Self {
        self.value |= 1 << 9;
        return self;
    }
}

/// Asynchronous Event Request completion information.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AsyncEventInfo {
    /// Completion Queue DWord 0.
    pub dw0: u32,
}

impl AsyncEventInfo {
    /// Returns the event type.
    pub fn evt_type(&self) -> u8 {
        return ((self.dw0 >> 0) & 0x7) as u8;
    }

    /// Returns the event information.
    pub fn evt_info(&self) -> u8 {
        return ((self.dw0 >> 8) & 0xFF) as u8;
    }

    /// Returns the associated log page ID.
    pub fn log_page(&self) -> u8 {
        return ((self.dw0 >> 16) & 0xFF) as u8;
    }
}

/// Async Event Type: Error.
pub const AER_TYPE_ERROR: u8 = 0;
/// Async Event Type: SMART / Health Status.
pub const AER_TYPE_SMART: u8 = 1;
/// Async Event Type: Notice.
pub const AER_TYPE_NOTICE: u8 = 2;
/// Async Event Type: Vendor Specific.
pub const AER_TYPE_VENDOR: u8 = 7;
