#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: ID,
    idchip: IDCHIP,
    idmanuf: IDMANUF,
    _reserved3: [u8; 0x04],
    stcon: STCON,
    _reserved4: [u8; 0x18],
    gpr0: GPR0,
    gpr1: GPR1,
    _reserved6: [u8; 0x18],
    ccucon: CCUCON,
    _reserved7: [u8; 0x3c],
    dtscon: DTSCON,
    dtsstat: DTSSTAT,
    _reserved9: [u8; 0x0c],
    g0orcen: G0ORCEN,
    g1orcen: G1ORCEN,
    dtemplim: DTEMPLIM,
    dtempalarm: DTEMPALARM,
    _reserved13: [u8; 0x14],
    mirrsts: MIRRSTS,
    rmacr: RMACR,
    rmdata: RMDATA,
    mirrallstat: MIRRALLSTAT,
    mirrallreq: MIRRALLREQ,
}
impl RegisterBlock {
    #[doc = "0x00 - SCU Module ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &ID {
        &self.id
    }
    #[doc = "0x04 - Chip ID Register"]
    #[inline(always)]
    pub const fn idchip(&self) -> &IDCHIP {
        &self.idchip
    }
    #[doc = "0x08 - Manufactory ID Register"]
    #[inline(always)]
    pub const fn idmanuf(&self) -> &IDMANUF {
        &self.idmanuf
    }
    #[doc = "0x10 - Startup Configuration Register"]
    #[inline(always)]
    pub const fn stcon(&self) -> &STCON {
        &self.stcon
    }
    #[doc = "0x2c - General Purpose Register 0"]
    #[inline(always)]
    pub const fn gpr0(&self) -> &GPR0 {
        &self.gpr0
    }
    #[doc = "0x30 - General Purpose Register 1"]
    #[inline(always)]
    pub const fn gpr1(&self) -> &GPR1 {
        &self.gpr1
    }
    #[doc = "0x4c - CCU Control Register"]
    #[inline(always)]
    pub const fn ccucon(&self) -> &CCUCON {
        &self.ccucon
    }
    #[doc = "0x8c - Die Temperature Sensor Control Register"]
    #[inline(always)]
    pub const fn dtscon(&self) -> &DTSCON {
        &self.dtscon
    }
    #[doc = "0x90 - Die Temperature Sensor Status Register"]
    #[inline(always)]
    pub const fn dtsstat(&self) -> &DTSSTAT {
        &self.dtsstat
    }
    #[doc = "0xa0 - Out of Range Comparator Enable Register 0"]
    #[inline(always)]
    pub const fn g0orcen(&self) -> &G0ORCEN {
        &self.g0orcen
    }
    #[doc = "0xa4 - Out of Range Comparator Enable Register 1"]
    #[inline(always)]
    pub const fn g1orcen(&self) -> &G1ORCEN {
        &self.g1orcen
    }
    #[doc = "0xa8 - Die Temperature Sensor Limit Register"]
    #[inline(always)]
    pub const fn dtemplim(&self) -> &DTEMPLIM {
        &self.dtemplim
    }
    #[doc = "0xac - Die Temperature Sensor Alarm Register"]
    #[inline(always)]
    pub const fn dtempalarm(&self) -> &DTEMPALARM {
        &self.dtempalarm
    }
    #[doc = "0xc4 - Mirror Write Status Register"]
    #[inline(always)]
    pub const fn mirrsts(&self) -> &MIRRSTS {
        &self.mirrsts
    }
    #[doc = "0xc8 - Retention Memory Access Control Register"]
    #[inline(always)]
    pub const fn rmacr(&self) -> &RMACR {
        &self.rmacr
    }
    #[doc = "0xcc - Retention Memory Access Data Register"]
    #[inline(always)]
    pub const fn rmdata(&self) -> &RMDATA {
        &self.rmdata
    }
    #[doc = "0xd0 - Mirror All Status"]
    #[inline(always)]
    pub const fn mirrallstat(&self) -> &MIRRALLSTAT {
        &self.mirrallstat
    }
    #[doc = "0xd4 - Mirror All Request"]
    #[inline(always)]
    pub const fn mirrallreq(&self) -> &MIRRALLREQ {
        &self.mirrallreq
    }
}
#[doc = "ID (r) register accessor: SCU Module ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "IDCHIP (r) register accessor: Chip ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idchip::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idchip`]
module"]
pub type IDCHIP = crate::Reg<idchip::IDCHIP_SPEC>;
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "IDMANUF (r) register accessor: Manufactory ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idmanuf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmanuf`]
module"]
pub type IDMANUF = crate::Reg<idmanuf::IDMANUF_SPEC>;
#[doc = "Manufactory ID Register"]
pub mod idmanuf;
#[doc = "STCON (rw) register accessor: Startup Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcon`]
module"]
pub type STCON = crate::Reg<stcon::STCON_SPEC>;
#[doc = "Startup Configuration Register"]
pub mod stcon;
#[doc = "GPR0 (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr0`]
module"]
pub type GPR0 = crate::Reg<gpr0::GPR0_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "GPR1 (rw) register accessor: General Purpose Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr1`]
module"]
pub type GPR1 = crate::Reg<gpr1::GPR1_SPEC>;
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "CCUCON (rw) register accessor: CCU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccucon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccucon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccucon`]
module"]
pub type CCUCON = crate::Reg<ccucon::CCUCON_SPEC>;
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "DTSCON (rw) register accessor: Die Temperature Sensor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtscon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtscon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtscon`]
module"]
pub type DTSCON = crate::Reg<dtscon::DTSCON_SPEC>;
#[doc = "Die Temperature Sensor Control Register"]
pub mod dtscon;
#[doc = "DTSSTAT (r) register accessor: Die Temperature Sensor Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtsstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtsstat`]
module"]
pub type DTSSTAT = crate::Reg<dtsstat::DTSSTAT_SPEC>;
#[doc = "Die Temperature Sensor Status Register"]
pub mod dtsstat;
#[doc = "G0ORCEN (rw) register accessor: Out of Range Comparator Enable Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`g0orcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`g0orcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g0orcen`]
module"]
pub type G0ORCEN = crate::Reg<g0orcen::G0ORCEN_SPEC>;
#[doc = "Out of Range Comparator Enable Register 0"]
pub mod g0orcen;
#[doc = "G1ORCEN (rw) register accessor: Out of Range Comparator Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`g1orcen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`g1orcen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g1orcen`]
module"]
pub type G1ORCEN = crate::Reg<g1orcen::G1ORCEN_SPEC>;
#[doc = "Out of Range Comparator Enable Register 1"]
pub mod g1orcen;
#[doc = "DTEMPLIM (rw) register accessor: Die Temperature Sensor Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtemplim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtemplim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtemplim`]
module"]
pub type DTEMPLIM = crate::Reg<dtemplim::DTEMPLIM_SPEC>;
#[doc = "Die Temperature Sensor Limit Register"]
pub mod dtemplim;
#[doc = "DTEMPALARM (r) register accessor: Die Temperature Sensor Alarm Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtempalarm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtempalarm`]
module"]
pub type DTEMPALARM = crate::Reg<dtempalarm::DTEMPALARM_SPEC>;
#[doc = "Die Temperature Sensor Alarm Register"]
pub mod dtempalarm;
#[doc = "MIRRSTS (r) register accessor: Mirror Write Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mirrsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mirrsts`]
module"]
pub type MIRRSTS = crate::Reg<mirrsts::MIRRSTS_SPEC>;
#[doc = "Mirror Write Status Register"]
pub mod mirrsts;
#[doc = "RMACR (rw) register accessor: Retention Memory Access Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmacr`]
module"]
pub type RMACR = crate::Reg<rmacr::RMACR_SPEC>;
#[doc = "Retention Memory Access Control Register"]
pub mod rmacr;
#[doc = "RMDATA (rw) register accessor: Retention Memory Access Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmdata`]
module"]
pub type RMDATA = crate::Reg<rmdata::RMDATA_SPEC>;
#[doc = "Retention Memory Access Data Register"]
pub mod rmdata;
#[doc = "MIRRALLSTAT (r) register accessor: Mirror All Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mirrallstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mirrallstat`]
module"]
pub type MIRRALLSTAT = crate::Reg<mirrallstat::MIRRALLSTAT_SPEC>;
#[doc = "Mirror All Status"]
pub mod mirrallstat;
#[doc = "MIRRALLREQ (w) register accessor: Mirror All Request\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mirrallreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mirrallreq`]
module"]
pub type MIRRALLREQ = crate::Reg<mirrallreq::MIRRALLREQ_SPEC>;
#[doc = "Mirror All Request"]
pub mod mirrallreq;
