#[doc = "Register `HRCCTRG` writer"]
pub struct W(crate::W<HRCCTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRCCTRG_SPEC>;
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
impl From<crate::W<HRCCTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRCCTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H0EC` writer - HRC0 high resolution values shadow transfer Enable Clear"]
pub type H0EC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
#[doc = "Field `H0DEC` writer - HRC0 dead time value shadow transfer Enable Clear"]
pub type H0DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
#[doc = "Field `H1EC` writer - HRC1 high resolution values shadow transfer Enable Clear"]
pub type H1EC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
#[doc = "Field `H1DEC` writer - HRC1 dead time value shadow transfer Enable Clear"]
pub type H1DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
#[doc = "Field `H2CEC` writer - HRC2 high resolution values shadow transfer Enable Clear"]
pub type H2CEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
#[doc = "Field `H2DEC` writer - HRC2 dead time value shadow transfer Enable Clear"]
pub type H2DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
#[doc = "Field `H3EC` writer - HRC3 high resolution values shadow transfer Enable Clear"]
pub type H3EC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
#[doc = "Field `H3DEC` writer - HRC3 dead time value shadow transfer Enable Clear"]
pub type H3DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCTRG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h0ec(&mut self) -> H0EC_W<0> {
        H0EC_W::new(self)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h0dec(&mut self) -> H0DEC_W<1> {
        H0DEC_W::new(self)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h1ec(&mut self) -> H1EC_W<4> {
        H1EC_W::new(self)
    }
    #[doc = "Bit 5 - HRC1 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h1dec(&mut self) -> H1DEC_W<5> {
        H1DEC_W::new(self)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h2cec(&mut self) -> H2CEC_W<8> {
        H2CEC_W::new(self)
    }
    #[doc = "Bit 9 - HRC2 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h2dec(&mut self) -> H2DEC_W<9> {
        H2DEC_W::new(self)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h3ec(&mut self) -> H3EC_W<12> {
        H3EC_W::new(self)
    }
    #[doc = "Bit 13 - HRC3 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn h3dec(&mut self) -> H3DEC_W<13> {
        H3DEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global HRC shadow trigger clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrcctrg](index.html) module"]
pub struct HRCCTRG_SPEC;
impl crate::RegisterSpec for HRCCTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hrcctrg::W](W) writer structure"]
impl crate::Writable for HRCCTRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRCCTRG to value 0"]
impl crate::Resettable for HRCCTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
