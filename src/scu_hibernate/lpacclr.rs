#[doc = "Writer for register LPACCLR"]
pub type W = crate::W<u32, super::LPACCLR>;
#[doc = "Register LPACCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPACCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger VBAT Single Compare Operation Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSCMP_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Clear the sticky bit"]
    VALUE2,
}
impl From<VBATSCMP_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATSCMP_AW) -> Self {
        match variant {
            VBATSCMP_AW::VALUE1 => false,
            VBATSCMP_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `VBATSCMP`"]
pub struct VBATSCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATSCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATSCMP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATSCMP_AW::VALUE1)
    }
    #[doc = "Clear the sticky bit"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0SCMP_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Clear the sticky bit"]
    VALUE2,
}
impl From<AHIBIO0SCMP_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0SCMP_AW) -> Self {
        match variant {
            AHIBIO0SCMP_AW::VALUE1 => false,
            AHIBIO0SCMP_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `AHIBIO0SCMP`"]
pub struct AHIBIO0SCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0SCMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO0SCMP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0SCMP_AW::VALUE1)
    }
    #[doc = "Clear the sticky bit"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "VBAT Compare Operation Initial Value Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATVAL_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Below programmed threshold"]
    VALUE2,
}
impl From<VBATVAL_AW> for bool {
    #[inline(always)]
    fn from(variant: VBATVAL_AW) -> Self {
        match variant {
            VBATVAL_AW::VALUE1 => false,
            VBATVAL_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `VBATVAL`"]
pub struct VBATVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VBATVAL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VBATVAL_AW::VALUE1)
    }
    #[doc = "Below programmed threshold"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "HIB_IO_0 Input Compare Initial Value Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0VAL_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Below programmed threshold"]
    VALUE2,
}
impl From<AHIBIO0VAL_AW> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0VAL_AW) -> Self {
        match variant {
            AHIBIO0VAL_AW::VALUE1 => false,
            AHIBIO0VAL_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `AHIBIO0VAL`"]
pub struct AHIBIO0VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHIBIO0VAL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHIBIO0VAL_AW::VALUE1)
    }
    #[doc = "Below programmed threshold"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Clear"]
    #[inline(always)]
    pub fn vbatscmp(&mut self) -> VBATSCMP_W {
        VBATSCMP_W { w: self }
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Clear"]
    #[inline(always)]
    pub fn ahibio0scmp(&mut self) -> AHIBIO0SCMP_W {
        AHIBIO0SCMP_W { w: self }
    }
    #[doc = "Bit 16 - VBAT Compare Operation Initial Value Clear"]
    #[inline(always)]
    pub fn vbatval(&mut self) -> VBATVAL_W {
        VBATVAL_W { w: self }
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Initial Value Clear"]
    #[inline(always)]
    pub fn ahibio0val(&mut self) -> AHIBIO0VAL_W {
        AHIBIO0VAL_W { w: self }
    }
}
