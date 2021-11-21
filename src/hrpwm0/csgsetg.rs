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
pub struct SD0R_W<'a> {
    w: &'a mut W,
}
impl<'a> SD0R_W<'a> {
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
#[doc = "Field `SC0R` writer - CMP0 run bit set"]
pub struct SC0R_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0R_W<'a> {
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
#[doc = "Field `SC0P` writer - CMP0 passive level set"]
pub struct SC0P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SD1R` writer - DAC1 run bit set"]
pub struct SD1R_W<'a> {
    w: &'a mut W,
}
impl<'a> SD1R_W<'a> {
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
#[doc = "Field `SC1R` writer - CMP1 run bit set"]
pub struct SC1R_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1R_W<'a> {
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
#[doc = "Field `SC1P` writer - CMP1 passive level set"]
pub struct SC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SD2R` writer - DAC2 run bit set"]
pub struct SD2R_W<'a> {
    w: &'a mut W,
}
impl<'a> SD2R_W<'a> {
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
#[doc = "Field `SC2R` writer - CMP2 run bit set"]
pub struct SC2R_W<'a> {
    w: &'a mut W,
}
impl<'a> SC2R_W<'a> {
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
#[doc = "Field `SC2P` writer - CMP2 passive level set"]
pub struct SC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC2P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 run bit set"]
    #[inline(always)]
    pub fn sd0r(&mut self) -> SD0R_W {
        SD0R_W { w: self }
    }
    #[doc = "Bit 1 - CMP0 run bit set"]
    #[inline(always)]
    pub fn sc0r(&mut self) -> SC0R_W {
        SC0R_W { w: self }
    }
    #[doc = "Bit 2 - CMP0 passive level set"]
    #[inline(always)]
    pub fn sc0p(&mut self) -> SC0P_W {
        SC0P_W { w: self }
    }
    #[doc = "Bit 4 - DAC1 run bit set"]
    #[inline(always)]
    pub fn sd1r(&mut self) -> SD1R_W {
        SD1R_W { w: self }
    }
    #[doc = "Bit 5 - CMP1 run bit set"]
    #[inline(always)]
    pub fn sc1r(&mut self) -> SC1R_W {
        SC1R_W { w: self }
    }
    #[doc = "Bit 6 - CMP1 passive level set"]
    #[inline(always)]
    pub fn sc1p(&mut self) -> SC1P_W {
        SC1P_W { w: self }
    }
    #[doc = "Bit 8 - DAC2 run bit set"]
    #[inline(always)]
    pub fn sd2r(&mut self) -> SD2R_W {
        SD2R_W { w: self }
    }
    #[doc = "Bit 9 - CMP2 run bit set"]
    #[inline(always)]
    pub fn sc2r(&mut self) -> SC2R_W {
        SC2R_W { w: self }
    }
    #[doc = "Bit 10 - CMP2 passive level set"]
    #[inline(always)]
    pub fn sc2p(&mut self) -> SC2P_W {
        SC2P_W { w: self }
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
}
#[doc = "`reset()` method sets CSGSETG to value 0"]
impl crate::Resettable for CSGSETG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
