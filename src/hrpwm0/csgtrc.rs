#[doc = "Register `CSGTRC` writer"]
pub type W = crate::W<CsgtrcSpec>;
#[doc = "Field `D0SEC` writer - DAC0 shadow transfer enable clear"]
pub type D0secW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1SEC` writer - DAC1 shadow transfer enable clear"]
pub type D1secW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SEC` writer - DAC2 shadow transfer enable clear"]
pub type D2secW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn d0sec(&mut self) -> D0secW<CsgtrcSpec> {
        D0secW::new(self, 0)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn d1sec(&mut self) -> D1secW<CsgtrcSpec> {
        D1secW::new(self, 4)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable clear"]
    #[inline(always)]
    #[must_use]
    pub fn d2sec(&mut self) -> D2secW<CsgtrcSpec> {
        D2secW::new(self, 8)
    }
}
#[doc = "Global CSG shadow trigger clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgtrc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgtrcSpec;
impl crate::RegisterSpec for CsgtrcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgtrc::W`](W) writer structure"]
impl crate::Writable for CsgtrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGTRC to value 0"]
impl crate::Resettable for CsgtrcSpec {
    const RESET_VALUE: u32 = 0;
}
