#[doc = "Writer for register HINTCLR"]
pub type W = crate::W<u32, super::HINTCLR>;
#[doc = "Register HINTCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::HINTCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Internally Controlled Hibernate Sequence Request Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBNINT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Hibernate bit clear"]
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
    #[doc = "Hibernate bit clear"]
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
#[doc = "VDDP Supply Switch of Flash Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHOFF_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Switch on VDDP supply of Flash"]
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
    #[doc = "Switch on VDDP supply of Flash"]
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
#[doc = "Flash Power Down Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHPD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Flash power down mode leave request"]
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
    #[doc = "Flash power down mode leave request"]
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
#[doc = "PORST Pull-up OFF Direct Control Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up on"]
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
    #[doc = "Pull-up on"]
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
#[doc = "PORST Pull-up OFF in Hibernate Mode Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFH_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up on"]
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
    #[doc = "Pull-up on"]
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
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Clear"]
    #[inline(always)]
    pub fn hibnint(&mut self) -> HIBNINT_W {
        HIBNINT_W { w: self }
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Clear"]
    #[inline(always)]
    pub fn flashoff(&mut self) -> FLASHOFF_W {
        FLASHOFF_W { w: self }
    }
    #[doc = "Bit 3 - Flash Power Down Clear"]
    #[inline(always)]
    pub fn flashpd(&mut self) -> FLASHPD_W {
        FLASHPD_W { w: self }
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Clear"]
    #[inline(always)]
    pub fn poffd(&mut self) -> POFFD_W {
        POFFD_W { w: self }
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
    #[inline(always)]
    pub fn ppodel(&mut self) -> PPODEL_W {
        PPODEL_W { w: self }
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Clear"]
    #[inline(always)]
    pub fn poffh(&mut self) -> POFFH_W {
        POFFH_W { w: self }
    }
}
