#[doc = "Register `PRSTAT1` reader"]
pub struct R(crate::R<PRSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "LEDTS Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEDTSCU0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<LEDTSCU0RS_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` reader - LEDTS Reset Status"]
pub struct LEDTSCU0RS_R(crate::FieldReader<bool, LEDTSCU0RS_A>);
impl LEDTSCU0RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEDTSCU0RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDTSCU0RS_A {
        match self.bits {
            false => LEDTSCU0RS_A::VALUE1,
            true => LEDTSCU0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == LEDTSCU0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LEDTSCU0RS_A::VALUE2
    }
}
impl core::ops::Deref for LEDTSCU0RS_R {
    type Target = crate::FieldReader<bool, LEDTSCU0RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "MultiCAN Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCAN0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<MCAN0RS_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` reader - MultiCAN Reset Status"]
pub struct MCAN0RS_R(crate::FieldReader<bool, MCAN0RS_A>);
impl MCAN0RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCAN0RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCAN0RS_A {
        match self.bits {
            false => MCAN0RS_A::VALUE1,
            true => MCAN0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MCAN0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MCAN0RS_A::VALUE2
    }
}
impl core::ops::Deref for MCAN0RS_R {
    type Target = crate::FieldReader<bool, MCAN0RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DAC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<DACRS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` reader - DAC Reset Status"]
pub struct DACRS_R(crate::FieldReader<bool, DACRS_A>);
impl DACRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DACRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACRS_A {
        match self.bits {
            false => DACRS_A::VALUE1,
            true => DACRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DACRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DACRS_A::VALUE2
    }
}
impl core::ops::Deref for DACRS_R {
    type Target = crate::FieldReader<bool, DACRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "USIC1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<USIC1RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` reader - USIC1 Reset Status"]
pub struct USIC1RS_R(crate::FieldReader<bool, USIC1RS_A>);
impl USIC1RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        USIC1RS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC1RS_A {
        match self.bits {
            false => USIC1RS_A::VALUE1,
            true => USIC1RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == USIC1RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == USIC1RS_A::VALUE2
    }
}
impl core::ops::Deref for USIC1RS_R {
    type Target = crate::FieldReader<bool, USIC1RS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PORTS Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPORTSRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<PPORTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` reader - PORTS Reset Status"]
pub struct PPORTSRS_R(crate::FieldReader<bool, PPORTSRS_A>);
impl PPORTSRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPORTSRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPORTSRS_A {
        match self.bits {
            false => PPORTSRS_A::VALUE1,
            true => PPORTSRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PPORTSRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PPORTSRS_A::VALUE2
    }
}
impl core::ops::Deref for PPORTSRS_R {
    type Target = crate::FieldReader<bool, PPORTSRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - LEDTS Reset Status"]
    #[inline(always)]
    pub fn ledtscu0rs(&self) -> LEDTSCU0RS_R {
        LEDTSCU0RS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Reset Status"]
    #[inline(always)]
    pub fn mcan0rs(&self) -> MCAN0RS_R {
        MCAN0RS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DAC Reset Status"]
    #[inline(always)]
    pub fn dacrs(&self) -> DACRS_R {
        DACRS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USIC1 Reset Status"]
    #[inline(always)]
    pub fn usic1rs(&self) -> USIC1RS_R {
        USIC1RS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PORTS Reset Status"]
    #[inline(always)]
    pub fn pportsrs(&self) -> PPORTSRS_R {
        PPORTSRS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "RCU Peripheral 1 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat1](index.html) module"]
pub struct PRSTAT1_SPEC;
impl crate::RegisterSpec for PRSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstat1::R](R) reader structure"]
impl crate::Readable for PRSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTAT1 to value 0xb8"]
impl crate::Resettable for PRSTAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb8
    }
}
