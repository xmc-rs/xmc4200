#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hdstat: Hdstat,
    hdclr: Hdclr,
    hdset: Hdset,
    hdcr: Hdcr,
    _reserved4: [u8; 0x04],
    oscsictrl: Oscsictrl,
    osculstat: Osculstat,
    osculctrl: Osculctrl,
    lpacconf: Lpacconf,
    lpacth0: Lpacth0,
    lpacth1: Lpacth1,
    lpacst: Lpacst,
    lpacclr: Lpacclr,
    lpacset: Lpacset,
    hintst: Hintst,
    hintclr: Hintclr,
    hintset: Hintset,
}
impl RegisterBlock {
    #[doc = "0x00 - Hibernate Domain Status Register"]
    #[inline(always)]
    pub const fn hdstat(&self) -> &Hdstat {
        &self.hdstat
    }
    #[doc = "0x04 - Hibernate Domain Status Clear Register"]
    #[inline(always)]
    pub const fn hdclr(&self) -> &Hdclr {
        &self.hdclr
    }
    #[doc = "0x08 - Hibernate Domain Status Set Register"]
    #[inline(always)]
    pub const fn hdset(&self) -> &Hdset {
        &self.hdset
    }
    #[doc = "0x0c - Hibernate Domain Control Register"]
    #[inline(always)]
    pub const fn hdcr(&self) -> &Hdcr {
        &self.hdcr
    }
    #[doc = "0x14 - fOSI Control Register"]
    #[inline(always)]
    pub const fn oscsictrl(&self) -> &Oscsictrl {
        &self.oscsictrl
    }
    #[doc = "0x18 - OSC_ULP Status Register"]
    #[inline(always)]
    pub const fn osculstat(&self) -> &Osculstat {
        &self.osculstat
    }
    #[doc = "0x1c - OSC_ULP Control Register"]
    #[inline(always)]
    pub const fn osculctrl(&self) -> &Osculctrl {
        &self.osculctrl
    }
    #[doc = "0x20 - Analog Wake-up Configuration Register"]
    #[inline(always)]
    pub const fn lpacconf(&self) -> &Lpacconf {
        &self.lpacconf
    }
    #[doc = "0x24 - LPAC Threshold Register 0"]
    #[inline(always)]
    pub const fn lpacth0(&self) -> &Lpacth0 {
        &self.lpacth0
    }
    #[doc = "0x28 - LPAC Threshold Register 1"]
    #[inline(always)]
    pub const fn lpacth1(&self) -> &Lpacth1 {
        &self.lpacth1
    }
    #[doc = "0x2c - Hibernate Analog Control State Register"]
    #[inline(always)]
    pub const fn lpacst(&self) -> &Lpacst {
        &self.lpacst
    }
    #[doc = "0x30 - LPAC Control Clear Register"]
    #[inline(always)]
    pub const fn lpacclr(&self) -> &Lpacclr {
        &self.lpacclr
    }
    #[doc = "0x34 - LPAC Control Set Register"]
    #[inline(always)]
    pub const fn lpacset(&self) -> &Lpacset {
        &self.lpacset
    }
    #[doc = "0x38 - Hibernate Internal Control State Register"]
    #[inline(always)]
    pub const fn hintst(&self) -> &Hintst {
        &self.hintst
    }
    #[doc = "0x3c - Hibernate Internal Control Clear Register"]
    #[inline(always)]
    pub const fn hintclr(&self) -> &Hintclr {
        &self.hintclr
    }
    #[doc = "0x40 - Hibernate Internal Control Set Register"]
    #[inline(always)]
    pub const fn hintset(&self) -> &Hintset {
        &self.hintset
    }
}
#[doc = "HDSTAT (r) register accessor: Hibernate Domain Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdstat`]
module"]
#[doc(alias = "HDSTAT")]
pub type Hdstat = crate::Reg<hdstat::HdstatSpec>;
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "HDCLR (w) register accessor: Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdclr`]
module"]
#[doc(alias = "HDCLR")]
pub type Hdclr = crate::Reg<hdclr::HdclrSpec>;
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "HDSET (w) register accessor: Hibernate Domain Status Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdset`]
module"]
#[doc(alias = "HDSET")]
pub type Hdset = crate::Reg<hdset::HdsetSpec>;
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "HDCR (rw) register accessor: Hibernate Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdcr`]
module"]
#[doc(alias = "HDCR")]
pub type Hdcr = crate::Reg<hdcr::HdcrSpec>;
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "OSCSICTRL (rw) register accessor: fOSI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscsictrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscsictrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscsictrl`]
module"]
#[doc(alias = "OSCSICTRL")]
pub type Oscsictrl = crate::Reg<oscsictrl::OscsictrlSpec>;
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSCULSTAT (r) register accessor: OSC_ULP Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculstat`]
module"]
#[doc(alias = "OSCULSTAT")]
pub type Osculstat = crate::Reg<osculstat::OsculstatSpec>;
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSCULCTRL (rw) register accessor: OSC_ULP Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osculctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osculctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osculctrl`]
module"]
#[doc(alias = "OSCULCTRL")]
pub type Osculctrl = crate::Reg<osculctrl::OsculctrlSpec>;
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
#[doc = "LPACCONF (rw) register accessor: Analog Wake-up Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacconf`]
module"]
#[doc(alias = "LPACCONF")]
pub type Lpacconf = crate::Reg<lpacconf::LpacconfSpec>;
#[doc = "Analog Wake-up Configuration Register"]
pub mod lpacconf;
#[doc = "LPACTH0 (rw) register accessor: LPAC Threshold Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacth0`]
module"]
#[doc(alias = "LPACTH0")]
pub type Lpacth0 = crate::Reg<lpacth0::Lpacth0Spec>;
#[doc = "LPAC Threshold Register 0"]
pub mod lpacth0;
#[doc = "LPACTH1 (rw) register accessor: LPAC Threshold Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacth1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacth1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacth1`]
module"]
#[doc(alias = "LPACTH1")]
pub type Lpacth1 = crate::Reg<lpacth1::Lpacth1Spec>;
#[doc = "LPAC Threshold Register 1"]
pub mod lpacth1;
#[doc = "LPACST (r) register accessor: Hibernate Analog Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacst`]
module"]
#[doc(alias = "LPACST")]
pub type Lpacst = crate::Reg<lpacst::LpacstSpec>;
#[doc = "Hibernate Analog Control State Register"]
pub mod lpacst;
#[doc = "LPACCLR (w) register accessor: LPAC Control Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacclr`]
module"]
#[doc(alias = "LPACCLR")]
pub type Lpacclr = crate::Reg<lpacclr::LpacclrSpec>;
#[doc = "LPAC Control Clear Register"]
pub mod lpacclr;
#[doc = "LPACSET (w) register accessor: LPAC Control Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpacset`]
module"]
#[doc(alias = "LPACSET")]
pub type Lpacset = crate::Reg<lpacset::LpacsetSpec>;
#[doc = "LPAC Control Set Register"]
pub mod lpacset;
#[doc = "HINTST (r) register accessor: Hibernate Internal Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hintst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hintst`]
module"]
#[doc(alias = "HINTST")]
pub type Hintst = crate::Reg<hintst::HintstSpec>;
#[doc = "Hibernate Internal Control State Register"]
pub mod hintst;
#[doc = "HINTCLR (w) register accessor: Hibernate Internal Control Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hintclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hintclr`]
module"]
#[doc(alias = "HINTCLR")]
pub type Hintclr = crate::Reg<hintclr::HintclrSpec>;
#[doc = "Hibernate Internal Control Clear Register"]
pub mod hintclr;
#[doc = "HINTSET (w) register accessor: Hibernate Internal Control Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hintset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hintset`]
module"]
#[doc(alias = "HINTSET")]
pub type Hintset = crate::Reg<hintset::HintsetSpec>;
#[doc = "Hibernate Internal Control Set Register"]
pub mod hintset;
