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
#[doc = "Field `LEDTSCU0RS` reader - LEDTS Reset Status"]
pub type LEDTSCU0RS_R = crate::BitReader<LEDTSCU0RS_A>;
#[doc = "LEDTS Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl LEDTSCU0RS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == LEDTSCU0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEDTSCU0RS_A::VALUE2
    }
}
#[doc = "Field `MCAN0RS` reader - MultiCAN Reset Status"]
pub type MCAN0RS_R = crate::BitReader<MCAN0RS_A>;
#[doc = "MultiCAN Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MCAN0RS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == MCAN0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCAN0RS_A::VALUE2
    }
}
#[doc = "Field `DACRS` reader - DAC Reset Status"]
pub type DACRS_R = crate::BitReader<DACRS_A>;
#[doc = "DAC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DACRS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DACRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DACRS_A::VALUE2
    }
}
#[doc = "Field `USIC1RS` reader - USIC1 Reset Status"]
pub type USIC1RS_R = crate::BitReader<USIC1RS_A>;
#[doc = "USIC1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl USIC1RS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == USIC1RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1RS_A::VALUE2
    }
}
#[doc = "Field `PPORTSRS` reader - PORTS Reset Status"]
pub type PPORTSRS_R = crate::BitReader<PPORTSRS_A>;
#[doc = "PORTS Reset Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl PPORTSRS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == PPORTSRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPORTSRS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 3 - LEDTS Reset Status"]
    #[inline(always)]
    pub fn ledtscu0rs(&self) -> LEDTSCU0RS_R {
        LEDTSCU0RS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MultiCAN Reset Status"]
    #[inline(always)]
    pub fn mcan0rs(&self) -> MCAN0RS_R {
        MCAN0RS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Reset Status"]
    #[inline(always)]
    pub fn dacrs(&self) -> DACRS_R {
        DACRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - USIC1 Reset Status"]
    #[inline(always)]
    pub fn usic1rs(&self) -> USIC1RS_R {
        USIC1RS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - PORTS Reset Status"]
    #[inline(always)]
    pub fn pportsrs(&self) -> PPORTSRS_R {
        PPORTSRS_R::new(((self.bits >> 9) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0xb8;
}
