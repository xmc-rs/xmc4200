#[doc = "Register `CSGCLRG` writer"]
pub struct W(crate::W<CSGCLRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCLRG_SPEC>;
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
impl From<crate::W<CSGCLRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCLRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD0R` writer - DAC0 run bit clear"]
pub type CD0R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CC0R` writer - CMP0 run bit clear"]
pub type CC0R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CC0P` writer - CMP0 passive level clear"]
pub type CC0P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CD1R` writer - DAC1 run bit clear"]
pub type CD1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CC1R` writer - CMP1 run bit clear"]
pub type CC1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CC1P` writer - CMP1 passive level clear"]
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CD2R` writer - DAC2 run bit clear"]
pub type CD2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CC2R` writer - CMP2 run bit clear"]
pub type CC2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
#[doc = "Field `CC2P` writer - CMP2 passive level clear"]
pub type CC2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGCLRG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DAC0 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd0r(&mut self) -> CD0R_W<0> {
        CD0R_W::new(self)
    }
    #[doc = "Bit 1 - CMP0 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc0r(&mut self) -> CC0R_W<1> {
        CC0R_W::new(self)
    }
    #[doc = "Bit 2 - CMP0 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc0p(&mut self) -> CC0P_W<2> {
        CC0P_W::new(self)
    }
    #[doc = "Bit 4 - DAC1 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd1r(&mut self) -> CD1R_W<4> {
        CD1R_W::new(self)
    }
    #[doc = "Bit 5 - CMP1 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc1r(&mut self) -> CC1R_W<5> {
        CC1R_W::new(self)
    }
    #[doc = "Bit 6 - CMP1 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<6> {
        CC1P_W::new(self)
    }
    #[doc = "Bit 8 - DAC2 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd2r(&mut self) -> CD2R_W<8> {
        CD2R_W::new(self)
    }
    #[doc = "Bit 9 - CMP2 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc2r(&mut self) -> CC2R_W<9> {
        CC2R_W::new(self)
    }
    #[doc = "Bit 10 - CMP2 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<10> {
        CC2P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CSG run bit clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgclrg](index.html) module"]
pub struct CSGCLRG_SPEC;
impl crate::RegisterSpec for CSGCLRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csgclrg::W](W) writer structure"]
impl crate::Writable for CSGCLRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSGCLRG to value 0"]
impl crate::Resettable for CSGCLRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
