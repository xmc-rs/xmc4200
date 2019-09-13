#[doc = "Reader of register IES"]
pub type R = crate::R<u32, super::IES>;
#[doc = "Writer for register IES"]
pub type W = crate::W<u32, super::IES>;
#[doc = "Register IES `reset()`'s with value 0"]
impl crate::ResetValue for super::IES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External value switch function level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVLS_A {
    #[doc = "0: Function disabled"]
    VALUE1,
    #[doc = "1: Active when input is HIGH"]
    VALUE2,
    #[doc = "2: Active when input is LOW"]
    VALUE3,
}
impl From<SVLS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVLS_A) -> Self {
        match variant {
            SVLS_A::VALUE1 => 0,
            SVLS_A::VALUE2 => 1,
            SVLS_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `SVLS`"]
pub type SVLS_R = crate::R<u8, SVLS_A>;
impl SVLS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SVLS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SVLS_A::VALUE1),
            1 => Val(SVLS_A::VALUE2),
            2 => Val(SVLS_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SVLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SVLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SVLS_A::VALUE3
    }
}
#[doc = "Write proxy for field `SVLS`"]
pub struct SVLS_W<'a> {
    w: &'a mut W,
}
impl<'a> SVLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVLS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVLS_A::VALUE1)
    }
    #[doc = "Active when input is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVLS_A::VALUE2)
    }
    #[doc = "Active when input is LOW"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SVLS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "External start function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRES_A {
    #[doc = "0: Function disabled"]
    VALUE1,
    #[doc = "1: Active on rising edge"]
    VALUE2,
    #[doc = "2: Active on falling edge"]
    VALUE3,
    #[doc = "3: Active on both edges"]
    VALUE4,
}
impl From<STRES_A> for u8 {
    #[inline(always)]
    fn from(variant: STRES_A) -> Self {
        match variant {
            STRES_A::VALUE1 => 0,
            STRES_A::VALUE2 => 1,
            STRES_A::VALUE3 => 2,
            STRES_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `STRES`"]
pub type STRES_R = crate::R<u8, STRES_A>;
impl STRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRES_A {
        match self.bits {
            0 => STRES_A::VALUE1,
            1 => STRES_A::VALUE2,
            2 => STRES_A::VALUE3,
            3 => STRES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STRES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STRES_A::VALUE4
    }
}
#[doc = "Write proxy for field `STRES`"]
pub struct STRES_W<'a> {
    w: &'a mut W,
}
impl<'a> STRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STRES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STRES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "External stop function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPES_A {
    #[doc = "0: Function disabled"]
    VALUE1,
    #[doc = "1: Active on rising edge"]
    VALUE2,
    #[doc = "2: Active on falling edge"]
    VALUE3,
    #[doc = "3: Active on both edges"]
    VALUE4,
}
impl From<STPES_A> for u8 {
    #[inline(always)]
    fn from(variant: STPES_A) -> Self {
        match variant {
            STPES_A::VALUE1 => 0,
            STPES_A::VALUE2 => 1,
            STPES_A::VALUE3 => 2,
            STPES_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `STPES`"]
pub type STPES_R = crate::R<u8, STPES_A>;
impl STPES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPES_A {
        match self.bits {
            0 => STPES_A::VALUE1,
            1 => STPES_A::VALUE2,
            2 => STPES_A::VALUE3,
            3 => STPES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STPES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STPES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STPES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STPES_A::VALUE4
    }
}
#[doc = "Write proxy for field `STPES`"]
pub struct STPES_W<'a> {
    w: &'a mut W,
}
impl<'a> STPES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STPES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STPES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STPES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STPES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "External trigger function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGES_A {
    #[doc = "0: Function disabled"]
    VALUE1,
    #[doc = "1: Active on rising edge"]
    VALUE2,
    #[doc = "2: Active on falling edge"]
    VALUE3,
    #[doc = "3: Active on both edges"]
    VALUE4,
}
impl From<TRGES_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGES_A) -> Self {
        match variant {
            TRGES_A::VALUE1 => 0,
            TRGES_A::VALUE2 => 1,
            TRGES_A::VALUE3 => 2,
            TRGES_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `TRGES`"]
pub type TRGES_R = crate::R<u8, TRGES_A>;
impl TRGES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGES_A {
        match self.bits {
            0 => TRGES_A::VALUE1,
            1 => TRGES_A::VALUE2,
            2 => TRGES_A::VALUE3,
            3 => TRGES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRGES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRGES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRGES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRGES_A::VALUE4
    }
}
#[doc = "Write proxy for field `TRGES`"]
pub struct TRGES_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "External shadow transfer enable edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STES_A {
    #[doc = "0: Function disabled"]
    VALUE1,
    #[doc = "1: Active on rising edge"]
    VALUE2,
    #[doc = "2: Active on falling edge"]
    VALUE3,
    #[doc = "3: Active on both edges"]
    VALUE4,
}
impl From<STES_A> for u8 {
    #[inline(always)]
    fn from(variant: STES_A) -> Self {
        match variant {
            STES_A::VALUE1 => 0,
            STES_A::VALUE2 => 1,
            STES_A::VALUE3 => 2,
            STES_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `STES`"]
pub type STES_R = crate::R<u8, STES_A>;
impl STES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STES_A {
        match self.bits {
            0 => STES_A::VALUE1,
            1 => STES_A::VALUE2,
            2 => STES_A::VALUE3,
            3 => STES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STES_A::VALUE4
    }
}
#[doc = "Write proxy for field `STES`"]
pub struct STES_W<'a> {
    w: &'a mut W,
}
impl<'a> STES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    pub fn svls(&self) -> SVLS_R {
        SVLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    pub fn stres(&self) -> STRES_R {
        STRES_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    pub fn stpes(&self) -> STPES_R {
        STPES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    pub fn trges(&self) -> TRGES_R {
        TRGES_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    pub fn stes(&self) -> STES_R {
        STES_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    pub fn svls(&mut self) -> SVLS_W {
        SVLS_W { w: self }
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    pub fn stres(&mut self) -> STRES_W {
        STRES_W { w: self }
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    pub fn stpes(&mut self) -> STPES_W {
        STPES_W { w: self }
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    pub fn trges(&mut self) -> TRGES_W {
        TRGES_W { w: self }
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    pub fn stes(&mut self) -> STES_W {
        STES_W { w: self }
    }
}
