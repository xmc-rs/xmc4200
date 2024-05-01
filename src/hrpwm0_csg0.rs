#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dci: Dci,
    ies: Ies,
    sc: Sc,
    pc: Pc,
    dsv1: Dsv1,
    dsv2: Dsv2,
    sdsv1: Sdsv1,
    spc: Spc,
    cc: Cc,
    plc: Plc,
    blv: Blv,
    sre: Sre,
    srs: Srs,
    sws: Sws,
    swc: Swc,
    istat: Istat,
}
impl RegisterBlock {
    #[doc = "0x00 - External input selection"]
    #[inline(always)]
    pub const fn dci(&self) -> &Dci {
        &self.dci
    }
    #[doc = "0x04 - External input selection"]
    #[inline(always)]
    pub const fn ies(&self) -> &Ies {
        &self.ies
    }
    #[doc = "0x08 - Slope generation control"]
    #[inline(always)]
    pub const fn sc(&self) -> &Sc {
        &self.sc
    }
    #[doc = "0x0c - Pulse swallow configuration"]
    #[inline(always)]
    pub const fn pc(&self) -> &Pc {
        &self.pc
    }
    #[doc = "0x10 - DAC reference value 1"]
    #[inline(always)]
    pub const fn dsv1(&self) -> &Dsv1 {
        &self.dsv1
    }
    #[doc = "0x14 - DAC reference value 1"]
    #[inline(always)]
    pub const fn dsv2(&self) -> &Dsv2 {
        &self.dsv2
    }
    #[doc = "0x18 - Shadow reference value 1"]
    #[inline(always)]
    pub const fn sdsv1(&self) -> &Sdsv1 {
        &self.sdsv1
    }
    #[doc = "0x1c - Shadow Pulse swallow value"]
    #[inline(always)]
    pub const fn spc(&self) -> &Spc {
        &self.spc
    }
    #[doc = "0x20 - Comparator configuration"]
    #[inline(always)]
    pub const fn cc(&self) -> &Cc {
        &self.cc
    }
    #[doc = "0x24 - Passive level configuration"]
    #[inline(always)]
    pub const fn plc(&self) -> &Plc {
        &self.plc
    }
    #[doc = "0x28 - Comparator blanking value"]
    #[inline(always)]
    pub const fn blv(&self) -> &Blv {
        &self.blv
    }
    #[doc = "0x2c - Service request enable"]
    #[inline(always)]
    pub const fn sre(&self) -> &Sre {
        &self.sre
    }
    #[doc = "0x30 - Service request line selector"]
    #[inline(always)]
    pub const fn srs(&self) -> &Srs {
        &self.srs
    }
    #[doc = "0x34 - Service request SW set"]
    #[inline(always)]
    pub const fn sws(&self) -> &Sws {
        &self.sws
    }
    #[doc = "0x38 - Service request SW clear"]
    #[inline(always)]
    pub const fn swc(&self) -> &Swc {
        &self.swc
    }
    #[doc = "0x3c - Service request status"]
    #[inline(always)]
    pub const fn istat(&self) -> &Istat {
        &self.istat
    }
}
#[doc = "DCI (rw) register accessor: External input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dci::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dci::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dci`]
module"]
#[doc(alias = "DCI")]
pub type Dci = crate::Reg<dci::DciSpec>;
#[doc = "External input selection"]
pub mod dci;
#[doc = "IES (rw) register accessor: External input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ies::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ies::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ies`]
module"]
#[doc(alias = "IES")]
pub type Ies = crate::Reg<ies::IesSpec>;
#[doc = "External input selection"]
pub mod ies;
#[doc = "SC (rw) register accessor: Slope generation control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
#[doc(alias = "SC")]
pub type Sc = crate::Reg<sc::ScSpec>;
#[doc = "Slope generation control"]
pub mod sc;
#[doc = "PC (r) register accessor: Pulse swallow configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`]
module"]
#[doc(alias = "PC")]
pub type Pc = crate::Reg<pc::PcSpec>;
#[doc = "Pulse swallow configuration"]
pub mod pc;
#[doc = "DSV1 (r) register accessor: DAC reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv1`]
module"]
#[doc(alias = "DSV1")]
pub type Dsv1 = crate::Reg<dsv1::Dsv1Spec>;
#[doc = "DAC reference value 1"]
pub mod dsv1;
#[doc = "DSV2 (rw) register accessor: DAC reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv2`]
module"]
#[doc(alias = "DSV2")]
pub type Dsv2 = crate::Reg<dsv2::Dsv2Spec>;
#[doc = "DAC reference value 1"]
pub mod dsv2;
#[doc = "SDSV1 (rw) register accessor: Shadow reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdsv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdsv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdsv1`]
module"]
#[doc(alias = "SDSV1")]
pub type Sdsv1 = crate::Reg<sdsv1::Sdsv1Spec>;
#[doc = "Shadow reference value 1"]
pub mod sdsv1;
#[doc = "SPC (rw) register accessor: Shadow Pulse swallow value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spc`]
module"]
#[doc(alias = "SPC")]
pub type Spc = crate::Reg<spc::SpcSpec>;
#[doc = "Shadow Pulse swallow value"]
pub mod spc;
#[doc = "CC (rw) register accessor: Comparator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Comparator configuration"]
pub mod cc;
#[doc = "PLC (rw) register accessor: Passive level configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plc`]
module"]
#[doc(alias = "PLC")]
pub type Plc = crate::Reg<plc::PlcSpec>;
#[doc = "Passive level configuration"]
pub mod plc;
#[doc = "BLV (rw) register accessor: Comparator blanking value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blv`]
module"]
#[doc(alias = "BLV")]
pub type Blv = crate::Reg<blv::BlvSpec>;
#[doc = "Comparator blanking value"]
pub mod blv;
#[doc = "SRE (rw) register accessor: Service request enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sre`]
module"]
#[doc(alias = "SRE")]
pub type Sre = crate::Reg<sre::SreSpec>;
#[doc = "Service request enable"]
pub mod sre;
#[doc = "SRS (rw) register accessor: Service request line selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs`]
module"]
#[doc(alias = "SRS")]
pub type Srs = crate::Reg<srs::SrsSpec>;
#[doc = "Service request line selector"]
pub mod srs;
#[doc = "SWS (w) register accessor: Service request SW set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sws::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sws`]
module"]
#[doc(alias = "SWS")]
pub type Sws = crate::Reg<sws::SwsSpec>;
#[doc = "Service request SW set"]
pub mod sws;
#[doc = "SWC (w) register accessor: Service request SW clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swc`]
module"]
#[doc(alias = "SWC")]
pub type Swc = crate::Reg<swc::SwcSpec>;
#[doc = "Service request SW clear"]
pub mod swc;
#[doc = "ISTAT (r) register accessor: Service request status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`]
module"]
#[doc(alias = "ISTAT")]
pub type Istat = crate::Reg<istat::IstatSpec>;
#[doc = "Service request status"]
pub mod istat;
