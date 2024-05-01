#[doc = "Register `CSGTRG` writer"]
pub type W = crate::W<CsgtrgSpec>;
#[doc = "Field `D0SES` writer - DAC0 shadow transfer enable set"]
pub type D0sesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D0SVS` writer - CMP0 inverting input switch request"]
pub type D0svsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1SES` writer - DAC1 shadow transfer enable set"]
pub type D1sesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1SVS` writer - CMP1 inverting input switch request"]
pub type D1svsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SES` writer - DAC2 shadow transfer enable set"]
pub type D2sesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SVS` writer - CMP2 inverting input switch request"]
pub type D2svsW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn d0ses(&mut self) -> D0sesW<CsgtrgSpec> {
        D0sesW::new(self, 0)
    }
    #[doc = "Bit 1 - CMP0 inverting input switch request"]
    #[inline(always)]
    #[must_use]
    pub fn d0svs(&mut self) -> D0svsW<CsgtrgSpec> {
        D0svsW::new(self, 1)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn d1ses(&mut self) -> D1sesW<CsgtrgSpec> {
        D1sesW::new(self, 4)
    }
    #[doc = "Bit 5 - CMP1 inverting input switch request"]
    #[inline(always)]
    #[must_use]
    pub fn d1svs(&mut self) -> D1svsW<CsgtrgSpec> {
        D1svsW::new(self, 5)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn d2ses(&mut self) -> D2sesW<CsgtrgSpec> {
        D2sesW::new(self, 8)
    }
    #[doc = "Bit 9 - CMP2 inverting input switch request"]
    #[inline(always)]
    #[must_use]
    pub fn d2svs(&mut self) -> D2svsW<CsgtrgSpec> {
        D2svsW::new(self, 9)
    }
}
#[doc = "Global CSG shadow/switch trigger\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgtrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgtrgSpec;
impl crate::RegisterSpec for CsgtrgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgtrg::W`](W) writer structure"]
impl crate::Writable for CsgtrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGTRG to value 0"]
impl crate::Resettable for CsgtrgSpec {
    const RESET_VALUE: u32 = 0;
}
