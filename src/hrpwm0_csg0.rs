#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dci: DCI,
    ies: IES,
    sc: SC,
    pc: PC,
    dsv1: DSV1,
    dsv2: DSV2,
    sdsv1: SDSV1,
    spc: SPC,
    cc: CC,
    plc: PLC,
    blv: BLV,
    sre: SRE,
    srs: SRS,
    sws: SWS,
    swc: SWC,
    istat: ISTAT,
}
impl RegisterBlock {
    #[doc = "0x00 - External input selection"]
    #[inline(always)]
    pub const fn dci(&self) -> &DCI {
        &self.dci
    }
    #[doc = "0x04 - External input selection"]
    #[inline(always)]
    pub const fn ies(&self) -> &IES {
        &self.ies
    }
    #[doc = "0x08 - Slope generation control"]
    #[inline(always)]
    pub const fn sc(&self) -> &SC {
        &self.sc
    }
    #[doc = "0x0c - Pulse swallow configuration"]
    #[inline(always)]
    pub const fn pc(&self) -> &PC {
        &self.pc
    }
    #[doc = "0x10 - DAC reference value 1"]
    #[inline(always)]
    pub const fn dsv1(&self) -> &DSV1 {
        &self.dsv1
    }
    #[doc = "0x14 - DAC reference value 1"]
    #[inline(always)]
    pub const fn dsv2(&self) -> &DSV2 {
        &self.dsv2
    }
    #[doc = "0x18 - Shadow reference value 1"]
    #[inline(always)]
    pub const fn sdsv1(&self) -> &SDSV1 {
        &self.sdsv1
    }
    #[doc = "0x1c - Shadow Pulse swallow value"]
    #[inline(always)]
    pub const fn spc(&self) -> &SPC {
        &self.spc
    }
    #[doc = "0x20 - Comparator configuration"]
    #[inline(always)]
    pub const fn cc(&self) -> &CC {
        &self.cc
    }
    #[doc = "0x24 - Passive level configuration"]
    #[inline(always)]
    pub const fn plc(&self) -> &PLC {
        &self.plc
    }
    #[doc = "0x28 - Comparator blanking value"]
    #[inline(always)]
    pub const fn blv(&self) -> &BLV {
        &self.blv
    }
    #[doc = "0x2c - Service request enable"]
    #[inline(always)]
    pub const fn sre(&self) -> &SRE {
        &self.sre
    }
    #[doc = "0x30 - Service request line selector"]
    #[inline(always)]
    pub const fn srs(&self) -> &SRS {
        &self.srs
    }
    #[doc = "0x34 - Service request SW set"]
    #[inline(always)]
    pub const fn sws(&self) -> &SWS {
        &self.sws
    }
    #[doc = "0x38 - Service request SW clear"]
    #[inline(always)]
    pub const fn swc(&self) -> &SWC {
        &self.swc
    }
    #[doc = "0x3c - Service request status"]
    #[inline(always)]
    pub const fn istat(&self) -> &ISTAT {
        &self.istat
    }
}
#[doc = "DCI (rw) register accessor: External input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dci::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dci::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dci`]
module"]
pub type DCI = crate::Reg<dci::DCI_SPEC>;
#[doc = "External input selection"]
pub mod dci;
#[doc = "IES (rw) register accessor: External input selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ies::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ies::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ies`]
module"]
pub type IES = crate::Reg<ies::IES_SPEC>;
#[doc = "External input selection"]
pub mod ies;
#[doc = "SC (rw) register accessor: Slope generation control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Slope generation control"]
pub mod sc;
#[doc = "PC (r) register accessor: Pulse swallow configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`]
module"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Pulse swallow configuration"]
pub mod pc;
#[doc = "DSV1 (r) register accessor: DAC reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv1`]
module"]
pub type DSV1 = crate::Reg<dsv1::DSV1_SPEC>;
#[doc = "DAC reference value 1"]
pub mod dsv1;
#[doc = "DSV2 (rw) register accessor: DAC reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dsv2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsv2`]
module"]
pub type DSV2 = crate::Reg<dsv2::DSV2_SPEC>;
#[doc = "DAC reference value 1"]
pub mod dsv2;
#[doc = "SDSV1 (rw) register accessor: Shadow reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdsv1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdsv1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdsv1`]
module"]
pub type SDSV1 = crate::Reg<sdsv1::SDSV1_SPEC>;
#[doc = "Shadow reference value 1"]
pub mod sdsv1;
#[doc = "SPC (rw) register accessor: Shadow Pulse swallow value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spc`]
module"]
pub type SPC = crate::Reg<spc::SPC_SPEC>;
#[doc = "Shadow Pulse swallow value"]
pub mod spc;
#[doc = "CC (rw) register accessor: Comparator configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Comparator configuration"]
pub mod cc;
#[doc = "PLC (rw) register accessor: Passive level configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plc`]
module"]
pub type PLC = crate::Reg<plc::PLC_SPEC>;
#[doc = "Passive level configuration"]
pub mod plc;
#[doc = "BLV (rw) register accessor: Comparator blanking value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blv`]
module"]
pub type BLV = crate::Reg<blv::BLV_SPEC>;
#[doc = "Comparator blanking value"]
pub mod blv;
#[doc = "SRE (rw) register accessor: Service request enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sre`]
module"]
pub type SRE = crate::Reg<sre::SRE_SPEC>;
#[doc = "Service request enable"]
pub mod sre;
#[doc = "SRS (rw) register accessor: Service request line selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs`]
module"]
pub type SRS = crate::Reg<srs::SRS_SPEC>;
#[doc = "Service request line selector"]
pub mod srs;
#[doc = "SWS (w) register accessor: Service request SW set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sws::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sws`]
module"]
pub type SWS = crate::Reg<sws::SWS_SPEC>;
#[doc = "Service request SW set"]
pub mod sws;
#[doc = "SWC (w) register accessor: Service request SW clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swc`]
module"]
pub type SWC = crate::Reg<swc::SWC_SPEC>;
#[doc = "Service request SW clear"]
pub mod swc;
#[doc = "ISTAT (r) register accessor: Service request status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@istat`]
module"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Service request status"]
pub mod istat;
