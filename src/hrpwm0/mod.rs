use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bias and suspend configuration"]
    pub hrbsc: HRBSC,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - Module identification register"]
    pub midr: MIDR,
    _reserved1: [u8; 8usize],
    #[doc = "0x14 - Global Analog Configuration"]
    pub glbana: GLBANA,
    _reserved2: [u8; 8usize],
    #[doc = "0x20 - Global CSG configuration"]
    pub csgcfg: CSGCFG,
    #[doc = "0x24 - Global CSG run bit set"]
    pub csgsetg: CSGSETG,
    #[doc = "0x28 - Global CSG run bit clear"]
    pub csgclrg: CSGCLRG,
    #[doc = "0x2c - Global CSG run bit status"]
    pub csgstatg: CSGSTATG,
    #[doc = "0x30 - Global CSG slope/prescaler control"]
    pub csgfcg: CSGFCG,
    #[doc = "0x34 - Global CSG slope/prescaler status"]
    pub csgfsg: CSGFSG,
    #[doc = "0x38 - Global CSG shadow/switch trigger"]
    pub csgtrg: CSGTRG,
    #[doc = "0x3c - Global CSG shadow trigger clear"]
    pub csgtrc: CSGTRC,
    #[doc = "0x40 - Global CSG shadow/switch status"]
    pub csgtrsg: CSGTRSG,
    _reserved3: [u8; 28usize],
    #[doc = "0x60 - Global HRC configuration"]
    pub hrccfg: HRCCFG,
    #[doc = "0x64 - Global HRC shadow trigger set"]
    pub hrcstrg: HRCSTRG,
    #[doc = "0x68 - Global HRC shadow trigger clear"]
    pub hrcctrg: HRCCTRG,
    #[doc = "0x6c - Global HRC shadow transfer status"]
    pub hrcstsg: HRCSTSG,
    #[doc = "0x70 - High Resolution Generation Status"]
    pub hrghrs: HRGHRS,
}
#[doc = "Bias and suspend configuration"]
pub struct HRBSC {
    register: VolatileCell<u32>,
}
#[doc = "Bias and suspend configuration"]
pub mod hrbsc;
#[doc = "Module identification register"]
pub struct MIDR {
    register: VolatileCell<u32>,
}
#[doc = "Module identification register"]
pub mod midr;
#[doc = "Global Analog Configuration"]
pub struct GLBANA {
    register: VolatileCell<u32>,
}
#[doc = "Global Analog Configuration"]
pub mod glbana;
#[doc = "Global CSG configuration"]
pub struct CSGCFG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG configuration"]
pub mod csgcfg;
#[doc = "Global CSG run bit set"]
pub struct CSGSETG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG run bit set"]
pub mod csgsetg;
#[doc = "Global CSG run bit clear"]
pub struct CSGCLRG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG run bit clear"]
pub mod csgclrg;
#[doc = "Global CSG run bit status"]
pub struct CSGSTATG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG run bit status"]
pub mod csgstatg;
#[doc = "Global CSG slope/prescaler control"]
pub struct CSGFCG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG slope/prescaler control"]
pub mod csgfcg;
#[doc = "Global CSG slope/prescaler status"]
pub struct CSGFSG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG slope/prescaler status"]
pub mod csgfsg;
#[doc = "Global CSG shadow/switch trigger"]
pub struct CSGTRG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG shadow/switch trigger"]
pub mod csgtrg;
#[doc = "Global CSG shadow trigger clear"]
pub struct CSGTRC {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG shadow trigger clear"]
pub mod csgtrc;
#[doc = "Global CSG shadow/switch status"]
pub struct CSGTRSG {
    register: VolatileCell<u32>,
}
#[doc = "Global CSG shadow/switch status"]
pub mod csgtrsg;
#[doc = "Global HRC configuration"]
pub struct HRCCFG {
    register: VolatileCell<u32>,
}
#[doc = "Global HRC configuration"]
pub mod hrccfg;
#[doc = "Global HRC shadow trigger set"]
pub struct HRCSTRG {
    register: VolatileCell<u32>,
}
#[doc = "Global HRC shadow trigger set"]
pub mod hrcstrg;
#[doc = "Global HRC shadow trigger clear"]
pub struct HRCCTRG {
    register: VolatileCell<u32>,
}
#[doc = "Global HRC shadow trigger clear"]
pub mod hrcctrg;
#[doc = "Global HRC shadow transfer status"]
pub struct HRCSTSG {
    register: VolatileCell<u32>,
}
#[doc = "Global HRC shadow transfer status"]
pub mod hrcstsg;
#[doc = "High Resolution Generation Status"]
pub struct HRGHRS {
    register: VolatileCell<u32>,
}
#[doc = "High Resolution Generation Status"]
pub mod hrghrs;
