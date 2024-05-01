#[doc = "Register `GC` reader"]
pub type R = crate::R<GcSpec>;
#[doc = "Register `GC` writer"]
pub type W = crate::W<GcSpec>;
#[doc = "HRCy high resolution mode configuration for source selector 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hrm0 {
    #[doc = "0: Rising edge high resolution signal positioning enabled"]
    Value1 = 0,
    #[doc = "1: Falling edge high resolution signal positioning enabled"]
    Value2 = 1,
    #[doc = "2: Both edges high resolution signal positioning is enabled"]
    Value3 = 2,
    #[doc = "3: No high resolution positioning"]
    Value4 = 3,
}
impl From<Hrm0> for u8 {
    #[inline(always)]
    fn from(variant: Hrm0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hrm0 {
    type Ux = u8;
}
impl crate::IsEnum for Hrm0 {}
#[doc = "Field `HRM0` reader - HRCy high resolution mode configuration for source selector 0"]
pub type Hrm0R = crate::FieldReader<Hrm0>;
impl Hrm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrm0 {
        match self.bits {
            0 => Hrm0::Value1,
            1 => Hrm0::Value2,
            2 => Hrm0::Value3,
            3 => Hrm0::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrm0::Value1
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrm0::Value2
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Hrm0::Value3
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Hrm0::Value4
    }
}
#[doc = "Field `HRM0` writer - HRCy high resolution mode configuration for source selector 0"]
pub type Hrm0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hrm0, crate::Safe>;
impl<'a, REG> Hrm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm0::Value1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm0::Value2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm0::Value3)
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm0::Value4)
    }
}
#[doc = "HRCy high resolution mode configuration for source selector 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hrm1 {
    #[doc = "0: Rising edge high resolution signal positioning enabled"]
    Value1 = 0,
    #[doc = "1: Falling edge high resolution signal positioning enabled"]
    Value2 = 1,
    #[doc = "2: Both edges high resolution signal positioning is enabled"]
    Value3 = 2,
    #[doc = "3: No high resolution positioning"]
    Value4 = 3,
}
impl From<Hrm1> for u8 {
    #[inline(always)]
    fn from(variant: Hrm1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hrm1 {
    type Ux = u8;
}
impl crate::IsEnum for Hrm1 {}
#[doc = "Field `HRM1` reader - HRCy high resolution mode configuration for source selector 1"]
pub type Hrm1R = crate::FieldReader<Hrm1>;
impl Hrm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrm1 {
        match self.bits {
            0 => Hrm1::Value1,
            1 => Hrm1::Value2,
            2 => Hrm1::Value3,
            3 => Hrm1::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrm1::Value1
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrm1::Value2
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Hrm1::Value3
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Hrm1::Value4
    }
}
#[doc = "Field `HRM1` writer - HRCy high resolution mode configuration for source selector 1"]
pub type Hrm1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Hrm1, crate::Safe>;
impl<'a, REG> Hrm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm1::Value1)
    }
    #[doc = "Falling edge high resolution signal positioning enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm1::Value2)
    }
    #[doc = "Both edges high resolution signal positioning is enabled"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm1::Value3)
    }
    #[doc = "No high resolution positioning"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Hrm1::Value4)
    }
}
#[doc = "HRCy dead time enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dte {
    #[doc = "0: Dead time insertion is disabled"]
    Value1 = 0,
    #[doc = "1: Dead time insertion is enabled"]
    Value2 = 1,
}
impl From<Dte> for bool {
    #[inline(always)]
    fn from(variant: Dte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE` reader - HRCy dead time enable"]
pub type DteR = crate::BitReader<Dte>;
impl DteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dte {
        match self.bits {
            false => Dte::Value1,
            true => Dte::Value2,
        }
    }
    #[doc = "Dead time insertion is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dte::Value1
    }
    #[doc = "Dead time insertion is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dte::Value2
    }
}
#[doc = "Field `DTE` writer - HRCy dead time enable"]
pub type DteW<'a, REG> = crate::BitWriter<'a, REG, Dte>;
impl<'a, REG> DteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dead time insertion is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dte::Value1)
    }
    #[doc = "Dead time insertion is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dte::Value2)
    }
}
#[doc = "HRCy trap enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tr0e {
    #[doc = "0: Trap function for HRPWMx.HROUTy0 is disabled"]
    Value1 = 0,
    #[doc = "1: Trap function for HRPWMx.HROUTy0 is enabled"]
    Value2 = 1,
}
impl From<Tr0e> for bool {
    #[inline(always)]
    fn from(variant: Tr0e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR0E` reader - HRCy trap enable"]
pub type Tr0eR = crate::BitReader<Tr0e>;
impl Tr0eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tr0e {
        match self.bits {
            false => Tr0e::Value1,
            true => Tr0e::Value2,
        }
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tr0e::Value1
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tr0e::Value2
    }
}
#[doc = "Field `TR0E` writer - HRCy trap enable"]
pub type Tr0eW<'a, REG> = crate::BitWriter<'a, REG, Tr0e>;
impl<'a, REG> Tr0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap function for HRPWMx.HROUTy0 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tr0e::Value1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy0 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tr0e::Value2)
    }
}
#[doc = "HRCy complementary trap enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tr1e {
    #[doc = "0: Trap function for HRPWMx.HROUTy1 is disabled"]
    Value1 = 0,
    #[doc = "1: Trap function for HRPWMx.HROUTy1 is enabled"]
    Value2 = 1,
}
impl From<Tr1e> for bool {
    #[inline(always)]
    fn from(variant: Tr1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR1E` reader - HRCy complementary trap enable"]
pub type Tr1eR = crate::BitReader<Tr1e>;
impl Tr1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tr1e {
        match self.bits {
            false => Tr1e::Value1,
            true => Tr1e::Value2,
        }
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tr1e::Value1
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tr1e::Value2
    }
}
#[doc = "Field `TR1E` writer - HRCy complementary trap enable"]
pub type Tr1eW<'a, REG> = crate::BitWriter<'a, REG, Tr1e>;
impl<'a, REG> Tr1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trap function for HRPWMx.HROUTy1 is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tr1e::Value1)
    }
    #[doc = "Trap function for HRPWMx.HROUTy1 is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tr1e::Value2)
    }
}
#[doc = "HRCy shadow transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stc {
    #[doc = "0: HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    Value1 = 0,
    #[doc = "1: HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    Value2 = 1,
}
impl From<Stc> for bool {
    #[inline(always)]
    fn from(variant: Stc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STC` reader - HRCy shadow transfer configuration"]
pub type StcR = crate::BitReader<Stc>;
impl StcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stc {
        match self.bits {
            false => Stc::Value1,
            true => Stc::Value2,
        }
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stc::Value1
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stc::Value2
    }
}
#[doc = "Field `STC` writer - HRCy shadow transfer configuration"]
pub type StcW<'a, REG> = crate::BitWriter<'a, REG, Stc>;
impl<'a, REG> StcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stc::Value1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stc::Value2)
    }
}
#[doc = "HRCy dead time shadow transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dstc {
    #[doc = "0: HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    Value1 = 0,
    #[doc = "1: HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    Value2 = 1,
}
impl From<Dstc> for bool {
    #[inline(always)]
    fn from(variant: Dstc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTC` reader - HRCy dead time shadow transfer configuration"]
pub type DstcR = crate::BitReader<Dstc>;
impl DstcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstc {
        match self.bits {
            false => Dstc::Value1,
            true => Dstc::Value2,
        }
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dstc::Value1
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dstc::Value2
    }
}
#[doc = "Field `DSTC` writer - HRCy dead time shadow transfer configuration"]
pub type DstcW<'a, REG> = crate::BitWriter<'a, REG, Dstc>;
impl<'a, REG> DstcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is not linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dstc::Value1)
    }
    #[doc = "HRCy shadow transfer enable for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values is linked with the specific Capture/Compare Unit timer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dstc::Value2)
    }
}
#[doc = "HRPWMx.OUTy0 channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocs0 {
    #[doc = "0: HRPWMx.OUTy0 is connected to the latch Q channel"]
    Value1 = 0,
    #[doc = "1: HRPWMx.OUTy0 is connected to the latch Qn channel"]
    Value2 = 1,
}
impl From<Ocs0> for bool {
    #[inline(always)]
    fn from(variant: Ocs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS0` reader - HRPWMx.OUTy0 channel selector"]
pub type Ocs0R = crate::BitReader<Ocs0>;
impl Ocs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocs0 {
        match self.bits {
            false => Ocs0::Value1,
            true => Ocs0::Value2,
        }
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ocs0::Value1
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ocs0::Value2
    }
}
#[doc = "Field `OCS0` writer - HRPWMx.OUTy0 channel selector"]
pub type Ocs0W<'a, REG> = crate::BitWriter<'a, REG, Ocs0>;
impl<'a, REG> Ocs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy0 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs0::Value1)
    }
    #[doc = "HRPWMx.OUTy0 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs0::Value2)
    }
}
#[doc = "HRPWMx.OUTy1 channel selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ocs1 {
    #[doc = "0: HRPWMx.OUTy1 is connected to the latch Qn channel"]
    Value1 = 0,
    #[doc = "1: HRPWMx.OUTy1 is connected to the latch Q channel"]
    Value2 = 1,
}
impl From<Ocs1> for bool {
    #[inline(always)]
    fn from(variant: Ocs1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCS1` reader - HRPWMx.OUTy1 channel selector"]
pub type Ocs1R = crate::BitReader<Ocs1>;
impl Ocs1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ocs1 {
        match self.bits {
            false => Ocs1::Value1,
            true => Ocs1::Value2,
        }
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ocs1::Value1
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ocs1::Value2
    }
}
#[doc = "Field `OCS1` writer - HRPWMx.OUTy1 channel selector"]
pub type Ocs1W<'a, REG> = crate::BitWriter<'a, REG, Ocs1>;
impl<'a, REG> Ocs1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy1 is connected to the latch Qn channel"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs1::Value1)
    }
    #[doc = "HRPWMx.OUTy1 is connected to the latch Q channel"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ocs1::Value2)
    }
}
#[doc = "Dead Time update trigger selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtus {
    #[doc = "0: The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    Value1 = 0,
    #[doc = "1: The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    Value2 = 1,
}
impl From<Dtus> for bool {
    #[inline(always)]
    fn from(variant: Dtus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTUS` reader - Dead Time update trigger selector"]
pub type DtusR = crate::BitReader<Dtus>;
impl DtusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtus {
        match self.bits {
            false => Dtus::Value1,
            true => Dtus::Value2,
        }
    }
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dtus::Value1
    }
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dtus::Value2
    }
}
#[doc = "Field `DTUS` writer - Dead Time update trigger selector"]
pub type DtusW<'a, REG> = crate::BitWriter<'a, REG, Dtus>;
impl<'a, REG> DtusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The update of the values is done with the trigger generated by the timers. This is the same trigger that is used to update the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, .."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtus::Value1)
    }
    #[doc = "The update of the dead time values is done when the dead time counter is not running, independently of the HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . registers."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dtus::Value2)
    }
}
impl R {
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline(always)]
    pub fn hrm0(&self) -> Hrm0R {
        Hrm0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline(always)]
    pub fn hrm1(&self) -> Hrm1R {
        Hrm1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline(always)]
    pub fn dte(&self) -> DteR {
        DteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline(always)]
    pub fn tr0e(&self) -> Tr0eR {
        Tr0eR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline(always)]
    pub fn tr1e(&self) -> Tr1eR {
        Tr1eR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline(always)]
    pub fn stc(&self) -> StcR {
        StcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline(always)]
    pub fn dstc(&self) -> DstcR {
        DstcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline(always)]
    pub fn ocs0(&self) -> Ocs0R {
        Ocs0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline(always)]
    pub fn ocs1(&self) -> Ocs1R {
        Ocs1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline(always)]
    pub fn dtus(&self) -> DtusR {
        DtusR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - HRCy high resolution mode configuration for source selector 0"]
    #[inline(always)]
    #[must_use]
    pub fn hrm0(&mut self) -> Hrm0W<GcSpec> {
        Hrm0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - HRCy high resolution mode configuration for source selector 1"]
    #[inline(always)]
    #[must_use]
    pub fn hrm1(&mut self) -> Hrm1W<GcSpec> {
        Hrm1W::new(self, 2)
    }
    #[doc = "Bit 8 - HRCy dead time enable"]
    #[inline(always)]
    #[must_use]
    pub fn dte(&mut self) -> DteW<GcSpec> {
        DteW::new(self, 8)
    }
    #[doc = "Bit 9 - HRCy trap enable"]
    #[inline(always)]
    #[must_use]
    pub fn tr0e(&mut self) -> Tr0eW<GcSpec> {
        Tr0eW::new(self, 9)
    }
    #[doc = "Bit 10 - HRCy complementary trap enable"]
    #[inline(always)]
    #[must_use]
    pub fn tr1e(&mut self) -> Tr1eW<GcSpec> {
        Tr1eW::new(self, 10)
    }
    #[doc = "Bit 11 - HRCy shadow transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn stc(&mut self) -> StcW<GcSpec> {
        StcW::new(self, 11)
    }
    #[doc = "Bit 12 - HRCy dead time shadow transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dstc(&mut self) -> DstcW<GcSpec> {
        DstcW::new(self, 12)
    }
    #[doc = "Bit 13 - HRPWMx.OUTy0 channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn ocs0(&mut self) -> Ocs0W<GcSpec> {
        Ocs0W::new(self, 13)
    }
    #[doc = "Bit 14 - HRPWMx.OUTy1 channel selector"]
    #[inline(always)]
    #[must_use]
    pub fn ocs1(&mut self) -> Ocs1W<GcSpec> {
        Ocs1W::new(self, 14)
    }
    #[doc = "Bit 16 - Dead Time update trigger selector"]
    #[inline(always)]
    #[must_use]
    pub fn dtus(&mut self) -> DtusW<GcSpec> {
        DtusW::new(self, 16)
    }
}
#[doc = "HRC mode configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcSpec;
impl crate::RegisterSpec for GcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gc::R`](R) reader structure"]
impl crate::Readable for GcSpec {}
#[doc = "`write(|w| ..)` method takes [`gc::W`](W) writer structure"]
impl crate::Writable for GcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GcSpec {
    const RESET_VALUE: u32 = 0;
}
