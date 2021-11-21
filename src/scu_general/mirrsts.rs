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
#[doc = "HDCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HDCLR` reader - HDCLR Mirror Register Write Status"]
pub struct HDCLR_R(crate::FieldReader<bool, HDCLR_A>);
impl HDCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HDCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HDCLR_A::VALUE2
    }
}
impl core::ops::Deref for HDCLR_R {
    type Target = crate::FieldReader<bool, HDCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HDSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HDSET` reader - HDSET Mirror Register Write Status"]
pub struct HDSET_R(crate::FieldReader<bool, HDSET_A>);
impl HDSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HDSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HDSET_A::VALUE2
    }
}
impl core::ops::Deref for HDSET_R {
    type Target = crate::FieldReader<bool, HDSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HDCR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HDCR` reader - HDCR Mirror Register Write Status"]
pub struct HDCR_R(crate::FieldReader<bool, HDCR_A>);
impl HDCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HDCR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HDCR_A::VALUE2
    }
}
impl core::ops::Deref for HDCR_R {
    type Target = crate::FieldReader<bool, HDCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OSCSICTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `OSCSICTRL` reader - OSCSICTRL Mirror Register Write Status"]
pub struct OSCSICTRL_R(crate::FieldReader<bool, OSCSICTRL_A>);
impl OSCSICTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCSICTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == OSCSICTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OSCSICTRL_A::VALUE2
    }
}
impl core::ops::Deref for OSCSICTRL_R {
    type Target = crate::FieldReader<bool, OSCSICTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "OSCULCTRL Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `OSCULCTRL` reader - OSCULCTRL Mirror Register Write Status"]
