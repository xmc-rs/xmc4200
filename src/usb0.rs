#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8usize],
    #[doc = "0x08 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x0c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x10 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x14 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0x1c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0x20 - Receive Status Read and Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0x24 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x28 - Non-Periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    _reserved9: [u8; 16usize],
    #[doc = "0x3c - USB Module Identification Register"]
    pub guid: GUID,
    _reserved10: [u8; 28usize],
    #[doc = "0x5c - Global DFIFO Software Config Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved11: [u8; 164usize],
    #[doc = "0x104 - Device IN Endpoint 1 Transmit FIFO Size Register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x108 - Device IN Endpoint 2 Transmit FIFO Size Register"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x10c - Device IN Endpoint 3 Transmit FIFO Size Register"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0x110 - Device IN Endpoint 4 Transmit FIFO Size Register"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0x114 - Device IN Endpoint 5 Transmit FIFO Size Register"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0x118 - Device IN Endpoint 6 Transmit FIFO Size Register"]
    pub dieptxf6: DIEPTXF6,
    _reserved17: [u8; 1764usize],
    #[doc = "0x800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved20: [u8; 4usize],
    #[doc = "0x810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved24: [u8; 8usize],
    #[doc = "0x828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved26: [u8; 4usize],
    #[doc = "0x834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved27: [u8; 1480usize],
    #[doc = "0xe00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "AHB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gahbcfg](gahbcfg) module"]
