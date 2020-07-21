#[doc = "Reader of register GLBANA"]
pub type R = crate::R<u32, super::GLBANA>;
#[doc = "Writer for register GLBANA"]
pub type W = crate::W<u32, super::GLBANA>;
#[doc = "Register GLBANA `reset()`'s with value 0x4b8c"]
impl crate::ResetValue for super::GLBANA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4b8c
    }
}
#[doc = "Reader of field `SLDLY`"]
pub type SLDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLDLY`"]
pub struct SLDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `FUP`"]
pub type FUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FUP`"]
pub struct FUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FUP_W<'a> {
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
#[doc = "Reader of field `FDN`"]
pub type FDN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDN`"]
pub struct FDN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDN_W<'a> {
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
#[doc = "Reader of field `SLCP`"]
pub type SLCP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLCP`"]
pub struct SLCP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLCP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `SLIBLDO`"]
pub type SLIBLDO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLIBLDO`"]
pub struct SLIBLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLIBLDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `SLIBLF`"]
pub type SLIBLF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLIBLF`"]
pub struct SLIBLF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLIBLF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `SLVREF`"]
pub type SLVREF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLVREF`"]
pub struct SLVREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `TRIBIAS`"]
pub type TRIBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIBIAS`"]
pub struct TRIBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Force chargepump down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GHREN_A {
    #[doc = "0: Global high resolution generation is enabled"]
    VALUE1 = 0,
    #[doc = "1: Global high resolution generation is disabled"]
    VALUE2 = 1,
}
impl From<GHREN_A> for bool {
    #[inline(always)]
    fn from(variant: GHREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GHREN`"]
pub type GHREN_R = crate::R<bool, GHREN_A>;
impl GHREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GHREN_A {
        match self.bits {
            false => GHREN_A::VALUE1,
            true => GHREN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GHREN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GHREN_A::VALUE2
    }
}
#[doc = "Write proxy for field `GHREN`"]
pub struct GHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> GHREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GHREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Global high resolution generation is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GHREN_A::VALUE1)
    }
    #[doc = "Global high resolution generation is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GHREN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    pub fn sldly(&self) -> SLDLY_R {
        SLDLY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    pub fn fdn(&self) -> FDN_R {
        FDN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    pub fn slcp(&self) -> SLCP_R {
        SLCP_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    pub fn slibldo(&self) -> SLIBLDO_R {
        SLIBLDO_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    pub fn sliblf(&self) -> SLIBLF_R {
        SLIBLF_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    pub fn slvref(&self) -> SLVREF_R {
        SLVREF_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    pub fn tribias(&self) -> TRIBIAS_R {
        TRIBIAS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    pub fn ghren(&self) -> GHREN_R {
        GHREN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    pub fn sldly(&mut self) -> SLDLY_W {
        SLDLY_W { w: self }
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W {
        FUP_W { w: self }
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    pub fn fdn(&mut self) -> FDN_W {
        FDN_W { w: self }
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    pub fn slcp(&mut self) -> SLCP_W {
        SLCP_W { w: self }
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    pub fn slibldo(&mut self) -> SLIBLDO_W {
        SLIBLDO_W { w: self }
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    pub fn sliblf(&mut self) -> SLIBLF_W {
        SLIBLF_W { w: self }
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    pub fn slvref(&mut self) -> SLVREF_W {
        SLVREF_W { w: self }
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    pub fn tribias(&mut self) -> TRIBIAS_W {
        TRIBIAS_W { w: self }
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    pub fn ghren(&mut self) -> GHREN_W {
        GHREN_W { w: self }
    }
}
