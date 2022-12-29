#[doc = "Register `CSGSETG` writer"]
pub struct W(crate::W<CSGSETG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGSETG_SPEC>;
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
impl From<crate::W<CSGSETG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGSETG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SD0R` writer - DAC0 run bit set"]
pub type SD0R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SC0R` writer - CMP0 run bit set"]
pub type SC0R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SC0P` writer - CMP0 passive level set"]
pub type SC0P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SD1R` writer - DAC1 run bit set"]
pub type SD1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SC1R` writer - CMP1 run bit set"]
pub type SC1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SC1P` writer - CMP1 passive level set"]
pub type SC1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SD2R` writer - DAC2 run bit set"]
pub type SD2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SC2R` writer - CMP2 run bit set"]
pub type SC2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
#[doc = "Field `SC2P` writer - CMP2 passive level set"]
pub type SC2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSGSETG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DAC0 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd0r(&mut self) -> SD0R_W<0> {
        SD0R_W::new(self)
    }
    #[doc = "Bit 1 - CMP0 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc0r(&mut self) -> SC0R_W<1> {
        SC0R_W::new(self)
    }
    #[doc = "Bit 2 - CMP0 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc0p(&mut self) -> SC0P_W<2> {
        SC0P_W::new(self)
    }
    #[doc = "Bit 4 - DAC1 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd1r(&mut self) -> SD1R_W<4> {
        SD1R_W::new(self)
    }
    #[doc = "Bit 5 - CMP1 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc1r(&mut self) -> SC1R_W<5> {
        SC1R_W::new(self)
    }
    #[doc = "Bit 6 - CMP1 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc1p(&mut self) -> SC1P_W<6> {
        SC1P_W::new(self)
    }
    #[doc = "Bit 8 - DAC2 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd2r(&mut self) -> SD2R_W<8> {
        SD2R_W::new(self)
    }
    #[doc = "Bit 9 - CMP2 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc2r(&mut self) -> SC2R_W<9> {
        SC2R_W::new(self)
    }
    #[doc = "Bit 10 - CMP2 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc2p(&mut self) -> SC2P_W<10> {
        SC2P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CSG run bit set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgsetg](index.html) module"]
pub struct CSGSETG_SPEC;
impl crate::RegisterSpec for CSGSETG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csgsetg::W](W) writer structure"]
impl crate::Writable for CSGSETG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSGSETG to value 0"]
impl crate::Resettable for CSGSETG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
