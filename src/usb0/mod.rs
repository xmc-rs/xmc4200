use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0x1c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0x20 - Receive Status Read and Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0x24 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    _reserved1: [u8; 16usize],
    #[doc = "0x3c - USB Module Identification Register"]
    pub guid: GUID,
    _reserved2: [u8; 28usize],
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved3: [u8; 164usize],
    #[doc = "0x104 - Device IN Endpoint 1 Transmit FIFO Size Register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - Device IN Endpoint 2 Transmit FIFO Size Register"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x10c - Device IN Endpoint 3 Transmit FIFO Size Register"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x110 - Device IN Endpoint 4 Transmit FIFO Size Register"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x114 - Device IN Endpoint 5 Transmit FIFO Size Register"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0x118 - Device IN Endpoint 6 Transmit FIFO Size Register"]
    pub dieptxf6: DIEPTXF6,
    _reserved4: [u8; 1764usize],
    #[doc = "0x800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved5: [u8; 4usize],
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved6: [u8; 8usize],
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved7: [u8; 4usize],
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved8: [u8; 1480usize],
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "AHB Configuration Register"]
pub struct GAHBCFG {
    register: VolatileCell<u32>,
}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register"]
pub struct GUSBCFG {
    register: VolatileCell<u32>,
}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register"]
pub struct GRSTCTL {
    register: VolatileCell<u32>,
}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register"]
pub struct GINTSTS {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "Interrupt Mask Register"]
pub struct GINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "Receive Status Debug Read Register"]
pub struct GRXSTSR {
    register: VolatileCell<u32>,
}
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "Receive Status Read and Pop Register"]
pub struct GRXSTSP {
    register: VolatileCell<u32>,
}
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "Receive FIFO Size Register"]
pub struct GRXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-Periodic Transmit FIFO Size Register"]
pub struct GNPTXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Non-Periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "USB Module Identification Register"]
pub struct GUID {
    register: VolatileCell<u32>,
}
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "Global DFIFO Software Config Register"]
pub struct GDFIFOCFG {
    register: VolatileCell<u32>,
}
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "Device IN Endpoint 1 Transmit FIFO Size Register"]
pub struct DIEPTXF1 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 1 Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint 2 Transmit FIFO Size Register"]
pub struct DIEPTXF2 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 2 Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint 3 Transmit FIFO Size Register"]
pub struct DIEPTXF3 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 3 Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "Device IN Endpoint 4 Transmit FIFO Size Register"]
pub struct DIEPTXF4 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 4 Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "Device IN Endpoint 5 Transmit FIFO Size Register"]
pub struct DIEPTXF5 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 5 Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "Device IN Endpoint 6 Transmit FIFO Size Register"]
pub struct DIEPTXF6 {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint 6 Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "Device Configuration Register"]
pub struct DCFG {
    register: VolatileCell<u32>,
}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register"]
pub struct DCTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register"]
pub struct DSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub struct DIEPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub struct DOEPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register"]
pub struct DAINT {
    register: VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub struct DAINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device VBUS Discharge Time Register"]
pub struct DVBUSDIS {
    register: VolatileCell<u32>,
}
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "Device VBUS Pulsing Time Register"]
pub struct DVBUSPULSE {
    register: VolatileCell<u32>,
}
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub struct DIEPEMPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Power and Clock Gating Control Register"]
pub struct PCGCCTL {
    register: VolatileCell<u32>,
}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
