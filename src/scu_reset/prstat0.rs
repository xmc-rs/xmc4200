#[doc = "Register `PRSTAT0` reader"]
pub struct R(crate::R<PRSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VADCRS` reader - VADC Reset Status"]
pub type VADCRS_R = crate::BitReader<VADCRS_A>;
#[doc = "VADC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADCRS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<VADCRS_A> for bool {
    #[inline(always)]
    fn from(variant: VADCRS_A) -> Self {
        variant as u8 != 0
    }
}
impl VADCRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VADCRS_A {
        match self.bits {
            false => VADCRS_A::VALUE1,
            true => VADCRS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VADCRS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VADCRS_A::VALUE2
    }
}
#[doc = "Field `CCU40RS` reader - CCU40 Reset Status"]
pub type CCU40RS_R = crate::BitReader<CCU40RS_A>;
#[doc = "CCU40 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU40RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU40RS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU40RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU40RS_A {
        match self.bits {
            false => CCU40RS_A::VALUE1,
            true => CCU40RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU40RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU40RS_A::VALUE2
    }
}
#[doc = "Field `CCU41RS` reader - CCU41 Reset Status"]
pub type CCU41RS_R = crate::BitReader<CCU41RS_A>;
#[doc = "CCU41 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU41RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU41RS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU41RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU41RS_A {
        match self.bits {
            false => CCU41RS_A::VALUE1,
            true => CCU41RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU41RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU41RS_A::VALUE2
    }
}
#[doc = "Field `CCU80RS` reader - CCU80 Reset Status"]
pub type CCU80RS_R = crate::BitReader<CCU80RS_A>;
#[doc = "CCU80 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<CCU80RS_A> for bool {
    #[inline(always)]
    fn from(variant: CCU80RS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCU80RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCU80RS_A {
        match self.bits {
            false => CCU80RS_A::VALUE1,
            true => CCU80RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCU80RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCU80RS_A::VALUE2
    }
}
#[doc = "Field `POSIF0RS` reader - POSIF0 Reset Status"]
pub type POSIF0RS_R = crate::BitReader<POSIF0RS_A>;
#[doc = "POSIF0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<POSIF0RS_A> for bool {
    #[inline(always)]
    fn from(variant: POSIF0RS_A) -> Self {
        variant as u8 != 0
    }
}
impl POSIF0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSIF0RS_A {
        match self.bits {
            false => POSIF0RS_A::VALUE1,
            true => POSIF0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POSIF0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POSIF0RS_A::VALUE2
    }
}
#[doc = "Field `USIC0RS` reader - USIC0 Reset Status"]
pub type USIC0RS_R = crate::BitReader<USIC0RS_A>;
#[doc = "USIC0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<USIC0RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0RS_A) -> Self {
        variant as u8 != 0
    }
}
impl USIC0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0RS_A {
        match self.bits {
            false => USIC0RS_A::VALUE1,
            true => USIC0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0RS_A::VALUE2
    }
}
#[doc = "Field `ERU1RS` reader - ERU1 Reset Status"]
pub type ERU1RS_R = crate::BitReader<ERU1RS_A>;
#[doc = "ERU1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<ERU1RS_A> for bool {
    #[inline(always)]
    fn from(variant: ERU1RS_A) -> Self {
        variant as u8 != 0
    }
}
impl ERU1RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERU1RS_A {
        match self.bits {
            false => ERU1RS_A::VALUE1,
            true => ERU1RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ERU1RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ERU1RS_A::VALUE2
    }
}
#[doc = "Field `HRPWM0RS` reader - HRPWM0 Reset Status"]
pub type HRPWM0RS_R = crate::BitReader<HRPWM0RS_A>;
#[doc = "HRPWM0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRPWM0RS_A {
    #[doc = "0: Reset de-asserted"]
    VALUE1 = 0,
    #[doc = "1: Reset asserted"]
    VALUE2 = 1,
}
impl From<HRPWM0RS_A> for bool {
    #[inline(always)]
    fn from(variant: HRPWM0RS_A) -> Self {
        variant as u8 != 0
    }
}
impl HRPWM0RS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRPWM0RS_A {
        match self.bits {
            false => HRPWM0RS_A::VALUE1,
            true => HRPWM0RS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRPWM0RS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRPWM0RS_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - VADC Reset Status"]
    #[inline(always)]
    pub fn vadcrs(&self) -> VADCRS_R {
        VADCRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline(always)]
    pub fn ccu40rs(&self) -> CCU40RS_R {
        CCU40RS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline(always)]
    pub fn ccu41rs(&self) -> CCU41RS_R {
        CCU41RS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline(always)]
    pub fn ccu80rs(&self) -> CCU80RS_R {
        CCU80RS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Reset Status"]
    #[inline(always)]
    pub fn posif0rs(&self) -> POSIF0RS_R {
        POSIF0RS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline(always)]
    pub fn usic0rs(&self) -> USIC0RS_R {
        USIC0RS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline(always)]
    pub fn eru1rs(&self) -> ERU1RS_R {
        ERU1RS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - HRPWM0 Reset Status"]
    #[inline(always)]
    pub fn hrpwm0rs(&self) -> HRPWM0RS_R {
        HRPWM0RS_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "RCU Peripheral 0 Reset Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prstat0](index.html) module"]
pub struct PRSTAT0_SPEC;
impl crate::RegisterSpec for PRSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prstat0::R](R) reader structure"]
impl crate::Readable for PRSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSTAT0 to value 0x0081_0a8d"]
impl crate::Resettable for PRSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0081_0a8d;
}
