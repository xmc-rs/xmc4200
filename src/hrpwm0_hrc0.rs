#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HRC mode configuration"]
    pub gc: GC,
    #[doc = "0x04 - HRC output passive level"]
    pub pl: PL,
    #[doc = "0x08 - HRC global control selection"]
    pub gsel: GSEL,
    #[doc = "0x0c - HRC timer selection"]
    pub tsel: TSEL,
    #[doc = "0x10 - HRC current source for shadow"]
    pub sc: SC,
    #[doc = "0x14 - HRC dead time rising value"]
    pub dcr: DCR,
    #[doc = "0x18 - HRC dead time falling value"]
    pub dcf: DCF,
    #[doc = "0x1c - HRC rising edge value"]
    pub cr1: CR1,
    #[doc = "0x20 - HRC falling edge value"]
    pub cr2: CR2,
    #[doc = "0x24 - HRC next source for shadow"]
    pub ssc: SSC,
    #[doc = "0x28 - HRC shadow dead time rising"]
    pub sdcr: SDCR,
    #[doc = "0x2c - HRC shadow dead time falling"]
    pub sdcf: SDCF,
    #[doc = "0x30 - HRC shadow rising edge value"]
    pub scr1: SCR1,
    #[doc = "0x34 - HRC shadow falling edge value"]
    pub scr2: SCR2,
}
#[doc = "HRC mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gc](gc) module"]
pub type GC = crate::Reg<u32, _GC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GC;
#[doc = "`read()` method returns [gc::R](gc::R) reader structure"]
impl crate::Readable for GC {}
#[doc = "`write(|w| ..)` method takes [gc::W](gc::W) writer structure"]
impl crate::Writable for GC {}
#[doc = "HRC mode configuration"]
pub mod gc;
#[doc = "HRC output passive level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pl](pl) module"]
pub type PL = crate::Reg<u32, _PL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL;
#[doc = "`read()` method returns [pl::R](pl::R) reader structure"]
impl crate::Readable for PL {}
#[doc = "`write(|w| ..)` method takes [pl::W](pl::W) writer structure"]
impl crate::Writable for PL {}
#[doc = "HRC output passive level"]
pub mod pl;
#[doc = "HRC global control selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gsel](gsel) module"]
pub type GSEL = crate::Reg<u32, _GSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSEL;
#[doc = "`read()` method returns [gsel::R](gsel::R) reader structure"]
impl crate::Readable for GSEL {}
#[doc = "`write(|w| ..)` method takes [gsel::W](gsel::W) writer structure"]
impl crate::Writable for GSEL {}
#[doc = "HRC global control selection"]
pub mod gsel;
#[doc = "HRC timer selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsel](tsel) module"]
pub type TSEL = crate::Reg<u32, _TSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSEL;
#[doc = "`read()` method returns [tsel::R](tsel::R) reader structure"]
impl crate::Readable for TSEL {}
#[doc = "`write(|w| ..)` method takes [tsel::W](tsel::W) writer structure"]
impl crate::Writable for TSEL {}
#[doc = "HRC timer selection"]
pub mod tsel;
#[doc = "HRC current source for shadow\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "HRC current source for shadow"]
pub mod sc;
#[doc = "HRC dead time rising value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcr](dcr) module"]
pub type DCR = crate::Reg<u32, _DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCR;
#[doc = "`read()` method returns [dcr::R](dcr::R) reader structure"]
impl crate::Readable for DCR {}
#[doc = "HRC dead time rising value"]
pub mod dcr;
#[doc = "HRC dead time falling value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcf](dcf) module"]
pub type DCF = crate::Reg<u32, _DCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCF;
#[doc = "`read()` method returns [dcf::R](dcf::R) reader structure"]
impl crate::Readable for DCF {}
#[doc = "HRC dead time falling value"]
pub mod dcf;
#[doc = "HRC rising edge value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "HRC rising edge value"]
pub mod cr1;
#[doc = "HRC falling edge value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "HRC falling edge value"]
pub mod cr2;
#[doc = "HRC next source for shadow\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssc](ssc) module"]
pub type SSC = crate::Reg<u32, _SSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC;
#[doc = "`read()` method returns [ssc::R](ssc::R) reader structure"]
impl crate::Readable for SSC {}
#[doc = "`write(|w| ..)` method takes [ssc::W](ssc::W) writer structure"]
impl crate::Writable for SSC {}
#[doc = "HRC next source for shadow"]
pub mod ssc;
#[doc = "HRC shadow dead time rising\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdcr](sdcr) module"]
pub type SDCR = crate::Reg<u32, _SDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCR;
#[doc = "`read()` method returns [sdcr::R](sdcr::R) reader structure"]
impl crate::Readable for SDCR {}
#[doc = "`write(|w| ..)` method takes [sdcr::W](sdcr::W) writer structure"]
impl crate::Writable for SDCR {}
#[doc = "HRC shadow dead time rising"]
pub mod sdcr;
#[doc = "HRC shadow dead time falling\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdcf](sdcf) module"]
pub type SDCF = crate::Reg<u32, _SDCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCF;
#[doc = "`read()` method returns [sdcf::R](sdcf::R) reader structure"]
impl crate::Readable for SDCF {}
#[doc = "`write(|w| ..)` method takes [sdcf::W](sdcf::W) writer structure"]
impl crate::Writable for SDCF {}
#[doc = "HRC shadow dead time falling"]
pub mod sdcf;
#[doc = "HRC shadow rising edge value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scr1](scr1) module"]
pub type SCR1 = crate::Reg<u32, _SCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR1;
#[doc = "`read()` method returns [scr1::R](scr1::R) reader structure"]
impl crate::Readable for SCR1 {}
#[doc = "`write(|w| ..)` method takes [scr1::W](scr1::W) writer structure"]
impl crate::Writable for SCR1 {}
#[doc = "HRC shadow rising edge value"]
pub mod scr1;
#[doc = "HRC shadow falling edge value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scr2](scr2) module"]
pub type SCR2 = crate::Reg<u32, _SCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR2;
#[doc = "`read()` method returns [scr2::R](scr2::R) reader structure"]
impl crate::Readable for SCR2 {}
#[doc = "`write(|w| ..)` method takes [scr2::W](scr2::W) writer structure"]
impl crate::Writable for SCR2 {}
#[doc = "HRC shadow falling edge value"]
pub mod scr2;
