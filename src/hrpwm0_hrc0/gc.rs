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
#[doc = "Field `HRM0` reader - HRCy high resolution mode configuration for source selector 0"]
pub type HRM0_R = crate::FieldReader<u8, HRM0_A>;
#[doc = "HRCy high resolution mode configuration for source selector 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl HRM0_R {
    #[doc = "Get enumerated values variant"]
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
        *self == HRM0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRM0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HRM0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HRM0_A::VALUE4
    }
}
#[doc = "Field `HRM0` writer - HRCy high resolution mode configuration for source selector 0"]
pub type HRM0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GC_SPEC, u8, HRM0_A, 2, O>;
impl<'a, const O: u8> HRM0_W<'a, O> {
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
}
#[doc = "Field `HRM1` reader - HRCy high resolution mode configuration for source selector 1"]
pub type HRM1_R = crate::FieldReader<u8, HRM1_A>;
#[doc = "HRCy high resolution mode configuration for source selector 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl HRM1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == HRM1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRM1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HRM1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HRM1_A::VALUE4
    }
}
#[doc = "Field `HRM1` writer - HRCy high resolution mode configuration for source selector 1"]
pub type HRM1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, GC_SPEC, u8, HRM1_A, 2, O>;
impl<'a, const O: u8> HRM1_W<'a, O> {
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
}
#[doc = "Field `DTE` reader - HRCy dead time enable"]
pub type DTE_R = crate::BitReader<DTE_A>;
#[doc = "HRCy dead time enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DTE_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTE_A::VALUE2
    }
}
#[doc = "Field `DTE` writer - HRCy dead time enable"]
pub type DTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, DTE_A, O>;
impl<'a, const O: u8> DTE_W<'a, O> {
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
}
#[doc = "Field `TR0E` reader - HRCy trap enable"]
pub type TR0E_R = crate::BitReader<TR0E_A>;
#[doc = "HRCy trap enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TR0E_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TR0E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TR0E_A::VALUE2
    }
}
#[doc = "Field `TR0E` writer - HRCy trap enable"]
pub type TR0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, TR0E_A, O>;
impl<'a, const O: u8> TR0E_W<'a, O> {
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
}
#[doc = "Field `TR1E` reader - HRCy complementary trap enable"]
pub type TR1E_R = crate::BitReader<TR1E_A>;
#[doc = "HRCy complementary trap enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TR1E_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TR1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TR1E_A::VALUE2
    }
}
#[doc = "Field `TR1E` writer - HRCy complementary trap enable"]
pub type TR1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, TR1E_A, O>;
impl<'a, const O: u8> TR1E_W<'a, O> {
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
}
#[doc = "Field `STC` reader - HRCy shadow transfer configuration"]
pub type STC_R = crate::BitReader<STC_A>;
#[doc = "HRCy shadow transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == STC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STC_A::VALUE2
    }
}
#[doc = "Field `STC` writer - HRCy shadow transfer configuration"]
pub type STC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, STC_A, O>;
impl<'a, const O: u8> STC_W<'a, O> {
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
}
#[doc = "Field `DSTC` reader - HRCy dead time shadow transfer configuration"]
pub type DSTC_R = crate::BitReader<DSTC_A>;
#[doc = "HRCy dead time shadow transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DSTC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DSTC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSTC_A::VALUE2
    }
}
#[doc = "Field `DSTC` writer - HRCy dead time shadow transfer configuration"]
pub type DSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, DSTC_A, O>;
impl<'a, const O: u8> DSTC_W<'a, O> {
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
}
#[doc = "Field `OCS0` reader - HRPWMx.OUTy0 channel selector"]
pub type OCS0_R = crate::BitReader<OCS0_A>;
#[doc = "HRPWMx.OUTy0 channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl OCS0_R {
    #[doc = "Get enumerated values variant"]
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
        *self == OCS0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS0_A::VALUE2
    }
}
#[doc = "Field `OCS0` writer - HRPWMx.OUTy0 channel selector"]
pub type OCS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, OCS0_A, O>;
impl<'a, const O: u8> OCS0_W<'a, O> {
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
}
#[doc = "Field `OCS1` reader - HRPWMx.OUTy1 channel selector"]
pub type OCS1_R = crate::BitReader<OCS1_A>;
#[doc = "HRPWMx.OUTy1 channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl OCS1_R {
    #[doc = "Get enumerated values variant"]
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
        *self == OCS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS1_A::VALUE2
    }
}
#[doc = "Field `OCS1` writer - HRPWMx.OUTy1 channel selector"]
pub type OCS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, OCS1_A, O>;
impl<'a, const O: u8> OCS1_W<'a, O> {
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
}
#[doc = "Field `DTUS` reader - Dead Time update trigger selector"]
pub type DTUS_R = crate::BitReader<DTUS_A>;
#[doc = "Dead Time update trigger selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DTUS_R {
    #[doc = "Get enumerated values variant"]
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
        *self == DTUS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTUS_A::VALUE2
    }
}
#[doc = "Field `DTUS` writer - Dead Time update trigger selector"]
pub type DTUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, DTUS_A, O>;
impl<'a, const O: u8> DTUS_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline(always)]
    pub fn hrm0(&self) -> HRM0_R {
        HRM0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline(always)]
    pub fn hrm1(&self) -> HRM1_R {
        HRM1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline(always)]
    pub fn tr0e(&self) -> TR0E_R {
        TR0E_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline(always)]
    pub fn tr1e(&self) -> TR1E_R {
        TR1E_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline(always)]
    pub fn stc(&self) -> STC_R {
        STC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline(always)]
    pub fn dstc(&self) -> DSTC_R {
        DSTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline(always)]
    pub fn ocs0(&self) -> OCS0_R {
        OCS0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline(always)]
    pub fn ocs1(&self) -> OCS1_R {
        OCS1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline(always)]
    pub fn dtus(&self) -> DTUS_R {
        DTUS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline(always)]
    #[must_use]
    pub fn hrm0(&mut self) -> HRM0_W<0> {
        HRM0_W::new(self)
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline(always)]
    #[must_use]
    pub fn hrm1(&mut self) -> HRM1_W<2> {
        HRM1_W::new(self)
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline(always)]
    #[must_use]
    pub fn dte(&mut self) -> DTE_W<8> {
        DTE_W::new(self)
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline(always)]
    #[must_use]
    pub fn tr0e(&mut self) -> TR0E_W<9> {
        TR0E_W::new(self)
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline(always)]
    #[must_use]
    pub fn tr1e(&mut self) -> TR1E_W<10> {
        TR1E_W::new(self)
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn stc(&mut self) -> STC_W<11> {
        STC_W::new(self)
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dstc(&mut self) -> DSTC_W<12> {
        DSTC_W::new(self)
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn ocs0(&mut self) -> OCS0_W<13> {
        OCS0_W::new(self)
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn ocs1(&mut self) -> OCS1_W<14> {
        OCS1_W::new(self)
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline(always)]
    #[must_use]
    pub fn dtus(&mut self) -> DTUS_W<16> {
        DTUS_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
