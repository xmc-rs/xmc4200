#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - External input selection"]
    pub dci: DCI,
    #[doc = "0x04 - External input selection"]
    pub ies: IES,
    #[doc = "0x08 - Slope generation control"]
    pub sc: SC,
    #[doc = "0x0c - Pulse swallow configuration"]
    pub pc: PC,
    #[doc = "0x10 - DAC reference value 1"]
    pub dsv1: DSV1,
    #[doc = "0x14 - DAC reference value 1"]
    pub dsv2: DSV2,
    #[doc = "0x18 - Shadow reference value 1"]
    pub sdsv1: SDSV1,
    #[doc = "0x1c - Shadow Pulse swallow value"]
    pub spc: SPC,
    #[doc = "0x20 - Comparator configuration"]
    pub cc: CC,
    #[doc = "0x24 - Passive level configuration"]
    pub plc: PLC,
    #[doc = "0x28 - Comparator blanking value"]
    pub blv: BLV,
    #[doc = "0x2c - Service request enable"]
    pub sre: SRE,
    #[doc = "0x30 - Service request line selector"]
    pub srs: SRS,
    #[doc = "0x34 - Service request SW set"]
    pub sws: SWS,
    #[doc = "0x38 - Service request SW clear"]
    pub swc: SWC,
    #[doc = "0x3c - Service request status"]
    pub istat: ISTAT,
}
#[doc = "External input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dci](dci) module"]
pub type DCI = crate::Reg<u32, _DCI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCI;
#[doc = "`read()` method returns [dci::R](dci::R) reader structure"]
impl crate::Readable for DCI {}
#[doc = "`write(|w| ..)` method takes [dci::W](dci::W) writer structure"]
impl crate::Writable for DCI {}
#[doc = "External input selection"]
pub mod dci;
#[doc = "External input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ies](ies) module"]
pub type IES = crate::Reg<u32, _IES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IES;
#[doc = "`read()` method returns [ies::R](ies::R) reader structure"]
impl crate::Readable for IES {}
#[doc = "`write(|w| ..)` method takes [ies::W](ies::W) writer structure"]
impl crate::Writable for IES {}
#[doc = "External input selection"]
pub mod ies;
#[doc = "Slope generation control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](sc) module"]
pub type SC = crate::Reg<u32, _SC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC;
#[doc = "`read()` method returns [sc::R](sc::R) reader structure"]
impl crate::Readable for SC {}
#[doc = "`write(|w| ..)` method takes [sc::W](sc::W) writer structure"]
impl crate::Writable for SC {}
#[doc = "Slope generation control"]
pub mod sc;
#[doc = "Pulse swallow configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "Pulse swallow configuration"]
pub mod pc;
#[doc = "DAC reference value 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsv1](dsv1) module"]
pub type DSV1 = crate::Reg<u32, _DSV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSV1;
#[doc = "`read()` method returns [dsv1::R](dsv1::R) reader structure"]
impl crate::Readable for DSV1 {}
#[doc = "DAC reference value 1"]
pub mod dsv1;
#[doc = "DAC reference value 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsv2](dsv2) module"]
pub type DSV2 = crate::Reg<u32, _DSV2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSV2;
#[doc = "`read()` method returns [dsv2::R](dsv2::R) reader structure"]
impl crate::Readable for DSV2 {}
#[doc = "`write(|w| ..)` method takes [dsv2::W](dsv2::W) writer structure"]
impl crate::Writable for DSV2 {}
#[doc = "DAC reference value 1"]
pub mod dsv2;
#[doc = "Shadow reference value 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdsv1](sdsv1) module"]
pub type SDSV1 = crate::Reg<u32, _SDSV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDSV1;
#[doc = "`read()` method returns [sdsv1::R](sdsv1::R) reader structure"]
impl crate::Readable for SDSV1 {}
#[doc = "`write(|w| ..)` method takes [sdsv1::W](sdsv1::W) writer structure"]
impl crate::Writable for SDSV1 {}
#[doc = "Shadow reference value 1"]
pub mod sdsv1;
#[doc = "Shadow Pulse swallow value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spc](spc) module"]
pub type SPC = crate::Reg<u32, _SPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPC;
#[doc = "`read()` method returns [spc::R](spc::R) reader structure"]
impl crate::Readable for SPC {}
#[doc = "`write(|w| ..)` method takes [spc::W](spc::W) writer structure"]
impl crate::Writable for SPC {}
#[doc = "Shadow Pulse swallow value"]
pub mod spc;
#[doc = "Comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Comparator configuration"]
pub mod cc;
#[doc = "Passive level configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plc](plc) module"]
pub type PLC = crate::Reg<u32, _PLC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLC;
#[doc = "`read()` method returns [plc::R](plc::R) reader structure"]
impl crate::Readable for PLC {}
#[doc = "`write(|w| ..)` method takes [plc::W](plc::W) writer structure"]
impl crate::Writable for PLC {}
#[doc = "Passive level configuration"]
pub mod plc;
#[doc = "Comparator blanking value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blv](blv) module"]
pub type BLV = crate::Reg<u32, _BLV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLV;
#[doc = "`read()` method returns [blv::R](blv::R) reader structure"]
impl crate::Readable for BLV {}
#[doc = "`write(|w| ..)` method takes [blv::W](blv::W) writer structure"]
impl crate::Writable for BLV {}
#[doc = "Comparator blanking value"]
pub mod blv;
#[doc = "Service request enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sre](sre) module"]
pub type SRE = crate::Reg<u32, _SRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRE;
#[doc = "`read()` method returns [sre::R](sre::R) reader structure"]
impl crate::Readable for SRE {}
#[doc = "`write(|w| ..)` method takes [sre::W](sre::W) writer structure"]
impl crate::Writable for SRE {}
#[doc = "Service request enable"]
pub mod sre;
#[doc = "Service request line selector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs](srs) module"]
pub type SRS = crate::Reg<u32, _SRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRS;
#[doc = "`read()` method returns [srs::R](srs::R) reader structure"]
impl crate::Readable for SRS {}
#[doc = "`write(|w| ..)` method takes [srs::W](srs::W) writer structure"]
impl crate::Writable for SRS {}
#[doc = "Service request line selector"]
pub mod srs;
#[doc = "Service request SW set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sws](sws) module"]
pub type SWS = crate::Reg<u32, _SWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWS;
#[doc = "`write(|w| ..)` method takes [sws::W](sws::W) writer structure"]
impl crate::Writable for SWS {}
#[doc = "Service request SW set"]
pub mod sws;
#[doc = "Service request SW clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swc](swc) module"]
pub type SWC = crate::Reg<u32, _SWC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWC;
#[doc = "`write(|w| ..)` method takes [swc::W](swc::W) writer structure"]
impl crate::Writable for SWC {}
#[doc = "Service request SW clear"]
pub mod swc;
#[doc = "Service request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istat](istat) module"]
pub type ISTAT = crate::Reg<u32, _ISTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISTAT;
#[doc = "`read()` method returns [istat::R](istat::R) reader structure"]
impl crate::Readable for ISTAT {}
#[doc = "Service request status"]
pub mod istat;
