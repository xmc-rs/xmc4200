#[doc = "Register `DSV1` reader"]
pub struct R(crate::R<DSV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DSV1` reader - DAC reference value 1"]
pub type DSV1_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC reference value 1"]
    #[inline(always)]
    pub fn dsv1(&self) -> DSV1_R {
        DSV1_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DAC reference value 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsv1](index.html) module"]
pub struct DSV1_SPEC;
impl crate::RegisterSpec for DSV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsv1::R](R) reader structure"]
impl crate::Readable for DSV1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSV1 to value 0"]
impl crate::Resettable for DSV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
