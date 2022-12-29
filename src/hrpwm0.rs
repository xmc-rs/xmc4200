#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bias and suspend configuration"]
    pub hrbsc: HRBSC,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Module identification register"]
    pub midr: MIDR,
    _reserved2: [u8; 0x08],
    #[doc = "0x14 - Global Analog Configuration"]
    pub glbana: GLBANA,
    _reserved3: [u8; 0x08],
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
    _reserved12: [u8; 0x1c],
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
#[doc = "HRBSC (rw) register accessor: an alias for `Reg<HRBSC_SPEC>`"]
pub type HRBSC = crate::Reg<hrbsc::HRBSC_SPEC>;
#[doc = "Bias and suspend configuration"]
pub mod hrbsc;
#[doc = "MIDR (r) register accessor: an alias for `Reg<MIDR_SPEC>`"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module identification register"]
pub mod midr;
#[doc = "GLBANA (rw) register accessor: an alias for `Reg<GLBANA_SPEC>`"]
pub type GLBANA = crate::Reg<glbana::GLBANA_SPEC>;
#[doc = "Global Analog Configuration"]
pub mod glbana;
#[doc = "CSGCFG (rw) register accessor: an alias for `Reg<CSGCFG_SPEC>`"]
pub type CSGCFG = crate::Reg<csgcfg::CSGCFG_SPEC>;
#[doc = "Global CSG configuration"]
pub mod csgcfg;
#[doc = "CSGSETG (w) register accessor: an alias for `Reg<CSGSETG_SPEC>`"]
pub type CSGSETG = crate::Reg<csgsetg::CSGSETG_SPEC>;
#[doc = "Global CSG run bit set"]
pub mod csgsetg;
#[doc = "CSGCLRG (w) register accessor: an alias for `Reg<CSGCLRG_SPEC>`"]
pub type CSGCLRG = crate::Reg<csgclrg::CSGCLRG_SPEC>;
#[doc = "Global CSG run bit clear"]
pub mod csgclrg;
#[doc = "CSGSTATG (r) register accessor: an alias for `Reg<CSGSTATG_SPEC>`"]
pub type CSGSTATG = crate::Reg<csgstatg::CSGSTATG_SPEC>;
#[doc = "Global CSG run bit status"]
pub mod csgstatg;
#[doc = "CSGFCG (w) register accessor: an alias for `Reg<CSGFCG_SPEC>`"]
pub type CSGFCG = crate::Reg<csgfcg::CSGFCG_SPEC>;
#[doc = "Global CSG slope/prescaler control"]
pub mod csgfcg;
#[doc = "CSGFSG (r) register accessor: an alias for `Reg<CSGFSG_SPEC>`"]
pub type CSGFSG = crate::Reg<csgfsg::CSGFSG_SPEC>;
#[doc = "Global CSG slope/prescaler status"]
pub mod csgfsg;
#[doc = "CSGTRG (w) register accessor: an alias for `Reg<CSGTRG_SPEC>`"]
pub type CSGTRG = crate::Reg<csgtrg::CSGTRG_SPEC>;
#[doc = "Global CSG shadow/switch trigger"]
pub mod csgtrg;
#[doc = "CSGTRC (w) register accessor: an alias for `Reg<CSGTRC_SPEC>`"]
pub type CSGTRC = crate::Reg<csgtrc::CSGTRC_SPEC>;
#[doc = "Global CSG shadow trigger clear"]
pub mod csgtrc;
#[doc = "CSGTRSG (r) register accessor: an alias for `Reg<CSGTRSG_SPEC>`"]
pub type CSGTRSG = crate::Reg<csgtrsg::CSGTRSG_SPEC>;
#[doc = "Global CSG shadow/switch status"]
pub mod csgtrsg;
#[doc = "HRCCFG (rw) register accessor: an alias for `Reg<HRCCFG_SPEC>`"]
pub type HRCCFG = crate::Reg<hrccfg::HRCCFG_SPEC>;
#[doc = "Global HRC configuration"]
pub mod hrccfg;
#[doc = "HRCSTRG (w) register accessor: an alias for `Reg<HRCSTRG_SPEC>`"]
pub type HRCSTRG = crate::Reg<hrcstrg::HRCSTRG_SPEC>;
#[doc = "Global HRC shadow trigger set"]
pub mod hrcstrg;
#[doc = "HRCCTRG (w) register accessor: an alias for `Reg<HRCCTRG_SPEC>`"]
pub type HRCCTRG = crate::Reg<hrcctrg::HRCCTRG_SPEC>;
#[doc = "Global HRC shadow trigger clear"]
pub mod hrcctrg;
#[doc = "HRCSTSG (r) register accessor: an alias for `Reg<HRCSTSG_SPEC>`"]
pub type HRCSTSG = crate::Reg<hrcstsg::HRCSTSG_SPEC>;
#[doc = "Global HRC shadow transfer status"]
pub mod hrcstsg;
#[doc = "HRGHRS (r) register accessor: an alias for `Reg<HRGHRS_SPEC>`"]
pub type HRGHRS = crate::Reg<hrghrs::HRGHRS_SPEC>;
#[doc = "High Resolution Generation Status"]
pub mod hrghrs;
