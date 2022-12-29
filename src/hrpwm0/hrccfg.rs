#[doc = "Register `HRCCFG` reader"]
pub struct R(crate::R<HRCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRCCFG` writer"]
pub struct W(crate::W<HRCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRCCFG_SPEC>;
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
impl From<crate::W<HRCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRCPM` reader - High resolution channels power mode"]
pub type HRCPM_R = crate::BitReader<HRCPM_A>;
#[doc = "High resolution channels power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRCPM_A {
    #[doc = "0: High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    VALUE1 = 0,
    #[doc = "1: High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    VALUE2 = 1,
}
impl From<HRCPM_A> for bool {
    #[inline(always)]
    fn from(variant: HRCPM_A) -> Self {
        variant as u8 != 0
    }
}
impl HRCPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRCPM_A {
        match self.bits {
            false => HRCPM_A::VALUE1,
            true => HRCPM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRCPM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRCPM_A::VALUE2
    }
}
#[doc = "Field `HRCPM` writer - High resolution channels power mode"]
pub type HRCPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, HRCPM_A, O>;
impl<'a, const O: u8> HRCPM_W<'a, O> {
    #[doc = "High resolution generation logic is OFF. It is not possible to generate high resolution signals throughout any of the high resolution channels, HRCy."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRCPM_A::VALUE1)
    }
    #[doc = "High resolution generation logic is ON. In this mode it is possible to generate a high resolution signal placement with the HRCy subunits."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRCPM_A::VALUE2)
    }
}
#[doc = "Field `HRC0E` reader - HRC0 high resolution enable"]
pub type HRC0E_R = crate::BitReader<HRC0E_A>;
#[doc = "HRC0 high resolution enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRC0E_A {
    #[doc = "0: HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2 = 1,
}
impl From<HRC0E_A> for bool {
    #[inline(always)]
    fn from(variant: HRC0E_A) -> Self {
        variant as u8 != 0
    }
}
impl HRC0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRC0E_A {
        match self.bits {
            false => HRC0E_A::VALUE1,
            true => HRC0E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRC0E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRC0E_A::VALUE2
    }
}
#[doc = "Field `HRC0E` writer - HRC0 high resolution enable"]
pub type HRC0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, HRC0E_A, O>;
impl<'a, const O: u8> HRC0E_W<'a, O> {
    #[doc = "HRC0 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC0E_A::VALUE1)
    }
    #[doc = "HRC0 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC0E_A::VALUE2)
    }
}
#[doc = "Field `HRC1E` reader - HRC1 high resolution channel enable"]
pub type HRC1E_R = crate::BitReader<HRC1E_A>;
#[doc = "HRC1 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRC1E_A {
    #[doc = "0: HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2 = 1,
}
impl From<HRC1E_A> for bool {
    #[inline(always)]
    fn from(variant: HRC1E_A) -> Self {
        variant as u8 != 0
    }
}
impl HRC1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRC1E_A {
        match self.bits {
            false => HRC1E_A::VALUE1,
            true => HRC1E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRC1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRC1E_A::VALUE2
    }
}
#[doc = "Field `HRC1E` writer - HRC1 high resolution channel enable"]
pub type HRC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, HRC1E_A, O>;
impl<'a, const O: u8> HRC1E_W<'a, O> {
    #[doc = "HRC1 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC1E_A::VALUE1)
    }
    #[doc = "HRC1 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC1E_A::VALUE2)
    }
}
#[doc = "Field `HRC2E` reader - HRC2 high resolution channel enable"]
pub type HRC2E_R = crate::BitReader<HRC2E_A>;
#[doc = "HRC2 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRC2E_A {
    #[doc = "0: HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2 = 1,
}
impl From<HRC2E_A> for bool {
    #[inline(always)]
    fn from(variant: HRC2E_A) -> Self {
        variant as u8 != 0
    }
}
impl HRC2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRC2E_A {
        match self.bits {
            false => HRC2E_A::VALUE1,
            true => HRC2E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRC2E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRC2E_A::VALUE2
    }
}
#[doc = "Field `HRC2E` writer - HRC2 high resolution channel enable"]
pub type HRC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, HRC2E_A, O>;
impl<'a, const O: u8> HRC2E_W<'a, O> {
    #[doc = "HRC2 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC2E_A::VALUE1)
    }
    #[doc = "HRC2 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC2E_A::VALUE2)
    }
}
#[doc = "Field `HRC3E` reader - HRC3 high resolution channel enable"]
pub type HRC3E_R = crate::BitReader<HRC3E_A>;
#[doc = "HRC3 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRC3E_A {
    #[doc = "0: HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    VALUE2 = 1,
}
impl From<HRC3E_A> for bool {
    #[inline(always)]
    fn from(variant: HRC3E_A) -> Self {
        variant as u8 != 0
    }
}
impl HRC3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRC3E_A {
        match self.bits {
            false => HRC3E_A::VALUE1,
            true => HRC3E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRC3E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRC3E_A::VALUE2
    }
}
#[doc = "Field `HRC3E` writer - HRC3 high resolution channel enable"]
pub type HRC3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, HRC3E_A, O>;
impl<'a, const O: u8> HRC3E_W<'a, O> {
    #[doc = "HRC3 High Resolution Path is disabled. In this mode, is not possible to use the High Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRC3E_A::VALUE1)
    }
    #[doc = "HRC3 High Resolution Path is enabled. In this mode it is possible to generate a high resolution PWM signal if HRCPM = 1#."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRC3E_A::VALUE2)
    }
}
#[doc = "Field `CLKC` reader - Clock information control"]
pub type CLKC_R = crate::FieldReader<u8, CLKC_A>;
#[doc = "Clock information control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKC_A {
    #[doc = "0: No clock frequency is selected"]
    VALUE1 = 0,
    #[doc = "1: Module clock frequency is 180 MHz"]
    VALUE2 = 1,
    #[doc = "2: Module clock frequency is 120 MHz"]
    VALUE3 = 2,
    #[doc = "3: Module clock frequency is 80 MHz"]
    VALUE4 = 3,
}
impl From<CLKC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKC_A) -> Self {
        variant as _
    }
}
impl CLKC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKC_A> {
        match self.bits {
            0 => Some(CLKC_A::VALUE1),
            1 => Some(CLKC_A::VALUE2),
            2 => Some(CLKC_A::VALUE3),
            3 => Some(CLKC_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CLKC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CLKC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CLKC_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CLKC_A::VALUE4
    }
}
#[doc = "Field `CLKC` writer - Clock information control"]
pub type CLKC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRCCFG_SPEC, u8, CLKC_A, 3, O>;
impl<'a, const O: u8> CLKC_W<'a, O> {
    #[doc = "No clock frequency is selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CLKC_A::VALUE1)
    }
    #[doc = "Module clock frequency is 180 MHz"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CLKC_A::VALUE2)
    }
    #[doc = "Module clock frequency is 120 MHz"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CLKC_A::VALUE3)
    }
    #[doc = "Module clock frequency is 80 MHz"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CLKC_A::VALUE4)
    }
}
#[doc = "Field `LRC0E` reader - HRC0 low resolution channel enable"]
pub type LRC0E_R = crate::BitReader<LRC0E_A>;
#[doc = "HRC0 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRC0E_A {
    #[doc = "0: HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2 = 1,
}
impl From<LRC0E_A> for bool {
    #[inline(always)]
    fn from(variant: LRC0E_A) -> Self {
        variant as u8 != 0
    }
}
impl LRC0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRC0E_A {
        match self.bits {
            false => LRC0E_A::VALUE1,
            true => LRC0E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LRC0E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LRC0E_A::VALUE2
    }
}
#[doc = "Field `LRC0E` writer - HRC0 low resolution channel enable"]
pub type LRC0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, LRC0E_A, O>;
impl<'a, const O: u8> LRC0E_W<'a, O> {
    #[doc = "HRC0 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC0 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC0E_A::VALUE1)
    }
    #[doc = "HRC0 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC0E_A::VALUE2)
    }
}
#[doc = "Field `LRC1E` reader - HRC1 low resolution channel enable"]
pub type LRC1E_R = crate::BitReader<LRC1E_A>;
#[doc = "HRC1 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRC1E_A {
    #[doc = "0: HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2 = 1,
}
impl From<LRC1E_A> for bool {
    #[inline(always)]
    fn from(variant: LRC1E_A) -> Self {
        variant as u8 != 0
    }
}
impl LRC1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRC1E_A {
        match self.bits {
            false => LRC1E_A::VALUE1,
            true => LRC1E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LRC1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LRC1E_A::VALUE2
    }
}
#[doc = "Field `LRC1E` writer - HRC1 low resolution channel enable"]
pub type LRC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, LRC1E_A, O>;
impl<'a, const O: u8> LRC1E_W<'a, O> {
    #[doc = "HRC1 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC1 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC1E_A::VALUE1)
    }
    #[doc = "HRC1 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC1E_A::VALUE2)
    }
}
#[doc = "Field `LRC2E` reader - HRC2 low resolution channel enable"]
pub type LRC2E_R = crate::BitReader<LRC2E_A>;
#[doc = "HRC2 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRC2E_A {
    #[doc = "0: HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2 = 1,
}
impl From<LRC2E_A> for bool {
    #[inline(always)]
    fn from(variant: LRC2E_A) -> Self {
        variant as u8 != 0
    }
}
impl LRC2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRC2E_A {
        match self.bits {
            false => LRC2E_A::VALUE1,
            true => LRC2E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LRC2E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LRC2E_A::VALUE2
    }
}
#[doc = "Field `LRC2E` writer - HRC2 low resolution channel enable"]
pub type LRC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, LRC2E_A, O>;
impl<'a, const O: u8> LRC2E_W<'a, O> {
    #[doc = "HRC2 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC2 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC2E_A::VALUE1)
    }
    #[doc = "HRC2 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC2E_A::VALUE2)
    }
}
#[doc = "Field `LRC3E` reader - HRC3 low resolution channel enable"]
pub type LRC3E_R = crate::BitReader<LRC3E_A>;
#[doc = "HRC3 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRC3E_A {
    #[doc = "0: HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    VALUE1 = 0,
    #[doc = "1: HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    VALUE2 = 1,
}
impl From<LRC3E_A> for bool {
    #[inline(always)]
    fn from(variant: LRC3E_A) -> Self {
        variant as u8 != 0
    }
}
impl LRC3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRC3E_A {
        match self.bits {
            false => LRC3E_A::VALUE1,
            true => LRC3E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LRC3E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LRC3E_A::VALUE2
    }
}
#[doc = "Field `LRC3E` writer - HRC3 low resolution channel enable"]
pub type LRC3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, HRCCFG_SPEC, LRC3E_A, O>;
impl<'a, const O: u8> LRC3E_W<'a, O> {
    #[doc = "HRC3 Low Resolution Path is disabled. In this mode, is not possible to use the Low Resolution Path inside of HRC3 to generate an output PWM signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LRC3E_A::VALUE1)
    }
    #[doc = "HRC3 Low Resolution Path is enabled. In this mode it is possible to generate a an output PWM signal via the Low Resolution Path."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LRC3E_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline(always)]
    pub fn hrcpm(&self) -> HRCPM_R {
        HRCPM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline(always)]
    pub fn hrc0e(&self) -> HRC0E_R {
        HRC0E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc1e(&self) -> HRC1E_R {
        HRC1E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc2e(&self) -> HRC2E_R {
        HRC2E_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc3e(&self) -> HRC3E_R {
        HRC3E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline(always)]
    pub fn clkc(&self) -> CLKC_R {
        CLKC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc0e(&self) -> LRC0E_R {
        LRC0E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc1e(&self) -> LRC1E_R {
        LRC1E_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc2e(&self) -> LRC2E_R {
        LRC2E_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc3e(&self) -> LRC3E_R {
        LRC3E_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline(always)]
    #[must_use]
    pub fn hrcpm(&mut self) -> HRCPM_W<0> {
        HRCPM_W::new(self)
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc0e(&mut self) -> HRC0E_W<4> {
        HRC0E_W::new(self)
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc1e(&mut self) -> HRC1E_W<5> {
        HRC1E_W::new(self)
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc2e(&mut self) -> HRC2E_W<6> {
        HRC2E_W::new(self)
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn hrc3e(&mut self) -> HRC3E_W<7> {
        HRC3E_W::new(self)
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline(always)]
    #[must_use]
    pub fn clkc(&mut self) -> CLKC_W<16> {
        CLKC_W::new(self)
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc0e(&mut self) -> LRC0E_W<20> {
        LRC0E_W::new(self)
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc1e(&mut self) -> LRC1E_W<21> {
        LRC1E_W::new(self)
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc2e(&mut self) -> LRC2E_W<22> {
        LRC2E_W::new(self)
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn lrc3e(&mut self) -> LRC3E_W<23> {
        LRC3E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global HRC configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrccfg](index.html) module"]
pub struct HRCCFG_SPEC;
impl crate::RegisterSpec for HRCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrccfg::R](R) reader structure"]
impl crate::Readable for HRCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrccfg::W](W) writer structure"]
impl crate::Writable for HRCCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HRCCFG to value 0"]
impl crate::Resettable for HRCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
