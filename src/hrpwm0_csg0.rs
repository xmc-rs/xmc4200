#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External input selection"]
    pub dci: DCI,
    #[doc = "0x04 - External input selection"]
    pub ies: IES,
    #[doc = "0x08 - Slope generation control"]
    pub sc: SC,
    #[doc = "0x0c - Pulse swallow configuration"]
    pub pc: PC,
    #[doc = "0x10 - DAC reference value 1"]
    pub dsv1: DSV1,
    #[doc = "0x14 - DAC reference value 1"]
    pub dsv2: DSV2,
    #[doc = "0x18 - Shadow reference value 1"]
    pub sdsv1: SDSV1,
    #[doc = "0x1c - Shadow Pulse swallow value"]
    pub spc: SPC,
    #[doc = "0x20 - Comparator configuration"]
    pub cc: CC,
    #[doc = "0x24 - Passive level configuration"]
    pub plc: PLC,
    #[doc = "0x28 - Comparator blanking value"]
    pub blv: BLV,
    #[doc = "0x2c - Service request enable"]
    pub sre: SRE,
    #[doc = "0x30 - Service request line selector"]
    pub srs: SRS,
    #[doc = "0x34 - Service request SW set"]
    pub sws: SWS,
    #[doc = "0x38 - Service request SW clear"]
    pub swc: SWC,
    #[doc = "0x3c - Service request status"]
    pub istat: ISTAT,
}
#[doc = "DCI (rw) register accessor: an alias for `Reg<DCI_SPEC>`"]
pub type DCI = crate::Reg<dci::DCI_SPEC>;
#[doc = "External input selection"]
pub mod dci;
#[doc = "IES (rw) register accessor: an alias for `Reg<IES_SPEC>`"]
pub type IES = crate::Reg<ies::IES_SPEC>;
#[doc = "External input selection"]
pub mod ies;
#[doc = "SC (rw) register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "Slope generation control"]
pub mod sc;
#[doc = "PC (r) register accessor: an alias for `Reg<PC_SPEC>`"]
pub type PC = crate::Reg<pc::PC_SPEC>;
#[doc = "Pulse swallow configuration"]
pub mod pc;
#[doc = "DSV1 (r) register accessor: an alias for `Reg<DSV1_SPEC>`"]
pub type DSV1 = crate::Reg<dsv1::DSV1_SPEC>;
#[doc = "DAC reference value 1"]
pub mod dsv1;
#[doc = "DSV2 (rw) register accessor: an alias for `Reg<DSV2_SPEC>`"]
pub type DSV2 = crate::Reg<dsv2::DSV2_SPEC>;
#[doc = "DAC reference value 1"]
pub mod dsv2;
#[doc = "SDSV1 (rw) register accessor: an alias for `Reg<SDSV1_SPEC>`"]
pub type SDSV1 = crate::Reg<sdsv1::SDSV1_SPEC>;
#[doc = "Shadow reference value 1"]
pub mod sdsv1;
#[doc = "SPC (rw) register accessor: an alias for `Reg<SPC_SPEC>`"]
pub type SPC = crate::Reg<spc::SPC_SPEC>;
#[doc = "Shadow Pulse swallow value"]
pub mod spc;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Comparator configuration"]
pub mod cc;
#[doc = "PLC (rw) register accessor: an alias for `Reg<PLC_SPEC>`"]
pub type PLC = crate::Reg<plc::PLC_SPEC>;
#[doc = "Passive level configuration"]
pub mod plc;
#[doc = "BLV (rw) register accessor: an alias for `Reg<BLV_SPEC>`"]
pub type BLV = crate::Reg<blv::BLV_SPEC>;
#[doc = "Comparator blanking value"]
pub mod blv;
#[doc = "SRE (rw) register accessor: an alias for `Reg<SRE_SPEC>`"]
pub type SRE = crate::Reg<sre::SRE_SPEC>;
#[doc = "Service request enable"]
pub mod sre;
#[doc = "SRS (rw) register accessor: an alias for `Reg<SRS_SPEC>`"]
pub type SRS = crate::Reg<srs::SRS_SPEC>;
#[doc = "Service request line selector"]
pub mod srs;
#[doc = "SWS (w) register accessor: an alias for `Reg<SWS_SPEC>`"]
pub type SWS = crate::Reg<sws::SWS_SPEC>;
#[doc = "Service request SW set"]
pub mod sws;
#[doc = "SWC (w) register accessor: an alias for `Reg<SWC_SPEC>`"]
pub type SWC = crate::Reg<swc::SWC_SPEC>;
#[doc = "Service request SW clear"]
pub mod swc;
#[doc = "ISTAT (r) register accessor: an alias for `Reg<ISTAT_SPEC>`"]
pub type ISTAT = crate::Reg<istat::ISTAT_SPEC>;
#[doc = "Service request status"]
pub mod istat;
