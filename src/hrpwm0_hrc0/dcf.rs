#[doc = "Register `DCF` reader"]
pub struct R(crate::R<DCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DTFV` reader - Dead time falling value"]
pub struct DTFV_R(crate::FieldReader<u16, u16>);
impl DTFV_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTFV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Dead time falling value"]
    #[inline(always)]
    pub fn dtfv(&self) -> DTFV_R {
        DTFV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HRC dead time falling value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcf](index.html) module"]
pub struct DCF_SPEC;
impl crate::RegisterSpec for DCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcf::R](R) reader structure"]
impl crate::Readable for DCF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCF to value 0x01"]
impl crate::Resettable for DCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
