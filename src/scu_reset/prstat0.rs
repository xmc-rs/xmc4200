#[doc = "Reader of register PRSTAT0"]
pub type R = crate::R<u32, super::PRSTAT0>;
#[doc = "VADC Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `VADCRS`"]
pub type VADCRS_R = crate::R<bool, VADCRS_A>;
impl VADCRS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "CCU40 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CCU40RS`"]
pub type CCU40RS_R = crate::R<bool, CCU40RS_A>;
impl CCU40RS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "CCU41 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CCU41RS`"]
pub type CCU41RS_R = crate::R<bool, CCU41RS_A>;
impl CCU41RS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "CCU80 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CCU80RS`"]
pub type CCU80RS_R = crate::R<bool, CCU80RS_A>;
impl CCU80RS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "POSIF0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `POSIF0RS`"]
pub type POSIF0RS_R = crate::R<bool, POSIF0RS_A>;
impl POSIF0RS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "USIC0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `USIC0RS`"]
pub type USIC0RS_R = crate::R<bool, USIC0RS_A>;
impl USIC0RS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "ERU1 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `ERU1RS`"]
pub type ERU1RS_R = crate::R<bool, ERU1RS_A>;
impl ERU1RS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "HRPWM0 Reset Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HRPWM0RS`"]
pub type HRPWM0RS_R = crate::R<bool, HRPWM0RS_A>;
impl HRPWM0RS_R {
    #[doc = r"Get enumerated values variant"]
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
        VADCRS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCU40 Reset Status"]
    #[inline(always)]
    pub fn ccu40rs(&self) -> CCU40RS_R {
        CCU40RS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CCU41 Reset Status"]
    #[inline(always)]
    pub fn ccu41rs(&self) -> CCU41RS_R {
        CCU41RS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CCU80 Reset Status"]
    #[inline(always)]
    pub fn ccu80rs(&self) -> CCU80RS_R {
        CCU80RS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - POSIF0 Reset Status"]
    #[inline(always)]
    pub fn posif0rs(&self) -> POSIF0RS_R {
        POSIF0RS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USIC0 Reset Status"]
    #[inline(always)]
    pub fn usic0rs(&self) -> USIC0RS_R {
        USIC0RS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ERU1 Reset Status"]
    #[inline(always)]
    pub fn eru1rs(&self) -> ERU1RS_R {
        ERU1RS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 23 - HRPWM0 Reset Status"]
    #[inline(always)]
    pub fn hrpwm0rs(&self) -> HRPWM0RS_R {
        HRPWM0RS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
