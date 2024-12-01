#[doc = "Register `CSGTRG` writer"]
pub type W = crate::W<CSGTRG_SPEC>;
#[doc = "Field `D0SES` writer - DAC0 shadow transfer enable set"]
pub type D0SES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D0SVS` writer - CMP0 inverting input switch request"]
pub type D0SVS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1SES` writer - DAC1 shadow transfer enable set"]
pub type D1SES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1SVS` writer - CMP1 inverting input switch request"]
pub type D1SVS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SES` writer - DAC2 shadow transfer enable set"]
pub type D2SES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SVS` writer - CMP2 inverting input switch request"]
pub type D2SVS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable set"]
    #[inline(always)]
    pub fn d0ses(&mut self) -> D0SES_W<CSGTRG_SPEC> {
        D0SES_W::new(self, 0)
    }
    #[doc = "Bit 1 - CMP0 inverting input switch request"]
    #[inline(always)]
    pub fn d0svs(&mut self) -> D0SVS_W<CSGTRG_SPEC> {
        D0SVS_W::new(self, 1)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable set"]
    #[inline(always)]
    pub fn d1ses(&mut self) -> D1SES_W<CSGTRG_SPEC> {
        D1SES_W::new(self, 4)
    }
    #[doc = "Bit 5 - CMP1 inverting input switch request"]
    #[inline(always)]
    pub fn d1svs(&mut self) -> D1SVS_W<CSGTRG_SPEC> {
        D1SVS_W::new(self, 5)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable set"]
    #[inline(always)]
    pub fn d2ses(&mut self) -> D2SES_W<CSGTRG_SPEC> {
        D2SES_W::new(self, 8)
    }
    #[doc = "Bit 9 - CMP2 inverting input switch request"]
    #[inline(always)]
    pub fn d2svs(&mut self) -> D2SVS_W<CSGTRG_SPEC> {
        D2SVS_W::new(self, 9)
    }
}
#[doc = "Global CSG shadow/switch trigger\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgtrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGTRG_SPEC;
impl crate::RegisterSpec for CSGTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgtrg::W`](W) writer structure"]
impl crate::Writable for CSGTRG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGTRG to value 0"]
impl crate::Resettable for CSGTRG_SPEC {
    const RESET_VALUE: u32 = 0;
}
