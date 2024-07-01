#[doc = "Register `CSGTRC` writer"]
pub type W = crate::W<CSGTRC_SPEC>;
#[doc = "Field `D0SEC` writer - DAC0 shadow transfer enable clear"]
pub type D0SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1SEC` writer - DAC1 shadow transfer enable clear"]
pub type D1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SEC` writer - DAC2 shadow transfer enable clear"]
pub type D2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn d0sec(&mut self) -> D0SEC_W<CSGTRC_SPEC> {
        D0SEC_W::new(self, 0)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn d1sec(&mut self) -> D1SEC_W<CSGTRC_SPEC> {
        D1SEC_W::new(self, 4)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn d2sec(&mut self) -> D2SEC_W<CSGTRC_SPEC> {
        D2SEC_W::new(self, 8)
    }
}
#[doc = "Global CSG shadow trigger clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgtrc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGTRC_SPEC;
impl crate::RegisterSpec for CSGTRC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgtrc::W`](W) writer structure"]
impl crate::Writable for CSGTRC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGTRC to value 0"]
impl crate::Resettable for CSGTRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
