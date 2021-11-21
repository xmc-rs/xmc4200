#[doc = "Register `GC` reader"]
pub struct R(crate::R<GC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GC` writer"]
pub struct W(crate::W<GC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GC_SPEC>;
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
impl From<crate::W<GC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HRCy high resolution mode configuration for source selector 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HRM0_A {
    #[doc = "0: Rising edge high resolution signal positioning enabled"]
    VALUE1 = 0,
    #[doc = "1: Falling edge high resolution signal positioning enabled"]
    VALUE2 = 1,
    #[doc = "2: Both edges high resolution signal positioning is enabled"]
    VALUE3 = 2,
    #[doc = "3: No high resolution positioning"]
    VALUE4 = 3,
}
impl From<HRM0_A> for u8 {
    #[inline(always)]
    fn from(variant: HRM0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HRM0` reader - HRCy high resolution mode configuration for source selector 0"]
pub struct HRM0_R(crate::FieldReader<u8, HRM0_A>);
impl HRM0_R {
    pub(crate) fn new(bits: u8) -> Self {
        HRM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRM0_A {
        match self.bits {
            0 => HRM0_A::VALUE1,
            1 => HRM0_A::VALUE2,
            2 => HRM0_A::VALUE3,
            3 => HRM0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HRM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HRM0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == HRM0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == HRM0_A::VALUE4
    }
}
impl core::ops::Deref for HRM0_R {
    type Target = crate::FieldReader<u8, HRM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRM0` writer - HRCy high resolution mode configuration for source selector 0"]
pub struct HRM0_W<'a> {
    w: &'a mut W,
}
impl<'a> HRM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRM0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRM0_A::VALUE1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRM0_A::VALUE2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HRM0_A::VALUE3)
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(HRM0_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "HRCy high resolution mode configuration for source selector 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HRM1_A {
    #[doc = "0: Rising edge high resolution signal positioning enabled"]
    VALUE1 = 0,
    #[doc = "1: Falling edge high resolution signal positioning enabled"]
    VALUE2 = 1,
    #[doc = "2: Both edges high resolution signal positioning is enabled"]
    VALUE3 = 2,
    #[doc = "3: No high resolution positioning"]
    VALUE4 = 3,
}
impl From<HRM1_A> for u8 {
    #[inline(always)]
    fn from(variant: HRM1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HRM1` reader - HRCy high resolution mode configuration for source selector 1"]
pub struct HRM1_R(crate::FieldReader<u8, HRM1_A>);
impl HRM1_R {
    pub(crate) fn new(bits: u8) -> Self {
        HRM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRM1_A {
        match self.bits {
            0 => HRM1_A::VALUE1,
            1 => HRM1_A::VALUE2,
            2 => HRM1_A::VALUE3,
            3 => HRM1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HRM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HRM1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == HRM1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == HRM1_A::VALUE4
    }
}
impl core::ops::Deref for HRM1_R {
    type Target = crate::FieldReader<u8, HRM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRM1` writer - HRCy high resolution mode configuration for source selector 1"]
pub struct HRM1_W<'a> {
    w: &'a mut W,
}
impl<'a> HRM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRM1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HRM1_A::VALUE1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HRM1_A::VALUE2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HRM1_A::VALUE3)
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(HRM1_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "HRCy dead time enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTE_A {
    #[doc = "0: Dead time insertion is disabled"]
    VALUE1 = 0,
    #[doc = "1: Dead time insertion is enabled"]
    VALUE2 = 1,
}
impl From<DTE_A> for bool {
    #[inline(always)]
    fn from(variant: DTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE` reader - HRCy dead time enable"]
pub struct DTE_R(crate::FieldReader<bool, DTE_A>);
impl DTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTE_A {
        match self.bits {
            false => DTE_A::VALUE1,
            true => DTE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DTE_A::VALUE2
    }
}
impl core::ops::Deref for DTE_R {
    type Target = crate::FieldReader<bool, DTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTE` writer - HRCy dead time enable"]
pub struct DTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Dead time insertion is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTE_A::VALUE1)
    }
    #[doc = "Dead time insertion is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "HRCy trap enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TR0E_A {
    #[doc = "0: Trap function for HRPWMx.HROUTy0 is disabled"]
    VALUE1 = 0,
    #[doc = "1: Trap function for HRPWMx.HROUTy0 is enabled"]
    VALUE2 = 1,
}
impl From<TR0E_A> for bool {
    #[inline(always)]
    fn from(variant: TR0E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR0E` reader - HRCy trap enable"]
pub struct TR0E_R(crate::FieldReader<bool, TR0E_A>);
impl TR0E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR0E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR0E_A {
        match self.bits {
            false => TR0E_A::VALUE1,
            true => TR0E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TR0E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TR0E_A::VALUE2
    }
}
impl core::ops::Deref for TR0E_R {
    type Target = crate::FieldReader<bool, TR0E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR0E` writer - HRCy trap enable"]
pub struct TR0E_W<'a> {
    w: &'a mut W,
}
impl<'a> TR0E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TR0E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TR0E_A::VALUE1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TR0E_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "HRCy complementary trap enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TR1E_A {
    #[doc = "0: Trap function for HRPWMx.HROUTy1 is disabled"]
    VALUE1 = 0,
    #[doc = "1: Trap function for HRPWMx.HROUTy1 is enabled"]
    VALUE2 = 1,
}
impl From<TR1E_A> for bool {
    #[inline(always)]
    fn from(variant: TR1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR1E` reader - HRCy complementary trap enable"]
pub struct TR1E_R(crate::FieldReader<bool, TR1E_A>);
impl TR1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR1E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TR1E_A {
        match self.bits {
            false => TR1E_A::VALUE1,
            true => TR1E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TR1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TR1E_A::VALUE2
    }
}
impl core::ops::Deref for TR1E_R {
    type Target = crate::FieldReader<bool, TR1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR1E` writer - HRCy complementary trap enable"]
pub struct TR1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TR1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TR1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TR1E_A::VALUE1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TR1E_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "HRCy shadow transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STC_A {
    #[doc = "0: HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    VALUE1 = 0,
    #[doc = "1: HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    VALUE2 = 1,
}
impl From<STC_A> for bool {
    #[inline(always)]
    fn from(variant: STC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STC` reader - HRCy shadow transfer configuration"]
pub struct STC_R(crate::FieldReader<bool, STC_A>);
impl STC_R {
    pub(crate) fn new(bits: bool) -> Self {
        STC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STC_A {
        match self.bits {
            false => STC_A::VALUE1,
            true => STC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STC_A::VALUE2
    }
}
impl core::ops::Deref for STC_R {
    type Target = crate::FieldReader<bool, STC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STC` writer - HRCy shadow transfer configuration"]
pub struct STC_W<'a> {
    w: &'a mut W,
}
impl<'a> STC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STC_A::VALUE1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "HRCy dead time shadow transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSTC_A {
    #[doc = "0: HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    VALUE1 = 0,
    #[doc = "1: HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    VALUE2 = 1,
}
impl From<DSTC_A> for bool {
    #[inline(always)]
    fn from(variant: DSTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTC` reader - HRCy dead time shadow transfer configuration"]
pub struct DSTC_R(crate::FieldReader<bool, DSTC_A>);
impl DSTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSTC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTC_A {
        match self.bits {
            false => DSTC_A::VALUE1,
            true => DSTC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DSTC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DSTC_A::VALUE2
    }
}
impl core::ops::Deref for DSTC_R {
    type Target = crate::FieldReader<bool, DSTC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSTC` writer - HRCy dead time shadow transfer configuration"]
pub struct DSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DSTC_A::VALUE1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DSTC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "HRPWMx.OUTy0 channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS0_A {
    #[doc = "0: HRPWMx.OUTy0 is connected to the latch Q channel"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.OUTy0 is connected to the latch Qn channel"]
    VALUE2 = 1,
}
impl From<OCS0_A> for bool {
    #[inline(always)]
    fn from(variant: OCS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS0` reader - HRPWMx.OUTy0 channel selector"]
pub struct OCS0_R(crate::FieldReader<bool, OCS0_A>);
impl OCS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS0_A {
        match self.bits {
            false => OCS0_A::VALUE1,
            true => OCS0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == OCS0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OCS0_A::VALUE2
    }
}
impl core::ops::Deref for OCS0_R {
    type Target = crate::FieldReader<bool, OCS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCS0` writer - HRPWMx.OUTy0 channel selector"]
pub struct OCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> OCS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCS0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS0_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "HRPWMx.OUTy1 channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCS1_A {
    #[doc = "0: HRPWMx.OUTy1 is connected to the latch Qn channel"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.OUTy1 is connected to the latch Q channel"]
    VALUE2 = 1,
}
impl From<OCS1_A> for bool {
    #[inline(always)]
    fn from(variant: OCS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS1` reader - HRPWMx.OUTy1 channel selector"]
pub struct OCS1_R(crate::FieldReader<bool, OCS1_A>);
impl OCS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCS1_A {
        match self.bits {
            false => OCS1_A::VALUE1,
            true => OCS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == OCS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == OCS1_A::VALUE2
    }
}
impl core::ops::Deref for OCS1_R {
    type Target = crate::FieldReader<bool, OCS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCS1` writer - HRPWMx.OUTy1 channel selector"]
pub struct OCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> OCS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCS1_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCS1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Dead Time update trigger selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTUS_A {
    #[doc = "0: The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    VALUE1 = 0,
    #[doc = "1: The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    VALUE2 = 1,
}
impl From<DTUS_A> for bool {
    #[inline(always)]
    fn from(variant: DTUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTUS` reader - Dead Time update trigger selector"]
pub struct DTUS_R(crate::FieldReader<bool, DTUS_A>);
impl DTUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTUS_A {
        match self.bits {
            false => DTUS_A::VALUE1,
            true => DTUS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DTUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DTUS_A::VALUE2
    }
}
impl core::ops::Deref for DTUS_R {
    type Target = crate::FieldReader<bool, DTUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTUS` writer - Dead Time update trigger selector"]
pub struct DTUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTUS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTUS_A::VALUE1)
    }
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTUS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline(always)]
    pub fn hrm0(&self) -> HRM0_R {
        HRM0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline(always)]
    pub fn hrm1(&self) -> HRM1_R {
        HRM1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline(always)]
    pub fn tr0e(&self) -> TR0E_R {
        TR0E_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline(always)]
    pub fn tr1e(&self) -> TR1E_R {
        TR1E_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline(always)]
    pub fn stc(&self) -> STC_R {
        STC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline(always)]
    pub fn dstc(&self) -> DSTC_R {
        DSTC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline(always)]
    pub fn ocs0(&self) -> OCS0_R {
        OCS0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline(always)]
    pub fn ocs1(&self) -> OCS1_R {
        OCS1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline(always)]
    pub fn dtus(&self) -> DTUS_R {
        DTUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline(always)]
    pub fn hrm0(&mut self) -> HRM0_W {
        HRM0_W { w: self }
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline(always)]
    pub fn hrm1(&mut self) -> HRM1_W {
        HRM1_W { w: self }
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> DTE_W {
        DTE_W { w: self }
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline(always)]
    pub fn tr0e(&mut self) -> TR0E_W {
        TR0E_W { w: self }
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline(always)]
    pub fn tr1e(&mut self) -> TR1E_W {
        TR1E_W { w: self }
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline(always)]
    pub fn stc(&mut self) -> STC_W {
        STC_W { w: self }
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline(always)]
    pub fn dstc(&mut self) -> DSTC_W {
        DSTC_W { w: self }
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline(always)]
    pub fn ocs0(&mut self) -> OCS0_W {
        OCS0_W { w: self }
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline(always)]
    pub fn ocs1(&mut self) -> OCS1_W {
        OCS1_W { w: self }
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline(always)]
    pub fn dtus(&mut self) -> DTUS_W {
        DTUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gc](index.html) module"]
pub struct GC_SPEC;
impl crate::RegisterSpec for GC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gc::R](R) reader structure"]
impl crate::Readable for GC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gc::W](W) writer structure"]
impl crate::Writable for GC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
