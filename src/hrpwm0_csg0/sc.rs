#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<ScSpec>;
#[doc = "Prescaler external start configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psrm {
    #[doc = "0: External start trigger is ignored"]
    Value1 = 0,
    #[doc = "1: Start prescaler"]
    Value2 = 1,
    #[doc = "2: Clear prescaler"]
    Value3 = 2,
    #[doc = "3: Clear &amp; Start prescaler"]
    Value4 = 3,
}
impl From<Psrm> for u8 {
    #[inline(always)]
    fn from(variant: Psrm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psrm {
    type Ux = u8;
}
#[doc = "Field `PSRM` reader - Prescaler external start configuration"]
pub type PsrmR = crate::FieldReader<Psrm>;
impl PsrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psrm {
        match self.bits {
            0 => Psrm::Value1,
            1 => Psrm::Value2,
            2 => Psrm::Value3,
            3 => Psrm::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External start trigger is ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psrm::Value1
    }
    #[doc = "Start prescaler"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psrm::Value2
    }
    #[doc = "Clear prescaler"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Psrm::Value3
    }
    #[doc = "Clear &amp; Start prescaler"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Psrm::Value4
    }
}
#[doc = "Field `PSRM` writer - Prescaler external start configuration"]
pub type PsrmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Psrm>;
impl<'a, REG> PsrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External start trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psrm::Value1)
    }
    #[doc = "Start prescaler"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psrm::Value2)
    }
    #[doc = "Clear prescaler"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Psrm::Value3)
    }
    #[doc = "Clear &amp; Start prescaler"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Psrm::Value4)
    }
}
#[doc = "Prescaler external stop configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pstm {
    #[doc = "0: External stop trigger is ignored"]
    Value1 = 0,
    #[doc = "1: Stop prescaler"]
    Value2 = 1,
    #[doc = "2: Clear prescaler"]
    Value3 = 2,
    #[doc = "3: Clear &amp; Stop prescaler"]
    Value4 = 3,
}
impl From<Pstm> for u8 {
    #[inline(always)]
    fn from(variant: Pstm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pstm {
    type Ux = u8;
}
#[doc = "Field `PSTM` reader - Prescaler external stop configuration"]
pub type PstmR = crate::FieldReader<Pstm>;
impl PstmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pstm {
        match self.bits {
            0 => Pstm::Value1,
            1 => Pstm::Value2,
            2 => Pstm::Value3,
            3 => Pstm::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "External stop trigger is ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pstm::Value1
    }
    #[doc = "Stop prescaler"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pstm::Value2
    }
    #[doc = "Clear prescaler"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pstm::Value3
    }
    #[doc = "Clear &amp; Stop prescaler"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Pstm::Value4
    }
}
#[doc = "Field `PSTM` writer - Prescaler external stop configuration"]
pub type PstmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Pstm>;
impl<'a, REG> PstmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External stop trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pstm::Value1)
    }
    #[doc = "Stop prescaler"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pstm::Value2)
    }
    #[doc = "Clear prescaler"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pstm::Value3)
    }
    #[doc = "Clear &amp; Stop prescaler"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Pstm::Value4)
    }
}
#[doc = "Fixed division disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpd {
    #[doc = "0: Division by 4 enabled"]
    Value1 = 0,
    #[doc = "1: Division by 4 disabled"]
    Value2 = 1,
}
impl From<Fpd> for bool {
    #[inline(always)]
    fn from(variant: Fpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPD` reader - Fixed division disable"]
pub type FpdR = crate::BitReader<Fpd>;
impl FpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpd {
        match self.bits {
            false => Fpd::Value1,
            true => Fpd::Value2,
        }
    }
    #[doc = "Division by 4 enabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Fpd::Value1
    }
    #[doc = "Division by 4 disabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Fpd::Value2
    }
}
#[doc = "Field `FPD` writer - Fixed division disable"]
pub type FpdW<'a, REG> = crate::BitWriter<'a, REG, Fpd>;
impl<'a, REG> FpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Division by 4 enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Fpd::Value1)
    }
    #[doc = "Division by 4 disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Fpd::Value2)
    }
}
#[doc = "Prescaler division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psv {
    #[doc = "0: division by 1"]
    Value1 = 0,
    #[doc = "1: division by 2"]
    Value2 = 1,
    #[doc = "2: division by 4"]
    Value3 = 2,
    #[doc = "3: division by 8"]
    Value4 = 3,
}
impl From<Psv> for u8 {
    #[inline(always)]
    fn from(variant: Psv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psv {
    type Ux = u8;
}
#[doc = "Field `PSV` reader - Prescaler division factor"]
pub type PsvR = crate::FieldReader<Psv>;
impl PsvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psv {
        match self.bits {
            0 => Psv::Value1,
            1 => Psv::Value2,
            2 => Psv::Value3,
            3 => Psv::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "division by 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psv::Value1
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psv::Value2
    }
    #[doc = "division by 4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Psv::Value3
    }
    #[doc = "division by 8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Psv::Value4
    }
}
#[doc = "Field `PSV` writer - Prescaler division factor"]
pub type PsvW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Psv>;
impl<'a, REG> PsvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "division by 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psv::Value1)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psv::Value2)
    }
    #[doc = "division by 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Psv::Value3)
    }
    #[doc = "division by 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Psv::Value4)
    }
}
#[doc = "Slope control mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scm {
    #[doc = "0: Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    Value1 = 0,
    #[doc = "1: Decrementing slope generation."]
    Value2 = 1,
    #[doc = "2: Incrementing slope generation."]
    Value3 = 2,
    #[doc = "3: Triangular slope generation."]
    Value4 = 3,
}
impl From<Scm> for u8 {
    #[inline(always)]
    fn from(variant: Scm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scm {
    type Ux = u8;
}
#[doc = "Field `SCM` reader - Slope control mode"]
pub type ScmR = crate::FieldReader<Scm>;
impl ScmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scm {
        match self.bits {
            0 => Scm::Value1,
            1 => Scm::Value2,
            2 => Scm::Value3,
            3 => Scm::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Scm::Value1
    }
    #[doc = "Decrementing slope generation."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Scm::Value2
    }
    #[doc = "Incrementing slope generation."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Scm::Value3
    }
    #[doc = "Triangular slope generation."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Scm::Value4
    }
}
#[doc = "Field `SCM` writer - Slope control mode"]
pub type ScmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Scm>;
impl<'a, REG> ScmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Scm::Value1)
    }
    #[doc = "Decrementing slope generation."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Scm::Value2)
    }
    #[doc = "Incrementing slope generation."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Scm::Value3)
    }
    #[doc = "Triangular slope generation."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Scm::Value4)
    }
}
#[doc = "Slope external start configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssrm {
    #[doc = "0: External start trigger is ignored"]
    Value1 = 0,
    #[doc = "1: Start/restart slope generation"]
    Value2 = 1,
    #[doc = "2: Resumes slope"]
    Value3 = 2,
}
impl From<Ssrm> for u8 {
    #[inline(always)]
    fn from(variant: Ssrm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssrm {
    type Ux = u8;
}
#[doc = "Field `SSRM` reader - Slope external start configuration"]
pub type SsrmR = crate::FieldReader<Ssrm>;
impl SsrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssrm> {
        match self.bits {
            0 => Some(Ssrm::Value1),
            1 => Some(Ssrm::Value2),
            2 => Some(Ssrm::Value3),
            _ => None,
        }
    }
    #[doc = "External start trigger is ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ssrm::Value1
    }
    #[doc = "Start/restart slope generation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ssrm::Value2
    }
    #[doc = "Resumes slope"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Ssrm::Value3
    }
}
#[doc = "Field `SSRM` writer - Slope external start configuration"]
pub type SsrmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ssrm>;
impl<'a, REG> SsrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External start trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrm::Value1)
    }
    #[doc = "Start/restart slope generation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrm::Value2)
    }
    #[doc = "Resumes slope"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Ssrm::Value3)
    }
}
#[doc = "Slope external stop configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sstm {
    #[doc = "0: External stop trigger is ignored"]
    Value1 = 0,
    #[doc = "1: Stops/Halts the slope generation"]
    Value2 = 1,
    #[doc = "2: Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    Value3 = 2,
}
impl From<Sstm> for u8 {
    #[inline(always)]
    fn from(variant: Sstm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sstm {
    type Ux = u8;
}
#[doc = "Field `SSTM` reader - Slope external stop configuration"]
pub type SstmR = crate::FieldReader<Sstm>;
impl SstmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sstm> {
        match self.bits {
            0 => Some(Sstm::Value1),
            1 => Some(Sstm::Value2),
            2 => Some(Sstm::Value3),
            _ => None,
        }
    }
    #[doc = "External stop trigger is ignored"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sstm::Value1
    }
    #[doc = "Stops/Halts the slope generation"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sstm::Value2
    }
    #[doc = "Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Sstm::Value3
    }
}
#[doc = "Field `SSTM` writer - Slope external stop configuration"]
pub type SstmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sstm>;
impl<'a, REG> SstmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External stop trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sstm::Value1)
    }
    #[doc = "Stops/Halts the slope generation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sstm::Value2)
    }
    #[doc = "Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Sstm::Value3)
    }
}
#[doc = "Slope reference value mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Svsc {
    #[doc = "0: Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    Value1 = 0,
    #[doc = "1: The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    Value2 = 1,
    #[doc = "2: The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    Value3 = 2,
}
impl From<Svsc> for u8 {
    #[inline(always)]
    fn from(variant: Svsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Svsc {
    type Ux = u8;
}
#[doc = "Field `SVSC` reader - Slope reference value mode"]
pub type SvscR = crate::FieldReader<Svsc>;
impl SvscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Svsc> {
        match self.bits {
            0 => Some(Svsc::Value1),
            1 => Some(Svsc::Value2),
            2 => Some(Svsc::Value3),
            _ => None,
        }
    }
    #[doc = "Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Svsc::Value1
    }
    #[doc = "The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Svsc::Value2
    }
    #[doc = "The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Svsc::Value3
    }
}
#[doc = "Field `SVSC` writer - Slope reference value mode"]
pub type SvscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Svsc>;
impl<'a, REG> SvscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Svsc::Value1)
    }
    #[doc = "The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Svsc::Value2)
    }
    #[doc = "The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Svsc::Value3)
    }
}
#[doc = "Initial DAC start mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Swsm {
    #[doc = "0: CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    Value1 = 0,
    #[doc = "1: CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    Value2 = 1,
    #[doc = "2: CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    Value3 = 2,
    #[doc = "3: CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    Value4 = 3,
}
impl From<Swsm> for u8 {
    #[inline(always)]
    fn from(variant: Swsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Swsm {
    type Ux = u8;
}
#[doc = "Field `SWSM` reader - Initial DAC start mode"]
pub type SwsmR = crate::FieldReader<Swsm>;
impl SwsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swsm {
        match self.bits {
            0 => Swsm::Value1,
            1 => Swsm::Value2,
            2 => Swsm::Value3,
            3 => Swsm::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Swsm::Value1
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Swsm::Value2
    }
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Swsm::Value3
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Swsm::Value4
    }
}
#[doc = "Field `SWSM` writer - Initial DAC start mode"]
pub type SwsmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Swsm>;
impl<'a, REG> SwsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Swsm::Value1)
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Swsm::Value2)
    }
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Swsm::Value3)
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Swsm::Value4)
    }
}
#[doc = "Slope step gain configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gcfg {
    #[doc = "0: Each slope step has an increment/decrement of 1"]
    Value1 = 0,
    #[doc = "1: Each slope step has an increment/decrement of 2"]
    Value2 = 1,
    #[doc = "2: Each slope step has an increment/decrement of 4"]
    Value3 = 2,
    #[doc = "3: Each slope step has an increment/decrement of 8"]
    Value4 = 3,
}
impl From<Gcfg> for u8 {
    #[inline(always)]
    fn from(variant: Gcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gcfg {
    type Ux = u8;
}
#[doc = "Field `GCFG` reader - Slope step gain configuration"]
pub type GcfgR = crate::FieldReader<Gcfg>;
impl GcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcfg {
        match self.bits {
            0 => Gcfg::Value1,
            1 => Gcfg::Value2,
            2 => Gcfg::Value3,
            3 => Gcfg::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Each slope step has an increment/decrement of 1"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gcfg::Value1
    }
    #[doc = "Each slope step has an increment/decrement of 2"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gcfg::Value2
    }
    #[doc = "Each slope step has an increment/decrement of 4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Gcfg::Value3
    }
    #[doc = "Each slope step has an increment/decrement of 8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Gcfg::Value4
    }
}
#[doc = "Field `GCFG` writer - Slope step gain configuration"]
pub type GcfgW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gcfg>;
impl<'a, REG> GcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each slope step has an increment/decrement of 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gcfg::Value1)
    }
    #[doc = "Each slope step has an increment/decrement of 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gcfg::Value2)
    }
    #[doc = "Each slope step has an increment/decrement of 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Gcfg::Value3)
    }
    #[doc = "Each slope step has an increment/decrement of 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Gcfg::Value4)
    }
}
#[doc = "Field `IST` reader - Immediate shadow transfer"]
pub type IstR = crate::BitReader;
#[doc = "Field `IST` writer - Immediate shadow transfer"]
pub type IstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Pulse swallow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pse {
    #[doc = "0: Pulse swallow disabled"]
    Value1 = 0,
    #[doc = "1: Pulse swallow enabled"]
    Value2 = 1,
}
impl From<Pse> for bool {
    #[inline(always)]
    fn from(variant: Pse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSE` reader - Pulse swallow enable"]
pub type PseR = crate::BitReader<Pse>;
impl PseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pse {
        match self.bits {
            false => Pse::Value1,
            true => Pse::Value2,
        }
    }
    #[doc = "Pulse swallow disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pse::Value1
    }
    #[doc = "Pulse swallow enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pse::Value2
    }
}
#[doc = "Field `PSE` writer - Pulse swallow enable"]
pub type PseW<'a, REG> = crate::BitWriter<'a, REG, Pse>;
impl<'a, REG> PseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pulse swallow disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pse::Value1)
    }
    #[doc = "Pulse swallow enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pse::Value2)
    }
}
#[doc = "Pulse swallow window mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pswm {
    #[doc = "0: 16 clock cycle window"]
    Value1 = 0,
    #[doc = "1: 32 clock cycle window"]
    Value2 = 1,
    #[doc = "2: 64 clock cycle window"]
    Value3 = 2,
}
impl From<Pswm> for u8 {
    #[inline(always)]
    fn from(variant: Pswm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pswm {
    type Ux = u8;
}
#[doc = "Field `PSWM` reader - Pulse swallow window mode"]
pub type PswmR = crate::FieldReader<Pswm>;
impl PswmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pswm> {
        match self.bits {
            0 => Some(Pswm::Value1),
            1 => Some(Pswm::Value2),
            2 => Some(Pswm::Value3),
            _ => None,
        }
    }
    #[doc = "16 clock cycle window"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pswm::Value1
    }
    #[doc = "32 clock cycle window"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pswm::Value2
    }
    #[doc = "64 clock cycle window"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Pswm::Value3
    }
}
#[doc = "Field `PSWM` writer - Pulse swallow window mode"]
pub type PswmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pswm>;
impl<'a, REG> PswmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 clock cycle window"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pswm::Value1)
    }
    #[doc = "32 clock cycle window"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pswm::Value2)
    }
    #[doc = "64 clock cycle window"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Pswm::Value3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline(always)]
    pub fn psrm(&self) -> PsrmR {
        PsrmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline(always)]
    pub fn pstm(&self) -> PstmR {
        PstmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline(always)]
    pub fn fpd(&self) -> FpdR {
        FpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline(always)]
    pub fn psv(&self) -> PsvR {
        PsvR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline(always)]
    pub fn scm(&self) -> ScmR {
        ScmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline(always)]
    pub fn ssrm(&self) -> SsrmR {
        SsrmR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline(always)]
    pub fn sstm(&self) -> SstmR {
        SstmR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline(always)]
    pub fn svsc(&self) -> SvscR {
        SvscR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline(always)]
    pub fn swsm(&self) -> SwsmR {
        SwsmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline(always)]
    pub fn gcfg(&self) -> GcfgR {
        GcfgR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline(always)]
    pub fn ist(&self) -> IstR {
        IstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline(always)]
    pub fn pswm(&self) -> PswmR {
        PswmR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline(always)]
    #[must_use]
    pub fn psrm(&mut self) -> PsrmW<ScSpec> {
        PsrmW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pstm(&mut self) -> PstmW<ScSpec> {
        PstmW::new(self, 2)
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline(always)]
    #[must_use]
    pub fn fpd(&mut self) -> FpdW<ScSpec> {
        FpdW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline(always)]
    #[must_use]
    pub fn psv(&mut self) -> PsvW<ScSpec> {
        PsvW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline(always)]
    #[must_use]
    pub fn scm(&mut self) -> ScmW<ScSpec> {
        ScmW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ssrm(&mut self) -> SsrmW<ScSpec> {
        SsrmW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sstm(&mut self) -> SstmW<ScSpec> {
        SstmW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline(always)]
    #[must_use]
    pub fn svsc(&mut self) -> SvscW<ScSpec> {
        SvscW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline(always)]
    #[must_use]
    pub fn swsm(&mut self) -> SwsmW<ScSpec> {
        SwsmW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline(always)]
    #[must_use]
    pub fn gcfg(&mut self) -> GcfgW<ScSpec> {
        GcfgW::new(self, 18)
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ist(&mut self) -> IstW<ScSpec> {
        IstW::new(self, 20)
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PseW<ScSpec> {
        PseW::new(self, 21)
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline(always)]
    #[must_use]
    pub fn pswm(&mut self) -> PswmW<ScSpec> {
        PswmW::new(self, 24)
    }
}
#[doc = "Slope generation control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for ScSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {
    const RESET_VALUE: u32 = 0;
}