pub type GAHBCFG = crate::Reg<u32, _GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAHBCFG;
#[doc = "`read()` method returns [gahbcfg::R](gahbcfg::R) reader structure"]
impl crate::Readable for GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](gahbcfg::W) writer structure"]
impl crate::Writable for GAHBCFG {}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gusbcfg](gusbcfg) module"]
pub type GUSBCFG = crate::Reg<u32, _GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUSBCFG;
#[doc = "`read()` method returns [gusbcfg::R](gusbcfg::R) reader structure"]
impl crate::Readable for GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](gusbcfg::W) writer structure"]
impl crate::Writable for GUSBCFG {}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grstctl](grstctl) module"]
pub type GRSTCTL = crate::Reg<u32, _GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTCTL;
#[doc = "`read()` method returns [grstctl::R](grstctl::R) reader structure"]
impl crate::Readable for GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [grstctl::W](grstctl::W) writer structure"]
impl crate::Writable for GRSTCTL {}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintsts](gintsts) module"]
pub type GINTSTS = crate::Reg<u32, _GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTSTS;
#[doc = "`read()` method returns [gintsts::R](gintsts::R) reader structure"]
impl crate::Readable for GINTSTS {}
#[doc = "`write(|w| ..)` method takes [gintsts::W](gintsts::W) writer structure"]
impl crate::Writable for GINTSTS {}
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gintmsk](gintmsk) module"]
pub type GINTMSK = crate::Reg<u32, _GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTMSK;
#[doc = "`read()` method returns [gintmsk::R](gintmsk::R) reader structure"]
impl crate::Readable for GINTMSK {}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](gintmsk::W) writer structure"]
impl crate::Writable for GINTMSK {}
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "Receive Status Debug Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsr](grxstsr) module"]
pub type GRXSTSR = crate::Reg<u32, _GRXSTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR;
#[doc = "`read()` method returns [grxstsr::R](grxstsr::R) reader structure"]
impl crate::Readable for GRXSTSR {}
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "Receive Status Read and Pop Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxstsp](grxstsp) module"]
pub type GRXSTSP = crate::Reg<u32, _GRXSTSP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP;
#[doc = "`read()` method returns [grxstsp::R](grxstsp::R) reader structure"]
impl crate::Readable for GRXSTSP {}
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "Receive FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [grxfsiz](grxfsiz) module"]
pub type GRXFSIZ = crate::Reg<u32, _GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXFSIZ;
#[doc = "`read()` method returns [grxfsiz::R](grxfsiz::R) reader structure"]
impl crate::Readable for GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [grxfsiz::W](grxfsiz::W) writer structure"]
impl crate::Writable for GRXFSIZ {}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-Periodic Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gnptxfsiz](gnptxfsiz) module"]
pub type GNPTXFSIZ = crate::Reg<u32, _GNPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXFSIZ;
#[doc = "`read()` method returns [gnptxfsiz::R](gnptxfsiz::R) reader structure"]
impl crate::Readable for GNPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz::W](gnptxfsiz::W) writer structure"]
impl crate::Writable for GNPTXFSIZ {}
#[doc = "Non-Periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "USB Module Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [guid](guid) module"]
pub type GUID = crate::Reg<u32, _GUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUID;
#[doc = "`read()` method returns [guid::R](guid::R) reader structure"]
impl crate::Readable for GUID {}
#[doc = "`write(|w| ..)` method takes [guid::W](guid::W) writer structure"]
impl crate::Writable for GUID {}
#[doc = "USB Module Identification Register"]
pub mod guid;
#[doc = "Global DFIFO Software Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gdfifocfg](gdfifocfg) module"]
pub type GDFIFOCFG = crate::Reg<u32, _GDFIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GDFIFOCFG;
#[doc = "`read()` method returns [gdfifocfg::R](gdfifocfg::R) reader structure"]
impl crate::Readable for GDFIFOCFG {}
#[doc = "`write(|w| ..)` method takes [gdfifocfg::W](gdfifocfg::W) writer structure"]
impl crate::Writable for GDFIFOCFG {}
#[doc = "Global DFIFO Software Config Register"]
pub mod gdfifocfg;
#[doc = "Device IN Endpoint 1 Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf1](dieptxf1) module"]
pub type DIEPTXF1 = crate::Reg<u32, _DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF1;
#[doc = "`read()` method returns [dieptxf1::R](dieptxf1::R) reader structure"]
impl crate::Readable for DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](dieptxf1::W) writer structure"]
impl crate::Writable for DIEPTXF1 {}
#[doc = "Device IN Endpoint 1 Transmit FIFO Size Register"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint 2 Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf2](dieptxf2) module"]
pub type DIEPTXF2 = crate::Reg<u32, _DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF2;
#[doc = "`read()` method returns [dieptxf2::R](dieptxf2::R) reader structure"]
impl crate::Readable for DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [dieptxf2::W](dieptxf2::W) writer structure"]
impl crate::Writable for DIEPTXF2 {}
#[doc = "Device IN Endpoint 2 Transmit FIFO Size Register"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint 3 Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf3](dieptxf3) module"]
pub type DIEPTXF3 = crate::Reg<u32, _DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF3;
#[doc = "`read()` method returns [dieptxf3::R](dieptxf3::R) reader structure"]
impl crate::Readable for DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [dieptxf3::W](dieptxf3::W) writer structure"]
impl crate::Writable for DIEPTXF3 {}
#[doc = "Device IN Endpoint 3 Transmit FIFO Size Register"]
pub mod dieptxf3;
#[doc = "Device IN Endpoint 4 Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf4](dieptxf4) module"]
pub type DIEPTXF4 = crate::Reg<u32, _DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF4;
#[doc = "`read()` method returns [dieptxf4::R](dieptxf4::R) reader structure"]
impl crate::Readable for DIEPTXF4 {}
#[doc = "`write(|w| ..)` method takes [dieptxf4::W](dieptxf4::W) writer structure"]
impl crate::Writable for DIEPTXF4 {}
#[doc = "Device IN Endpoint 4 Transmit FIFO Size Register"]
pub mod dieptxf4;
#[doc = "Device IN Endpoint 5 Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf5](dieptxf5) module"]
pub type DIEPTXF5 = crate::Reg<u32, _DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF5;
#[doc = "`read()` method returns [dieptxf5::R](dieptxf5::R) reader structure"]
impl crate::Readable for DIEPTXF5 {}
#[doc = "`write(|w| ..)` method takes [dieptxf5::W](dieptxf5::W) writer structure"]
impl crate::Writable for DIEPTXF5 {}
#[doc = "Device IN Endpoint 5 Transmit FIFO Size Register"]
pub mod dieptxf5;
#[doc = "Device IN Endpoint 6 Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dieptxf6](dieptxf6) module"]
pub type DIEPTXF6 = crate::Reg<u32, _DIEPTXF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF6;
#[doc = "`read()` method returns [dieptxf6::R](dieptxf6::R) reader structure"]
impl crate::Readable for DIEPTXF6 {}
#[doc = "`write(|w| ..)` method takes [dieptxf6::W](dieptxf6::W) writer structure"]
impl crate::Writable for DIEPTXF6 {}
#[doc = "Device IN Endpoint 6 Transmit FIFO Size Register"]
pub mod dieptxf6;
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dctl](dctl) module"]
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
#[doc = "`read()` method returns [dctl::R](dctl::R) reader structure"]
impl crate::Readable for DCTL {}
#[doc = "`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure"]
impl crate::Writable for DCTL {}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dsts](dsts) module"]
pub type DSTS = crate::Reg<u32, _DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTS;
#[doc = "`read()` method returns [dsts::R](dsts::R) reader structure"]
impl crate::Readable for DSTS {}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepmsk](diepmsk) module"]
pub type DIEPMSK = crate::Reg<u32, _DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPMSK;
#[doc = "`read()` method returns [diepmsk::R](diepmsk::R) reader structure"]
impl crate::Readable for DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](diepmsk::W) writer structure"]
impl crate::Writable for DIEPMSK {}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [doepmsk](doepmsk) module"]
pub type DOEPMSK = crate::Reg<u32, _DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPMSK;
#[doc = "`read()` method returns [doepmsk::R](doepmsk::R) reader structure"]
impl crate::Readable for DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](doepmsk::W) writer structure"]
impl crate::Writable for DOEPMSK {}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daint](daint) module"]
pub type DAINT = crate::Reg<u32, _DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINT;
#[doc = "`read()` method returns [daint::R](daint::R) reader structure"]
impl crate::Readable for DAINT {}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daintmsk](daintmsk) module"]
pub type DAINTMSK = crate::Reg<u32, _DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINTMSK;
#[doc = "`read()` method returns [daintmsk::R](daintmsk::R) reader structure"]
impl crate::Readable for DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](daintmsk::W) writer structure"]
impl crate::Writable for DAINTMSK {}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device VBUS Discharge Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dvbusdis](dvbusdis) module"]
pub type DVBUSDIS = crate::Reg<u32, _DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSDIS;
#[doc = "`read()` method returns [dvbusdis::R](dvbusdis::R) reader structure"]
impl crate::Readable for DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [dvbusdis::W](dvbusdis::W) writer structure"]
impl crate::Writable for DVBUSDIS {}
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "Device VBUS Pulsing Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dvbuspulse](dvbuspulse) module"]
pub type DVBUSPULSE = crate::Reg<u32, _DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSPULSE;
#[doc = "`read()` method returns [dvbuspulse::R](dvbuspulse::R) reader structure"]
impl crate::Readable for DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [dvbuspulse::W](dvbuspulse::W) writer structure"]
impl crate::Writable for DVBUSPULSE {}
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [diepempmsk](diepempmsk) module"]
pub type DIEPEMPMSK = crate::Reg<u32, _DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEMPMSK;
#[doc = "`read()` method returns [diepempmsk::R](diepempmsk::R) reader structure"]
impl crate::Readable for DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](diepempmsk::W) writer structure"]
impl crate::Writable for DIEPEMPMSK {}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Power and Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pcgcctl](pcgcctl) module"]
pub type PCGCCTL = crate::Reg<u32, _PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCGCCTL;
#[doc = "`read()` method returns [pcgcctl::R](pcgcctl::R) reader structure"]
impl crate::Readable for PCGCCTL {}
#[doc = "`write(|w| ..)` method takes [pcgcctl::W](pcgcctl::W) writer structure"]
impl crate::Writable for PCGCCTL {}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
