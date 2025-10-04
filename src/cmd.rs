#[repr(C)]
#[derive(Clone, Copy)]
pub struct Sqe {
    pub cdw0: u32,
    pub nsid: u32,
    pub cdw2: u32,
    pub cdw3: u32,
    pub mptr: u64,
    pub prp1: u64,
    pub prp2: u64,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32,
    pub cdw13: u32,
    pub cdw14: u32,
    pub cdw15: u32
}

pub struct Cmd {
    pub opc: u8,
    pub nsid: u32,
    pub prp1: u64,
    pub prp2: u64,
    pub cdw10: u32,
    pub cdw11: u32,
    pub cdw12: u32
}

impl Cmd {
    pub fn new(opc: u8) -> Self {
        return Self {
            opc,
            nsid: 0,
            prp1: 0,
            prp2: 0,
            cdw10: 0,
            cdw11: 0,
            cdw12: 0
        };
    }

    pub fn to_sqe(&self, cid: u16) -> Sqe {
        return Sqe {
            cdw0: (cid as u32) << 16 | self.opc as u32,
            nsid: self.nsid,
            cdw2: 0,
            cdw3: 0,
            mptr: 0,
            prp1: self.prp1,
            prp2: self.prp2,
            cdw10: self.cdw10,
            cdw11: self.cdw11,
            cdw12: self.cdw12,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0
        };
    }

    pub fn id_ctrl(prp1: u64) -> Self {
        let mut cmd = Self::new(0x06);
        cmd.prp1 = prp1;
        cmd.cdw10 = 1;
        return cmd;
    }

    pub fn id_ns(nsid: u32, prp1: u64) -> Self {
        let mut cmd = Self::new(0x06);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.cdw10 = 0;
        return cmd;
    }

    pub fn set_feat(fid: u8, cdw11: u32) -> Self {
        let mut cmd = Self::new(0x09);
        cmd.cdw10 = fid as u32;
        cmd.cdw11 = cdw11;
        return cmd;
    }

    pub fn cq_create(qid: u16, qsize: u16, prp1: u64) -> Self {
        let mut cmd = Self::new(0x05);
        cmd.prp1 = prp1;
        cmd.cdw10 = ((qsize - 1) as u32) << 16 | qid as u32;
        cmd.cdw11 = 0x1;
        return cmd;
    }

    pub fn sq_create(qid: u16, qsize: u16, cqid: u16, prp1: u64) -> Self {
        let mut cmd = Self::new(0x01);
        cmd.prp1 = prp1;
        cmd.cdw10 = ((qsize - 1) as u32) << 16 | qid as u32;
        cmd.cdw11 = (cqid as u32) << 16 | 0x1;
        return cmd;
    }

    pub fn read(nsid: u32, lba: u64, nlb: u16, prp1: u64, prp2: u64) -> Self {
        let mut cmd = Self::new(0x02);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.prp2 = prp2;
        cmd.cdw10 = lba as u32;
        cmd.cdw11 = (lba >> 32) as u32;
        cmd.cdw12 = (nlb - 1) as u32;
        return cmd;
    }

    pub fn write(nsid: u32, lba: u64, nlb: u16, prp1: u64, prp2: u64) -> Self {
        let mut cmd = Self::new(0x01);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.prp2 = prp2;
        cmd.cdw10 = lba as u32;
        cmd.cdw11 = (lba >> 32) as u32;
        cmd.cdw12 = (nlb - 1) as u32;
        return cmd;
    }

    pub fn flush(nsid: u32) -> Self {
        let mut cmd = Self::new(0x00);
        cmd.nsid = nsid;
        return cmd;
    }

    pub fn get_feat(fid: u8) -> Self {
        let mut cmd = Self::new(0x0A);
        cmd.cdw10 = fid as u32;
        return cmd;
    }

    pub fn get_log(lid: u8, numdl: u16, prp1: u64, prp2: u64) -> Self {
        let mut cmd = Self::new(0x02);
        cmd.prp1 = prp1;
        cmd.prp2 = prp2;
        cmd.cdw10 = ((numdl as u32) << 16) | (lid as u32);
        return cmd;
    }

