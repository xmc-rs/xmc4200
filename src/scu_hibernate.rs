#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    pub hdstat: HDSTAT,
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    pub hdclr: HDCLR,
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    pub hdset: HDSET,
    #[doc = "0x0c - Hibernate Domain Control Register"]
    pub hdcr: HDCR,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - fOSI Control Register"]
    pub oscsictrl: OSCSICTRL,
    #[doc = "0x18 - OSC_ULP Status Register"]
    pub osculstat: OSCULSTAT,
    #[doc = "0x1c - OSC_ULP Control Register"]
    pub osculctrl: OSCULCTRL,
    #[doc = "0x20 - Analog Wake-up Configuration Register"]
    pub lpacconf: LPACCONF,
    #[doc = "0x24 - LPAC Threshold Register 0"]
    pub lpacth0: LPACTH0,
    #[doc = "0x28 - LPAC Threshold Register 1"]
    pub lpacth1: LPACTH1,
    #[doc = "0x2c - Hibernate Analog Control State Register"]
    pub lpacst: LPACST,
    #[doc = "0x30 - LPAC Control Clear Register"]
    pub lpacclr: LPACCLR,
    #[doc = "0x34 - LPAC Control Set Register"]
    pub lpacset: LPACSET,
    #[doc = "0x38 - Hibernate Internal Control State Register"]
    pub hintst: HINTST,
    #[doc = "0x3c - Hibernate Internal Control Clear Register"]
    pub hintclr: HINTCLR,
    #[doc = "0x40 - Hibernate Internal Control Set Register"]
    pub hintset: HINTSET,
}
#[doc = "HDSTAT (r) register accessor: an alias for `Reg<HDSTAT_SPEC>`"]
pub type HDSTAT = crate::Reg<hdstat::HDSTAT_SPEC>;
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "HDCLR (w) register accessor: an alias for `Reg<HDCLR_SPEC>`"]
pub type HDCLR = crate::Reg<hdclr::HDCLR_SPEC>;
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "HDSET (w) register accessor: an alias for `Reg<HDSET_SPEC>`"]
pub type HDSET = crate::Reg<hdset::HDSET_SPEC>;
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "HDCR (rw) register accessor: an alias for `Reg<HDCR_SPEC>`"]
pub type HDCR = crate::Reg<hdcr::HDCR_SPEC>;
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "OSCSICTRL (rw) register accessor: an alias for `Reg<OSCSICTRL_SPEC>`"]
pub type OSCSICTRL = crate::Reg<oscsictrl::OSCSICTRL_SPEC>;
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSCULSTAT (r) register accessor: an alias for `Reg<OSCULSTAT_SPEC>`"]
pub type OSCULSTAT = crate::Reg<osculstat::OSCULSTAT_SPEC>;
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSCULCTRL (rw) register accessor: an alias for `Reg<OSCULCTRL_SPEC>`"]
pub type OSCULCTRL = crate::Reg<osculctrl::OSCULCTRL_SPEC>;
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
#[doc = "LPACCONF (rw) register accessor: an alias for `Reg<LPACCONF_SPEC>`"]
pub type LPACCONF = crate::Reg<lpacconf::LPACCONF_SPEC>;
#[doc = "Analog Wake-up Configuration Register"]
pub mod lpacconf;
#[doc = "LPACTH0 (rw) register accessor: an alias for `Reg<LPACTH0_SPEC>`"]
pub type LPACTH0 = crate::Reg<lpacth0::LPACTH0_SPEC>;
#[doc = "LPAC Threshold Register 0"]
pub mod lpacth0;
#[doc = "LPACTH1 (rw) register accessor: an alias for `Reg<LPACTH1_SPEC>`"]
pub type LPACTH1 = crate::Reg<lpacth1::LPACTH1_SPEC>;
#[doc = "LPAC Threshold Register 1"]
pub mod lpacth1;
#[doc = "LPACST (r) register accessor: an alias for `Reg<LPACST_SPEC>`"]
pub type LPACST = crate::Reg<lpacst::LPACST_SPEC>;
#[doc = "Hibernate Analog Control State Register"]
pub mod lpacst;
#[doc = "LPACCLR (w) register accessor: an alias for `Reg<LPACCLR_SPEC>`"]
pub type LPACCLR = crate::Reg<lpacclr::LPACCLR_SPEC>;
#[doc = "LPAC Control Clear Register"]
pub mod lpacclr;
#[doc = "LPACSET (w) register accessor: an alias for `Reg<LPACSET_SPEC>`"]
pub type LPACSET = crate::Reg<lpacset::LPACSET_SPEC>;
#[doc = "LPAC Control Set Register"]
pub mod lpacset;
#[doc = "HINTST (r) register accessor: an alias for `Reg<HINTST_SPEC>`"]
pub type HINTST = crate::Reg<hintst::HINTST_SPEC>;
#[doc = "Hibernate Internal Control State Register"]
pub mod hintst;
#[doc = "HINTCLR (w) register accessor: an alias for `Reg<HINTCLR_SPEC>`"]
pub type HINTCLR = crate::Reg<hintclr::HINTCLR_SPEC>;
#[doc = "Hibernate Internal Control Clear Register"]
pub mod hintclr;
#[doc = "HINTSET (w) register accessor: an alias for `Reg<HINTSET_SPEC>`"]
pub type HINTSET = crate::Reg<hintset::HINTSET_SPEC>;
#[doc = "Hibernate Internal Control Set Register"]
pub mod hintset;
