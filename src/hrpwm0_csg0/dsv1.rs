#[doc = "Register `DSV1` reader"]
pub type R = crate::R<DSV1_SPEC>;
#[doc = "Field `DSV1` reader - DAC reference value 1"]
pub type DSV1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - DAC reference value 1"]
    #[inline(always)]
    pub fn dsv1(&self) -> DSV1_R {
        DSV1_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DAC reference value 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsv1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSV1_SPEC;
impl crate::RegisterSpec for DSV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsv1::R`](R) reader structure"]
impl crate::Readable for DSV1_SPEC {}
#[doc = "`reset()` method sets DSV1 to value 0"]
impl crate::Resettable for DSV1_SPEC {
    const RESET_VALUE: u32 = 0;
}
