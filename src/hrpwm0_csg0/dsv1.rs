#[doc = "Register `DSV1` reader"]
pub type R = crate::R<Dsv1Spec>;
#[doc = "Field `DSV1` reader - DAC reference value 1"]
pub type Dsv1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - DAC reference value 1"]
    #[inline(always)]
    pub fn dsv1(&self) -> Dsv1R {
        Dsv1R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DAC reference value 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsv1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsv1Spec;
impl crate::RegisterSpec for Dsv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv1::R`](R) reader structure"]
impl crate::Readable for Dsv1Spec {}
#[doc = "`reset()` method sets DSV1 to value 0"]
impl crate::Resettable for Dsv1Spec {
    const RESET_VALUE: u32 = 0;
}
