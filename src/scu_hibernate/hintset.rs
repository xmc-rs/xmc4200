#[doc = "Writer for register HINTSET"]
pub type W = crate::W<u32, super::HINTSET>;
#[doc = "Register HINTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::HINTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Write proxy for field `HIBNINT`"]
pub struct HIBNINT_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBNINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HIBNINT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
#[doc = "Write proxy for field `VCOREOFF`"]
pub struct VCOREOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOREOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCOREOFF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
#[doc = "Write proxy for field `FLASHOFF`"]
pub struct FLASHOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHOFF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
#[doc = "Write proxy for field `FLASHPD`"]
pub struct FLASHPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHPD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
#[doc = "Write proxy for field `POFFD`"]
pub struct POFFD_W<'a> {
    w: &'a mut W,
}
impl<'a> POFFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POFFD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `PPODEL`"]
pub struct PPODEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPODEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
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
#[doc = "Write proxy for field `POFFH`"]
pub struct POFFH_W<'a> {
    w: &'a mut W,
}
impl<'a> POFFH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POFFH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
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
}
