#[doc = "Register `SRRAW` reader"]
pub type R = crate::R<SRRAW_SPEC>;
#[doc = "Field `PRWARN` reader - WDT pre-warning Interrupt Status Before Masking"]
pub type PRWARN_R = crate::BitReader<PRWARN_A>;
#[doc = "WDT pre-warning Interrupt Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRWARN_A {
    #[doc = "0: Inactive"]
    VALUE1 = 0,
    #[doc = "1: Active"]
    VALUE2 = 1,
}
impl From<PRWARN_A> for bool {
    #[inline(always)]
    fn from(variant: PRWARN_A) -> Self {
        variant as u8 != 0
    }
}
impl PRWARN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRWARN_A {
        match self.bits {
            false => PRWARN_A::VALUE1,
            true => PRWARN_A::VALUE2,
        }
    }
    #[doc = "Inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PRWARN_A::VALUE1
    }
    #[doc = "Active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PRWARN_A::VALUE2
    }
}
#[doc = "Field `PI` reader - RTC Raw Periodic Interrupt Status Before Masking"]
pub type PI_R = crate::BitReader;
#[doc = "Field `AI` reader - RTC Raw Alarm Interrupt Status Before Masking"]
pub type AI_R = crate::BitReader;
#[doc = "Field `DLROVR` reader - DLR Request Overrun Interrupt Status Before Masking"]
pub type DLROVR_R = crate::BitReader;
#[doc = "Field `LPACCR` reader - LPACLR Mirror Register Update Status Before Masking"]
pub type LPACCR_R = crate::BitReader<LPACCR_A>;
#[doc = "LPACLR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCR_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
    VALUE2 = 1,
}
impl From<LPACCR_A> for bool {
    #[inline(always)]
    fn from(variant: LPACCR_A) -> Self {
        variant as u8 != 0
    }
}
impl LPACCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACCR_A {
        match self.bits {
            false => LPACCR_A::VALUE1,
            true => LPACCR_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCR_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCR_A::VALUE2
    }
}
#[doc = "Field `LPACTH0` reader - LPACTH0 Mirror Register Update Status Before Masking"]
pub type LPACTH0_R = crate::BitReader<LPACTH0_A>;
#[doc = "LPACTH0 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH0_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> LPACTH0_A {
        match self.bits {
            false => LPACTH0_A::VALUE1,
            true => LPACTH0_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH0_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH0_A::VALUE2
    }
}
#[doc = "Field `LPACTH1` reader - LPACTH1 Mirror Register Update Status Before Masking"]
pub type LPACTH1_R = crate::BitReader<LPACTH1_A>;
#[doc = "LPACTH1 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACTH1_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> LPACTH1_A {
        match self.bits {
            false => LPACTH1_A::VALUE1,
            true => LPACTH1_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACTH1_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACTH1_A::VALUE2
    }
}
#[doc = "Field `LPACST` reader - LPACST Mirror Register Update Status Before Masking"]
pub type LPACST_R = crate::BitReader<LPACST_A>;
#[doc = "LPACST Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACST_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
    VALUE2 = 1,
}
impl From<LPACST_A> for bool {
    #[inline(always)]
    fn from(variant: LPACST_A) -> Self {
        variant as u8 != 0
    }
}
impl LPACST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPACST_A {
        match self.bits {
            false => LPACST_A::VALUE1,
            true => LPACST_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACST_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACST_A::VALUE2
    }
}
#[doc = "Field `LPACCLR` reader - LPACCLR Mirror Register Update Status Before Masking"]
pub type LPACCLR_R = crate::BitReader<LPACCLR_A>;
#[doc = "LPACCLR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACCLR_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> LPACCLR_A {
        match self.bits {
            false => LPACCLR_A::VALUE1,
            true => LPACCLR_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACCLR_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACCLR_A::VALUE2
    }
}
#[doc = "Field `LPACSET` reader - LPACSET Mirror Register Update Status Before Masking"]
pub type LPACSET_R = crate::BitReader<LPACSET_A>;
#[doc = "LPACSET Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPACSET_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> LPACSET_A {
        match self.bits {
            false => LPACSET_A::VALUE1,
            true => LPACSET_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LPACSET_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LPACSET_A::VALUE2
    }
}
#[doc = "Field `HINTST` reader - HINTST Mirror Register Update Status Before Masking"]
pub type HINTST_R = crate::BitReader<HINTST_A>;
#[doc = "HINTST Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTST_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
    VALUE2 = 1,
}
impl From<HINTST_A> for bool {
    #[inline(always)]
    fn from(variant: HINTST_A) -> Self {
        variant as u8 != 0
    }
}
impl HINTST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HINTST_A {
        match self.bits {
            false => HINTST_A::VALUE1,
            true => HINTST_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTST_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTST_A::VALUE2
    }
}
#[doc = "Field `HINTCLR` reader - HINTCLR Mirror Register Update Status Before Masking"]
pub type HINTCLR_R = crate::BitReader<HINTCLR_A>;
#[doc = "HINTCLR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTCLR_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> HINTCLR_A {
        match self.bits {
            false => HINTCLR_A::VALUE1,
            true => HINTCLR_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTCLR_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTCLR_A::VALUE2
    }
}
#[doc = "Field `HINTSET` reader - HINTSET Mirror Register Update Status Before Masking"]
pub type HINTSET_R = crate::BitReader<HINTSET_A>;
#[doc = "HINTSET Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HINTSET_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> HINTSET_A {
        match self.bits {
            false => HINTSET_A::VALUE1,
            true => HINTSET_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HINTSET_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HINTSET_A::VALUE2
    }
}
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Update Status Before Masking"]
pub type HDCLR_R = crate::BitReader<HDCLR_A>;
#[doc = "HDCLR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCLR_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> HDCLR_A {
        match self.bits {
            false => HDCLR_A::VALUE1,
            true => HDCLR_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCLR_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCLR_A::VALUE2
    }
}
#[doc = "Field `HDSET` reader - HDSET Mirror Register Update Status Before Masking"]
pub type HDSET_R = crate::BitReader<HDSET_A>;
#[doc = "HDSET Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSET_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> HDSET_A {
        match self.bits {
            false => HDSET_A::VALUE1,
            true => HDSET_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDSET_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDSET_A::VALUE2
    }
}
#[doc = "Field `HDCR` reader - HDCR Mirror Register Update Status Before Masking"]
pub type HDCR_R = crate::BitReader<HDCR_A>;
#[doc = "HDCR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDCR_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> HDCR_A {
        match self.bits {
            false => HDCR_A::VALUE1,
            true => HDCR_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HDCR_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HDCR_A::VALUE2
    }
}
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Update Status Before Masking"]
pub type OSCSICTRL_R = crate::BitReader<OSCSICTRL_A>;
#[doc = "OSCSICTRL Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCSICTRL_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> OSCSICTRL_A {
        match self.bits {
            false => OSCSICTRL_A::VALUE1,
            true => OSCSICTRL_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCSICTRL_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCSICTRL_A::VALUE2
    }
}
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Update Status Before Masking"]
pub type OSCULCTRL_R = crate::BitReader<OSCULCTRL_A>;
#[doc = "OSCULCTRL Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSCULCTRL_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> OSCULCTRL_A {
        match self.bits {
            false => OSCULCTRL_A::VALUE1,
            true => OSCULCTRL_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OSCULCTRL_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OSCULCTRL_A::VALUE2
    }
}
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Update Status Before Masking"]
pub type RTC_CTR_R = crate::BitReader<RTC_CTR_A>;
#[doc = "RTC CTR Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_CTR_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> RTC_CTR_A {
        match self.bits {
            false => RTC_CTR_A::VALUE1,
            true => RTC_CTR_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_CTR_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_CTR_A::VALUE2
    }
}
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Update Status Before Masking"]
pub type RTC_ATIM0_R = crate::BitReader<RTC_ATIM0_A>;
#[doc = "RTC ATIM0 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM0_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> RTC_ATIM0_A {
        match self.bits {
            false => RTC_ATIM0_A::VALUE1,
            true => RTC_ATIM0_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM0_A::VALUE2
    }
}
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Update Status Before Masking"]
pub type RTC_ATIM1_R = crate::BitReader<RTC_ATIM1_A>;
#[doc = "RTC ATIM1 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_ATIM1_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> RTC_ATIM1_A {
        match self.bits {
            false => RTC_ATIM1_A::VALUE1,
            true => RTC_ATIM1_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_ATIM1_A::VALUE2
    }
}
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Update Before Masking Status"]
pub type RTC_TIM0_R = crate::BitReader<RTC_TIM0_A>;
#[doc = "RTC TIM0 Mirror Register Update Before Masking Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM0_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> RTC_TIM0_A {
        match self.bits {
            false => RTC_TIM0_A::VALUE1,
            true => RTC_TIM0_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM0_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM0_A::VALUE2
    }
}
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Update Status Before Masking"]
pub type RTC_TIM1_R = crate::BitReader<RTC_TIM1_A>;
#[doc = "RTC TIM1 Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_TIM1_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> RTC_TIM1_A {
        match self.bits {
            false => RTC_TIM1_A::VALUE1,
            true => RTC_TIM1_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTC_TIM1_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTC_TIM1_A::VALUE2
    }
}
#[doc = "Field `RMX` reader - Retention Memory Mirror Register Update Status Before Masking"]
pub type RMX_R = crate::BitReader<RMX_A>;
#[doc = "Retention Memory Mirror Register Update Status Before Masking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMX_A {
    #[doc = "0: Not updated"]
    VALUE1 = 0,
    #[doc = "1: Update completed"]
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
    pub const fn variant(&self) -> RMX_A {
        match self.bits {
            false => RMX_A::VALUE1,
            true => RMX_A::VALUE2,
        }
    }
    #[doc = "Not updated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RMX_A::VALUE1
    }
    #[doc = "Update completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RMX_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - WDT pre-warning Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn prwarn(&self) -> PRWARN_R {
        PRWARN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Raw Periodic Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Raw Alarm Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DLR Request Overrun Interrupt Status Before Masking"]
    #[inline(always)]
    pub fn dlrovr(&self) -> DLROVR_R {
        DLROVR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - LPACLR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn lpaccr(&self) -> LPACCR_R {
        LPACCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPACTH0 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn lpacth0(&self) -> LPACTH0_R {
        LPACTH0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LPACTH1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn lpacth1(&self) -> LPACTH1_R {
        LPACTH1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPACST Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn lpacst(&self) -> LPACST_R {
        LPACST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPACCLR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn lpacclr(&self) -> LPACCLR_R {
        LPACCLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPACSET Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn lpacset(&self) -> LPACSET_R {
        LPACSET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HINTST Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hintst(&self) -> HINTST_R {
        HINTST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HINTCLR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hintclr(&self) -> HINTCLR_R {
        HINTCLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HINTSET Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hintset(&self) -> HINTSET_R {
        HINTSET_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - HDCLR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HDSET Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HDCR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OSCSICTRL Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - OSCULCTRL Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RTC CTR Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RTC ATIM0 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - RTC ATIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RTC TIM0 Mirror Register Update Before Masking Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RTC TIM1 Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Retention Memory Mirror Register Update Status Before Masking"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "SCU Raw Service Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srraw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRRAW_SPEC;
impl crate::RegisterSpec for SRRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srraw::R`](R) reader structure"]
impl crate::Readable for SRRAW_SPEC {}
#[doc = "`reset()` method sets SRRAW to value 0"]
impl crate::Resettable for SRRAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
