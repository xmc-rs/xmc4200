#[doc = "Register `PC` reader"]
pub type R = crate::R<PC_SPEC>;
#[doc = "Field `PSWV` reader - Pulse swallow configuration"]
pub type PSWV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Pulse swallow configuration"]
    #[inline(always)]
    pub fn pswv(&self) -> PSWV_R {
        PSWV_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Pulse swallow configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC_SPEC;
impl crate::RegisterSpec for PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PC_SPEC {}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
