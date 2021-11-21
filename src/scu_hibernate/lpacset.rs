#[doc = "Register `LPACSET` writer"]
pub struct W(crate::W<LPACSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPACSET_SPEC>;
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
impl From<crate::W<LPACSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPACSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Trigger VBAT Single Compare Operation Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSCMP_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Start compare operation"]
    VALUE2 = 1,
}
impl From<VBATSCMP_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATSCMP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATSCMP` writer - Trigger VBAT Single Compare Operation Set"]
pub struct VBATSCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATSCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATSCMP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATSCMP_AW::VALUE1)
    }
    #[doc = "Start compare operation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATSCMP_AW::VALUE2)
    }
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
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0SCMP_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Start compare operation"]
    VALUE2 = 1,
}
impl From<AHIBIO0SCMP_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0SCMP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0SCMP` writer - Trigger HIB_IO_0 Input Single Compare Operation Set"]
pub struct AHIBIO0SCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0SCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO0SCMP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0SCMP_AW::VALUE1)
    }
    #[doc = "Start compare operation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0SCMP_AW::VALUE2)
    }
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
#[doc = "VBAT Compare Operation Initial Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATVAL_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Above programmed threshold"]
    VALUE2 = 1,
}
impl From<VBATVAL_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATVAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATVAL` writer - VBAT Compare Operation Initial Value Set"]
pub struct VBATVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATVAL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATVAL_AW::VALUE1)
    }
    #[doc = "Above programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VBATVAL_AW::VALUE2)
    }
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
#[doc = "HIB_IO_0 Input Compare Initial Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0VAL_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Above programmed threshold"]
    VALUE2 = 1,
}
impl From<AHIBIO0VAL_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0VAL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0VAL` writer - HIB_IO_0 Input Compare Initial Value Set"]
pub struct AHIBIO0VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO0VAL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0VAL_AW::VALUE1)
    }
    #[doc = "Above programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHIBIO0VAL_AW::VALUE2)
    }
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
impl W {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Set"]
    #[inline(always)]
    pub fn vbatscmp(&mut self) -> VBATSCMP_W {
        VBATSCMP_W { w: self }
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Set"]
    #[inline(always)]
    pub fn ahibio0scmp(&mut self) -> AHIBIO0SCMP_W {
        AHIBIO0SCMP_W { w: self }
    }
    #[doc = "Bit 16 - VBAT Compare Operation Initial Value Set"]
    #[inline(always)]
    pub fn vbatval(&mut self) -> VBATVAL_W {
        VBATVAL_W { w: self }
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Initial Value Set"]
    #[inline(always)]
    pub fn ahibio0val(&mut self) -> AHIBIO0VAL_W {
        AHIBIO0VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPAC Control Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacset](index.html) module"]
pub struct LPACSET_SPEC;
impl crate::RegisterSpec for LPACSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lpacset::W](W) writer structure"]
impl crate::Writable for LPACSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPACSET to value 0"]
impl crate::Resettable for LPACSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
