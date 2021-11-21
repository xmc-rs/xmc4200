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
pub struct H0ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H0ES_W<'a> {
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
#[doc = "Field `H0DES` writer - HRC0 dead time value shadow transfer enable set"]
pub struct H0DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H0DES_W<'a> {
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
#[doc = "Field `H1ES` writer - HRC1 high resolution values shadow transfer Enable Set"]
pub struct H1ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H1ES_W<'a> {
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
#[doc = "Field `H1DES` writer - HRC0 dead time value shadow transfer enable set"]
pub struct H1DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H1DES_W<'a> {
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
#[doc = "Field `H2ES` writer - HRC2 high resolution values shadow transfer Enable Set"]
pub struct H2ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H2ES_W<'a> {
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
#[doc = "Field `H2DES` writer - HRC0 dead time value shadow transfer enable set"]
pub struct H2DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H2DES_W<'a> {
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
#[doc = "Field `H3ES` writer - HRC3 high resolution values shadow transfer Enable Set"]
pub struct H3ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H3ES_W<'a> {
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
#[doc = "Field `H3DES` writer - HRC0 dead time value shadow transfer enable set"]
pub struct H3DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H3DES_W<'a> {
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
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h0es(&mut self) -> H0ES_W {
        H0ES_W { w: self }
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h0des(&mut self) -> H0DES_W {
        H0DES_W { w: self }
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h1es(&mut self) -> H1ES_W {
        H1ES_W { w: self }
    }
    #[doc = "Bit 5 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h1des(&mut self) -> H1DES_W {
        H1DES_W { w: self }
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h2es(&mut self) -> H2ES_W {
        H2ES_W { w: self }
    }
    #[doc = "Bit 9 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h2des(&mut self) -> H2DES_W {
        H2DES_W { w: self }
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h3es(&mut self) -> H3ES_W {
        H3ES_W { w: self }
    }
    #[doc = "Bit 13 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h3des(&mut self) -> H3DES_W {
        H3DES_W { w: self }
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
}
#[doc = "`reset()` method sets HRCSTRG to value 0"]
impl crate::Resettable for HRCSTRG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