    pub fn async_req() -> Self {
        Self::new(0x0C)
    }

    pub fn cq_del(qid: u16) -> Self {
        let mut cmd = Self::new(0x04);
        cmd.cdw10 = qid as u32;
        return cmd;
    }

    pub fn sq_del(qid: u16) -> Self {
        let mut cmd = Self::new(0x00);
        cmd.cdw10 = qid as u32;
        return cmd;
    }

    pub fn abort(sqid: u16, cid: u16) -> Self {
        let mut cmd = Self::new(0x08);
        cmd.cdw10 = ((cid as u32) << 16) | (sqid as u32);
        return cmd;
    }

    pub fn id_nss(prp1: u64) -> Self {
        let mut cmd = Self::new(0x06);
        cmd.prp1 = prp1;
        cmd.cdw10 = 0x02;
        return cmd;
    }

    pub fn id_nss_alc(nsid: u32, prp1: u64) -> Self {
        let mut cmd = Self::new(0x06);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.cdw10 = 0x10;
        return cmd;
    }

    pub fn id_nss_ctrl(nsid: u32, cntid: u16, prp1: u64) -> Self {
        let mut cmd = Self::new(0x06);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.cdw10 = 0x13 | ((cntid as u32) << 16);
        return cmd;
    }

    pub fn id_ns_desc(nsid: u32, prp1: u64) -> Self {
        let mut cmd = Self::new(0x06);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.cdw10 = 0x03;
        return cmd;
    }

    pub fn dset_mgmt(nsid: u32, nr: u8, prp1: u64, attr: u32) -> Self {
        let mut cmd = Self::new(0x09);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.cdw10 = nr as u32;
        cmd.cdw11 = attr;
        return cmd;
    }

    pub fn wr_zero(nsid: u32, slba: u64, nlb: u16) -> Self {
        let mut cmd = Self::new(0x08);
        cmd.nsid = nsid;
        cmd.cdw10 = slba as u32;
        cmd.cdw11 = (slba >> 32) as u32;
        cmd.cdw12 = nlb as u32;
        return cmd;
    }

    pub fn cmp(nsid: u32, slba: u64, nlb: u16, prp1: u64, prp2: u64) -> Self {
        let mut cmd = Self::new(0x05);
        cmd.nsid = nsid;
        cmd.prp1 = prp1;
        cmd.prp2 = prp2;
        cmd.cdw10 = slba as u32;
        cmd.cdw11 = (slba >> 32) as u32;
        cmd.cdw12 = nlb as u32;
        return cmd;
    }

    pub fn sanitise(action: u8, ause: bool, owpass: u8, oipbp: bool, nodas: bool) -> Self {
        let mut cmd = Self::new(0x84);
        let mut cdw10 = (action & 0x7) as u32;
        if ause {
            cdw10 |= 1 << 3;
        }
        cdw10 |= ((owpass & 0xF) as u32) << 4;
        if oipbp {
            cdw10 |= 1 << 8;
        }
        if nodas {
            cdw10 |= 1 << 9;
        }
        cmd.cdw10 = cdw10;
        return cmd;
    }

    pub fn verify(nsid: u32, slba: u64, nlb: u16) -> Self {
        let mut cmd = Self::new(0x0C);
        cmd.nsid = nsid;
        cmd.cdw10 = slba as u32;
        cmd.cdw11 = (slba >> 32) as u32;
        cmd.cdw12 = nlb as u32;
        return cmd;
    }

    pub fn fw_dl(prp1: u64, prp2: u64, numd: u32, offset: u32) -> Self {
        let mut cmd = Self::new(0x11);
        cmd.prp1 = prp1;
        cmd.prp2 = prp2;
        cmd.cdw10 = numd;
        cmd.cdw11 = offset;
        return cmd;
    }

    pub fn fw_commit(slot: u8, action: u8) -> Self {
        let mut cmd = Self::new(0x10);
        cmd.cdw10 = ((action as u32) << 3) | (slot as u32);
        return cmd;
    }
}
