#[doc = "Register `DCR` reader"]
pub struct R(crate::R<DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DTRV` reader - Dead time rising value"]
pub struct DTRV_R(crate::FieldReader<u16, u16>);
impl DTRV_R {
    pub(crate) fn new(bits: u16) -> Self {
        DTRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTRV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Dead time rising value"]
    #[inline(always)]
    pub fn dtrv(&self) -> DTRV_R {
        DTRV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HRC dead time rising value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](index.html) module"]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr::R](R) reader structure"]
impl crate::Readable for DCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCR to value 0x01"]
impl crate::Resettable for DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
