#[doc = "Register `DCR` reader"]
pub type R = crate::R<DCR_SPEC>;
#[doc = "Field `DTRV` reader - Dead time rising value"]
pub type DTRV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Dead time rising value"]
    #[inline(always)]
    pub fn dtrv(&self) -> DTRV_R {
        DTRV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HRC dead time rising value\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DCR_SPEC {}
#[doc = "`reset()` method sets DCR to value 0x01"]
impl crate::Resettable for DCR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
