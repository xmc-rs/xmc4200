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
pub struct S0STR_W<'a> {
    w: &'a mut W,
}
impl<'a> S0STR_W<'a> {
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
#[doc = "Field `S0STP` writer - Slope 0 stop"]
pub struct S0STP_W<'a> {
    w: &'a mut W,
}
impl<'a> S0STP_W<'a> {
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
#[doc = "Field `PS0STR` writer - Prescaler 0 start"]
pub struct PS0STR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0STR_W<'a> {
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
#[doc = "Field `PS0STP` writer - Prescaler 0 stop"]
pub struct PS0STP_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0STP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PS0CLR` writer - Prescaler 0 clear"]
pub struct PS0CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0CLR_W<'a> {
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
#[doc = "Field `S1STR` writer - Slope 1 start"]
pub struct S1STR_W<'a> {
    w: &'a mut W,
}
impl<'a> S1STR_W<'a> {
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
#[doc = "Field `S1STP` writer - Slope 1 stop"]
pub struct S1STP_W<'a> {
    w: &'a mut W,
}
impl<'a> S1STP_W<'a> {
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
#[doc = "Field `PS1STR` writer - Prescaler 1 start"]
pub struct PS1STR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1STR_W<'a> {
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
#[doc = "Field `PS1STP` writer - Prescaler 1 stop"]
pub struct PS1STP_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1STP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PS1CLR` writer - Prescaler 1 clear"]
pub struct PS1CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1CLR_W<'a> {
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
#[doc = "Field `S2STR` writer - Slope 2 start"]
pub struct S2STR_W<'a> {
    w: &'a mut W,
}
impl<'a> S2STR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `S2STP` writer - Slope 2 stop"]
pub struct S2STP_W<'a> {
    w: &'a mut W,
}
impl<'a> S2STP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PS2STR` writer - Prescaler 2 start"]
pub struct PS2STR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2STR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PS2STP` writer - Prescaler 2 stop"]
pub struct PS2STP_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2STP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PS2CLR` writer - Prescaler 2 clear"]
pub struct PS2CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Slope 0 start"]
    #[inline(always)]
    pub fn s0str(&mut self) -> S0STR_W {
        S0STR_W { w: self }
    }
    #[doc = "Bit 1 - Slope 0 stop"]
    #[inline(always)]
    pub fn s0stp(&mut self) -> S0STP_W {
        S0STP_W { w: self }
    }
    #[doc = "Bit 2 - Prescaler 0 start"]
    #[inline(always)]
    pub fn ps0str(&mut self) -> PS0STR_W {
        PS0STR_W { w: self }
    }
    #[doc = "Bit 3 - Prescaler 0 stop"]
    #[inline(always)]
    pub fn ps0stp(&mut self) -> PS0STP_W {
        PS0STP_W { w: self }
    }
    #[doc = "Bit 4 - Prescaler 0 clear"]
    #[inline(always)]
    pub fn ps0clr(&mut self) -> PS0CLR_W {
        PS0CLR_W { w: self }
    }
    #[doc = "Bit 8 - Slope 1 start"]
    #[inline(always)]
    pub fn s1str(&mut self) -> S1STR_W {
        S1STR_W { w: self }
    }
    #[doc = "Bit 9 - Slope 1 stop"]
    #[inline(always)]
    pub fn s1stp(&mut self) -> S1STP_W {
        S1STP_W { w: self }
    }
    #[doc = "Bit 10 - Prescaler 1 start"]
    #[inline(always)]
    pub fn ps1str(&mut self) -> PS1STR_W {
        PS1STR_W { w: self }
    }
    #[doc = "Bit 11 - Prescaler 1 stop"]
    #[inline(always)]
    pub fn ps1stp(&mut self) -> PS1STP_W {
        PS1STP_W { w: self }
    }
    #[doc = "Bit 12 - Prescaler 1 clear"]
    #[inline(always)]
    pub fn ps1clr(&mut self) -> PS1CLR_W {
        PS1CLR_W { w: self }
    }
    #[doc = "Bit 16 - Slope 2 start"]
    #[inline(always)]
    pub fn s2str(&mut self) -> S2STR_W {
        S2STR_W { w: self }
    }
    #[doc = "Bit 17 - Slope 2 stop"]
    #[inline(always)]
    pub fn s2stp(&mut self) -> S2STP_W {
        S2STP_W { w: self }
    }
    #[doc = "Bit 18 - Prescaler 2 start"]
    #[inline(always)]
    pub fn ps2str(&mut self) -> PS2STR_W {
        PS2STR_W { w: self }
    }
    #[doc = "Bit 19 - Prescaler 2 stop"]
    #[inline(always)]
    pub fn ps2stp(&mut self) -> PS2STP_W {
        PS2STP_W { w: self }
    }
    #[doc = "Bit 20 - Prescaler 2 clear"]
    #[inline(always)]
    pub fn ps2clr(&mut self) -> PS2CLR_W {
        PS2CLR_W { w: self }
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
}
#[doc = "`reset()` method sets CSGFCG to value 0"]
impl crate::Resettable for CSGFCG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
