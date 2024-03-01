#[doc = "Register `DCF` reader"]
pub type R = crate::R<DcfSpec>;
#[doc = "Field `DTFV` reader - Dead time falling value"]
pub type DtfvR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Dead time falling value"]
    #[inline(always)]
    pub fn dtfv(&self) -> DtfvR {
        DtfvR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "HRC dead time falling value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcfSpec;
impl crate::RegisterSpec for DcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcf::R`](R) reader structure"]
impl crate::Readable for DcfSpec {}
#[doc = "`reset()` method sets DCF to value 0x01"]
impl crate::Resettable for DcfSpec {
    const RESET_VALUE: u32 = 0x01;
}
