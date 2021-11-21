#[doc = "Register `CSGFSG` reader"]
pub struct R(crate::R<CSGFSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGFSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGFSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGFSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DAC0 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RB_A {
    #[doc = "0: Slope generation is stopped."]
    VALUE1 = 0,
    #[doc = "1: Slope generation is running."]
    VALUE2 = 1,
}
impl From<S0RB_A> for bool {
    #[inline(always)]
    fn from(variant: S0RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RB` reader - DAC0 slope generation status"]
pub struct S0RB_R(crate::FieldReader<bool, S0RB_A>);
impl S0RB_R {
    pub(crate) fn new(bits: bool) -> Self {
        S0RB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0RB_A {
        match self.bits {
            false => S0RB_A::VALUE1,
            true => S0RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S0RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S0RB_A::VALUE2
    }
}
impl core::ops::Deref for S0RB_R {
    type Target = crate::FieldReader<bool, S0RB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CSG0 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P0RB_A {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running."]
    VALUE2 = 1,
}
impl From<P0RB_A> for bool {
    #[inline(always)]
    fn from(variant: P0RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0RB` reader - CSG0 prescaler status"]
pub struct P0RB_R(crate::FieldReader<bool, P0RB_A>);
impl P0RB_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0RB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P0RB_A {
        match self.bits {
            false => P0RB_A::VALUE1,
            true => P0RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == P0RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == P0RB_A::VALUE2
    }
}
impl core::ops::Deref for P0RB_R {
    type Target = crate::FieldReader<bool, P0RB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DAC1 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RB_A {
    #[doc = "0: Slope generation is stopped."]
    VALUE1 = 0,
    #[doc = "1: Slope generation is running."]
    VALUE2 = 1,
}
impl From<S1RB_A> for bool {
    #[inline(always)]
    fn from(variant: S1RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RB` reader - DAC1 slope generation status"]
pub struct S1RB_R(crate::FieldReader<bool, S1RB_A>);
impl S1RB_R {
    pub(crate) fn new(bits: bool) -> Self {
        S1RB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1RB_A {
        match self.bits {
            false => S1RB_A::VALUE1,
            true => S1RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S1RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S1RB_A::VALUE2
    }
}
impl core::ops::Deref for S1RB_R {
    type Target = crate::FieldReader<bool, S1RB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CSG1 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P1RB_A {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running."]
    VALUE2 = 1,
}
impl From<P1RB_A> for bool {
    #[inline(always)]
    fn from(variant: P1RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1RB` reader - CSG1 prescaler status"]
pub struct P1RB_R(crate::FieldReader<bool, P1RB_A>);
impl P1RB_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1RB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P1RB_A {
        match self.bits {
            false => P1RB_A::VALUE1,
            true => P1RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == P1RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == P1RB_A::VALUE2
    }
}
impl core::ops::Deref for P1RB_R {
    type Target = crate::FieldReader<bool, P1RB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DAC2 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S2RB_A {
    #[doc = "0: Slope generation is stopped."]
    VALUE1 = 0,
    #[doc = "1: Slope generation is running."]
    VALUE2 = 1,
}
impl From<S2RB_A> for bool {
    #[inline(always)]
    fn from(variant: S2RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RB` reader - DAC2 slope generation status"]
pub struct S2RB_R(crate::FieldReader<bool, S2RB_A>);
impl S2RB_R {
    pub(crate) fn new(bits: bool) -> Self {
        S2RB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S2RB_A {
        match self.bits {
            false => S2RB_A::VALUE1,
            true => S2RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == S2RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == S2RB_A::VALUE2
    }
}
impl core::ops::Deref for S2RB_R {
    type Target = crate::FieldReader<bool, S2RB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "CSG2 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum P2RB_A {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running."]
    VALUE2 = 1,
}
impl From<P2RB_A> for bool {
    #[inline(always)]
    fn from(variant: P2RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2RB` reader - CSG2 prescaler status"]
pub struct P2RB_R(crate::FieldReader<bool, P2RB_A>);
impl P2RB_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2RB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P2RB_A {
        match self.bits {
            false => P2RB_A::VALUE1,
            true => P2RB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == P2RB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == P2RB_A::VALUE2
    }
}
impl core::ops::Deref for P2RB_R {
    type Target = crate::FieldReader<bool, P2RB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 slope generation status"]
    #[inline(always)]
    pub fn s0rb(&self) -> S0RB_R {
        S0RB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CSG0 prescaler status"]
    #[inline(always)]
    pub fn p0rb(&self) -> P0RB_R {
        P0RB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAC1 slope generation status"]
    #[inline(always)]
    pub fn s1rb(&self) -> S1RB_R {
        S1RB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CSG1 prescaler status"]
    #[inline(always)]
    pub fn p1rb(&self) -> P1RB_R {
        P1RB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DAC2 slope generation status"]
    #[inline(always)]
    pub fn s2rb(&self) -> S2RB_R {
        S2RB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CSG2 prescaler status"]
    #[inline(always)]
    pub fn p2rb(&self) -> P2RB_R {
        P2RB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Global CSG slope/prescaler status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgfsg](index.html) module"]
pub struct CSGFSG_SPEC;
impl crate::RegisterSpec for CSGFSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgfsg::R](R) reader structure"]
impl crate::Readable for CSGFSG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSGFSG to value 0"]
impl crate::Resettable for CSGFSG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
