#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    idchip: Idchip,
    idmanuf: Idmanuf,
    _reserved3: [u8; 0x04],
    stcon: Stcon,
    _reserved4: [u8; 0x18],
    gpr0: Gpr0,
    gpr1: Gpr1,
    _reserved6: [u8; 0x18],
    ccucon: Ccucon,
    _reserved7: [u8; 0x3c],
    dtscon: Dtscon,
    dtsstat: Dtsstat,
    _reserved9: [u8; 0x0c],
    g0orcen: G0orcen,
    g1orcen: G1orcen,
    dtemplim: Dtemplim,
    dtempalarm: Dtempalarm,
    _reserved13: [u8; 0x14],
    mirrsts: Mirrsts,
    rmacr: Rmacr,
    rmdata: Rmdata,
    mirrallstat: Mirrallstat,
    mirrallreq: Mirrallreq,
}
impl RegisterBlock {
    #[doc = "0x00 - SCU Module ID Register"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - Chip ID Register"]
    #[inline(always)]
    pub const fn idchip(&self) -> &Idchip {
        &self.idchip
    }
    #[doc = "0x08 - Manufactory ID Register"]
    #[inline(always)]
    pub const fn idmanuf(&self) -> &Idmanuf {
        &self.idmanuf
    }
    #[doc = "0x10 - Startup Configuration Register"]
    #[inline(always)]
    pub const fn stcon(&self) -> &Stcon {
        &self.stcon
    }
    #[doc = "0x2c - General Purpose Register 0"]
    #[inline(always)]
    pub const fn gpr0(&self) -> &Gpr0 {
        &self.gpr0
    }
    #[doc = "0x30 - General Purpose Register 1"]
    #[inline(always)]
    pub const fn gpr1(&self) -> &Gpr1 {
        &self.gpr1
    }
    #[doc = "0x4c - CCU Control Register"]
    #[inline(always)]
    pub const fn ccucon(&self) -> &Ccucon {
        &self.ccucon
    }
    #[doc = "0x8c - Die Temperature Sensor Control Register"]
    #[inline(always)]
    pub const fn dtscon(&self) -> &Dtscon {
        &self.dtscon
    }
    #[doc = "0x90 - Die Temperature Sensor Status Register"]
    #[inline(always)]
    pub const fn dtsstat(&self) -> &Dtsstat {
        &self.dtsstat
    }
    #[doc = "0xa0 - Out of Range Comparator Enable Register 0"]
    #[inline(always)]
    pub const fn g0orcen(&self) -> &G0orcen {
        &self.g0orcen
    }
    #[doc = "0xa4 - Out of Range Comparator Enable Register 1"]
    #[inline(always)]
    pub const fn g1orcen(&self) -> &G1orcen {
        &self.g1orcen
    }
    #[doc = "0xa8 - Die Temperature Sensor Limit Register"]
    #[inline(always)]
    pub const fn dtemplim(&self) -> &Dtemplim {
        &self.dtemplim
    }
    #[doc = "0xac - Die Temperature Sensor Alarm Register"]
    #[inline(always)]
    pub const fn dtempalarm(&self) -> &Dtempalarm {
        &self.dtempalarm
    }
    #[doc = "0xc4 - Mirror Write Status Register"]
    #[inline(always)]
    pub const fn mirrsts(&self) -> &Mirrsts {
        &self.mirrsts
    }
    #[doc = "0xc8 - Retention Memory Access Control Register"]
    #[inline(always)]
    pub const fn rmacr(&self) -> &Rmacr {
        &self.rmacr
    }
    #[doc = "0xcc - Retention Memory Access Data Register"]
    #[inline(always)]
    pub const fn rmdata(&self) -> &Rmdata {
        &self.rmdata
    }
    #[doc = "0xd0 - Mirror All Status"]
    #[inline(always)]
    pub const fn mirrallstat(&self) -> &Mirrallstat {
        &self.mirrallstat
    }
    #[doc = "0xd4 - Mirror All Request"]
    #[inline(always)]
    pub const fn mirrallreq(&self) -> &Mirrallreq {
        &self.mirrallreq
    }
}
#[doc = "ID (r) register accessor: SCU Module ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "SCU Module ID Register"]
pub mod id;
#[doc = "IDCHIP (r) register accessor: Chip ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idchip::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idchip`]
module"]
#[doc(alias = "IDCHIP")]
pub type Idchip = crate::Reg<idchip::IdchipSpec>;
#[doc = "Chip ID Register"]
pub mod idchip;
#[doc = "IDMANUF (r) register accessor: Manufactory ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmanuf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmanuf`]
module"]
#[doc(alias = "IDMANUF")]
pub type Idmanuf = crate::Reg<idmanuf::IdmanufSpec>;
#[doc = "Manufactory ID Register"]
pub mod idmanuf;
#[doc = "STCON (rw) register accessor: Startup Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stcon`]
module"]
#[doc(alias = "STCON")]
pub type Stcon = crate::Reg<stcon::StconSpec>;
#[doc = "Startup Configuration Register"]
pub mod stcon;
#[doc = "GPR0 (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr0`]
module"]
#[doc(alias = "GPR0")]
pub type Gpr0 = crate::Reg<gpr0::Gpr0Spec>;
#[doc = "General Purpose Register 0"]
pub mod gpr0;
#[doc = "GPR1 (rw) register accessor: General Purpose Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr1`]
module"]
#[doc(alias = "GPR1")]
pub type Gpr1 = crate::Reg<gpr1::Gpr1Spec>;
#[doc = "General Purpose Register 1"]
pub mod gpr1;
#[doc = "CCUCON (rw) register accessor: CCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccucon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccucon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccucon`]
module"]
#[doc(alias = "CCUCON")]
pub type Ccucon = crate::Reg<ccucon::CcuconSpec>;
#[doc = "CCU Control Register"]
pub mod ccucon;
#[doc = "DTSCON (rw) register accessor: Die Temperature Sensor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtscon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtscon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtscon`]
module"]
#[doc(alias = "DTSCON")]
pub type Dtscon = crate::Reg<dtscon::DtsconSpec>;
#[doc = "Die Temperature Sensor Control Register"]
pub mod dtscon;
#[doc = "DTSSTAT (r) register accessor: Die Temperature Sensor Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtsstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtsstat`]
module"]
#[doc(alias = "DTSSTAT")]
pub type Dtsstat = crate::Reg<dtsstat::DtsstatSpec>;
#[doc = "Die Temperature Sensor Status Register"]
pub mod dtsstat;
#[doc = "G0ORCEN (rw) register accessor: Out of Range Comparator Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g0orcen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g0orcen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g0orcen`]
module"]
#[doc(alias = "G0ORCEN")]
pub type G0orcen = crate::Reg<g0orcen::G0orcenSpec>;
#[doc = "Out of Range Comparator Enable Register 0"]
pub mod g0orcen;
#[doc = "G1ORCEN (rw) register accessor: Out of Range Comparator Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g1orcen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g1orcen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g1orcen`]
module"]
#[doc(alias = "G1ORCEN")]
pub type G1orcen = crate::Reg<g1orcen::G1orcenSpec>;
#[doc = "Out of Range Comparator Enable Register 1"]
pub mod g1orcen;
#[doc = "DTEMPLIM (rw) register accessor: Die Temperature Sensor Limit Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtemplim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtemplim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtemplim`]
module"]
#[doc(alias = "DTEMPLIM")]
pub type Dtemplim = crate::Reg<dtemplim::DtemplimSpec>;
#[doc = "Die Temperature Sensor Limit Register"]
pub mod dtemplim;
#[doc = "DTEMPALARM (r) register accessor: Die Temperature Sensor Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtempalarm::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtempalarm`]
module"]
#[doc(alias = "DTEMPALARM")]
pub type Dtempalarm = crate::Reg<dtempalarm::DtempalarmSpec>;
#[doc = "Die Temperature Sensor Alarm Register"]
pub mod dtempalarm;
#[doc = "MIRRSTS (r) register accessor: Mirror Write Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mirrsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mirrsts`]
module"]
#[doc(alias = "MIRRSTS")]
pub type Mirrsts = crate::Reg<mirrsts::MirrstsSpec>;
#[doc = "Mirror Write Status Register"]
pub mod mirrsts;
#[doc = "RMACR (rw) register accessor: Retention Memory Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmacr`]
module"]
#[doc(alias = "RMACR")]
pub type Rmacr = crate::Reg<rmacr::RmacrSpec>;
#[doc = "Retention Memory Access Control Register"]
pub mod rmacr;
#[doc = "RMDATA (rw) register accessor: Retention Memory Access Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmdata`]
module"]
#[doc(alias = "RMDATA")]
pub type Rmdata = crate::Reg<rmdata::RmdataSpec>;
#[doc = "Retention Memory Access Data Register"]
pub mod rmdata;
#[doc = "MIRRALLSTAT (r) register accessor: Mirror All Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mirrallstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mirrallstat`]
module"]
#[doc(alias = "MIRRALLSTAT")]
pub type Mirrallstat = crate::Reg<mirrallstat::MirrallstatSpec>;
#[doc = "Mirror All Status"]
pub mod mirrallstat;
#[doc = "MIRRALLREQ (w) register accessor: Mirror All Request\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mirrallreq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mirrallreq`]
module"]
#[doc(alias = "MIRRALLREQ")]
pub type Mirrallreq = crate::Reg<mirrallreq::MirrallreqSpec>;
#[doc = "Mirror All Request"]
pub mod mirrallreq;
