#[doc = "Register `CSGTRG` writer"]
pub struct W(crate::W<CSGTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGTRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CSGTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D0SES` writer - DAC0 shadow transfer enable set"]
pub type D0SES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGTRG_SPEC, bool, O>;
#[doc = "Field `D0SVS` writer - CMP0 inverting input switch request"]
pub type D0SVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGTRG_SPEC, bool, O>;
#[doc = "Field `D1SES` writer - DAC1 shadow transfer enable set"]
pub type D1SES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGTRG_SPEC, bool, O>;
#[doc = "Field `D1SVS` writer - CMP1 inverting input switch request"]
pub type D1SVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGTRG_SPEC, bool, O>;
#[doc = "Field `D2SES` writer - DAC2 shadow transfer enable set"]
pub type D2SES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGTRG_SPEC, bool, O>;
#[doc = "Field `D2SVS` writer - CMP2 inverting input switch request"]
pub type D2SVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGTRG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn d0ses(&mut self) -> D0SES_W<0> {
        D0SES_W::new(self)
    }
    #[doc = "Bit 1 - CMP0 inverting input switch request"]
    #[inline(always)]
    #[must_use]
    pub fn d0svs(&mut self) -> D0SVS_W<1> {
        D0SVS_W::new(self)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn d1ses(&mut self) -> D1SES_W<4> {
        D1SES_W::new(self)
    }
    #[doc = "Bit 5 - CMP1 inverting input switch request"]
    #[inline(always)]
    #[must_use]
    pub fn d1svs(&mut self) -> D1SVS_W<5> {
        D1SVS_W::new(self)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn d2ses(&mut self) -> D2SES_W<8> {
        D2SES_W::new(self)
    }
    #[doc = "Bit 9 - CMP2 inverting input switch request"]
    #[inline(always)]
    #[must_use]
    pub fn d2svs(&mut self) -> D2SVS_W<9> {
        D2SVS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CSG shadow/switch trigger\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgtrg](index.html) module"]
pub struct CSGTRG_SPEC;
impl crate::RegisterSpec for CSGTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csgtrg::W](W) writer structure"]
impl crate::Writable for CSGTRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSGTRG to value 0"]
impl crate::Resettable for CSGTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
