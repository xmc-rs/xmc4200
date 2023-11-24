#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2_SPEC>;
#[doc = "Field `CR2` reader - High resolution falling edge value"]
pub type CR2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn cr2(&self) -> CR2_R {
        CR2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "HRC falling edge value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2_SPEC {}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
