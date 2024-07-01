#[doc = "Register `DCF` reader"]
pub type R = crate::R<DCF_SPEC>;
#[doc = "Field `DTFV` reader - Dead time falling value"]
pub type DTFV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Dead time falling value"]
    #[inline(always)]
    pub fn dtfv(&self) -> DTFV_R {
        DTFV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HRC dead time falling value\n\nYou can [`read`](crate::Reg::read) this register and get [`dcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCF_SPEC;
impl crate::RegisterSpec for DCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcf::R`](R) reader structure"]
impl crate::Readable for DCF_SPEC {}
#[doc = "`reset()` method sets DCF to value 0x01"]
impl crate::Resettable for DCF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
