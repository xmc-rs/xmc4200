#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hrbsc: HRBSC,
    _reserved1: [u8; 0x04],
    midr: MIDR,
    _reserved2: [u8; 0x08],
    glbana: GLBANA,
    _reserved3: [u8; 0x08],
    csgcfg: CSGCFG,
    csgsetg: CSGSETG,
    csgclrg: CSGCLRG,
    csgstatg: CSGSTATG,
    csgfcg: CSGFCG,
    csgfsg: CSGFSG,
    csgtrg: CSGTRG,
    csgtrc: CSGTRC,
    csgtrsg: CSGTRSG,
    _reserved12: [u8; 0x1c],
    hrccfg: HRCCFG,
    hrcstrg: HRCSTRG,
    hrcctrg: HRCCTRG,
    hrcstsg: HRCSTSG,
    hrghrs: HRGHRS,
}
impl RegisterBlock {
    #[doc = "0x00 - Bias and suspend configuration"]
    #[inline(always)]
    pub const fn hrbsc(&self) -> &HRBSC {
        &self.hrbsc
    }
    #[doc = "0x08 - Module identification register"]
    #[inline(always)]
    pub const fn midr(&self) -> &MIDR {
        &self.midr
    }
    #[doc = "0x14 - Global Analog Configuration"]
    #[inline(always)]
    pub const fn glbana(&self) -> &GLBANA {
        &self.glbana
    }
    #[doc = "0x20 - Global CSG configuration"]
    #[inline(always)]
    pub const fn csgcfg(&self) -> &CSGCFG {
        &self.csgcfg
    }
    #[doc = "0x24 - Global CSG run bit set"]
    #[inline(always)]
    pub const fn csgsetg(&self) -> &CSGSETG {
        &self.csgsetg
    }
    #[doc = "0x28 - Global CSG run bit clear"]
    #[inline(always)]
    pub const fn csgclrg(&self) -> &CSGCLRG {
        &self.csgclrg
    }
    #[doc = "0x2c - Global CSG run bit status"]
    #[inline(always)]
    pub const fn csgstatg(&self) -> &CSGSTATG {
        &self.csgstatg
    }
    #[doc = "0x30 - Global CSG slope/prescaler control"]
    #[inline(always)]
    pub const fn csgfcg(&self) -> &CSGFCG {
        &self.csgfcg
    }
    #[doc = "0x34 - Global CSG slope/prescaler status"]
    #[inline(always)]
    pub const fn csgfsg(&self) -> &CSGFSG {
        &self.csgfsg
    }
    #[doc = "0x38 - Global CSG shadow/switch trigger"]
    #[inline(always)]
    pub const fn csgtrg(&self) -> &CSGTRG {
        &self.csgtrg
    }
    #[doc = "0x3c - Global CSG shadow trigger clear"]
    #[inline(always)]
    pub const fn csgtrc(&self) -> &CSGTRC {
        &self.csgtrc
    }
    #[doc = "0x40 - Global CSG shadow/switch status"]
    #[inline(always)]
    pub const fn csgtrsg(&self) -> &CSGTRSG {
        &self.csgtrsg
    }
    #[doc = "0x60 - Global HRC configuration"]
    #[inline(always)]
    pub const fn hrccfg(&self) -> &HRCCFG {
        &self.hrccfg
    }
    #[doc = "0x64 - Global HRC shadow trigger set"]
    #[inline(always)]
    pub const fn hrcstrg(&self) -> &HRCSTRG {
        &self.hrcstrg
    }
    #[doc = "0x68 - Global HRC shadow trigger clear"]
    #[inline(always)]
    pub const fn hrcctrg(&self) -> &HRCCTRG {
        &self.hrcctrg
    }
    #[doc = "0x6c - Global HRC shadow transfer status"]
    #[inline(always)]
    pub const fn hrcstsg(&self) -> &HRCSTSG {
        &self.hrcstsg
    }
    #[doc = "0x70 - High Resolution Generation Status"]
    #[inline(always)]
    pub const fn hrghrs(&self) -> &HRGHRS {
        &self.hrghrs
    }
}
#[doc = "HRBSC (rw) register accessor: Bias and suspend configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrbsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrbsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrbsc`]
module"]
pub type HRBSC = crate::Reg<hrbsc::HRBSC_SPEC>;
#[doc = "Bias and suspend configuration"]
pub mod hrbsc;
#[doc = "MIDR (r) register accessor: Module identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@midr`]
module"]
pub type MIDR = crate::Reg<midr::MIDR_SPEC>;
#[doc = "Module identification register"]
pub mod midr;
#[doc = "GLBANA (rw) register accessor: Global Analog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glbana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glbana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glbana`]
module"]
pub type GLBANA = crate::Reg<glbana::GLBANA_SPEC>;
#[doc = "Global Analog Configuration"]
pub mod glbana;
#[doc = "CSGCFG (rw) register accessor: Global CSG configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcfg`]
module"]
pub type CSGCFG = crate::Reg<csgcfg::CSGCFG_SPEC>;
#[doc = "Global CSG configuration"]
pub mod csgcfg;
#[doc = "CSGSETG (w) register accessor: Global CSG run bit set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgsetg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgsetg`]
module"]
pub type CSGSETG = crate::Reg<csgsetg::CSGSETG_SPEC>;
#[doc = "Global CSG run bit set"]
pub mod csgsetg;
#[doc = "CSGCLRG (w) register accessor: Global CSG run bit clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgclrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgclrg`]
module"]
pub type CSGCLRG = crate::Reg<csgclrg::CSGCLRG_SPEC>;
#[doc = "Global CSG run bit clear"]
pub mod csgclrg;
#[doc = "CSGSTATG (r) register accessor: Global CSG run bit status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgstatg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgstatg`]
module"]
pub type CSGSTATG = crate::Reg<csgstatg::CSGSTATG_SPEC>;
#[doc = "Global CSG run bit status"]
pub mod csgstatg;
#[doc = "CSGFCG (w) register accessor: Global CSG slope/prescaler control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgfcg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgfcg`]
module"]
pub type CSGFCG = crate::Reg<csgfcg::CSGFCG_SPEC>;
#[doc = "Global CSG slope/prescaler control"]
pub mod csgfcg;
#[doc = "CSGFSG (r) register accessor: Global CSG slope/prescaler status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgfsg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgfsg`]
module"]
pub type CSGFSG = crate::Reg<csgfsg::CSGFSG_SPEC>;
#[doc = "Global CSG slope/prescaler status"]
pub mod csgfsg;
#[doc = "CSGTRG (w) register accessor: Global CSG shadow/switch trigger\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgtrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgtrg`]
module"]
pub type CSGTRG = crate::Reg<csgtrg::CSGTRG_SPEC>;
#[doc = "Global CSG shadow/switch trigger"]
pub mod csgtrg;
#[doc = "CSGTRC (w) register accessor: Global CSG shadow trigger clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgtrc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgtrc`]
module"]
pub type CSGTRC = crate::Reg<csgtrc::CSGTRC_SPEC>;
#[doc = "Global CSG shadow trigger clear"]
pub mod csgtrc;
#[doc = "CSGTRSG (r) register accessor: Global CSG shadow/switch status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgtrsg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgtrsg`]
module"]
pub type CSGTRSG = crate::Reg<csgtrsg::CSGTRSG_SPEC>;
#[doc = "Global CSG shadow/switch status"]
pub mod csgtrsg;
#[doc = "HRCCFG (rw) register accessor: Global HRC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrccfg`]
module"]
pub type HRCCFG = crate::Reg<hrccfg::HRCCFG_SPEC>;
#[doc = "Global HRC configuration"]
pub mod hrccfg;
#[doc = "HRCSTRG (w) register accessor: Global HRC shadow trigger set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrcstrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrcstrg`]
module"]
pub type HRCSTRG = crate::Reg<hrcstrg::HRCSTRG_SPEC>;
#[doc = "Global HRC shadow trigger set"]
pub mod hrcstrg;
#[doc = "HRCCTRG (w) register accessor: Global HRC shadow trigger clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrcctrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrcctrg`]
module"]
pub type HRCCTRG = crate::Reg<hrcctrg::HRCCTRG_SPEC>;
#[doc = "Global HRC shadow trigger clear"]
pub mod hrcctrg;
#[doc = "HRCSTSG (r) register accessor: Global HRC shadow transfer status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrcstsg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrcstsg`]
module"]
pub type HRCSTSG = crate::Reg<hrcstsg::HRCSTSG_SPEC>;
#[doc = "Global HRC shadow transfer status"]
pub mod hrcstsg;
#[doc = "HRGHRS (r) register accessor: High Resolution Generation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrghrs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrghrs`]
module"]
pub type HRGHRS = crate::Reg<hrghrs::HRGHRS_SPEC>;
#[doc = "High Resolution Generation Status"]
pub mod hrghrs;
