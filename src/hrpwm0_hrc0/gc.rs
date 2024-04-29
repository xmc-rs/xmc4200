#[doc = "Register `GC` reader"]
pub type R = crate::R<GC_SPEC>;
#[doc = "Register `GC` writer"]
pub type W = crate::W<GC_SPEC>;
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
impl crate::FieldSpec for HRM0_A {
    type Ux = u8;
}
impl crate::IsEnum for HRM0_A {}
#[doc = "Field `HRM0` reader - HRCy high resolution mode configuration for source selector 0"]
pub type HRM0_R = crate::FieldReader<HRM0_A>;
impl HRM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HRM0_A {
        match self.bits {
            0 => HRM0_A::VALUE1,
            1 => HRM0_A::VALUE2,
            2 => HRM0_A::VALUE3,
            3 => HRM0_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRM0_A::VALUE1
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRM0_A::VALUE2
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HRM0_A::VALUE3
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HRM0_A::VALUE4
    }
}
#[doc = "Field `HRM0` writer - HRCy high resolution mode configuration for source selector 0"]
pub type HRM0_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HRM0_A, crate::Safe>;
impl<'a, REG> HRM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HRM0_A::VALUE1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HRM0_A::VALUE2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(HRM0_A::VALUE3)
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(HRM0_A::VALUE4)
    }
}
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
impl crate::FieldSpec for HRM1_A {
    type Ux = u8;
}
impl crate::IsEnum for HRM1_A {}
#[doc = "Field `HRM1` reader - HRCy high resolution mode configuration for source selector 1"]
pub type HRM1_R = crate::FieldReader<HRM1_A>;
impl HRM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HRM1_A {
        match self.bits {
            0 => HRM1_A::VALUE1,
            1 => HRM1_A::VALUE2,
            2 => HRM1_A::VALUE3,
            3 => HRM1_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRM1_A::VALUE1
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRM1_A::VALUE2
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HRM1_A::VALUE3
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HRM1_A::VALUE4
    }
}
#[doc = "Field `HRM1` writer - HRCy high resolution mode configuration for source selector 1"]
pub type HRM1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, HRM1_A, crate::Safe>;
impl<'a, REG> HRM1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HRM1_A::VALUE1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HRM1_A::VALUE2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(HRM1_A::VALUE3)
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(HRM1_A::VALUE4)
    }
}
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
#[doc = "Field `DTE` reader - HRCy dead time enable"]
pub type DTE_R = crate::BitReader<DTE_A>;
impl DTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTE_A {
        match self.bits {
            false => DTE_A::VALUE1,
            true => DTE_A::VALUE2,
        }
    }
    #[doc = "Dead time insertion is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTE_A::VALUE1
    }
    #[doc = "Dead time insertion is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTE_A::VALUE2
    }
}
#[doc = "Field `DTE` writer - HRCy dead time enable"]
pub type DTE_W<'a, REG> = crate::BitWriter<'a, REG, DTE_A>;
impl<'a, REG> DTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead time insertion is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DTE_A::VALUE1)
    }
    #[doc = "Dead time insertion is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DTE_A::VALUE2)
    }
}
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
#[doc = "Field `TR0E` reader - HRCy trap enable"]
pub type TR0E_R = crate::BitReader<TR0E_A>;
impl TR0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TR0E_A {
        match self.bits {
            false => TR0E_A::VALUE1,
            true => TR0E_A::VALUE2,
        }
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TR0E_A::VALUE1
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TR0E_A::VALUE2
    }
}
#[doc = "Field `TR0E` writer - HRCy trap enable"]
pub type TR0E_W<'a, REG> = crate::BitWriter<'a, REG, TR0E_A>;
impl<'a, REG> TR0E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TR0E_A::VALUE1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TR0E_A::VALUE2)
    }
}
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
#[doc = "Field `TR1E` reader - HRCy complementary trap enable"]
pub type TR1E_R = crate::BitReader<TR1E_A>;
impl TR1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TR1E_A {
        match self.bits {
            false => TR1E_A::VALUE1,
            true => TR1E_A::VALUE2,
        }
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TR1E_A::VALUE1
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TR1E_A::VALUE2
    }
}
#[doc = "Field `TR1E` writer - HRCy complementary trap enable"]
pub type TR1E_W<'a, REG> = crate::BitWriter<'a, REG, TR1E_A>;
impl<'a, REG> TR1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TR1E_A::VALUE1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TR1E_A::VALUE2)
    }
}
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
#[doc = "Field `STC` reader - HRCy shadow transfer configuration"]
pub type STC_R = crate::BitReader<STC_A>;
impl STC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STC_A {
        match self.bits {
            false => STC_A::VALUE1,
            true => STC_A::VALUE2,
        }
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STC_A::VALUE1
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STC_A::VALUE2
    }
}
#[doc = "Field `STC` writer - HRCy shadow transfer configuration"]
pub type STC_W<'a, REG> = crate::BitWriter<'a, REG, STC_A>;
impl<'a, REG> STC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STC_A::VALUE1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STC_A::VALUE2)
    }
}
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
#[doc = "Field `DSTC` reader - HRCy dead time shadow transfer configuration"]
pub type DSTC_R = crate::BitReader<DSTC_A>;
impl DSTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DSTC_A {
        match self.bits {
            false => DSTC_A::VALUE1,
            true => DSTC_A::VALUE2,
        }
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DSTC_A::VALUE1
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DSTC_A::VALUE2
    }
}
#[doc = "Field `DSTC` writer - HRCy dead time shadow transfer configuration"]
pub type DSTC_W<'a, REG> = crate::BitWriter<'a, REG, DSTC_A>;
impl<'a, REG> DSTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DSTC_A::VALUE1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DSTC_A::VALUE2)
    }
}
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
#[doc = "Field `OCS0` reader - HRPWMx.OUTy0 channel selector"]
pub type OCS0_R = crate::BitReader<OCS0_A>;
impl OCS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCS0_A {
        match self.bits {
            false => OCS0_A::VALUE1,
            true => OCS0_A::VALUE2,
        }
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS0_A::VALUE1
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS0_A::VALUE2
    }
}
#[doc = "Field `OCS0` writer - HRPWMx.OUTy0 channel selector"]
pub type OCS0_W<'a, REG> = crate::BitWriter<'a, REG, OCS0_A>;
impl<'a, REG> OCS0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OCS0_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OCS0_A::VALUE2)
    }
}
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
#[doc = "Field `OCS1` reader - HRPWMx.OUTy1 channel selector"]
pub type OCS1_R = crate::BitReader<OCS1_A>;
impl OCS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OCS1_A {
        match self.bits {
            false => OCS1_A::VALUE1,
            true => OCS1_A::VALUE2,
        }
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OCS1_A::VALUE1
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OCS1_A::VALUE2
    }
}
#[doc = "Field `OCS1` writer - HRPWMx.OUTy1 channel selector"]
pub type OCS1_W<'a, REG> = crate::BitWriter<'a, REG, OCS1_A>;
impl<'a, REG> OCS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(OCS1_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(OCS1_A::VALUE2)
    }
}
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
#[doc = "Field `DTUS` reader - Dead Time update trigger selector"]
pub type DTUS_R = crate::BitReader<DTUS_A>;
impl DTUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTUS_A {
        match self.bits {
            false => DTUS_A::VALUE1,
            true => DTUS_A::VALUE2,
        }
    }
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTUS_A::VALUE1
    }
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTUS_A::VALUE2
    }
}
#[doc = "Field `DTUS` writer - Dead Time update trigger selector"]
pub type DTUS_W<'a, REG> = crate::BitWriter<'a, REG, DTUS_A>;
impl<'a, REG> DTUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DTUS_A::VALUE1)
    }
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
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
    pub fn hrm0(&mut self) -> HRM0_W<GC_SPEC> {
        HRM0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline(always)]
    #[must_use]
    pub fn hrm1(&mut self) -> HRM1_W<GC_SPEC> {
        HRM1_W::new(self, 2)
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline(always)]
    #[must_use]
    pub fn dte(&mut self) -> DTE_W<GC_SPEC> {
        DTE_W::new(self, 8)
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline(always)]
    #[must_use]
    pub fn tr0e(&mut self) -> TR0E_W<GC_SPEC> {
        TR0E_W::new(self, 9)
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline(always)]
    #[must_use]
    pub fn tr1e(&mut self) -> TR1E_W<GC_SPEC> {
        TR1E_W::new(self, 10)
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn stc(&mut self) -> STC_W<GC_SPEC> {
        STC_W::new(self, 11)
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dstc(&mut self) -> DSTC_W<GC_SPEC> {
        DSTC_W::new(self, 12)
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn ocs0(&mut self) -> OCS0_W<GC_SPEC> {
        OCS0_W::new(self, 13)
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn ocs1(&mut self) -> OCS1_W<GC_SPEC> {
        OCS1_W::new(self, 14)
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline(always)]
    #[must_use]
    pub fn dtus(&mut self) -> DTUS_W<GC_SPEC> {
        DTUS_W::new(self, 16)
    }
}
#[doc = "HRC mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GC_SPEC;
impl crate::RegisterSpec for GC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gc::R`](R) reader structure"]
impl crate::Readable for GC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gc::W`](W) writer structure"]
impl crate::Writable for GC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GC_SPEC {
    const RESET_VALUE: u32 = 0;
}
