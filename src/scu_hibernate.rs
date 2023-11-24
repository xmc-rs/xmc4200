#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    hdstat: HDSTAT,
    hdclr: HDCLR,
    hdset: HDSET,
    hdcr: HDCR,
    _reserved4: [u8; 0x04],
    oscsictrl: OSCSICTRL,
    osculstat: OSCULSTAT,
    osculctrl: OSCULCTRL,
    lpacconf: LPACCONF,
    lpacth0: LPACTH0,
    lpacth1: LPACTH1,
    lpacst: LPACST,
    lpacclr: LPACCLR,
    lpacset: LPACSET,
    hintst: HINTST,
    hintclr: HINTCLR,
    hintset: HINTSET,
}
impl RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    #[inline(always)]
    pub const fn hdstat(&self) -> &HDSTAT {
        &self.hdstat
    }
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    #[inline(always)]
    pub const fn hdclr(&self) -> &HDCLR {
        &self.hdclr
    }
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    #[inline(always)]
    pub const fn hdset(&self) -> &HDSET {
        &self.hdset
    }
    #[doc = "0x0c - Hibernate Domain Control Register"]
    #[inline(always)]
    pub const fn hdcr(&self) -> &HDCR {
        &self.hdcr
    }
    #[doc = "0x14 - fOSI Control Register"]
    #[inline(always)]
    pub const fn oscsictrl(&self) -> &OSCSICTRL {
        &self.oscsictrl
    }
    #[doc = "0x18 - OSC_ULP Status Register"]
    #[inline(always)]
    pub const fn osculstat(&self) -> &OSCULSTAT {
        &self.osculstat
    }
    #[doc = "0x1c - OSC_ULP Control Register"]
    #[inline(always)]
    pub const fn osculctrl(&self) -> &OSCULCTRL {
        &self.osculctrl
    }
    #[doc = "0x20 - Analog Wake-up Configuration Register"]
    #[inline(always)]
    pub const fn lpacconf(&self) -> &LPACCONF {
        &self.lpacconf
    }
    #[doc = "0x24 - LPAC Threshold Register 0"]
    #[inline(always)]
    pub const fn lpacth0(&self) -> &LPACTH0 {
        &self.lpacth0
    }
    #[doc = "0x28 - LPAC Threshold Register 1"]
    #[inline(always)]
    pub const fn lpacth1(&self) -> &LPACTH1 {
        &self.lpacth1
    }
    #[doc = "0x2c - Hibernate Analog Control State Register"]
    #[inline(always)]
    pub const fn lpacst(&self) -> &LPACST {
        &self.lpacst
    }
    #[doc = "0x30 - LPAC Control Clear Register"]
    #[inline(always)]
    pub const fn lpacclr(&self) -> &LPACCLR {
        &self.lpacclr
    }
    #[doc = "0x34 - LPAC Control Set Register"]
    #[inline(always)]
    pub const fn lpacset(&self) -> &LPACSET {
        &self.lpacset
    }
    #[doc = "0x38 - Hibernate Internal Control State Register"]
    #[inline(always)]
    pub const fn hintst(&self) -> &HINTST {
        &self.hintst
    }
    #[doc = "0x3c - Hibernate Internal Control Clear Register"]
    #[inline(always)]
    pub const fn hintclr(&self) -> &HINTCLR {
        &self.hintclr
    }
    #[doc = "0x40 - Hibernate Internal Control Set Register"]
    #[inline(always)]
    pub const fn hintset(&self) -> &HINTSET {
        &self.hintset
    }
}
#[doc = "HDSTAT (r) register accessor: Hibernate Domain Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdstat`]
module"]
pub type HDSTAT = crate::Reg<hdstat::HDSTAT_SPEC>;
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "HDCLR (w) register accessor: Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdclr`]
module"]
pub type HDCLR = crate::Reg<hdclr::HDCLR_SPEC>;
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "HDSET (w) register accessor: Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdset`]
module"]
pub type HDSET = crate::Reg<hdset::HDSET_SPEC>;
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "HDCR (rw) register accessor: Hibernate Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcr`]
module"]
pub type HDCR = crate::Reg<hdcr::HDCR_SPEC>;
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "OSCSICTRL (rw) register accessor: fOSI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsictrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscsictrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscsictrl`]
module"]
pub type OSCSICTRL = crate::Reg<oscsictrl::OSCSICTRL_SPEC>;
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSCULSTAT (r) register accessor: OSC_ULP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculstat`]
module"]
pub type OSCULSTAT = crate::Reg<osculstat::OSCULSTAT_SPEC>;
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSCULCTRL (rw) register accessor: OSC_ULP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osculctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculctrl`]
module"]
pub type OSCULCTRL = crate::Reg<osculctrl::OSCULCTRL_SPEC>;
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
#[doc = "LPACCONF (rw) register accessor: Analog Wake-up Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacconf`]
module"]
pub type LPACCONF = crate::Reg<lpacconf::LPACCONF_SPEC>;
#[doc = "Analog Wake-up Configuration Register"]
pub mod lpacconf;
#[doc = "LPACTH0 (rw) register accessor: LPAC Threshold Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacth0`]
module"]
pub type LPACTH0 = crate::Reg<lpacth0::LPACTH0_SPEC>;
#[doc = "LPAC Threshold Register 0"]
pub mod lpacth0;
#[doc = "LPACTH1 (rw) register accessor: LPAC Threshold Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacth1`]
module"]
pub type LPACTH1 = crate::Reg<lpacth1::LPACTH1_SPEC>;
#[doc = "LPAC Threshold Register 1"]
pub mod lpacth1;
#[doc = "LPACST (r) register accessor: Hibernate Analog Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacst`]
module"]
pub type LPACST = crate::Reg<lpacst::LPACST_SPEC>;
#[doc = "Hibernate Analog Control State Register"]
pub mod lpacst;
#[doc = "LPACCLR (w) register accessor: LPAC Control Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacclr`]
module"]
pub type LPACCLR = crate::Reg<lpacclr::LPACCLR_SPEC>;
#[doc = "LPAC Control Clear Register"]
pub mod lpacclr;
#[doc = "LPACSET (w) register accessor: LPAC Control Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacset`]
module"]
pub type LPACSET = crate::Reg<lpacset::LPACSET_SPEC>;
#[doc = "LPAC Control Set Register"]
pub mod lpacset;
#[doc = "HINTST (r) register accessor: Hibernate Internal Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hintst`]
module"]
pub type HINTST = crate::Reg<hintst::HINTST_SPEC>;
#[doc = "Hibernate Internal Control State Register"]
pub mod hintst;
#[doc = "HINTCLR (w) register accessor: Hibernate Internal Control Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hintclr`]
module"]
pub type HINTCLR = crate::Reg<hintclr::HINTCLR_SPEC>;
#[doc = "Hibernate Internal Control Clear Register"]
pub mod hintclr;
#[doc = "HINTSET (w) register accessor: Hibernate Internal Control Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hintset`]
module"]
pub type HINTSET = crate::Reg<hintset::HINTSET_SPEC>;
#[doc = "Hibernate Internal Control Set Register"]
pub mod hintset;
