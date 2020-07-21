#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Bias and suspend configuration"]
    pub hrbsc: HRBSC,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Module identification register"]
    pub midr: MIDR,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Global Analog Configuration"]
    pub glbana: GLBANA,
    _reserved3: [u8; 8usize],
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
    _reserved12: [u8; 28usize],
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
#[doc = "Bias and suspend configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrbsc](hrbsc) module"]
pub type HRBSC = crate::Reg<u32, _HRBSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRBSC;
#[doc = "`read()` method returns [hrbsc::R](hrbsc::R) reader structure"]
impl crate::Readable for HRBSC {}
#[doc = "`write(|w| ..)` method takes [hrbsc::W](hrbsc::W) writer structure"]
impl crate::Writable for HRBSC {}
#[doc = "Bias and suspend configuration"]
pub mod hrbsc;
#[doc = "Module identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midr](midr) module"]
pub type MIDR = crate::Reg<u32, _MIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIDR;
#[doc = "`read()` method returns [midr::R](midr::R) reader structure"]
impl crate::Readable for MIDR {}
#[doc = "Module identification register"]
pub mod midr;
#[doc = "Global Analog Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glbana](glbana) module"]
pub type GLBANA = crate::Reg<u32, _GLBANA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GLBANA;
#[doc = "`read()` method returns [glbana::R](glbana::R) reader structure"]
impl crate::Readable for GLBANA {}
#[doc = "`write(|w| ..)` method takes [glbana::W](glbana::W) writer structure"]
impl crate::Writable for GLBANA {}
#[doc = "Global Analog Configuration"]
pub mod glbana;
#[doc = "Global CSG configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcfg](csgcfg) module"]
pub type CSGCFG = crate::Reg<u32, _CSGCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCFG;
#[doc = "`read()` method returns [csgcfg::R](csgcfg::R) reader structure"]
impl crate::Readable for CSGCFG {}
#[doc = "`write(|w| ..)` method takes [csgcfg::W](csgcfg::W) writer structure"]
impl crate::Writable for CSGCFG {}
#[doc = "Global CSG configuration"]
pub mod csgcfg;
#[doc = "Global CSG run bit set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgsetg](csgsetg) module"]
pub type CSGSETG = crate::Reg<u32, _CSGSETG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGSETG;
#[doc = "`write(|w| ..)` method takes [csgsetg::W](csgsetg::W) writer structure"]
impl crate::Writable for CSGSETG {}
#[doc = "Global CSG run bit set"]
pub mod csgsetg;
#[doc = "Global CSG run bit clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgclrg](csgclrg) module"]
pub type CSGCLRG = crate::Reg<u32, _CSGCLRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGCLRG;
#[doc = "`write(|w| ..)` method takes [csgclrg::W](csgclrg::W) writer structure"]
impl crate::Writable for CSGCLRG {}
#[doc = "Global CSG run bit clear"]
pub mod csgclrg;
#[doc = "Global CSG run bit status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgstatg](csgstatg) module"]
pub type CSGSTATG = crate::Reg<u32, _CSGSTATG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGSTATG;
#[doc = "`read()` method returns [csgstatg::R](csgstatg::R) reader structure"]
impl crate::Readable for CSGSTATG {}
#[doc = "Global CSG run bit status"]
pub mod csgstatg;
#[doc = "Global CSG slope/prescaler control\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgfcg](csgfcg) module"]
pub type CSGFCG = crate::Reg<u32, _CSGFCG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGFCG;
#[doc = "`write(|w| ..)` method takes [csgfcg::W](csgfcg::W) writer structure"]
impl crate::Writable for CSGFCG {}
#[doc = "Global CSG slope/prescaler control"]
pub mod csgfcg;
#[doc = "Global CSG slope/prescaler status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgfsg](csgfsg) module"]
pub type CSGFSG = crate::Reg<u32, _CSGFSG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGFSG;
#[doc = "`read()` method returns [csgfsg::R](csgfsg::R) reader structure"]
impl crate::Readable for CSGFSG {}
#[doc = "Global CSG slope/prescaler status"]
pub mod csgfsg;
#[doc = "Global CSG shadow/switch trigger\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgtrg](csgtrg) module"]
pub type CSGTRG = crate::Reg<u32, _CSGTRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGTRG;
#[doc = "`write(|w| ..)` method takes [csgtrg::W](csgtrg::W) writer structure"]
impl crate::Writable for CSGTRG {}
#[doc = "Global CSG shadow/switch trigger"]
pub mod csgtrg;
#[doc = "Global CSG shadow trigger clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgtrc](csgtrc) module"]
pub type CSGTRC = crate::Reg<u32, _CSGTRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGTRC;
#[doc = "`write(|w| ..)` method takes [csgtrc::W](csgtrc::W) writer structure"]
impl crate::Writable for CSGTRC {}
#[doc = "Global CSG shadow trigger clear"]
pub mod csgtrc;
#[doc = "Global CSG shadow/switch status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgtrsg](csgtrsg) module"]
pub type CSGTRSG = crate::Reg<u32, _CSGTRSG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSGTRSG;
#[doc = "`read()` method returns [csgtrsg::R](csgtrsg::R) reader structure"]
impl crate::Readable for CSGTRSG {}
#[doc = "Global CSG shadow/switch status"]
pub mod csgtrsg;
#[doc = "Global HRC configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrccfg](hrccfg) module"]
pub type HRCCFG = crate::Reg<u32, _HRCCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRCCFG;
#[doc = "`read()` method returns [hrccfg::R](hrccfg::R) reader structure"]
impl crate::Readable for HRCCFG {}
#[doc = "`write(|w| ..)` method takes [hrccfg::W](hrccfg::W) writer structure"]
impl crate::Writable for HRCCFG {}
#[doc = "Global HRC configuration"]
pub mod hrccfg;
#[doc = "Global HRC shadow trigger set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrcstrg](hrcstrg) module"]
pub type HRCSTRG = crate::Reg<u32, _HRCSTRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRCSTRG;
#[doc = "`write(|w| ..)` method takes [hrcstrg::W](hrcstrg::W) writer structure"]
impl crate::Writable for HRCSTRG {}
#[doc = "Global HRC shadow trigger set"]
pub mod hrcstrg;
#[doc = "Global HRC shadow trigger clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrcctrg](hrcctrg) module"]
pub type HRCCTRG = crate::Reg<u32, _HRCCTRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRCCTRG;
#[doc = "`write(|w| ..)` method takes [hrcctrg::W](hrcctrg::W) writer structure"]
impl crate::Writable for HRCCTRG {}
#[doc = "Global HRC shadow trigger clear"]
pub mod hrcctrg;
#[doc = "Global HRC shadow transfer status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrcstsg](hrcstsg) module"]
pub type HRCSTSG = crate::Reg<u32, _HRCSTSG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRCSTSG;
#[doc = "`read()` method returns [hrcstsg::R](hrcstsg::R) reader structure"]
impl crate::Readable for HRCSTSG {}
#[doc = "Global HRC shadow transfer status"]
pub mod hrcstsg;
#[doc = "High Resolution Generation Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrghrs](hrghrs) module"]
pub type HRGHRS = crate::Reg<u32, _HRGHRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRGHRS;
#[doc = "`read()` method returns [hrghrs::R](hrghrs::R) reader structure"]
impl crate::Readable for HRGHRS {}
#[doc = "High Resolution Generation Status"]
pub mod hrghrs;
