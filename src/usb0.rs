#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - AHB Configuration Register"]
    pub gahbcfg: crate::Reg<gahbcfg::GAHBCFG_SPEC>,
    #[doc = "0x0c - USB Configuration Register"]
    pub gusbcfg: crate::Reg<gusbcfg::GUSBCFG_SPEC>,
    #[doc = "0x10 - Reset Register"]
    pub grstctl: crate::Reg<grstctl::GRSTCTL_SPEC>,
    #[doc = "0x14 - Interrupt Register"]
    pub gintsts: crate::Reg<gintsts::GINTSTS_SPEC>,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub gintmsk: crate::Reg<gintmsk::GINTMSK_SPEC>,
    #[doc = "0x1c - Receive Status Debug Read Register"]
    pub grxstsr: crate::Reg<grxstsr::GRXSTSR_SPEC>,
    #[doc = "0x20 - Receive Status Read and Pop Register"]
    pub grxstsp: crate::Reg<grxstsp::GRXSTSP_SPEC>,
    #[doc = "0x24 - Receive FIFO Size Register"]
    pub grxfsiz: crate::Reg<grxfsiz::GRXFSIZ_SPEC>,
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x3c - USB Module Identification Register"]
    pub guid: crate::Reg<guid::GUID_SPEC>,
    _reserved10: [u8; 0x1c],
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    pub gdfifocfg: crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>,
    _reserved11: [u8; 0xa4],
    #[doc = "0x104 - Device IN Endpoint 1 Transmit FIFO Size Register"]
    pub dieptxf1: crate::Reg<dieptxf1::DIEPTXF1_SPEC>,
    #[doc = "0x108 - Device IN Endpoint 2 Transmit FIFO Size Register"]
    pub dieptxf2: crate::Reg<dieptxf2::DIEPTXF2_SPEC>,
    #[doc = "0x10c - Device IN Endpoint 3 Transmit FIFO Size Register"]
    pub dieptxf3: crate::Reg<dieptxf3::DIEPTXF3_SPEC>,
    #[doc = "0x110 - Device IN Endpoint 4 Transmit FIFO Size Register"]
    pub dieptxf4: crate::Reg<dieptxf4::DIEPTXF4_SPEC>,
    #[doc = "0x114 - Device IN Endpoint 5 Transmit FIFO Size Register"]
    pub dieptxf5: crate::Reg<dieptxf5::DIEPTXF5_SPEC>,
    #[doc = "0x118 - Device IN Endpoint 6 Transmit FIFO Size Register"]
    pub dieptxf6: crate::Reg<dieptxf6::DIEPTXF6_SPEC>,
    _reserved17: [u8; 0x06e4],
    #[doc = "0x800 - Device Configuration Register"]
    pub dcfg: crate::Reg<dcfg::DCFG_SPEC>,
    #[doc = "0x804 - Device Control Register"]
    pub dctl: crate::Reg<dctl::DCTL_SPEC>,
    #[doc = "0x808 - Device Status Register"]
    pub dsts: crate::Reg<dsts::DSTS_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: crate::Reg<diepmsk::DIEPMSK_SPEC>,
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: crate::Reg<doepmsk::DOEPMSK_SPEC>,
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    pub daint: crate::Reg<daint::DAINT_SPEC>,
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: crate::Reg<daintmsk::DAINTMSK_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: crate::Reg<dvbusdis::DVBUSDIS_SPEC>,
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>,
    _reserved27: [u8; 0x05c8],
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    pub pcgcctl: crate::Reg<pcgcctl::PCGCCTL_SPEC>,
}
#[doc = "GAHBCFG register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR register accessor: an alias for `Reg<GRXSTSR_SPEC>`"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP register accessor: an alias for `Reg<GRXSTSP_SPEC>`"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ register accessor: an alias for `Reg<GNPTXFSIZ_SPEC>`"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "Non-Periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GUID register accessor: an alias for `Reg<GUID_SPEC>`"]
pub type GUID = crate::Reg<guid::GUID_SPEC>;
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "GDFIFOCFG register accessor: an alias for `Reg<GDFIFOCFG_SPEC>`"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "DIEPTXF1 register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint 1 Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint 2 Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint 3 Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "DIEPTXF4 register accessor: an alias for `Reg<DIEPTXF4_SPEC>`"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "Device IN Endpoint 4 Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "DIEPTXF5 register accessor: an alias for `Reg<DIEPTXF5_SPEC>`"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "Device IN Endpoint 5 Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "DIEPTXF6 register accessor: an alias for `Reg<DIEPTXF6_SPEC>`"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "Device IN Endpoint 6 Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "DCFG register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DVBUSDIS register accessor: an alias for `Reg<DVBUSDIS_SPEC>`"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "DVBUSPULSE register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "DIEPEMPMSK register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "PCGCCTL register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
