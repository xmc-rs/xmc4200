use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HRC mode configuration"]
    pub gc: GC,
    #[doc = "0x04 - HRC output passive level"]
    pub pl: PL,
    #[doc = "0x08 - HRC global control selection"]
    pub gsel: GSEL,
    #[doc = "0x0c - HRC timer selection"]
    pub tsel: TSEL,
    #[doc = "0x10 - HRC current source for shadow"]
    pub sc: SC,
    #[doc = "0x14 - HRC dead time rising value"]
    pub dcr: DCR,
    #[doc = "0x18 - HRC dead time falling value"]
    pub dcf: DCF,
    #[doc = "0x1c - HRC rising edge value"]
    pub cr1: CR1,
    #[doc = "0x20 - HRC falling edge value"]
    pub cr2: CR2,
    #[doc = "0x24 - HRC next source for shadow"]
    pub ssc: SSC,
    #[doc = "0x28 - HRC shadow dead time rising"]
    pub sdcr: SDCR,
    #[doc = "0x2c - HRC shadow dead time falling"]
    pub sdcf: SDCF,
    #[doc = "0x30 - HRC shadow rising edge value"]
    pub scr1: SCR1,
    #[doc = "0x34 - HRC shadow falling edge value"]
    pub scr2: SCR2,
}
#[doc = "HRC mode configuration"]
pub struct GC {
    register: VolatileCell<u32>,
}
#[doc = "HRC mode configuration"]
pub mod gc;
#[doc = "HRC output passive level"]
pub struct PL {
    register: VolatileCell<u32>,
}
#[doc = "HRC output passive level"]
pub mod pl;
#[doc = "HRC global control selection"]
pub struct GSEL {
    register: VolatileCell<u32>,
}
#[doc = "HRC global control selection"]
pub mod gsel;
#[doc = "HRC timer selection"]
pub struct TSEL {
    register: VolatileCell<u32>,
}
#[doc = "HRC timer selection"]
pub mod tsel;
#[doc = "HRC current source for shadow"]
pub struct SC {
    register: VolatileCell<u32>,
}
#[doc = "HRC current source for shadow"]
pub mod sc;
#[doc = "HRC dead time rising value"]
pub struct DCR {
    register: VolatileCell<u32>,
}
#[doc = "HRC dead time rising value"]
pub mod dcr;
#[doc = "HRC dead time falling value"]
pub struct DCF {
    register: VolatileCell<u32>,
}
#[doc = "HRC dead time falling value"]
pub mod dcf;
#[doc = "HRC rising edge value"]
pub struct CR1 {
    register: VolatileCell<u32>,
}
#[doc = "HRC rising edge value"]
pub mod cr1;
#[doc = "HRC falling edge value"]
pub struct CR2 {
    register: VolatileCell<u32>,
}
#[doc = "HRC falling edge value"]
pub mod cr2;
#[doc = "HRC next source for shadow"]
pub struct SSC {
    register: VolatileCell<u32>,
}
#[doc = "HRC next source for shadow"]
pub mod ssc;
#[doc = "HRC shadow dead time rising"]
pub struct SDCR {
    register: VolatileCell<u32>,
}
#[doc = "HRC shadow dead time rising"]
pub mod sdcr;
#[doc = "HRC shadow dead time falling"]
pub struct SDCF {
    register: VolatileCell<u32>,
}
#[doc = "HRC shadow dead time falling"]
pub mod sdcf;
#[doc = "HRC shadow rising edge value"]
pub struct SCR1 {
    register: VolatileCell<u32>,
}
#[doc = "HRC shadow rising edge value"]
pub mod scr1;
#[doc = "HRC shadow falling edge value"]
pub struct SCR2 {
    register: VolatileCell<u32>,
}
#[doc = "HRC shadow falling edge value"]
pub mod scr2;
