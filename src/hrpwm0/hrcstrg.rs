#[doc = "Register `HRCSTRG` writer"]
pub struct W(crate::W<HRCSTRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRCSTRG_SPEC>;
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
impl From<crate::W<HRCSTRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRCSTRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H0ES` writer - HRC0 high resolution values shadow transfer Enable Set"]
pub type H0ES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
#[doc = "Field `H0DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H0DES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
#[doc = "Field `H1ES` writer - HRC1 high resolution values shadow transfer Enable Set"]
pub type H1ES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
#[doc = "Field `H1DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H1DES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
#[doc = "Field `H2ES` writer - HRC2 high resolution values shadow transfer Enable Set"]
pub type H2ES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
#[doc = "Field `H2DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H2DES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
#[doc = "Field `H3ES` writer - HRC3 high resolution values shadow transfer Enable Set"]
pub type H3ES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
#[doc = "Field `H3DES` writer - HRC0 dead time value shadow transfer enable set"]
pub type H3DES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCSTRG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h0es(&mut self) -> H0ES_W<0> {
        H0ES_W::new(self)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h0des(&mut self) -> H0DES_W<1> {
        H0DES_W::new(self)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h1es(&mut self) -> H1ES_W<4> {
        H1ES_W::new(self)
    }
    #[doc = "Bit 5 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h1des(&mut self) -> H1DES_W<5> {
        H1DES_W::new(self)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h2es(&mut self) -> H2ES_W<8> {
        H2ES_W::new(self)
    }
    #[doc = "Bit 9 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h2des(&mut self) -> H2DES_W<9> {
        H2DES_W::new(self)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn h3es(&mut self) -> H3ES_W<12> {
        H3ES_W::new(self)
    }
    #[doc = "Bit 13 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    #[must_use]
    pub fn h3des(&mut self) -> H3DES_W<13> {
        H3DES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global HRC shadow trigger set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrcstrg](index.html) module"]
pub struct HRCSTRG_SPEC;
impl crate::RegisterSpec for HRCSTRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hrcstrg::W](W) writer structure"]
impl crate::Writable for HRCSTRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRCSTRG to value 0"]
impl crate::Resettable for HRCSTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
