#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External input selection"]
    pub dci: crate::Reg<dci::DCI_SPEC>,
    #[doc = "0x04 - External input selection"]
    pub ies: crate::Reg<ies::IES_SPEC>,
    #[doc = "0x08 - Slope generation control"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x0c - Pulse swallow configuration"]
    pub pc: crate::Reg<pc::PC_SPEC>,
    #[doc = "0x10 - DAC reference value 1"]
    pub dsv1: crate::Reg<dsv1::DSV1_SPEC>,
    #[doc = "0x14 - DAC reference value 1"]
    pub dsv2: crate::Reg<dsv2::DSV2_SPEC>,
    #[doc = "0x18 - Shadow reference value 1"]
    pub sdsv1: crate::Reg<sdsv1::SDSV1_SPEC>,
    #[doc = "0x1c - Shadow Pulse swallow value"]
    pub spc: crate::Reg<spc::SPC_SPEC>,
    #[doc = "0x20 - Comparator configuration"]
    pub cc: crate::Reg<cc::CC_SPEC>,
    #[doc = "0x24 - Passive level configuration"]
    pub plc: crate::Reg<plc::PLC_SPEC>,
    #[doc = "0x28 - Comparator blanking value"]
    pub blv: crate::Reg<blv::BLV_SPEC>,
    #[doc = "0x2c - Service request enable"]
    pub sre: crate::Reg<sre::SRE_SPEC>,
    #[doc = "0x30 - Service request line selector"]
    pub srs: crate::Reg<srs::SRS_SPEC>,
    #[doc = "0x34 - Service request SW set"]
    pub sws: crate::Reg<sws::SWS_SPEC>,
    #[doc = "0x38 - Service request SW clear"]
    pub swc: crate::Reg<swc::SWC_SPEC>,
    #[doc = "0x3c - Service request status"]
    pub istat: crate::Reg<istat::ISTAT_SPEC>,
}
#[doc = "DCI register accessor: an alias for `Reg<DCI_SPEC>`"]
pub type DCI = crate::Reg<dci::DCI_SPEC>;
#[doc = "External input selection"]
pub mod dci;
#[doc = "IES register accessor: an alias for `Reg<IES_SPEC>`"]
pub type IES = crate::Reg<ies::IES_SPEC>;
#[doc = "External input selection"]
pub mod ies;
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Slope generation control"]
pub mod sc;
#[doc = "PC register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Pulse swallow configuration"]
pub mod pc;
#[doc = "DSV1 register accessor: an alias for `Reg<DSV1_SPEC>`"]
pub type DSV1 = crate::Reg<dsv1::DSV1_SPEC>;
#[doc = "DAC reference value 1"]
pub mod dsv1;
#[doc = "DSV2 register accessor: an alias for `Reg<DSV2_SPEC>`"]
pub type DSV2 = crate::Reg<dsv2::DSV2_SPEC>;
#[doc = "DAC reference value 1"]
pub mod dsv2;
#[doc = "SDSV1 register accessor: an alias for `Reg<SDSV1_SPEC>`"]
pub type SDSV1 = crate::Reg<sdsv1::SDSV1_SPEC>;
#[doc = "Shadow reference value 1"]
pub mod sdsv1;
#[doc = "SPC register accessor: an alias for `Reg<SPC_SPEC>`"]
pub type SPC = crate::Reg<spc::SPC_SPEC>;
#[doc = "Shadow Pulse swallow value"]
pub mod spc;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Comparator configuration"]
pub mod cc;
#[doc = "PLC register accessor: an alias for `Reg<PLC_SPEC>`"]
pub type PLC = crate::Reg<plc::PLC_SPEC>;
#[doc = "Passive level configuration"]
pub mod plc;
#[doc = "BLV register accessor: an alias for `Reg<BLV_SPEC>`"]
pub type BLV = crate::Reg<blv::BLV_SPEC>;
#[doc = "Comparator blanking value"]
pub mod blv;
#[doc = "SRE register accessor: an alias for `Reg<SRE_SPEC>`"]
pub type SRE = crate::Reg<sre::SRE_SPEC>;
#[doc = "Service request enable"]
pub mod sre;
#[doc = "SRS register accessor: an alias for `Reg<SRS_SPEC>`"]
pub type SRS = crate::Reg<srs::SRS_SPEC>;
#[doc = "Service request line selector"]
pub mod srs;
#[doc = "SWS register accessor: an alias for `Reg<SWS_SPEC>`"]
pub type SWS = crate::Reg<sws::SWS_SPEC>;
#[doc = "Service request SW set"]
pub mod sws;
#[doc = "SWC register accessor: an alias for `Reg<SWC_SPEC>`"]
pub type SWC = crate::Reg<swc::SWC_SPEC>;
#[doc = "Service request SW clear"]
pub mod swc;
#[doc = "ISTAT register accessor: an alias for `Reg<ISTAT_SPEC>`"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Service request status"]
pub mod istat;
