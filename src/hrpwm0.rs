#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hrbsc: Hrbsc,
    _reserved1: [u8; 0x04],
    midr: Midr,
    _reserved2: [u8; 0x08],
    glbana: Glbana,
    _reserved3: [u8; 0x08],
    csgcfg: Csgcfg,
    csgsetg: Csgsetg,
    csgclrg: Csgclrg,
    csgstatg: Csgstatg,
    csgfcg: Csgfcg,
    csgfsg: Csgfsg,
    csgtrg: Csgtrg,
    csgtrc: Csgtrc,
    csgtrsg: Csgtrsg,
    _reserved12: [u8; 0x1c],
    hrccfg: Hrccfg,
    hrcstrg: Hrcstrg,
    hrcctrg: Hrcctrg,
    hrcstsg: Hrcstsg,
    hrghrs: Hrghrs,
}
impl RegisterBlock {
    #[doc = "0x00 - Bias and suspend configuration"]
    #[inline(always)]
    pub const fn hrbsc(&self) -> &Hrbsc {
        &self.hrbsc
    }
    #[doc = "0x08 - Module identification register"]
    #[inline(always)]
    pub const fn midr(&self) -> &Midr {
        &self.midr
    }
    #[doc = "0x14 - Global Analog Configuration"]
    #[inline(always)]
    pub const fn glbana(&self) -> &Glbana {
        &self.glbana
    }
    #[doc = "0x20 - Global CSG configuration"]
    #[inline(always)]
    pub const fn csgcfg(&self) -> &Csgcfg {
        &self.csgcfg
    }
    #[doc = "0x24 - Global CSG run bit set"]
    #[inline(always)]
    pub const fn csgsetg(&self) -> &Csgsetg {
        &self.csgsetg
    }
    #[doc = "0x28 - Global CSG run bit clear"]
    #[inline(always)]
    pub const fn csgclrg(&self) -> &Csgclrg {
        &self.csgclrg
    }
    #[doc = "0x2c - Global CSG run bit status"]
    #[inline(always)]
    pub const fn csgstatg(&self) -> &Csgstatg {
        &self.csgstatg
    }
    #[doc = "0x30 - Global CSG slope/prescaler control"]
    #[inline(always)]
    pub const fn csgfcg(&self) -> &Csgfcg {
        &self.csgfcg
    }
    #[doc = "0x34 - Global CSG slope/prescaler status"]
    #[inline(always)]
    pub const fn csgfsg(&self) -> &Csgfsg {
        &self.csgfsg
    }
    #[doc = "0x38 - Global CSG shadow/switch trigger"]
    #[inline(always)]
    pub const fn csgtrg(&self) -> &Csgtrg {
        &self.csgtrg
    }
    #[doc = "0x3c - Global CSG shadow trigger clear"]
    #[inline(always)]
    pub const fn csgtrc(&self) -> &Csgtrc {
        &self.csgtrc
    }
    #[doc = "0x40 - Global CSG shadow/switch status"]
    #[inline(always)]
    pub const fn csgtrsg(&self) -> &Csgtrsg {
        &self.csgtrsg
    }
    #[doc = "0x60 - Global HRC configuration"]
    #[inline(always)]
    pub const fn hrccfg(&self) -> &Hrccfg {
        &self.hrccfg
    }
    #[doc = "0x64 - Global HRC shadow trigger set"]
    #[inline(always)]
    pub const fn hrcstrg(&self) -> &Hrcstrg {
        &self.hrcstrg
    }
    #[doc = "0x68 - Global HRC shadow trigger clear"]
    #[inline(always)]
    pub const fn hrcctrg(&self) -> &Hrcctrg {
        &self.hrcctrg
    }
    #[doc = "0x6c - Global HRC shadow transfer status"]
    #[inline(always)]
    pub const fn hrcstsg(&self) -> &Hrcstsg {
        &self.hrcstsg
    }
    #[doc = "0x70 - High Resolution Generation Status"]
    #[inline(always)]
    pub const fn hrghrs(&self) -> &Hrghrs {
        &self.hrghrs
    }
}
#[doc = "HRBSC (rw) register accessor: Bias and suspend configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrbsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrbsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrbsc`]
module"]
#[doc(alias = "HRBSC")]
pub type Hrbsc = crate::Reg<hrbsc::HrbscSpec>;
#[doc = "Bias and suspend configuration"]
pub mod hrbsc;
#[doc = "MIDR (r) register accessor: Module identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`midr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@midr`]
module"]
#[doc(alias = "MIDR")]
pub type Midr = crate::Reg<midr::MidrSpec>;
#[doc = "Module identification register"]
pub mod midr;
#[doc = "GLBANA (rw) register accessor: Global Analog Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glbana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glbana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glbana`]
module"]
#[doc(alias = "GLBANA")]
pub type Glbana = crate::Reg<glbana::GlbanaSpec>;
#[doc = "Global Analog Configuration"]
pub mod glbana;
#[doc = "CSGCFG (rw) register accessor: Global CSG configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgcfg`]
module"]
#[doc(alias = "CSGCFG")]
pub type Csgcfg = crate::Reg<csgcfg::CsgcfgSpec>;
#[doc = "Global CSG configuration"]
pub mod csgcfg;
#[doc = "CSGSETG (w) register accessor: Global CSG run bit set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgsetg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgsetg`]
module"]
#[doc(alias = "CSGSETG")]
pub type Csgsetg = crate::Reg<csgsetg::CsgsetgSpec>;
#[doc = "Global CSG run bit set"]
pub mod csgsetg;
#[doc = "CSGCLRG (w) register accessor: Global CSG run bit clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgclrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgclrg`]
module"]
#[doc(alias = "CSGCLRG")]
pub type Csgclrg = crate::Reg<csgclrg::CsgclrgSpec>;
#[doc = "Global CSG run bit clear"]
pub mod csgclrg;
#[doc = "CSGSTATG (r) register accessor: Global CSG run bit status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgstatg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgstatg`]
module"]
#[doc(alias = "CSGSTATG")]
pub type Csgstatg = crate::Reg<csgstatg::CsgstatgSpec>;
#[doc = "Global CSG run bit status"]
pub mod csgstatg;
#[doc = "CSGFCG (w) register accessor: Global CSG slope/prescaler control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgfcg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgfcg`]
module"]
#[doc(alias = "CSGFCG")]
pub type Csgfcg = crate::Reg<csgfcg::CsgfcgSpec>;
#[doc = "Global CSG slope/prescaler control"]
pub mod csgfcg;
#[doc = "CSGFSG (r) register accessor: Global CSG slope/prescaler status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgfsg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgfsg`]
module"]
#[doc(alias = "CSGFSG")]
pub type Csgfsg = crate::Reg<csgfsg::CsgfsgSpec>;
#[doc = "Global CSG slope/prescaler status"]
pub mod csgfsg;
#[doc = "CSGTRG (w) register accessor: Global CSG shadow/switch trigger\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgtrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgtrg`]
module"]
#[doc(alias = "CSGTRG")]
pub type Csgtrg = crate::Reg<csgtrg::CsgtrgSpec>;
#[doc = "Global CSG shadow/switch trigger"]
pub mod csgtrg;
#[doc = "CSGTRC (w) register accessor: Global CSG shadow trigger clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgtrc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgtrc`]
module"]
#[doc(alias = "CSGTRC")]
pub type Csgtrc = crate::Reg<csgtrc::CsgtrcSpec>;
#[doc = "Global CSG shadow trigger clear"]
pub mod csgtrc;
#[doc = "CSGTRSG (r) register accessor: Global CSG shadow/switch status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgtrsg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csgtrsg`]
module"]
#[doc(alias = "CSGTRSG")]
pub type Csgtrsg = crate::Reg<csgtrsg::CsgtrsgSpec>;
#[doc = "Global CSG shadow/switch status"]
pub mod csgtrsg;
#[doc = "HRCCFG (rw) register accessor: Global HRC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrccfg`]
module"]
#[doc(alias = "HRCCFG")]
pub type Hrccfg = crate::Reg<hrccfg::HrccfgSpec>;
#[doc = "Global HRC configuration"]
pub mod hrccfg;
#[doc = "HRCSTRG (w) register accessor: Global HRC shadow trigger set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrcstrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrcstrg`]
module"]
#[doc(alias = "HRCSTRG")]
pub type Hrcstrg = crate::Reg<hrcstrg::HrcstrgSpec>;
#[doc = "Global HRC shadow trigger set"]
pub mod hrcstrg;
#[doc = "HRCCTRG (w) register accessor: Global HRC shadow trigger clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrcctrg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrcctrg`]
module"]
#[doc(alias = "HRCCTRG")]
pub type Hrcctrg = crate::Reg<hrcctrg::HrcctrgSpec>;
#[doc = "Global HRC shadow trigger clear"]
pub mod hrcctrg;
#[doc = "HRCSTSG (r) register accessor: Global HRC shadow transfer status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrcstsg::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrcstsg`]
module"]
#[doc(alias = "HRCSTSG")]
pub type Hrcstsg = crate::Reg<hrcstsg::HrcstsgSpec>;
#[doc = "Global HRC shadow transfer status"]
pub mod hrcstsg;
#[doc = "HRGHRS (r) register accessor: High Resolution Generation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrghrs::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrghrs`]
module"]
#[doc(alias = "HRGHRS")]
pub type Hrghrs = crate::Reg<hrghrs::HrghrsSpec>;
#[doc = "High Resolution Generation Status"]
pub mod hrghrs;
