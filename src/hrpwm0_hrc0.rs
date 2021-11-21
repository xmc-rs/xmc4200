#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HRC mode configuration"]
    pub gc: crate::Reg<gc::GC_SPEC>,
    #[doc = "0x04 - HRC output passive level"]
    pub pl: crate::Reg<pl::PL_SPEC>,
    #[doc = "0x08 - HRC global control selection"]
    pub gsel: crate::Reg<gsel::GSEL_SPEC>,
    #[doc = "0x0c - HRC timer selection"]
    pub tsel: crate::Reg<tsel::TSEL_SPEC>,
    #[doc = "0x10 - HRC current source for shadow"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x14 - HRC dead time rising value"]
    pub dcr: crate::Reg<dcr::DCR_SPEC>,
    #[doc = "0x18 - HRC dead time falling value"]
    pub dcf: crate::Reg<dcf::DCF_SPEC>,
    #[doc = "0x1c - HRC rising edge value"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x20 - HRC falling edge value"]
    pub cr2: crate::Reg<cr2::CR2_SPEC>,
    #[doc = "0x24 - HRC next source for shadow"]
    pub ssc: crate::Reg<ssc::SSC_SPEC>,
    #[doc = "0x28 - HRC shadow dead time rising"]
    pub sdcr: crate::Reg<sdcr::SDCR_SPEC>,
    #[doc = "0x2c - HRC shadow dead time falling"]
    pub sdcf: crate::Reg<sdcf::SDCF_SPEC>,
    #[doc = "0x30 - HRC shadow rising edge value"]
    pub scr1: crate::Reg<scr1::SCR1_SPEC>,
    #[doc = "0x34 - HRC shadow falling edge value"]
    pub scr2: crate::Reg<scr2::SCR2_SPEC>,
}
#[doc = "GC register accessor: an alias for `Reg<GC_SPEC>`"]
pub type GC = crate::Reg<gc::GC_SPEC>;
#[doc = "HRC mode configuration"]
pub mod gc;
#[doc = "PL register accessor: an alias for `Reg<PL_SPEC>`"]
pub type PL = crate::Reg<pl::PL_SPEC>;
#[doc = "HRC output passive level"]
pub mod pl;
#[doc = "GSEL register accessor: an alias for `Reg<GSEL_SPEC>`"]
pub type GSEL = crate::Reg<gsel::GSEL_SPEC>;
#[doc = "HRC global control selection"]
pub mod gsel;
#[doc = "TSEL register accessor: an alias for `Reg<TSEL_SPEC>`"]
pub type TSEL = crate::Reg<tsel::TSEL_SPEC>;
#[doc = "HRC timer selection"]
pub mod tsel;
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "HRC current source for shadow"]
pub mod sc;
#[doc = "DCR register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "HRC dead time rising value"]
pub mod dcr;
#[doc = "DCF register accessor: an alias for `Reg<DCF_SPEC>`"]
pub type DCF = crate::Reg<dcf::DCF_SPEC>;
#[doc = "HRC dead time falling value"]
pub mod dcf;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "HRC rising edge value"]
pub mod cr1;
#[doc = "CR2 register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "HRC falling edge value"]
pub mod cr2;
#[doc = "SSC register accessor: an alias for `Reg<SSC_SPEC>`"]
pub type SSC = crate::Reg<ssc::SSC_SPEC>;
#[doc = "HRC next source for shadow"]
pub mod ssc;
#[doc = "SDCR register accessor: an alias for `Reg<SDCR_SPEC>`"]
pub type SDCR = crate::Reg<sdcr::SDCR_SPEC>;
#[doc = "HRC shadow dead time rising"]
pub mod sdcr;
#[doc = "SDCF register accessor: an alias for `Reg<SDCF_SPEC>`"]
pub type SDCF = crate::Reg<sdcf::SDCF_SPEC>;
#[doc = "HRC shadow dead time falling"]
pub mod sdcf;
#[doc = "SCR1 register accessor: an alias for `Reg<SCR1_SPEC>`"]
pub type SCR1 = crate::Reg<scr1::SCR1_SPEC>;
#[doc = "HRC shadow rising edge value"]
pub mod scr1;
#[doc = "SCR2 register accessor: an alias for `Reg<SCR2_SPEC>`"]
pub type SCR2 = crate::Reg<scr2::SCR2_SPEC>;
#[doc = "HRC shadow falling edge value"]
pub mod scr2;
