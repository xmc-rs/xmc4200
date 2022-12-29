#[doc = "Register `CSGFCG` writer"]
pub struct W(crate::W<CSGFCG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGFCG_SPEC>;
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
impl From<crate::W<CSGFCG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGFCG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S0STR` writer - Slope 0 start"]
pub type S0STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `S0STP` writer - Slope 0 stop"]
pub type S0STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS0STR` writer - Prescaler 0 start"]
pub type PS0STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS0STP` writer - Prescaler 0 stop"]
pub type PS0STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS0CLR` writer - Prescaler 0 clear"]
pub type PS0CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `S1STR` writer - Slope 1 start"]
pub type S1STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `S1STP` writer - Slope 1 stop"]
pub type S1STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS1STR` writer - Prescaler 1 start"]
pub type PS1STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS1STP` writer - Prescaler 1 stop"]
pub type PS1STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS1CLR` writer - Prescaler 1 clear"]
pub type PS1CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `S2STR` writer - Slope 2 start"]
pub type S2STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `S2STP` writer - Slope 2 stop"]
pub type S2STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS2STR` writer - Prescaler 2 start"]
pub type PS2STR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS2STP` writer - Prescaler 2 stop"]
pub type PS2STP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
#[doc = "Field `PS2CLR` writer - Prescaler 2 clear"]
pub type PS2CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGFCG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Slope 0 start"]
    #[inline(always)]
    #[must_use]
    pub fn s0str(&mut self) -> S0STR_W<0> {
        S0STR_W::new(self)
    }
    #[doc = "Bit 1 - Slope 0 stop"]
    #[inline(always)]
    #[must_use]
    pub fn s0stp(&mut self) -> S0STP_W<1> {
        S0STP_W::new(self)
    }
    #[doc = "Bit 2 - Prescaler 0 start"]
    #[inline(always)]
    #[must_use]
    pub fn ps0str(&mut self) -> PS0STR_W<2> {
        PS0STR_W::new(self)
    }
    #[doc = "Bit 3 - Prescaler 0 stop"]
    #[inline(always)]
    #[must_use]
    pub fn ps0stp(&mut self) -> PS0STP_W<3> {
        PS0STP_W::new(self)
    }
    #[doc = "Bit 4 - Prescaler 0 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ps0clr(&mut self) -> PS0CLR_W<4> {
        PS0CLR_W::new(self)
    }
    #[doc = "Bit 8 - Slope 1 start"]
    #[inline(always)]
    #[must_use]
    pub fn s1str(&mut self) -> S1STR_W<8> {
        S1STR_W::new(self)
    }
    #[doc = "Bit 9 - Slope 1 stop"]
    #[inline(always)]
    #[must_use]
    pub fn s1stp(&mut self) -> S1STP_W<9> {
        S1STP_W::new(self)
    }
    #[doc = "Bit 10 - Prescaler 1 start"]
    #[inline(always)]
    #[must_use]
    pub fn ps1str(&mut self) -> PS1STR_W<10> {
        PS1STR_W::new(self)
    }
    #[doc = "Bit 11 - Prescaler 1 stop"]
    #[inline(always)]
    #[must_use]
    pub fn ps1stp(&mut self) -> PS1STP_W<11> {
        PS1STP_W::new(self)
    }
    #[doc = "Bit 12 - Prescaler 1 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ps1clr(&mut self) -> PS1CLR_W<12> {
        PS1CLR_W::new(self)
    }
    #[doc = "Bit 16 - Slope 2 start"]
    #[inline(always)]
    #[must_use]
    pub fn s2str(&mut self) -> S2STR_W<16> {
        S2STR_W::new(self)
    }
    #[doc = "Bit 17 - Slope 2 stop"]
    #[inline(always)]
    #[must_use]
    pub fn s2stp(&mut self) -> S2STP_W<17> {
        S2STP_W::new(self)
    }
    #[doc = "Bit 18 - Prescaler 2 start"]
    #[inline(always)]
    #[must_use]
    pub fn ps2str(&mut self) -> PS2STR_W<18> {
        PS2STR_W::new(self)
    }
    #[doc = "Bit 19 - Prescaler 2 stop"]
    #[inline(always)]
    #[must_use]
    pub fn ps2stp(&mut self) -> PS2STP_W<19> {
        PS2STP_W::new(self)
    }
    #[doc = "Bit 20 - Prescaler 2 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ps2clr(&mut self) -> PS2CLR_W<20> {
        PS2CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CSG slope/prescaler control\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgfcg](index.html) module"]
pub struct CSGFCG_SPEC;
impl crate::RegisterSpec for CSGFCG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csgfcg::W](W) writer structure"]
impl crate::Writable for CSGFCG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSGFCG to value 0"]
impl crate::Resettable for CSGFCG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
