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
pub struct H0EC_W<'a> {
    w: &'a mut W,
}
impl<'a> H0EC_W<'a> {
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
#[doc = "Field `H0DEC` writer - HRC0 dead time value shadow transfer Enable Clear"]
pub struct H0DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> H0DEC_W<'a> {
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
#[doc = "Field `H1EC` writer - HRC1 high resolution values shadow transfer Enable Clear"]
pub struct H1EC_W<'a> {
    w: &'a mut W,
}
impl<'a> H1EC_W<'a> {
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
#[doc = "Field `H1DEC` writer - HRC1 dead time value shadow transfer Enable Clear"]
pub struct H1DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> H1DEC_W<'a> {
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
#[doc = "Field `H2CEC` writer - HRC2 high resolution values shadow transfer Enable Clear"]
pub struct H2CEC_W<'a> {
    w: &'a mut W,
}
impl<'a> H2CEC_W<'a> {
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
#[doc = "Field `H2DEC` writer - HRC2 dead time value shadow transfer Enable Clear"]
pub struct H2DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> H2DEC_W<'a> {
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
#[doc = "Field `H3EC` writer - HRC3 high resolution values shadow transfer Enable Clear"]
pub struct H3EC_W<'a> {
    w: &'a mut W,
}
impl<'a> H3EC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `H3DEC` writer - HRC3 dead time value shadow transfer Enable Clear"]
pub struct H3DEC_W<'a> {
    w: &'a mut W,
}
impl<'a> H3DEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h0ec(&mut self) -> H0EC_W {
        H0EC_W { w: self }
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h0dec(&mut self) -> H0DEC_W {
        H0DEC_W { w: self }
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h1ec(&mut self) -> H1EC_W {
        H1EC_W { w: self }
    }
    #[doc = "Bit 5 - HRC1 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h1dec(&mut self) -> H1DEC_W {
        H1DEC_W { w: self }
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h2cec(&mut self) -> H2CEC_W {
        H2CEC_W { w: self }
    }
    #[doc = "Bit 9 - HRC2 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h2dec(&mut self) -> H2DEC_W {
        H2DEC_W { w: self }
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h3ec(&mut self) -> H3EC_W {
        H3EC_W { w: self }
    }
    #[doc = "Bit 13 - HRC3 dead time value shadow transfer Enable Clear"]
    #[inline(always)]
    pub fn h3dec(&mut self) -> H3DEC_W {
        H3DEC_W { w: self }
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
}
#[doc = "`reset()` method sets HRCCTRG to value 0"]
impl crate::Resettable for HRCCTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
