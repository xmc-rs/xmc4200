#[doc = "Register `PC` reader"]
pub struct R(crate::R<PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSWV` reader - Pulse swallow configuration"]
pub type PSWV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Pulse swallow configuration"]
    #[inline(always)]
    pub fn pswv(&self) -> PSWV_R {
        PSWV_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Pulse swallow configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](index.html) module"]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc::R](R) reader structure"]
impl crate::Readable for PC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
