#[doc = "Register `PC` reader"]
pub type R = crate::R<PcSpec>;
#[doc = "Field `PSWV` reader - Pulse swallow configuration"]
pub type PswvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Pulse swallow configuration"]
    #[inline(always)]
    pub fn pswv(&self) -> PswvR {
        PswvR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Pulse swallow configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcSpec;
impl crate::RegisterSpec for PcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc::R`](R) reader structure"]
impl crate::Readable for PcSpec {}
#[doc = "`reset()` method sets PC to value 0"]
impl crate::Resettable for PcSpec {
    const RESET_VALUE: u32 = 0;
}
