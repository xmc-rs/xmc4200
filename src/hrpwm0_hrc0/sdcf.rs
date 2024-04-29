#[doc = "Register `SDCF` reader"]
pub type R = crate::R<SDCF_SPEC>;
#[doc = "Register `SDCF` writer"]
pub type W = crate::W<SDCF_SPEC>;
#[doc = "Field `SDTFV` reader - Shadow dead time falling value"]
pub type SDTFV_R = crate::FieldReader<u16>;
#[doc = "Field `SDTFV` writer - Shadow dead time falling value"]
pub type SDTFV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Shadow dead time falling value"]
    #[inline(always)]
    pub fn sdtfv(&self) -> SDTFV_R {
        SDTFV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow dead time falling value"]
    #[inline(always)]
    #[must_use]
    pub fn sdtfv(&mut self) -> SDTFV_W<SDCF_SPEC> {
        SDTFV_W::new(self, 0)
    }
}
#[doc = "HRC shadow dead time falling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdcf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdcf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDCF_SPEC;
impl crate::RegisterSpec for SDCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcf::R`](R) reader structure"]
impl crate::Readable for SDCF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdcf::W`](W) writer structure"]
impl crate::Writable for SDCF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDCF to value 0x01"]
impl crate::Resettable for SDCF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
