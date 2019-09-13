#[doc = "Reader of register TSEL"]
pub type R = crate::R<u32, super::TSEL>;
#[doc = "Writer for register TSEL"]
pub type W = crate::W<u32, super::TSEL>;
#[doc = "Register TSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::TSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Source Selector 0 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL0_A {
    #[doc = "0: Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1,
    #[doc = "1: Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2,
    #[doc = "2: Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3,
    #[doc = "3: Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4,
}
impl From<TSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL0_A) -> Self {
        match variant {
            TSEL0_A::VALUE1 => 0,
            TSEL0_A::VALUE2 => 1,
            TSEL0_A::VALUE3 => 2,
            TSEL0_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `TSEL0`"]
pub type TSEL0_R = crate::R<u8, TSEL0_A>;
impl TSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSEL0_A::VALUE1),
            1 => Val(TSEL0_A::VALUE2),
            2 => Val(TSEL0_A::VALUE3),
            3 => Val(TSEL0_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSEL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSEL0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSEL0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSEL0_A::VALUE4
    }
}
#[doc = "Write proxy for field `TSEL0`"]
pub struct TSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE1)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE2)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE3)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Source Selector 1 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEL1_A {
    #[doc = "0: Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1,
    #[doc = "1: Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2,
    #[doc = "2: Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3,
    #[doc = "3: Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        match variant {
            TSEL1_A::VALUE1 => 0,
            TSEL1_A::VALUE2 => 1,
            TSEL1_A::VALUE3 => 2,
            TSEL1_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `TSEL1`"]
pub type TSEL1_R = crate::R<u8, TSEL1_A>;
impl TSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSEL1_A::VALUE1),
            1 => Val(TSEL1_A::VALUE2),
            2 => Val(TSEL1_A::VALUE3),
            3 => Val(TSEL1_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSEL1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSEL1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSEL1_A::VALUE4
    }
}
#[doc = "Write proxy for field `TSEL1`"]
pub struct TSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE1)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE2)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE3)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Source selector 0 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS0E_A {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    VALUE1,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    VALUE2,
}
impl From<TS0E_A> for bool {
    #[inline(always)]
    fn from(variant: TS0E_A) -> Self {
        match variant {
            TS0E_A::VALUE1 => false,
            TS0E_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TS0E`"]
pub type TS0E_R = crate::R<bool, TS0E_A>;
impl TS0E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS0E_A {
        match self.bits {
            false => TS0E_A::VALUE1,
            true => TS0E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS0E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS0E_A::VALUE2
    }
}
#[doc = "Write proxy for field `TS0E`"]
pub struct TS0E_W<'a> {
    w: &'a mut W,
}
impl<'a> TS0E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS0E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TS0E_A::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TS0E_A::VALUE2)
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
#[doc = "Source selector 1 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TS1E_A {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    VALUE1,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    VALUE2,
}
impl From<TS1E_A> for bool {
    #[inline(always)]
    fn from(variant: TS1E_A) -> Self {
        match variant {
            TS1E_A::VALUE1 => false,
            TS1E_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TS1E`"]
pub type TS1E_R = crate::R<bool, TS1E_A>;
impl TS1E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS1E_A {
        match self.bits {
            false => TS1E_A::VALUE1,
            true => TS1E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS1E_A::VALUE2
    }
}
#[doc = "Write proxy for field `TS1E`"]
pub struct TS1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS1E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TS1E_A::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TS1E_A::VALUE2)
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
impl R {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    pub fn tsel0(&self) -> TSEL0_R {
        TSEL0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    pub fn ts0e(&self) -> TS0E_R {
        TS0E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    pub fn ts1e(&self) -> TS1E_R {
        TS1E_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    pub fn tsel0(&mut self) -> TSEL0_W {
        TSEL0_W { w: self }
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W {
        TSEL1_W { w: self }
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    pub fn ts0e(&mut self) -> TS0E_W {
        TS0E_W { w: self }
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    pub fn ts1e(&mut self) -> TS1E_W {
        TS1E_W { w: self }
    }
}
