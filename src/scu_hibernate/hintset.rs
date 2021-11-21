#[doc = "Register `HINTSET` writer"]
pub struct W(crate::W<HINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HINTSET_SPEC>;
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
impl From<crate::W<HINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internally Controlled Hibernate Sequence Request Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBNINT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Hardware controlled hibernate sequence request active"]
    VALUE2 = 1,
}
impl From<HIBNINT_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBNINT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNINT` writer - Internally Controlled Hibernate Sequence Request Set"]
pub struct HIBNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBNINT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HIBNINT_AW::VALUE1)
    }
    #[doc = "Hardware controlled hibernate sequence request active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HIBNINT_AW::VALUE2)
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
#[doc = "VDDC Generation off on EVR Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCOREOFF_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: VDDC off to EVR set"]
    VALUE2 = 1,
}
impl From<VCOREOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: VCOREOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOREOFF` writer - VDDC Generation off on EVR Set"]
pub struct VCOREOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOREOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOREOFF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VCOREOFF_AW::VALUE1)
    }
    #[doc = "VDDC off to EVR set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VCOREOFF_AW::VALUE2)
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
#[doc = "VDDP Supply Switch of Flash Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHOFF_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Switch off VDDP supply of Flash"]
    VALUE2 = 1,
}
impl From<FLASHOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHOFF` writer - VDDP Supply Switch of Flash Set"]
pub struct FLASHOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHOFF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLASHOFF_AW::VALUE1)
    }
    #[doc = "Switch off VDDP supply of Flash"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLASHOFF_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Flash Power Down Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHPD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Flash power down mode request set"]
    VALUE2 = 1,
}
impl From<FLASHPD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHPD` writer - Flash Power Down Set"]
pub struct FLASHPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHPD_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLASHPD_AW::VALUE1)
    }
    #[doc = "Flash power down mode request set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLASHPD_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "PORST Pull-up OFF Direct Control Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up off"]
    VALUE2 = 1,
}
impl From<POFFD_AW> for bool {
    #[inline(always)]
    fn from(variant: POFFD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFD` writer - PORST Pull-up OFF Direct Control Set"]
pub struct POFFD_W<'a> {
    w: &'a mut W,
}
impl<'a> POFFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POFFD_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POFFD_AW::VALUE1)
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POFFD_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PPODEL` writer - Delay on PORTS Pull-up Switching OFF on Hibernate Request Set"]
pub struct PPODEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPODEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "PORST Pull-up OFF in Hibernate Mode Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFH_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up off"]
    VALUE2 = 1,
}
impl From<POFFH_AW> for bool {
    #[inline(always)]
    fn from(variant: POFFH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFH` writer - PORST Pull-up OFF in Hibernate Mode Set"]
pub struct POFFH_W<'a> {
    w: &'a mut W,
}
impl<'a> POFFH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POFFH_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(POFFH_AW::VALUE1)
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(POFFH_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Set"]
    #[inline(always)]
    pub fn hibnint(&mut self) -> HIBNINT_W {
        HIBNINT_W { w: self }
    }
    #[doc = "Bit 1 - VDDC Generation off on EVR Set"]
    #[inline(always)]
    pub fn vcoreoff(&mut self) -> VCOREOFF_W {
        VCOREOFF_W { w: self }
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Set"]
    #[inline(always)]
    pub fn flashoff(&mut self) -> FLASHOFF_W {
        FLASHOFF_W { w: self }
    }
    #[doc = "Bit 3 - Flash Power Down Set"]
    #[inline(always)]
    pub fn flashpd(&mut self) -> FLASHPD_W {
        FLASHPD_W { w: self }
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Set"]
    #[inline(always)]
    pub fn poffd(&mut self) -> POFFD_W {
        POFFD_W { w: self }
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Set"]
    #[inline(always)]
    pub fn ppodel(&mut self) -> PPODEL_W {
        PPODEL_W { w: self }
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Set"]
    #[inline(always)]
    pub fn poffh(&mut self) -> POFFH_W {
        POFFH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hibernate Internal Control Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hintset](index.html) module"]
pub struct HINTSET_SPEC;
impl crate::RegisterSpec for HINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hintset::W](W) writer structure"]
impl crate::Writable for HINTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HINTSET to value 0"]
impl crate::Resettable for HINTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
