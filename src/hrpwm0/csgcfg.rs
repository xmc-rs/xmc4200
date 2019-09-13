#[doc = "Reader of register CSGCFG"]
pub type R = crate::R<u32, super::CSGCFG>;
#[doc = "Writer for register CSGCFG"]
pub type W = crate::W<u32, super::CSGCFG>;
#[doc = "Register CSGCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CSG0 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C0PM_A {
    #[doc = "0: CSG0 unit is powered OFF"]
    VALUE1,
    #[doc = "1: CSG0 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "3: CSG0 unit is set in High Speed Mode"]
    VALUE4,
}
impl From<C0PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C0PM_A) -> Self {
        match variant {
            C0PM_A::VALUE1 => 0,
            C0PM_A::VALUE2 => 1,
            C0PM_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `C0PM`"]
pub type C0PM_R = crate::R<u8, C0PM_A>;
impl C0PM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C0PM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C0PM_A::VALUE1),
            1 => Val(C0PM_A::VALUE2),
            3 => Val(C0PM_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0PM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0PM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C0PM_A::VALUE4
    }
}
#[doc = "Write proxy for field `C0PM`"]
pub struct C0PM_W<'a> {
    w: &'a mut W,
}
impl<'a> C0PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C0PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CSG0 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0PM_A::VALUE1)
    }
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0PM_A::VALUE2)
    }
    #[doc = "CSG0 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C0PM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "CSG1 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C1PM_A {
    #[doc = "0: CSG1 unit is powered OFF"]
    VALUE1,
    #[doc = "1: CSG1 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "3: CSG1 unit is set in High Speed Mode"]
    VALUE4,
}
impl From<C1PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C1PM_A) -> Self {
        match variant {
            C1PM_A::VALUE1 => 0,
            C1PM_A::VALUE2 => 1,
            C1PM_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `C1PM`"]
pub type C1PM_R = crate::R<u8, C1PM_A>;
impl C1PM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C1PM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C1PM_A::VALUE1),
            1 => Val(C1PM_A::VALUE2),
            3 => Val(C1PM_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1PM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1PM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C1PM_A::VALUE4
    }
}
#[doc = "Write proxy for field `C1PM`"]
pub struct C1PM_W<'a> {
    w: &'a mut W,
}
impl<'a> C1PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C1PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CSG1 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1PM_A::VALUE1)
    }
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1PM_A::VALUE2)
    }
    #[doc = "CSG1 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C1PM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "CSG2 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C2PM_A {
    #[doc = "0: CSG2 unit is powered OFF"]
    VALUE1,
    #[doc = "1: CSG2 unit is set in Low Speed Mode"]
    VALUE2,
    #[doc = "3: CSG2 unit is set in High Speed Mode"]
    VALUE4,
}
impl From<C2PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C2PM_A) -> Self {
        match variant {
            C2PM_A::VALUE1 => 0,
            C2PM_A::VALUE2 => 1,
            C2PM_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `C2PM`"]
pub type C2PM_R = crate::R<u8, C2PM_A>;
impl C2PM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C2PM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C2PM_A::VALUE1),
            1 => Val(C2PM_A::VALUE2),
            3 => Val(C2PM_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C2PM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C2PM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C2PM_A::VALUE4
    }
}
#[doc = "Write proxy for field `C2PM`"]
pub struct C2PM_W<'a> {
    w: &'a mut W,
}
impl<'a> C2PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C2PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CSG2 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C2PM_A::VALUE1)
    }
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C2PM_A::VALUE2)
    }
    #[doc = "CSG2 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C2PM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `C0CD`"]
pub type C0CD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C0CD`"]
pub struct C0CD_W<'a> {
    w: &'a mut W,
}
impl<'a> C0CD_W<'a> {
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
#[doc = "Reader of field `C1CD`"]
pub type C1CD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C1CD`"]
pub struct C1CD_W<'a> {
    w: &'a mut W,
}
impl<'a> C1CD_W<'a> {
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
#[doc = "Reader of field `C2CD`"]
pub type C2CD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C2CD`"]
pub struct C2CD_W<'a> {
    w: &'a mut W,
}
impl<'a> C2CD_W<'a> {
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
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    pub fn c0pm(&self) -> C0PM_R {
        C0PM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    pub fn c1pm(&self) -> C1PM_R {
        C1PM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    pub fn c2pm(&self) -> C2PM_R {
        C2PM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    pub fn c0cd(&self) -> C0CD_R {
        C0CD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    pub fn c1cd(&self) -> C1CD_R {
        C1CD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    pub fn c2cd(&self) -> C2CD_R {
        C2CD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    pub fn c0pm(&mut self) -> C0PM_W {
        C0PM_W { w: self }
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    pub fn c1pm(&mut self) -> C1PM_W {
        C1PM_W { w: self }
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    pub fn c2pm(&mut self) -> C2PM_W {
        C2PM_W { w: self }
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    pub fn c0cd(&mut self) -> C0CD_W {
        C0CD_W { w: self }
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    pub fn c1cd(&mut self) -> C1CD_W {
        C1CD_W { w: self }
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    pub fn c2cd(&mut self) -> C2CD_W {
        C2CD_W { w: self }
    }
}