pub struct OSCULCTRL_R(crate::FieldReader<bool, OSCULCTRL_A>);
impl OSCULCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCULCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == OSCULCTRL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OSCULCTRL_A::VALUE2
    }
}
impl core::ops::Deref for OSCULCTRL_R {
    type Target = crate::FieldReader<bool, OSCULCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC CTR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_CTR` reader - RTC CTR Mirror Register Write Status"]
pub struct RTC_CTR_R(crate::FieldReader<bool, RTC_CTR_A>);
impl RTC_CTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_CTR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTC_CTR_A::VALUE2
    }
}
impl core::ops::Deref for RTC_CTR_R {
    type Target = crate::FieldReader<bool, RTC_CTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC ATIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_ATIM0` reader - RTC ATIM0 Mirror Register Write Status"]
pub struct RTC_ATIM0_R(crate::FieldReader<bool, RTC_ATIM0_A>);
impl RTC_ATIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ATIM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_ATIM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTC_ATIM0_A::VALUE2
    }
}
impl core::ops::Deref for RTC_ATIM0_R {
    type Target = crate::FieldReader<bool, RTC_ATIM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC ATIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_ATIM1` reader - RTC ATIM1 Mirror Register Write Status"]
pub struct RTC_ATIM1_R(crate::FieldReader<bool, RTC_ATIM1_A>);
impl RTC_ATIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ATIM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_ATIM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTC_ATIM1_A::VALUE2
    }
}
impl core::ops::Deref for RTC_ATIM1_R {
    type Target = crate::FieldReader<bool, RTC_ATIM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC TIM0 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_TIM0` reader - RTC TIM0 Mirror Register Write Status"]
pub struct RTC_TIM0_R(crate::FieldReader<bool, RTC_TIM0_A>);
impl RTC_TIM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TIM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_TIM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTC_TIM0_A::VALUE2
    }
}
impl core::ops::Deref for RTC_TIM0_R {
    type Target = crate::FieldReader<bool, RTC_TIM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC TIM1 Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_TIM1` reader - RTC TIM1 Mirror Register Write Status"]
pub struct RTC_TIM1_R(crate::FieldReader<bool, RTC_TIM1_A>);
impl RTC_TIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_TIM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_TIM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTC_TIM1_A::VALUE2
    }
}
impl core::ops::Deref for RTC_TIM1_R {
    type Target = crate::FieldReader<bool, RTC_TIM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Retention Memory Access Register Update Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RMX` reader - Retention Memory Access Register Update Status"]
pub struct RMX_R(crate::FieldReader<bool, RMX_A>);
impl RMX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMX_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RMX_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RMX_A::VALUE2
    }
}
impl core::ops::Deref for RMX_R {
    type Target = crate::FieldReader<bool, RMX_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC MSKSSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_MSKSR` reader - RTC MSKSSR Mirror Register Write Status"]
pub struct RTC_MSKSR_R(crate::FieldReader<bool, RTC_MSKSR_A>);
impl RTC_MSKSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_MSKSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_MSKSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTC_MSKSR_A::VALUE2
    }
}
impl core::ops::Deref for RTC_MSKSR_R {
    type Target = crate::FieldReader<bool, RTC_MSKSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTC CLRSR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `RTC_CLRSR` reader - RTC CLRSR Mirror Register Write Status"]
pub struct RTC_CLRSR_R(crate::FieldReader<bool, RTC_CLRSR_A>);
impl RTC_CLRSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CLRSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == RTC_CLRSR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == RTC_CLRSR_A::VALUE2
    }
}
impl core::ops::Deref for RTC_CLRSR_R {
    type Target = crate::FieldReader<bool, RTC_CLRSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPACCONF Mirror Register Write Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LPACCONF` reader - LPACCONF Mirror Register Write Interrupt Set"]
pub struct LPACCONF_R(crate::FieldReader<bool, LPACCONF_A>);
impl LPACCONF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPACCONF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LPACCONF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LPACCONF_A::VALUE2
    }
}
impl core::ops::Deref for LPACCONF_R {
    type Target = crate::FieldReader<bool, LPACCONF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPACTH0 Mirror Register Write Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LPACTH0` reader - LPACTH0 Mirror Register Write Interrupt Set"]
pub struct LPACTH0_R(crate::FieldReader<bool, LPACTH0_A>);
impl LPACTH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPACTH0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LPACTH0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LPACTH0_A::VALUE2
    }
}
impl core::ops::Deref for LPACTH0_R {
    type Target = crate::FieldReader<bool, LPACTH0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPACTH1 Mirror Register Write Interrupt Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LPACTH1` reader - LPACTH1 Mirror Register Write Interrupt Set"]
pub struct LPACTH1_R(crate::FieldReader<bool, LPACTH1_A>);
impl LPACTH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPACTH1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LPACTH1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LPACTH1_A::VALUE2
    }
}
impl core::ops::Deref for LPACTH1_R {
    type Target = crate::FieldReader<bool, LPACTH1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPACCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LPACCLR` reader - LPACCLR Mirror Register Write Status"]
pub struct LPACCLR_R(crate::FieldReader<bool, LPACCLR_A>);
impl LPACCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPACCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LPACCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LPACCLR_A::VALUE2
    }
}
impl core::ops::Deref for LPACCLR_R {
    type Target = crate::FieldReader<bool, LPACCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LPACSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `LPACSET` reader - LPACSET Mirror Register Write Status"]
pub struct LPACSET_R(crate::FieldReader<bool, LPACSET_A>);
impl LPACSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPACSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == LPACSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == LPACSET_A::VALUE2
    }
}
impl core::ops::Deref for LPACSET_R {
    type Target = crate::FieldReader<bool, LPACSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HINTCLR Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HINTCLR` reader - HINTCLR Mirror Register Write Status"]
pub struct HINTCLR_R(crate::FieldReader<bool, HINTCLR_A>);
impl HINTCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HINTCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HINTCLR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HINTCLR_A::VALUE2
    }
}
impl core::ops::Deref for HINTCLR_R {
    type Target = crate::FieldReader<bool, HINTCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HINTSET Mirror Register Write Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HINTSET` reader - HINTSET Mirror Register Write Status"]
pub struct HINTSET_R(crate::FieldReader<bool, HINTSET_A>);
impl HINTSET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HINTSET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == HINTSET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HINTSET_A::VALUE2
    }
}
impl core::ops::Deref for HINTSET_R {
    type Target = crate::FieldReader<bool, HINTSET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - HDCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdclr(&self) -> HDCLR_R {
        HDCLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HDSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdset(&self) -> HDSET_R {
        HDSET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HDCR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hdcr(&self) -> HDCR_R {
        HDCR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OSCSICTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn oscsictrl(&self) -> OSCSICTRL_R {
        OSCSICTRL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - OSCULCTRL Mirror Register Write Status"]
    #[inline(always)]
    pub fn osculctrl(&self) -> OSCULCTRL_R {
        OSCULCTRL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTC CTR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_ctr(&self) -> RTC_CTR_R {
        RTC_CTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC ATIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim0(&self) -> RTC_ATIM0_R {
        RTC_ATIM0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC ATIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_atim1(&self) -> RTC_ATIM1_R {
        RTC_ATIM1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RTC TIM0 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim0(&self) -> RTC_TIM0_R {
        RTC_TIM0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RTC TIM1 Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_tim1(&self) -> RTC_TIM1_R {
        RTC_TIM1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Retention Memory Access Register Update Status"]
    #[inline(always)]
    pub fn rmx(&self) -> RMX_R {
        RMX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RTC MSKSSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_msksr(&self) -> RTC_MSKSR_R {
        RTC_MSKSR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RTC CLRSR Mirror Register Write Status"]
    #[inline(always)]
    pub fn rtc_clrsr(&self) -> RTC_CLRSR_R {
        RTC_CLRSR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPACCONF Mirror Register Write Interrupt Set"]
    #[inline(always)]
    pub fn lpacconf(&self) -> LPACCONF_R {
        LPACCONF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LPACTH0 Mirror Register Write Interrupt Set"]
    #[inline(always)]
    pub fn lpacth0(&self) -> LPACTH0_R {
        LPACTH0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LPACTH1 Mirror Register Write Interrupt Set"]
    #[inline(always)]
    pub fn lpacth1(&self) -> LPACTH1_R {
        LPACTH1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPACCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn lpacclr(&self) -> LPACCLR_R {
        LPACCLR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPACSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn lpacset(&self) -> LPACSET_R {
        LPACSET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - HINTCLR Mirror Register Write Status"]
    #[inline(always)]
    pub fn hintclr(&self) -> HINTCLR_R {
        HINTCLR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - HINTSET Mirror Register Write Status"]
    #[inline(always)]
    pub fn hintset(&self) -> HINTSET_R {
        HINTSET_R::new(((self.bits >> 24) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
