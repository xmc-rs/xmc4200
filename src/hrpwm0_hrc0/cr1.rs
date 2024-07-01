#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Field `CR1` reader - High resolution rising edge value"]
pub type CR1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn cr1(&self) -> CR1_R {
        CR1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "HRC rising edge value\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
