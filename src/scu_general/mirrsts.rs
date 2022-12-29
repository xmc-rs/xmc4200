#[doc = "Register `MIRRSTS` reader"]
pub struct R(crate::R<MIRRSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIRRSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIRRSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIRRSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Write Status"]
pub type HDCLR_R = crate::BitReader<HDCLR_A>;
#[doc = "HDCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<HDCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl HDCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCLR_A {
        match self.bits {
            false => HDCLR_A::VALUE1,
            true => HDCLR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCLR_A::VALUE2
    }
}
#[doc = "Field `HDSET` reader - HDSET Mirror Register Write Status"]
pub type HDSET_R = crate::BitReader<HDSET_A>;
#[doc = "HDSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<HDSET_A> for bool {
    #[inline(always)]
    fn from(variant: HDSET_A) -> Self {
        variant as u8 != 0
    }
}
impl HDSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSET_A {
        match self.bits {
            false => HDSET_A::VALUE1,
            true => HDSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDSET_A::VALUE2
    }
}
#[doc = "Field `HDCR` reader - HDCR Mirror Register Write Status"]
pub type HDCR_R = crate::BitReader<HDCR_A>;
#[doc = "HDCR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<HDCR_A> for bool {
    #[inline(always)]
    fn from(variant: HDCR_A) -> Self {
        variant as u8 != 0
    }
}
impl HDCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDCR_A {
        match self.bits {
            false => HDCR_A::VALUE1,
            true => HDCR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCR_A::VALUE2
    }
}
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Write Status"]
pub type OSCSICTRL_R = crate::BitReader<OSCSICTRL_A>;
#[doc = "OSCSICTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<OSCSICTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSICTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCSICTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSICTRL_A {
        match self.bits {
            false => OSCSICTRL_A::VALUE1,
            true => OSCSICTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCSICTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCSICTRL_A::VALUE2
    }
}
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Write Status"]
pub type OSCULCTRL_R = crate::BitReader<OSCULCTRL_A>;
#[doc = "OSCULCTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<OSCULCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCULCTRL_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCULCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCULCTRL_A {
        match self.bits {
            false => OSCULCTRL_A::VALUE1,
            true => OSCULCTRL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCULCTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCULCTRL_A::VALUE2
    }
}
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Write Status"]
pub type RTC_CTR_R = crate::BitReader<RTC_CTR_A>;
#[doc = "RTC CTR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RTC_CTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CTR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_CTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CTR_A {
        match self.bits {
            false => RTC_CTR_A::VALUE1,
            true => RTC_CTR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_CTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_CTR_A::VALUE2
    }
}
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Write Status"]
pub type RTC_ATIM0_R = crate::BitReader<RTC_ATIM0_A>;
#[doc = "RTC ATIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RTC_ATIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_ATIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM0_A {
        match self.bits {
            false => RTC_ATIM0_A::VALUE1,
            true => RTC_ATIM0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE2
    }
}
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Write Status"]
pub type RTC_ATIM1_R = crate::BitReader<RTC_ATIM1_A>;
#[doc = "RTC ATIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RTC_ATIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_ATIM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_ATIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_ATIM1_A {
        match self.bits {
            false => RTC_ATIM1_A::VALUE1,
            true => RTC_ATIM1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE2
    }
}
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Write Status"]
pub type RTC_TIM0_R = crate::BitReader<RTC_TIM0_A>;
#[doc = "RTC TIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RTC_TIM0_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM0_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_TIM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM0_A {
        match self.bits {
            false => RTC_TIM0_A::VALUE1,
            true => RTC_TIM0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM0_A::VALUE2
    }
}
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Write Status"]
pub type RTC_TIM1_R = crate::BitReader<RTC_TIM1_A>;
#[doc = "RTC TIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RTC_TIM1_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_TIM1_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_TIM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_TIM1_A {
        match self.bits {
            false => RTC_TIM1_A::VALUE1,
            true => RTC_TIM1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM1_A::VALUE2
    }
}
#[doc = "Field `RMX` reader - Retention Memory Access Register Update Status"]
pub type RMX_R = crate::BitReader<RMX_A>;
#[doc = "Retention Memory Access Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RMX_A> for bool {
    #[inline(always)]
    fn from(variant: RMX_A) -> Self {
        variant as u8 != 0
    }
}
impl RMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMX_A {
        match self.bits {
            false => RMX_A::VALUE1,
            true => RMX_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RMX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RMX_A::VALUE2
    }
}
#[doc = "Field `RTC_MSKSR` reader - RTC MSKSSR Mirror Register Write Status"]
pub type RTC_MSKSR_R = crate::BitReader<RTC_MSKSR_A>;
#[doc = "RTC MSKSSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_MSKSR_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RTC_MSKSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_MSKSR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_MSKSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_MSKSR_A {
        match self.bits {
            false => RTC_MSKSR_A::VALUE1,
            true => RTC_MSKSR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_MSKSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_MSKSR_A::VALUE2
    }
}
#[doc = "Field `RTC_CLRSR` reader - RTC CLRSR Mirror Register Write Status"]
pub type RTC_CLRSR_R = crate::BitReader<RTC_CLRSR_A>;
#[doc = "RTC CLRSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CLRSR_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<RTC_CLRSR_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_CLRSR_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_CLRSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_CLRSR_A {
        match self.bits {
            false => RTC_CLRSR_A::VALUE1,
            true => RTC_CLRSR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_CLRSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_CLRSR_A::VALUE2
    }
}
#[doc = "Field `LPACCONF` reader - LPACCONF Mirror Register Write Interrupt Set"]
pub type LPACCONF_R = crate::BitReader<LPACCONF_A>;
#[doc = "LPACCONF Mirror Register Write Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCONF_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<LPACCONF_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCONF_A) -> Self {
        variant as u8 != 0
    }
}
impl LPACCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACCONF_A {
        match self.bits {
            false => LPACCONF_A::VALUE1,
            true => LPACCONF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCONF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCONF_A::VALUE2
    }
}
#[doc = "Field `LPACTH0` reader - LPACTH0 Mirror Register Write Interrupt Set"]
pub type LPACTH0_R = crate::BitReader<LPACTH0_A>;
#[doc = "LPACTH0 Mirror Register Write Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH0_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<LPACTH0_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH0_A) -> Self {
        variant as u8 != 0
    }
}
impl LPACTH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACTH0_A {
        match self.bits {
            false => LPACTH0_A::VALUE1,
            true => LPACTH0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH0_A::VALUE2
    }
}
#[doc = "Field `LPACTH1` reader - LPACTH1 Mirror Register Write Interrupt Set"]
pub type LPACTH1_R = crate::BitReader<LPACTH1_A>;
#[doc = "LPACTH1 Mirror Register Write Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH1_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<LPACTH1_A> for bool {
    #[inline(always)]
    fn from(variant: LPACTH1_A) -> Self {
        variant as u8 != 0
    }
}
impl LPACTH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACTH1_A {
        match self.bits {
            false => LPACTH1_A::VALUE1,
            true => LPACTH1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH1_A::VALUE2
    }
}
#[doc = "Field `LPACCLR` reader - LPACCLR Mirror Register Write Status"]
pub type LPACCLR_R = crate::BitReader<LPACCLR_A>;
#[doc = "LPACCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCLR_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<LPACCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl LPACCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACCLR_A {
        match self.bits {
            false => LPACCLR_A::VALUE1,
            true => LPACCLR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCLR_A::VALUE2
    }
}
#[doc = "Field `LPACSET` reader - LPACSET Mirror Register Write Status"]
pub type LPACSET_R = crate::BitReader<LPACSET_A>;
#[doc = "LPACSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACSET_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<LPACSET_A> for bool {
    #[inline(always)]
    fn from(variant: LPACSET_A) -> Self {
        variant as u8 != 0
    }
}
impl LPACSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPACSET_A {
        match self.bits {
            false => LPACSET_A::VALUE1,
            true => LPACSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACSET_A::VALUE2
    }
}
#[doc = "Field `HINTCLR` reader - HINTCLR Mirror Register Write Status"]
pub type HINTCLR_R = crate::BitReader<HINTCLR_A>;
#[doc = "HINTCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTCLR_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<HINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: HINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl HINTCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HINTCLR_A {
        match self.bits {
            false => HINTCLR_A::VALUE1,
            true => HINTCLR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTCLR_A::VALUE2
    }
}
#[doc = "Field `HINTSET` reader - HINTSET Mirror Register Write Status"]
pub type HINTSET_R = crate::BitReader<HINTSET_A>;
#[doc = "HINTSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTSET_A {
    #[doc = "0: Ready"]
    VALUE1 = 0,
    #[doc = "1: Busy"]
    VALUE2 = 1,
}
impl From<HINTSET_A> for bool {
    #[inline(always)]
    fn from(variant: HINTSET_A) -> Self {
        variant as u8 != 0
    }
}
impl HINTSET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HINTSET_A {
        match self.bits {
            false => HINTSET_A::VALUE1,
            true => HINTSET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTSET_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 1 - HDCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HDSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HDCR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - OSCSICTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - OSCULCTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC CTR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC ATIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC ATIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTC TIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTC TIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Retention Memory Access Register Update Status"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC MSKSSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_msksr(&self) -> RTC_MSKSR_R {
        RTC_MSKSR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC CLRSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_clrsr(&self) -> RTC_CLRSR_R {
        RTC_CLRSR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LPACCONF Mirror Register Write Interrupt Set"]
    #[inline(always)]
    pub fn lpacconf(&self) -> LPACCONF_R {
        LPACCONF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LPACTH0 Mirror Register Write Interrupt Set"]
    #[inline(always)]
    pub fn lpacth0(&self) -> LPACTH0_R {
        LPACTH0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPACTH1 Mirror Register Write Interrupt Set"]
    #[inline(always)]
    pub fn lpacth1(&self) -> LPACTH1_R {
        LPACTH1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - LPACCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn lpacclr(&self) -> LPACCLR_R {
        LPACCLR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPACSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn lpacset(&self) -> LPACSET_R {
        LPACSET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - HINTCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hintclr(&self) -> HINTCLR_R {
        HINTCLR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - HINTSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn hintset(&self) -> HINTSET_R {
        HINTSET_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Mirror Write Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mirrsts](index.html) module"]
pub struct MIRRSTS_SPEC;
impl crate::RegisterSpec for MIRRSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mirrsts::R](R) reader structure"]
impl crate::Readable for MIRRSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIRRSTS to value 0"]
impl crate::Resettable for MIRRSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
