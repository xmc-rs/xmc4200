#[doc = "Register `SDCR` reader"]
pub type R = crate::R<SdcrSpec>;
#[doc = "Register `SDCR` writer"]
pub type W = crate::W<SdcrSpec>;
#[doc = "Field `SDTRV` reader - Shadow dead time rising value"]
pub type SdtrvR = crate::FieldReader<u16>;
#[doc = "Field `SDTRV` writer - Shadow dead time rising value"]
pub type SdtrvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    pub fn sdtrv(&self) -> SdtrvR {
        SdtrvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtrv(&mut self) -> SdtrvW<SdcrSpec> {
        SdtrvW::new(self, 0)
    }
}
#[doc = "HRC shadow dead time rising\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdcrSpec;
impl crate::RegisterSpec for SdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcr::R`](R) reader structure"]
impl crate::Readable for SdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdcr::W`](W) writer structure"]
impl crate::Writable for SdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDCR to value 0x01"]
impl crate::Resettable for SdcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
