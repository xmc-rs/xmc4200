#[doc = "Register `CSGTRC` writer"]
pub struct W(crate::W<CSGTRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGTRC_SPEC>;
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
impl From<crate::W<CSGTRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGTRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D0SEC` writer - DAC0 shadow transfer enable clear"]
pub struct D0SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> D0SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `D1SEC` writer - DAC1 shadow transfer enable clear"]
pub struct D1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> D1SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `D2SEC` writer - DAC2 shadow transfer enable clear"]
pub struct D2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> D2SEC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable clear"]
    #[inline(always)]
    pub fn d0sec(&mut self) -> D0SEC_W {
        D0SEC_W { w: self }
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable clear"]
    #[inline(always)]
    pub fn d1sec(&mut self) -> D1SEC_W {
        D1SEC_W { w: self }
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable clear"]
    #[inline(always)]
    pub fn d2sec(&mut self) -> D2SEC_W {
        D2SEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CSG shadow trigger clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgtrc](index.html) module"]
pub struct CSGTRC_SPEC;
impl crate::RegisterSpec for CSGTRC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csgtrc::W](W) writer structure"]
impl crate::Writable for CSGTRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGTRC to value 0"]
impl crate::Resettable for CSGTRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
