#[doc = "Reader of register GSEL"]
pub type R = crate::R<u32, super::GSEL>;
#[doc = "Writer for register GSEL"]
pub type W = crate::W<u32, super::GSEL>;
#[doc = "Register GSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::GSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Source selector 0 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C0SS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG1 unit can be used as set source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3 = 2,
}
impl From<C0SS_A> for u8 {
    #[inline(always)]
    fn from(variant: C0SS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C0SS`"]
pub type C0SS_R = crate::R<u8, C0SS_A>;
impl C0SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C0SS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C0SS_A::VALUE1),
            1 => Val(C0SS_A::VALUE2),
            2 => Val(C0SS_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0SS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0SS_A::VALUE3
    }
}
#[doc = "Write proxy for field `C0SS`"]
pub struct C0SS_W<'a> {
    w: &'a mut W,
}
impl<'a> C0SS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C0SS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0SS_A::VALUE1)
    }
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0SS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(C0SS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Source selector 0 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C0CS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG1 unit can be used as clear source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3 = 2,
}
impl From<C0CS_A> for u8 {
    #[inline(always)]
    fn from(variant: C0CS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C0CS`"]
pub type C0CS_R = crate::R<u8, C0CS_A>;
impl C0CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C0CS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C0CS_A::VALUE1),
            1 => Val(C0CS_A::VALUE2),
            2 => Val(C0CS_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0CS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0CS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0CS_A::VALUE3
    }
}
#[doc = "Write proxy for field `C0CS`"]
pub struct C0CS_W<'a> {
    w: &'a mut W,
}
impl<'a> C0CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C0CS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0CS_A::VALUE1)
    }
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0CS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(C0CS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Source selector 0 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S0M_A {
    #[doc = "0: Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    VALUE2 = 1,
}
impl From<S0M_A> for u8 {
    #[inline(always)]
    fn from(variant: S0M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `S0M`"]
pub type S0M_R = crate::R<u8, S0M_A>;
impl S0M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S0M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S0M_A::VALUE1),
            1 => Val(S0M_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0M_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0M_A::VALUE2
    }
}
#[doc = "Write proxy for field `S0M`"]
pub struct S0M_W<'a> {
    w: &'a mut W,
}
impl<'a> S0M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(S0M_A::VALUE1)
    }
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(S0M_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Source selector 0 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C0M_A {
    #[doc = "0: Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    VALUE2 = 1,
}
impl From<C0M_A> for u8 {
    #[inline(always)]
    fn from(variant: C0M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C0M`"]
pub type C0M_R = crate::R<u8, C0M_A>;
impl C0M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C0M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C0M_A::VALUE1),
            1 => Val(C0M_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0M_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0M_A::VALUE2
    }
}
#[doc = "Write proxy for field `C0M`"]
pub struct C0M_W<'a> {
    w: &'a mut W,
}
impl<'a> C0M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C0M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0M_A::VALUE1)
    }
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0M_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Source selector 0 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S0ES_A {
    #[doc = "0: Generation of the set signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Set signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<S0ES_A> for u8 {
    #[inline(always)]
    fn from(variant: S0ES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `S0ES`"]
pub type S0ES_R = crate::R<u8, S0ES_A>;
impl S0ES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0ES_A {
        match self.bits {
            0 => S0ES_A::VALUE1,
            1 => S0ES_A::VALUE2,
            2 => S0ES_A::VALUE3,
            3 => S0ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0ES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0ES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == S0ES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == S0ES_A::VALUE4
    }
}
#[doc = "Write proxy for field `S0ES`"]
pub struct S0ES_W<'a> {
    w: &'a mut W,
}
impl<'a> S0ES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0ES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(S0ES_A::VALUE1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(S0ES_A::VALUE2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(S0ES_A::VALUE3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(S0ES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Source selector 0 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C0ES_A {
    #[doc = "0: Generation of the clear signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Clear signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<C0ES_A> for u8 {
    #[inline(always)]
    fn from(variant: C0ES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C0ES`"]
pub type C0ES_R = crate::R<u8, C0ES_A>;
impl C0ES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C0ES_A {
        match self.bits {
            0 => C0ES_A::VALUE1,
            1 => C0ES_A::VALUE2,
            2 => C0ES_A::VALUE3,
            3 => C0ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0ES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0ES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0ES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C0ES_A::VALUE4
    }
}
#[doc = "Write proxy for field `C0ES`"]
pub struct C0ES_W<'a> {
    w: &'a mut W,
}
impl<'a> C0ES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C0ES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0ES_A::VALUE1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0ES_A::VALUE2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(C0ES_A::VALUE3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C0ES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Source selector 1 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C1SS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3 = 2,
}
impl From<C1SS_A> for u8 {
    #[inline(always)]
    fn from(variant: C1SS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C1SS`"]
pub type C1SS_R = crate::R<u8, C1SS_A>;
impl C1SS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C1SS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C1SS_A::VALUE1),
            1 => Val(C1SS_A::VALUE2),
            2 => Val(C1SS_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1SS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1SS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1SS_A::VALUE3
    }
}
#[doc = "Write proxy for field `C1SS`"]
pub struct C1SS_W<'a> {
    w: &'a mut W,
}
impl<'a> C1SS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C1SS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1SS_A::VALUE1)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1SS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(C1SS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Source selector 1 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C1CS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3 = 2,
}
impl From<C1CS_A> for u8 {
    #[inline(always)]
    fn from(variant: C1CS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C1CS`"]
pub type C1CS_R = crate::R<u8, C1CS_A>;
impl C1CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C1CS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C1CS_A::VALUE1),
            1 => Val(C1CS_A::VALUE2),
            2 => Val(C1CS_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1CS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1CS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1CS_A::VALUE3
    }
}
#[doc = "Write proxy for field `C1CS`"]
pub struct C1CS_W<'a> {
    w: &'a mut W,
}
impl<'a> C1CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C1CS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1CS_A::VALUE1)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1CS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(C1CS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Source selector 1 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S1M_A {
    #[doc = "0: Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    VALUE2 = 1,
}
impl From<S1M_A> for u8 {
    #[inline(always)]
    fn from(variant: S1M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `S1M`"]
pub type S1M_R = crate::R<u8, S1M_A>;
impl S1M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S1M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S1M_A::VALUE1),
            1 => Val(S1M_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1M_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1M_A::VALUE2
    }
}
#[doc = "Write proxy for field `S1M`"]
pub struct S1M_W<'a> {
    w: &'a mut W,
}
impl<'a> S1M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(S1M_A::VALUE1)
    }
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(S1M_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Source selector 1 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C1M_A {
    #[doc = "0: Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    VALUE2 = 1,
}
impl From<C1M_A> for u8 {
    #[inline(always)]
    fn from(variant: C1M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C1M`"]
pub type C1M_R = crate::R<u8, C1M_A>;
impl C1M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, C1M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(C1M_A::VALUE1),
            1 => Val(C1M_A::VALUE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1M_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1M_A::VALUE2
    }
}
#[doc = "Write proxy for field `C1M`"]
pub struct C1M_W<'a> {
    w: &'a mut W,
}
impl<'a> C1M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C1M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1M_A::VALUE1)
    }
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1M_A::VALUE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Source selector 1 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum S1ES_A {
    #[doc = "0: Generation of the set signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Set signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<S1ES_A> for u8 {
    #[inline(always)]
    fn from(variant: S1ES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `S1ES`"]
pub type S1ES_R = crate::R<u8, S1ES_A>;
impl S1ES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1ES_A {
        match self.bits {
            0 => S1ES_A::VALUE1,
            1 => S1ES_A::VALUE2,
            2 => S1ES_A::VALUE3,
            3 => S1ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1ES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1ES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == S1ES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == S1ES_A::VALUE4
    }
}
#[doc = "Write proxy for field `S1ES`"]
pub struct S1ES_W<'a> {
    w: &'a mut W,
}
impl<'a> S1ES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1ES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(S1ES_A::VALUE1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(S1ES_A::VALUE2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(S1ES_A::VALUE3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(S1ES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Source selector 1 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C1ES_A {
    #[doc = "0: Generation of the clear signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Clear signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<C1ES_A> for u8 {
    #[inline(always)]
    fn from(variant: C1ES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `C1ES`"]
pub type C1ES_R = crate::R<u8, C1ES_A>;
impl C1ES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1ES_A {
        match self.bits {
            0 => C1ES_A::VALUE1,
            1 => C1ES_A::VALUE2,
            2 => C1ES_A::VALUE3,
            3 => C1ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1ES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1ES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1ES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C1ES_A::VALUE4
    }
}
#[doc = "Write proxy for field `C1ES`"]
pub struct C1ES_W<'a> {
    w: &'a mut W,
}
impl<'a> C1ES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C1ES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1ES_A::VALUE1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1ES_A::VALUE2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(C1ES_A::VALUE3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C1ES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    pub fn c0ss(&self) -> C0SS_R {
        C0SS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    pub fn c0cs(&self) -> C0CS_R {
        C0CS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    pub fn s0m(&self) -> S0M_R {
        S0M_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    pub fn c0m(&self) -> C0M_R {
        C0M_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    pub fn s0es(&self) -> S0ES_R {
        S0ES_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    pub fn c0es(&self) -> C0ES_R {
        C0ES_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    pub fn c1ss(&self) -> C1SS_R {
        C1SS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    pub fn c1cs(&self) -> C1CS_R {
        C1CS_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    pub fn s1m(&self) -> S1M_R {
        S1M_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    pub fn c1m(&self) -> C1M_R {
        C1M_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    pub fn s1es(&self) -> S1ES_R {
        S1ES_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    pub fn c1es(&self) -> C1ES_R {
        C1ES_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    pub fn c0ss(&mut self) -> C0SS_W {
        C0SS_W { w: self }
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    pub fn c0cs(&mut self) -> C0CS_W {
        C0CS_W { w: self }
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    pub fn s0m(&mut self) -> S0M_W {
        S0M_W { w: self }
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    pub fn c0m(&mut self) -> C0M_W {
        C0M_W { w: self }
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    pub fn s0es(&mut self) -> S0ES_W {
        S0ES_W { w: self }
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    pub fn c0es(&mut self) -> C0ES_W {
        C0ES_W { w: self }
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    pub fn c1ss(&mut self) -> C1SS_W {
        C1SS_W { w: self }
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    pub fn c1cs(&mut self) -> C1CS_W {
        C1CS_W { w: self }
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    pub fn s1m(&mut self) -> S1M_W {
        S1M_W { w: self }
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    pub fn c1m(&mut self) -> C1M_W {
        C1M_W { w: self }
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    pub fn s1es(&mut self) -> S1ES_W {
        S1ES_W { w: self }
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    pub fn c1es(&mut self) -> C1ES_W {
        C1ES_W { w: self }
    }
}
