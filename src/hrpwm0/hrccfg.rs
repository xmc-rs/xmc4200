#[doc = "Reader of register HRCCFG"]
pub type R = crate::R<u32, super::HRCCFG>;
#[doc = "Writer for register HRCCFG"]
pub type W = crate::W<u32, super::HRCCFG>;
#[doc = "Register HRCCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::HRCCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "High resolution channels power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HRCPM`"]
pub type HRCPM_R = crate::R<bool, HRCPM_A>;
impl HRCPM_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `HRCPM`"]
pub struct HRCPM_W<'a> {
    w: &'a mut W,
}
impl<'a> HRCPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRCPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "HRC0 high resolution enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HRC0E`"]
pub type HRC0E_R = crate::R<bool, HRC0E_A>;
impl HRC0E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `HRC0E`"]
pub struct HRC0E_W<'a> {
    w: &'a mut W,
}
impl<'a> HRC0E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRC0E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "HRC1 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HRC1E`"]
pub type HRC1E_R = crate::R<bool, HRC1E_A>;
impl HRC1E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `HRC1E`"]
pub struct HRC1E_W<'a> {
    w: &'a mut W,
}
impl<'a> HRC1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRC1E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "HRC2 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HRC2E`"]
pub type HRC2E_R = crate::R<bool, HRC2E_A>;
impl HRC2E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `HRC2E`"]
pub struct HRC2E_W<'a> {
    w: &'a mut W,
}
impl<'a> HRC2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRC2E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "HRC3 high resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `HRC3E`"]
pub type HRC3E_R = crate::R<bool, HRC3E_A>;
impl HRC3E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `HRC3E`"]
pub struct HRC3E_W<'a> {
    w: &'a mut W,
}
impl<'a> HRC3E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRC3E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Clock information control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `CLKC`"]
pub type CLKC_R = crate::R<u8, CLKC_A>;
impl CLKC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKC_A::VALUE1),
            1 => Val(CLKC_A::VALUE2),
            2 => Val(CLKC_A::VALUE3),
            3 => Val(CLKC_A::VALUE4),
            i => Res(i),
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
#[doc = "Write proxy for field `CLKC`"]
pub struct CLKC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "HRC0 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `LRC0E`"]
pub type LRC0E_R = crate::R<bool, LRC0E_A>;
impl LRC0E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LRC0E`"]
pub struct LRC0E_W<'a> {
    w: &'a mut W,
}
impl<'a> LRC0E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRC0E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "HRC1 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `LRC1E`"]
pub type LRC1E_R = crate::R<bool, LRC1E_A>;
impl LRC1E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LRC1E`"]
pub struct LRC1E_W<'a> {
    w: &'a mut W,
}
impl<'a> LRC1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRC1E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "HRC2 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `LRC2E`"]
pub type LRC2E_R = crate::R<bool, LRC2E_A>;
impl LRC2E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LRC2E`"]
pub struct LRC2E_W<'a> {
    w: &'a mut W,
}
impl<'a> LRC2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRC2E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "HRC3 low resolution channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `LRC3E`"]
pub type LRC3E_R = crate::R<bool, LRC3E_A>;
impl LRC3E_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LRC3E`"]
pub struct LRC3E_W<'a> {
    w: &'a mut W,
}
impl<'a> LRC3E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRC3E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline(always)]
    pub fn hrcpm(&self) -> HRCPM_R {
        HRCPM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline(always)]
    pub fn hrc0e(&self) -> HRC0E_R {
        HRC0E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc1e(&self) -> HRC1E_R {
        HRC1E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc2e(&self) -> HRC2E_R {
        HRC2E_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc3e(&self) -> HRC3E_R {
        HRC3E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline(always)]
    pub fn clkc(&self) -> CLKC_R {
        CLKC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc0e(&self) -> LRC0E_R {
        LRC0E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc1e(&self) -> LRC1E_R {
        LRC1E_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc2e(&self) -> LRC2E_R {
        LRC2E_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc3e(&self) -> LRC3E_R {
        LRC3E_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High resolution channels power mode"]
    #[inline(always)]
    pub fn hrcpm(&mut self) -> HRCPM_W {
        HRCPM_W { w: self }
    }
    #[doc = "Bit 4 - HRC0 high resolution enable"]
    #[inline(always)]
    pub fn hrc0e(&mut self) -> HRC0E_W {
        HRC0E_W { w: self }
    }
    #[doc = "Bit 5 - HRC1 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc1e(&mut self) -> HRC1E_W {
        HRC1E_W { w: self }
    }
    #[doc = "Bit 6 - HRC2 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc2e(&mut self) -> HRC2E_W {
        HRC2E_W { w: self }
    }
    #[doc = "Bit 7 - HRC3 high resolution channel enable"]
    #[inline(always)]
    pub fn hrc3e(&mut self) -> HRC3E_W {
        HRC3E_W { w: self }
    }
    #[doc = "Bits 16:18 - Clock information control"]
    #[inline(always)]
    pub fn clkc(&mut self) -> CLKC_W {
        CLKC_W { w: self }
    }
    #[doc = "Bit 20 - HRC0 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc0e(&mut self) -> LRC0E_W {
        LRC0E_W { w: self }
    }
    #[doc = "Bit 21 - HRC1 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc1e(&mut self) -> LRC1E_W {
        LRC1E_W { w: self }
    }
    #[doc = "Bit 22 - HRC2 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc2e(&mut self) -> LRC2E_W {
        LRC2E_W { w: self }
    }
    #[doc = "Bit 23 - HRC3 low resolution channel enable"]
    #[inline(always)]
    pub fn lrc3e(&mut self) -> LRC3E_W {
        LRC3E_W { w: self }
    }
}
