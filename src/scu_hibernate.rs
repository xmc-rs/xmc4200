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
    _reserved4: [u8; 4usize],
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
#[doc = "Hibernate Domain Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdstat](hdstat) module"]
pub type HDSTAT = crate::Reg<u32, _HDSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDSTAT;
#[doc = "`read()` method returns [hdstat::R](hdstat::R) reader structure"]
impl crate::Readable for HDSTAT {}
#[doc = "Hibernate Domain Status Register"]
pub mod hdstat;
#[doc = "Hibernate Domain Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdclr](hdclr) module"]
pub type HDCLR = crate::Reg<u32, _HDCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDCLR;
#[doc = "`write(|w| ..)` method takes [hdclr::W](hdclr::W) writer structure"]
impl crate::Writable for HDCLR {}
#[doc = "Hibernate Domain Status Clear Register"]
pub mod hdclr;
#[doc = "Hibernate Domain Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdset](hdset) module"]
pub type HDSET = crate::Reg<u32, _HDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDSET;
#[doc = "`write(|w| ..)` method takes [hdset::W](hdset::W) writer structure"]
impl crate::Writable for HDSET {}
#[doc = "Hibernate Domain Status Set Register"]
pub mod hdset;
#[doc = "Hibernate Domain Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdcr](hdcr) module"]
pub type HDCR = crate::Reg<u32, _HDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDCR;
#[doc = "`read()` method returns [hdcr::R](hdcr::R) reader structure"]
impl crate::Readable for HDCR {}
#[doc = "`write(|w| ..)` method takes [hdcr::W](hdcr::W) writer structure"]
impl crate::Writable for HDCR {}
#[doc = "Hibernate Domain Control Register"]
pub mod hdcr;
#[doc = "fOSI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscsictrl](oscsictrl) module"]
pub type OSCSICTRL = crate::Reg<u32, _OSCSICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCSICTRL;
#[doc = "`read()` method returns [oscsictrl::R](oscsictrl::R) reader structure"]
impl crate::Readable for OSCSICTRL {}
#[doc = "`write(|w| ..)` method takes [oscsictrl::W](oscsictrl::W) writer structure"]
impl crate::Writable for OSCSICTRL {}
#[doc = "fOSI Control Register"]
pub mod oscsictrl;
#[doc = "OSC_ULP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculstat](osculstat) module"]
pub type OSCULSTAT = crate::Reg<u32, _OSCULSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCULSTAT;
#[doc = "`read()` method returns [osculstat::R](osculstat::R) reader structure"]
impl crate::Readable for OSCULSTAT {}
#[doc = "OSC_ULP Status Register"]
pub mod osculstat;
#[doc = "OSC_ULP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osculctrl](osculctrl) module"]
pub type OSCULCTRL = crate::Reg<u32, _OSCULCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCULCTRL;
#[doc = "`read()` method returns [osculctrl::R](osculctrl::R) reader structure"]
impl crate::Readable for OSCULCTRL {}
#[doc = "`write(|w| ..)` method takes [osculctrl::W](osculctrl::W) writer structure"]
impl crate::Writable for OSCULCTRL {}
#[doc = "OSC_ULP Control Register"]
pub mod osculctrl;
#[doc = "Analog Wake-up Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacconf](lpacconf) module"]
pub type LPACCONF = crate::Reg<u32, _LPACCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPACCONF;
#[doc = "`read()` method returns [lpacconf::R](lpacconf::R) reader structure"]
impl crate::Readable for LPACCONF {}
#[doc = "`write(|w| ..)` method takes [lpacconf::W](lpacconf::W) writer structure"]
impl crate::Writable for LPACCONF {}
#[doc = "Analog Wake-up Configuration Register"]
pub mod lpacconf;
#[doc = "LPAC Threshold Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacth0](lpacth0) module"]
pub type LPACTH0 = crate::Reg<u32, _LPACTH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPACTH0;
#[doc = "`read()` method returns [lpacth0::R](lpacth0::R) reader structure"]
impl crate::Readable for LPACTH0 {}
#[doc = "`write(|w| ..)` method takes [lpacth0::W](lpacth0::W) writer structure"]
impl crate::Writable for LPACTH0 {}
#[doc = "LPAC Threshold Register 0"]
pub mod lpacth0;
#[doc = "LPAC Threshold Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacth1](lpacth1) module"]
pub type LPACTH1 = crate::Reg<u32, _LPACTH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPACTH1;
#[doc = "`read()` method returns [lpacth1::R](lpacth1::R) reader structure"]
impl crate::Readable for LPACTH1 {}
#[doc = "`write(|w| ..)` method takes [lpacth1::W](lpacth1::W) writer structure"]
impl crate::Writable for LPACTH1 {}
#[doc = "LPAC Threshold Register 1"]
pub mod lpacth1;
#[doc = "Hibernate Analog Control State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacst](lpacst) module"]
pub type LPACST = crate::Reg<u32, _LPACST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPACST;
#[doc = "`read()` method returns [lpacst::R](lpacst::R) reader structure"]
impl crate::Readable for LPACST {}
#[doc = "Hibernate Analog Control State Register"]
pub mod lpacst;
#[doc = "LPAC Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacclr](lpacclr) module"]
pub type LPACCLR = crate::Reg<u32, _LPACCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPACCLR;
#[doc = "`write(|w| ..)` method takes [lpacclr::W](lpacclr::W) writer structure"]
impl crate::Writable for LPACCLR {}
#[doc = "LPAC Control Clear Register"]
pub mod lpacclr;
#[doc = "LPAC Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacset](lpacset) module"]
pub type LPACSET = crate::Reg<u32, _LPACSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPACSET;
#[doc = "`write(|w| ..)` method takes [lpacset::W](lpacset::W) writer structure"]
impl crate::Writable for LPACSET {}
#[doc = "LPAC Control Set Register"]
pub mod lpacset;
#[doc = "Hibernate Internal Control State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hintst](hintst) module"]
pub type HINTST = crate::Reg<u32, _HINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINTST;
#[doc = "`read()` method returns [hintst::R](hintst::R) reader structure"]
impl crate::Readable for HINTST {}
#[doc = "Hibernate Internal Control State Register"]
pub mod hintst;
#[doc = "Hibernate Internal Control Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hintclr](hintclr) module"]
pub type HINTCLR = crate::Reg<u32, _HINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINTCLR;
#[doc = "`write(|w| ..)` method takes [hintclr::W](hintclr::W) writer structure"]
impl crate::Writable for HINTCLR {}
#[doc = "Hibernate Internal Control Clear Register"]
pub mod hintclr;
#[doc = "Hibernate Internal Control Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hintset](hintset) module"]
pub type HINTSET = crate::Reg<u32, _HINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HINTSET;
#[doc = "`write(|w| ..)` method takes [hintset::W](hintset::W) writer structure"]
impl crate::Writable for HINTSET {}
#[doc = "Hibernate Internal Control Set Register"]
pub mod hintset;
