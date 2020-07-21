#[doc = "Reader of register LPACCONF"]
pub type R = crate::R<u32, super::LPACCONF>;
#[doc = "Writer for register LPACCONF"]
pub type W = crate::W<u32, super::LPACCONF>;
#[doc = "Register LPACCONF `reset()`'s with value 0x7000_0000"]
impl crate::ResetValue for super::LPACCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7000_0000
    }
}
#[doc = "Compare Enable for Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPEN_A {
    #[doc = "0: Comparator permanently in power down"]
    VALUE1 = 0,
    #[doc = "1: Comparator activated for VBAT input"]
    VALUE2 = 1,
    #[doc = "2: Comparator activated for HIB_IO_0 input"]
    VALUE3 = 2,
    #[doc = "4: Comparator activated for HIB_IO_1 input"]
    VALUE4 = 4,
}
impl From<CMPEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMPEN`"]
pub type CMPEN_R = crate::R<u8, CMPEN_A>;
impl CMPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMPEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMPEN_A::VALUE1),
            1 => Val(CMPEN_A::VALUE2),
            2 => Val(CMPEN_A::VALUE3),
            4 => Val(CMPEN_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMPEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CMPEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CMPEN_A::VALUE4
    }
}
#[doc = "Write proxy for field `CMPEN`"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Comparator permanently in power down"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE1)
    }
    #[doc = "Comparator activated for VBAT input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE2)
    }
    #[doc = "Comparator activated for HIB_IO_0 input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE3)
    }
    #[doc = "Comparator activated for HIB_IO_1 input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Analog Compare Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: Sub-second interval counter"]
    VALUE1 = 0,
    #[doc = "1: RTC alarm event"]
    VALUE2 = 1,
    #[doc = "2: RTC periodic event"]
    VALUE3 = 2,
    #[doc = "3: On digital WKUP input positive edge event"]
    VALUE4 = 3,
    #[doc = "5: On digital WKUP input negative edge event"]
    VALUE5 = 5,
    #[doc = "6: Continuous measurement"]
    VALUE6 = 6,
    #[doc = "7: Single-shot on software request"]
    VALUE7 = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSEL`"]
pub type TRIGSEL_R = crate::R<u8, TRIGSEL_A>;
impl TRIGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGSEL_A::VALUE1),
            1 => Val(TRIGSEL_A::VALUE2),
            2 => Val(TRIGSEL_A::VALUE3),
            3 => Val(TRIGSEL_A::VALUE4),
            5 => Val(TRIGSEL_A::VALUE5),
            6 => Val(TRIGSEL_A::VALUE6),
            7 => Val(TRIGSEL_A::VALUE7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRIGSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRIGSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRIGSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRIGSEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == TRIGSEL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == TRIGSEL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == TRIGSEL_A::VALUE7
    }
}
#[doc = "Write proxy for field `TRIGSEL`"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sub-second interval counter"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE1)
    }
    #[doc = "RTC alarm event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE2)
    }
    #[doc = "RTC periodic event"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE3)
    }
    #[doc = "On digital WKUP input positive edge event"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE4)
    }
    #[doc = "On digital WKUP input negative edge event"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE5)
    }
    #[doc = "Continuous measurement"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE6)
    }
    #[doc = "Single-shot on software request"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CONVDEL`"]
pub type CONVDEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONVDEL`"]
pub struct CONVDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVDEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `INTERVCNT`"]
pub type INTERVCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTERVCNT`"]
pub struct INTERVCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERVCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SETTLECNT`"]
pub type SETTLECNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SETTLECNT`"]
pub struct SETTLECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTLECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    pub fn convdel(&self) -> CONVDEL_R {
        CONVDEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    pub fn intervcnt(&self) -> INTERVCNT_R {
        INTERVCNT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    pub fn settlecnt(&self) -> SETTLECNT_R {
        SETTLECNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    pub fn convdel(&mut self) -> CONVDEL_W {
        CONVDEL_W { w: self }
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    pub fn intervcnt(&mut self) -> INTERVCNT_W {
        INTERVCNT_W { w: self }
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    pub fn settlecnt(&mut self) -> SETTLECNT_W {
        SETTLECNT_W { w: self }
    }
}
