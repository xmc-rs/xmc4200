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
pub struct D0SES_W<'a> {
    w: &'a mut W,
}
impl<'a> D0SES_W<'a> {
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
#[doc = "Field `D0SVS` writer - CMP0 inverting input switch request"]
pub struct D0SVS_W<'a> {
    w: &'a mut W,
}
impl<'a> D0SVS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `D1SES` writer - DAC1 shadow transfer enable set"]
pub struct D1SES_W<'a> {
    w: &'a mut W,
}
impl<'a> D1SES_W<'a> {
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
#[doc = "Field `D1SVS` writer - CMP1 inverting input switch request"]
pub struct D1SVS_W<'a> {
    w: &'a mut W,
}
impl<'a> D1SVS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `D2SES` writer - DAC2 shadow transfer enable set"]
pub struct D2SES_W<'a> {
    w: &'a mut W,
}
impl<'a> D2SES_W<'a> {
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
#[doc = "Field `D2SVS` writer - CMP2 inverting input switch request"]
pub struct D2SVS_W<'a> {
    w: &'a mut W,
}
impl<'a> D2SVS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable set"]
    #[inline(always)]
    pub fn d0ses(&mut self) -> D0SES_W {
        D0SES_W { w: self }
    }
    #[doc = "Bit 1 - CMP0 inverting input switch request"]
    #[inline(always)]
    pub fn d0svs(&mut self) -> D0SVS_W {
        D0SVS_W { w: self }
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable set"]
    #[inline(always)]
    pub fn d1ses(&mut self) -> D1SES_W {
        D1SES_W { w: self }
    }
    #[doc = "Bit 5 - CMP1 inverting input switch request"]
    #[inline(always)]
    pub fn d1svs(&mut self) -> D1SVS_W {
        D1SVS_W { w: self }
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable set"]
    #[inline(always)]
    pub fn d2ses(&mut self) -> D2SES_W {
        D2SES_W { w: self }
    }
    #[doc = "Bit 9 - CMP2 inverting input switch request"]
    #[inline(always)]
    pub fn d2svs(&mut self) -> D2SVS_W {
        D2SVS_W { w: self }
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
}
#[doc = "`reset()` method sets CSGTRG to value 0"]
impl crate::Resettable for CSGTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
