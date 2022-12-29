#[doc = "Register `GSEL` reader"]
pub struct R(crate::R<GSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GSEL` writer"]
pub struct W(crate::W<GSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C0SS` reader - Source selector 0 comparator set configuration"]
pub type C0SS_R = crate::FieldReader<u8, C0SS_A>;
#[doc = "Source selector 0 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C0SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C0SS_A> {
        match self.bits {
            0 => Some(C0SS_A::VALUE1),
            1 => Some(C0SS_A::VALUE2),
            2 => Some(C0SS_A::VALUE3),
            _ => None,
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
#[doc = "Field `C0SS` writer - Source selector 0 comparator set configuration"]
pub type C0SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, C0SS_A, 3, O>;
impl<'a, const O: u8> C0SS_W<'a, O> {
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
}
#[doc = "Field `C0CS` reader - Source selector 0 comparator clear configuration"]
pub type C0CS_R = crate::FieldReader<u8, C0CS_A>;
#[doc = "Source selector 0 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C0CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C0CS_A> {
        match self.bits {
            0 => Some(C0CS_A::VALUE1),
            1 => Some(C0CS_A::VALUE2),
            2 => Some(C0CS_A::VALUE3),
            _ => None,
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
#[doc = "Field `C0CS` writer - Source selector 0 comparator clear configuration"]
pub type C0CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, C0CS_A, 3, O>;
impl<'a, const O: u8> C0CS_W<'a, O> {
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
}
#[doc = "Field `S0M` reader - Source selector 0 set configuration"]
pub type S0M_R = crate::FieldReader<u8, S0M_A>;
#[doc = "Source selector 0 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl S0M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S0M_A> {
        match self.bits {
            0 => Some(S0M_A::VALUE1),
            1 => Some(S0M_A::VALUE2),
            _ => None,
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
#[doc = "Field `S0M` writer - Source selector 0 set configuration"]
pub type S0M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, S0M_A, 2, O>;
impl<'a, const O: u8> S0M_W<'a, O> {
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
}
#[doc = "Field `C0M` reader - Source selector 0 clear configuration"]
pub type C0M_R = crate::FieldReader<u8, C0M_A>;
#[doc = "Source selector 0 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C0M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C0M_A> {
        match self.bits {
            0 => Some(C0M_A::VALUE1),
            1 => Some(C0M_A::VALUE2),
            _ => None,
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
#[doc = "Field `C0M` writer - Source selector 0 clear configuration"]
pub type C0M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, C0M_A, 2, O>;
impl<'a, const O: u8> C0M_W<'a, O> {
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
}
#[doc = "Field `S0ES` reader - Source selector 0 set edge configuration"]
pub type S0ES_R = crate::FieldReader<u8, S0ES_A>;
#[doc = "Source selector 0 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl S0ES_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `S0ES` writer - Source selector 0 set edge configuration"]
pub type S0ES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GSEL_SPEC, u8, S0ES_A, 2, O>;
impl<'a, const O: u8> S0ES_W<'a, O> {
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
}
#[doc = "Field `C0ES` reader - Source selector 0 clear edge configuration"]
pub type C0ES_R = crate::FieldReader<u8, C0ES_A>;
#[doc = "Source selector 0 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C0ES_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `C0ES` writer - Source selector 0 clear edge configuration"]
pub type C0ES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GSEL_SPEC, u8, C0ES_A, 2, O>;
impl<'a, const O: u8> C0ES_W<'a, O> {
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
}
#[doc = "Field `C1SS` reader - Source selector 1 comparator set configuration"]
pub type C1SS_R = crate::FieldReader<u8, C1SS_A>;
#[doc = "Source selector 1 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C1SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C1SS_A> {
        match self.bits {
            0 => Some(C1SS_A::VALUE1),
            1 => Some(C1SS_A::VALUE2),
            2 => Some(C1SS_A::VALUE3),
            _ => None,
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
#[doc = "Field `C1SS` writer - Source selector 1 comparator set configuration"]
pub type C1SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, C1SS_A, 3, O>;
impl<'a, const O: u8> C1SS_W<'a, O> {
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
}
#[doc = "Field `C1CS` reader - Source selector 1 comparator clear configuration"]
pub type C1CS_R = crate::FieldReader<u8, C1CS_A>;
#[doc = "Source selector 1 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C1CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C1CS_A> {
        match self.bits {
            0 => Some(C1CS_A::VALUE1),
            1 => Some(C1CS_A::VALUE2),
            2 => Some(C1CS_A::VALUE3),
            _ => None,
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
#[doc = "Field `C1CS` writer - Source selector 1 comparator clear configuration"]
pub type C1CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, C1CS_A, 3, O>;
impl<'a, const O: u8> C1CS_W<'a, O> {
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
}
#[doc = "Field `S1M` reader - Source selector 1 set configuration"]
pub type S1M_R = crate::FieldReader<u8, S1M_A>;
#[doc = "Source selector 1 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl S1M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<S1M_A> {
        match self.bits {
            0 => Some(S1M_A::VALUE1),
            1 => Some(S1M_A::VALUE2),
            _ => None,
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
#[doc = "Field `S1M` writer - Source selector 1 set configuration"]
pub type S1M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, S1M_A, 2, O>;
impl<'a, const O: u8> S1M_W<'a, O> {
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
}
#[doc = "Field `C1M` reader - Source selector 1 clear configuration"]
pub type C1M_R = crate::FieldReader<u8, C1M_A>;
#[doc = "Source selector 1 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C1M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C1M_A> {
        match self.bits {
            0 => Some(C1M_A::VALUE1),
            1 => Some(C1M_A::VALUE2),
            _ => None,
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
#[doc = "Field `C1M` writer - Source selector 1 clear configuration"]
pub type C1M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GSEL_SPEC, u8, C1M_A, 2, O>;
impl<'a, const O: u8> C1M_W<'a, O> {
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
}
#[doc = "Field `S1ES` reader - Source selector 1 set edge configuration"]
pub type S1ES_R = crate::FieldReader<u8, S1ES_A>;
#[doc = "Source selector 1 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl S1ES_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `S1ES` writer - Source selector 1 set edge configuration"]
pub type S1ES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GSEL_SPEC, u8, S1ES_A, 2, O>;
impl<'a, const O: u8> S1ES_W<'a, O> {
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
}
#[doc = "Field `C1ES` reader - Source selector 1 clear edge configuration"]
pub type C1ES_R = crate::FieldReader<u8, C1ES_A>;
#[doc = "Source selector 1 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl C1ES_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `C1ES` writer - Source selector 1 clear edge configuration"]
pub type C1ES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GSEL_SPEC, u8, C1ES_A, 2, O>;
impl<'a, const O: u8> C1ES_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    pub fn c0ss(&self) -> C0SS_R {
        C0SS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    pub fn c0cs(&self) -> C0CS_R {
        C0CS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    pub fn s0m(&self) -> S0M_R {
        S0M_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    pub fn c0m(&self) -> C0M_R {
        C0M_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    pub fn s0es(&self) -> S0ES_R {
        S0ES_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    pub fn c0es(&self) -> C0ES_R {
        C0ES_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    pub fn c1ss(&self) -> C1SS_R {
        C1SS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    pub fn c1cs(&self) -> C1CS_R {
        C1CS_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    pub fn s1m(&self) -> S1M_R {
        S1M_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    pub fn c1m(&self) -> C1M_R {
        C1M_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    pub fn s1es(&self) -> S1ES_R {
        S1ES_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    pub fn c1es(&self) -> C1ES_R {
        C1ES_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0ss(&mut self) -> C0SS_W<0> {
        C0SS_W::new(self)
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0cs(&mut self) -> C0CS_W<3> {
        C0CS_W::new(self)
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s0m(&mut self) -> S0M_W<6> {
        S0M_W::new(self)
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0m(&mut self) -> C0M_W<8> {
        C0M_W::new(self)
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s0es(&mut self) -> S0ES_W<10> {
        S0ES_W::new(self)
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0es(&mut self) -> C0ES_W<12> {
        C0ES_W::new(self)
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1ss(&mut self) -> C1SS_W<16> {
        C1SS_W::new(self)
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1cs(&mut self) -> C1CS_W<19> {
        C1CS_W::new(self)
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s1m(&mut self) -> S1M_W<22> {
        S1M_W::new(self)
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1m(&mut self) -> C1M_W<24> {
        C1M_W::new(self)
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s1es(&mut self) -> S1ES_W<26> {
        S1ES_W::new(self)
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1es(&mut self) -> C1ES_W<28> {
        C1ES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC global control selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gsel](index.html) module"]
pub struct GSEL_SPEC;
impl crate::RegisterSpec for GSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gsel::R](R) reader structure"]
impl crate::Readable for GSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gsel::W](W) writer structure"]
impl crate::Writable for GSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GSEL to value 0"]
impl crate::Resettable for GSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
