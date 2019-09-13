#[doc = "Writer for register CLKCLR"]
pub type W = crate::W<u32, super::CLKCLR>;
#[doc = "Register CLKCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBCDI_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable clock"]
    VALUE2,
}
impl From<USBCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCDI_AW) -> Self {
        match variant {
            USBCDI_AW::VALUE1 => false,
            USBCDI_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `USBCDI`"]
pub struct USBCDI_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCDI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USBCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USBCDI_AW::VALUE2)
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
#[doc = "CCU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUCDI_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable clock"]
    VALUE2,
}
impl From<CCUCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCDI_AW) -> Self {
        match variant {
            CCUCDI_AW::VALUE1 => false,
            CCUCDI_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `CCUCDI`"]
pub struct CCUCDI_W<'a> {
    w: &'a mut W,
}
impl<'a> CCUCDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCUCDI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCUCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCUCDI_AW::VALUE2)
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
#[doc = "WDT Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCDI_AW {
    #[doc = "0: No effect"]
    VALUE1,
    #[doc = "1: Disable clock"]
    VALUE2,
}
impl From<WDTCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCDI_AW) -> Self {
        match variant {
            WDTCDI_AW::VALUE1 => false,
            WDTCDI_AW::VALUE2 => true,
        }
    }
}
#[doc = "Write proxy for field `WDTCDI`"]
pub struct WDTCDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCDI_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WDTCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WDTCDI_AW::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline(always)]
    pub fn usbcdi(&mut self) -> USBCDI_W {
        USBCDI_W { w: self }
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline(always)]
    pub fn ccucdi(&mut self) -> CCUCDI_W {
        CCUCDI_W { w: self }
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline(always)]
    pub fn wdtcdi(&mut self) -> WDTCDI_W {
        WDTCDI_W { w: self }
    }
}
