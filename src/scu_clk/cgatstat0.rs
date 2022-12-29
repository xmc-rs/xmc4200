#[doc = "Register `CGATSTAT0` reader"]
pub struct R(crate::R<CGATSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGATSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGATSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGATSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VADC` reader - VADC Gating Status"]
pub type VADC_R = crate::BitReader<VADC_A>;
#[doc = "VADC Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADC_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<VADC_A> for bool {
    #[inline(always)]
    fn from(variant: VADC_A) -> Self {
        variant as u8 != 0
    }
}
impl VADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADC_A {
        match self.bits {
            false => VADC_A::VALUE1,
            true => VADC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VADC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VADC_A::VALUE2
    }
}
#[doc = "Field `CCU40` reader - CCU40 Gating Status"]
pub type CCU40_R = crate::BitReader<CCU40_A>;
#[doc = "CCU40 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU40_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40_A {
        match self.bits {
            false => CCU40_A::VALUE1,
            true => CCU40_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU40_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU40_A::VALUE2
    }
}
#[doc = "Field `CCU41` reader - CCU41 Gating Status"]
pub type CCU41_R = crate::BitReader<CCU41_A>;
#[doc = "CCU41 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU41_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU41_A {
        match self.bits {
            false => CCU41_A::VALUE1,
            true => CCU41_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU41_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU41_A::VALUE2
    }
}
#[doc = "Field `CCU80` reader - CCU80 Gating Status"]
pub type CCU80_R = crate::BitReader<CCU80_A>;
#[doc = "CCU80 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<CCU80_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU80_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU80_A {
        match self.bits {
            false => CCU80_A::VALUE1,
            true => CCU80_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU80_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU80_A::VALUE2
    }
}
#[doc = "Field `POSIF0` reader - POSIF0 Gating Status"]
pub type POSIF0_R = crate::BitReader<POSIF0_A>;
#[doc = "POSIF0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<POSIF0_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl POSIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF0_A {
        match self.bits {
            false => POSIF0_A::VALUE1,
            true => POSIF0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSIF0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSIF0_A::VALUE2
    }
}
#[doc = "Field `USIC0` reader - USIC0 Gating Status"]
pub type USIC0_R = crate::BitReader<USIC0_A>;
#[doc = "USIC0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<USIC0_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0_A) -> Self {
        variant as u8 != 0
    }
}
impl USIC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0_A {
        match self.bits {
            false => USIC0_A::VALUE1,
            true => USIC0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0_A::VALUE2
    }
}
#[doc = "Field `ERU1` reader - ERU1 Gating Status"]
pub type ERU1_R = crate::BitReader<ERU1_A>;
#[doc = "ERU1 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<ERU1_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU1_A {
        match self.bits {
            false => ERU1_A::VALUE1,
            true => ERU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERU1_A::VALUE2
    }
}
#[doc = "Field `HRPWM0` reader - HRPWM0 Gating Status"]
pub type HRPWM0_R = crate::BitReader<HRPWM0_A>;
#[doc = "HRPWM0 Gating Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRPWM0_A {
    #[doc = "0: Gating de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Gating asserted"]
    VALUE2 = 1,
}
impl From<HRPWM0_A> for bool {
    #[inline(always)]
    fn from(variant: HRPWM0_A) -> Self {
        variant as u8 != 0
    }
}
impl HRPWM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRPWM0_A {
        match self.bits {
            false => HRPWM0_A::VALUE1,
            true => HRPWM0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRPWM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRPWM0_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - VADC Gating Status"]
    #[inline(always)]
    pub fn vadc(&self) -> VADC_R {
        VADC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Status"]
    #[inline(always)]
    pub fn ccu40(&self) -> CCU40_R {
        CCU40_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCU41 Gating Status"]
    #[inline(always)]
    pub fn ccu41(&self) -> CCU41_R {
        CCU41_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - CCU80 Gating Status"]
    #[inline(always)]
    pub fn ccu80(&self) -> CCU80_R {
        CCU80_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Gating Status"]
    #[inline(always)]
    pub fn posif0(&self) -> POSIF0_R {
        POSIF0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - USIC0 Gating Status"]
    #[inline(always)]
    pub fn usic0(&self) -> USIC0_R {
        USIC0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ERU1 Gating Status"]
    #[inline(always)]
    pub fn eru1(&self) -> ERU1_R {
        ERU1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - HRPWM0 Gating Status"]
    #[inline(always)]
    pub fn hrpwm0(&self) -> HRPWM0_R {
        HRPWM0_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Peripheral 0 Clock Gating Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgatstat0](index.html) module"]
pub struct CGATSTAT0_SPEC;
impl crate::RegisterSpec for CGATSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgatstat0::R](R) reader structure"]
impl crate::Readable for CGATSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CGATSTAT0 to value 0"]
impl crate::Resettable for CGATSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
