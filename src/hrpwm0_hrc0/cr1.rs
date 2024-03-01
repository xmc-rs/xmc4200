#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Field `CR1` reader - High resolution rising edge value"]
pub type Cr1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn cr1(&self) -> Cr1R {
        Cr1R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "HRC rising edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
