#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSRM` reader - Prescaler external start configuration"]
pub type PSRM_R = crate::FieldReader<u8, PSRM_A>;
#[doc = "Prescaler external start configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSRM_A {
    #[doc = "0: External start trigger is ignored"]
    VALUE1 = 0,
    #[doc = "1: Start prescaler"]
    VALUE2 = 1,
    #[doc = "2: Clear prescaler"]
    VALUE3 = 2,
    #[doc = "3: Clear & Start prescaler"]
    VALUE4 = 3,
}
impl From<PSRM_A> for u8 {
    #[inline(always)]
    fn from(variant: PSRM_A) -> Self {
        variant as _
    }
}
impl PSRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSRM_A {
        match self.bits {
            0 => PSRM_A::VALUE1,
            1 => PSRM_A::VALUE2,
            2 => PSRM_A::VALUE3,
            3 => PSRM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSRM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PSRM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PSRM_A::VALUE4
    }
}
#[doc = "Field `PSRM` writer - Prescaler external start configuration"]
pub type PSRM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, PSRM_A, 2, O>;
impl<'a, const O: u8> PSRM_W<'a, O> {
    #[doc = "External start trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSRM_A::VALUE1)
    }
    #[doc = "Start prescaler"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSRM_A::VALUE2)
    }
    #[doc = "Clear prescaler"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSRM_A::VALUE3)
    }
    #[doc = "Clear & Start prescaler"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PSRM_A::VALUE4)
    }
}
#[doc = "Field `PSTM` reader - Prescaler external stop configuration"]
pub type PSTM_R = crate::FieldReader<u8, PSTM_A>;
#[doc = "Prescaler external stop configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSTM_A {
    #[doc = "0: External stop trigger is ignored"]
    VALUE1 = 0,
    #[doc = "1: Stop prescaler"]
    VALUE2 = 1,
    #[doc = "2: Clear prescaler"]
    VALUE3 = 2,
    #[doc = "3: Clear & Stop prescaler"]
    VALUE4 = 3,
}
impl From<PSTM_A> for u8 {
    #[inline(always)]
    fn from(variant: PSTM_A) -> Self {
        variant as _
    }
}
impl PSTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSTM_A {
        match self.bits {
            0 => PSTM_A::VALUE1,
            1 => PSTM_A::VALUE2,
            2 => PSTM_A::VALUE3,
            3 => PSTM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSTM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PSTM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PSTM_A::VALUE4
    }
}
#[doc = "Field `PSTM` writer - Prescaler external stop configuration"]
pub type PSTM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, PSTM_A, 2, O>;
impl<'a, const O: u8> PSTM_W<'a, O> {
    #[doc = "External stop trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSTM_A::VALUE1)
    }
    #[doc = "Stop prescaler"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSTM_A::VALUE2)
    }
    #[doc = "Clear prescaler"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSTM_A::VALUE3)
    }
    #[doc = "Clear & Stop prescaler"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PSTM_A::VALUE4)
    }
}
#[doc = "Field `FPD` reader - Fixed division disable"]
pub type FPD_R = crate::BitReader<FPD_A>;
#[doc = "Fixed division disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPD_A {
    #[doc = "0: Division by 4 enabled"]
    VALUE1 = 0,
    #[doc = "1: Division by 4 disabled"]
    VALUE2 = 1,
}
impl From<FPD_A> for bool {
    #[inline(always)]
    fn from(variant: FPD_A) -> Self {
        variant as u8 != 0
    }
}
impl FPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPD_A {
        match self.bits {
            false => FPD_A::VALUE1,
            true => FPD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FPD_A::VALUE2
    }
}
#[doc = "Field `FPD` writer - Fixed division disable"]
pub type FPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, FPD_A, O>;
impl<'a, const O: u8> FPD_W<'a, O> {
    #[doc = "Division by 4 enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FPD_A::VALUE1)
    }
    #[doc = "Division by 4 disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FPD_A::VALUE2)
    }
}
#[doc = "Field `PSV` reader - Prescaler division factor"]
pub type PSV_R = crate::FieldReader<u8, PSV_A>;
#[doc = "Prescaler division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSV_A {
    #[doc = "0: division by 1"]
    VALUE1 = 0,
    #[doc = "1: division by 2"]
    VALUE2 = 1,
    #[doc = "2: division by 4"]
    VALUE3 = 2,
    #[doc = "3: division by 8"]
    VALUE4 = 3,
}
impl From<PSV_A> for u8 {
    #[inline(always)]
    fn from(variant: PSV_A) -> Self {
        variant as _
    }
}
impl PSV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSV_A {
        match self.bits {
            0 => PSV_A::VALUE1,
            1 => PSV_A::VALUE2,
            2 => PSV_A::VALUE3,
            3 => PSV_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PSV_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PSV_A::VALUE4
    }
}
#[doc = "Field `PSV` writer - Prescaler division factor"]
pub type PSV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, PSV_A, 2, O>;
impl<'a, const O: u8> PSV_W<'a, O> {
    #[doc = "division by 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSV_A::VALUE1)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSV_A::VALUE2)
    }
    #[doc = "division by 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSV_A::VALUE3)
    }
    #[doc = "division by 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PSV_A::VALUE4)
    }
}
#[doc = "Field `SCM` reader - Slope control mode"]
pub type SCM_R = crate::FieldReader<u8, SCM_A>;
#[doc = "Slope control mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCM_A {
    #[doc = "0: Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    VALUE1 = 0,
    #[doc = "1: Decrementing slope generation."]
    VALUE2 = 1,
    #[doc = "2: Incrementing slope generation."]
    VALUE3 = 2,
    #[doc = "3: Triangular slope generation."]
    VALUE4 = 3,
}
impl From<SCM_A> for u8 {
    #[inline(always)]
    fn from(variant: SCM_A) -> Self {
        variant as _
    }
}
impl SCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCM_A {
        match self.bits {
            0 => SCM_A::VALUE1,
            1 => SCM_A::VALUE2,
            2 => SCM_A::VALUE3,
            3 => SCM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SCM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SCM_A::VALUE4
    }
}
#[doc = "Field `SCM` writer - Slope control mode"]
pub type SCM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, SCM_A, 2, O>;
impl<'a, const O: u8> SCM_W<'a, O> {
    #[doc = "Slope generation disabled. Used when the switch between the two reference values, CSGyDSV1This register contains the actual value used for the DSV1 reference. and CSGyDSV2This register contains the actual value used for the DSV2 reference. is done via external signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SCM_A::VALUE1)
    }
    #[doc = "Decrementing slope generation."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SCM_A::VALUE2)
    }
    #[doc = "Incrementing slope generation."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SCM_A::VALUE3)
    }
    #[doc = "Triangular slope generation."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SCM_A::VALUE4)
    }
}
#[doc = "Field `SSRM` reader - Slope external start configuration"]
pub type SSRM_R = crate::FieldReader<u8, SSRM_A>;
#[doc = "Slope external start configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSRM_A {
    #[doc = "0: External start trigger is ignored"]
    VALUE1 = 0,
    #[doc = "1: Start/restart slope generation"]
    VALUE2 = 1,
    #[doc = "2: Resumes slope"]
    VALUE3 = 2,
}
impl From<SSRM_A> for u8 {
    #[inline(always)]
    fn from(variant: SSRM_A) -> Self {
        variant as _
    }
}
impl SSRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSRM_A> {
        match self.bits {
            0 => Some(SSRM_A::VALUE1),
            1 => Some(SSRM_A::VALUE2),
            2 => Some(SSRM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SSRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSRM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SSRM_A::VALUE3
    }
}
#[doc = "Field `SSRM` writer - Slope external start configuration"]
pub type SSRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SC_SPEC, u8, SSRM_A, 2, O>;
impl<'a, const O: u8> SSRM_W<'a, O> {
    #[doc = "External start trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSRM_A::VALUE1)
    }
    #[doc = "Start/restart slope generation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSRM_A::VALUE2)
    }
    #[doc = "Resumes slope"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSRM_A::VALUE3)
    }
}
#[doc = "Field `SSTM` reader - Slope external stop configuration"]
pub type SSTM_R = crate::FieldReader<u8, SSTM_A>;
#[doc = "Slope external stop configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSTM_A {
    #[doc = "0: External stop trigger is ignored"]
    VALUE1 = 0,
    #[doc = "1: Stops/Halts the slope generation"]
    VALUE2 = 1,
    #[doc = "2: Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    VALUE3 = 2,
}
impl From<SSTM_A> for u8 {
    #[inline(always)]
    fn from(variant: SSTM_A) -> Self {
        variant as _
    }
}
impl SSTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSTM_A> {
        match self.bits {
            0 => Some(SSTM_A::VALUE1),
            1 => Some(SSTM_A::VALUE2),
            2 => Some(SSTM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SSTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSTM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SSTM_A::VALUE3
    }
}
#[doc = "Field `SSTM` writer - Slope external stop configuration"]
pub type SSTM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SC_SPEC, u8, SSTM_A, 2, O>;
impl<'a, const O: u8> SSTM_W<'a, O> {
    #[doc = "External stop trigger is ignored"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSTM_A::VALUE1)
    }
    #[doc = "Stops/Halts the slope generation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSTM_A::VALUE2)
    }
    #[doc = "Used in hybrid mode. It freezes the slope generation and feeds constantly the value programmed in CSGyDSV2This register contains the actual value used for the DSV2 reference. to the DAC."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSTM_A::VALUE3)
    }
}
#[doc = "Field `SVSC` reader - Slope reference value mode"]
pub type SVSC_R = crate::FieldReader<u8, SVSC_A>;
#[doc = "Slope reference value mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVSC_A {
    #[doc = "0: Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    VALUE1 = 0,
    #[doc = "1: The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    VALUE2 = 1,
    #[doc = "2: The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    VALUE3 = 2,
}
impl From<SVSC_A> for u8 {
    #[inline(always)]
    fn from(variant: SVSC_A) -> Self {
        variant as _
    }
}
impl SVSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SVSC_A> {
        match self.bits {
            0 => Some(SVSC_A::VALUE1),
            1 => Some(SVSC_A::VALUE2),
            2 => Some(SVSC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SVSC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SVSC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SVSC_A::VALUE3
    }
}
#[doc = "Field `SVSC` writer - Slope reference value mode"]
pub type SVSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SC_SPEC, u8, SVSC_A, 2, O>;
impl<'a, const O: u8> SVSC_W<'a, O> {
    #[doc = "Only CSGyDSV1This register contains the actual value used for the DSV1 reference. value is used for the slope generation: if slope is incrementing, CSGyDSV1This register contains the actual value used for the DSV1 reference. is the bottom reference value from where the ramp starts; if decrementing, then CSGyDSV1This register contains the actual value used for the DSV1 reference. is the upper reference value from where the ramp starts."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVSC_A::VALUE1)
    }
    #[doc = "The two reference values are being used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the ramp starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as a static value (this value is constantly fed to the DAC after a stop trigger as been detected)."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVSC_A::VALUE2)
    }
    #[doc = "The two reference values are used: CSGyDSV1This register contains the actual value used for the DSV1 reference. is the low or high reference value from where the slope starts (incrementing or decrementing respectively); CSGyDSV2This register contains the actual value used for the DSV2 reference. is used as an internal re start condition for the slope."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SVSC_A::VALUE3)
    }
}
#[doc = "Field `SWSM` reader - Initial DAC start mode"]
pub type SWSM_R = crate::FieldReader<u8, SWSM_A>;
#[doc = "Initial DAC start mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSM_A {
    #[doc = "0: CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    VALUE1 = 0,
    #[doc = "1: CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    VALUE2 = 1,
    #[doc = "2: CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    VALUE3 = 2,
    #[doc = "3: CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    VALUE4 = 3,
}
impl From<SWSM_A> for u8 {
    #[inline(always)]
    fn from(variant: SWSM_A) -> Self {
        variant as _
    }
}
impl SWSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSM_A {
        match self.bits {
            0 => SWSM_A::VALUE1,
            1 => SWSM_A::VALUE2,
            2 => SWSM_A::VALUE3,
            3 => SWSM_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SWSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SWSM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SWSM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SWSM_A::VALUE4
    }
}
#[doc = "Field `SWSM` writer - Initial DAC start mode"]
pub type SWSM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, SWSM_A, 2, O>;
impl<'a, const O: u8> SWSM_W<'a, O> {
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SWSM_A::VALUE1)
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC and initial conversion trigger is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SWSM_A::VALUE2)
    }
    #[doc = "CSGyDSV2This register contains the actual value used for the DSV2 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SWSM_A::VALUE3)
    }
    #[doc = "CSGyDSV1This register contains the actual value used for the DSV1 reference. is fed to the DAC but initial conversion trigger is not generated."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SWSM_A::VALUE4)
    }
}
#[doc = "Field `GCFG` reader - Slope step gain configuration"]
pub type GCFG_R = crate::FieldReader<u8, GCFG_A>;
#[doc = "Slope step gain configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GCFG_A {
    #[doc = "0: Each slope step has an increment/decrement of 1"]
    VALUE1 = 0,
    #[doc = "1: Each slope step has an increment/decrement of 2"]
    VALUE2 = 1,
    #[doc = "2: Each slope step has an increment/decrement of 4"]
    VALUE3 = 2,
    #[doc = "3: Each slope step has an increment/decrement of 8"]
    VALUE4 = 3,
}
impl From<GCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: GCFG_A) -> Self {
        variant as _
    }
}
impl GCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCFG_A {
        match self.bits {
            0 => GCFG_A::VALUE1,
            1 => GCFG_A::VALUE2,
            2 => GCFG_A::VALUE3,
            3 => GCFG_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == GCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == GCFG_A::VALUE4
    }
}
#[doc = "Field `GCFG` writer - Slope step gain configuration"]
pub type GCFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, GCFG_A, 2, O>;
impl<'a, const O: u8> GCFG_W<'a, O> {
    #[doc = "Each slope step has an increment/decrement of 1"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GCFG_A::VALUE1)
    }
    #[doc = "Each slope step has an increment/decrement of 2"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GCFG_A::VALUE2)
    }
    #[doc = "Each slope step has an increment/decrement of 4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(GCFG_A::VALUE3)
    }
    #[doc = "Each slope step has an increment/decrement of 8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(GCFG_A::VALUE4)
    }
}
#[doc = "Field `IST` reader - Immediate shadow transfer"]
pub type IST_R = crate::BitReader<bool>;
#[doc = "Field `IST` writer - Immediate shadow transfer"]
pub type IST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
#[doc = "Field `PSE` reader - Pulse swallow enable"]
pub type PSE_R = crate::BitReader<PSE_A>;
#[doc = "Pulse swallow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSE_A {
    #[doc = "0: Pulse swallow disabled"]
    VALUE1 = 0,
    #[doc = "1: Pulse swallow enabled"]
    VALUE2 = 1,
}
impl From<PSE_A> for bool {
    #[inline(always)]
    fn from(variant: PSE_A) -> Self {
        variant as u8 != 0
    }
}
impl PSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSE_A {
        match self.bits {
            false => PSE_A::VALUE1,
            true => PSE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSE_A::VALUE2
    }
}
#[doc = "Field `PSE` writer - Pulse swallow enable"]
pub type PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, PSE_A, O>;
impl<'a, const O: u8> PSE_W<'a, O> {
    #[doc = "Pulse swallow disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSE_A::VALUE1)
    }
    #[doc = "Pulse swallow enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSE_A::VALUE2)
    }
}
#[doc = "Field `PSWM` reader - Pulse swallow window mode"]
pub type PSWM_R = crate::FieldReader<u8, PSWM_A>;
#[doc = "Pulse swallow window mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSWM_A {
    #[doc = "0: 16 clock cycle window"]
    VALUE1 = 0,
    #[doc = "1: 32 clock cycle window"]
    VALUE2 = 1,
    #[doc = "2: 64 clock cycle window"]
    VALUE3 = 2,
}
impl From<PSWM_A> for u8 {
    #[inline(always)]
    fn from(variant: PSWM_A) -> Self {
        variant as _
    }
}
impl PSWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSWM_A> {
        match self.bits {
            0 => Some(PSWM_A::VALUE1),
            1 => Some(PSWM_A::VALUE2),
            2 => Some(PSWM_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSWM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSWM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PSWM_A::VALUE3
    }
}
#[doc = "Field `PSWM` writer - Pulse swallow window mode"]
pub type PSWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SC_SPEC, u8, PSWM_A, 2, O>;
impl<'a, const O: u8> PSWM_W<'a, O> {
    #[doc = "16 clock cycle window"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSWM_A::VALUE1)
    }
    #[doc = "32 clock cycle window"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSWM_A::VALUE2)
    }
    #[doc = "64 clock cycle window"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PSWM_A::VALUE3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline(always)]
    pub fn psrm(&self) -> PSRM_R {
        PSRM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline(always)]
    pub fn pstm(&self) -> PSTM_R {
        PSTM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline(always)]
    pub fn fpd(&self) -> FPD_R {
        FPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline(always)]
    pub fn psv(&self) -> PSV_R {
        PSV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline(always)]
    pub fn scm(&self) -> SCM_R {
        SCM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline(always)]
    pub fn ssrm(&self) -> SSRM_R {
        SSRM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline(always)]
    pub fn sstm(&self) -> SSTM_R {
        SSTM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline(always)]
    pub fn svsc(&self) -> SVSC_R {
        SVSC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline(always)]
    pub fn swsm(&self) -> SWSM_R {
        SWSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline(always)]
    pub fn gcfg(&self) -> GCFG_R {
        GCFG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline(always)]
    pub fn ist(&self) -> IST_R {
        IST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline(always)]
    pub fn pswm(&self) -> PSWM_R {
        PSWM_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline(always)]
    #[must_use]
    pub fn psrm(&mut self) -> PSRM_W<0> {
        PSRM_W::new(self)
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pstm(&mut self) -> PSTM_W<2> {
        PSTM_W::new(self)
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline(always)]
    #[must_use]
    pub fn fpd(&mut self) -> FPD_W<4> {
        FPD_W::new(self)
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline(always)]
    #[must_use]
    pub fn psv(&mut self) -> PSV_W<5> {
        PSV_W::new(self)
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline(always)]
    #[must_use]
    pub fn scm(&mut self) -> SCM_W<8> {
        SCM_W::new(self)
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline(always)]
    #[must_use]
    pub fn ssrm(&mut self) -> SSRM_W<10> {
        SSRM_W::new(self)
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline(always)]
    #[must_use]
    pub fn sstm(&mut self) -> SSTM_W<12> {
        SSTM_W::new(self)
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline(always)]
    #[must_use]
    pub fn svsc(&mut self) -> SVSC_W<14> {
        SVSC_W::new(self)
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline(always)]
    #[must_use]
    pub fn swsm(&mut self) -> SWSM_W<16> {
        SWSM_W::new(self)
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline(always)]
    #[must_use]
    pub fn gcfg(&mut self) -> GCFG_W<18> {
        GCFG_W::new(self)
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ist(&mut self) -> IST_W<20> {
        IST_W::new(self)
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<21> {
        PSE_W::new(self)
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline(always)]
    #[must_use]
    pub fn pswm(&mut self) -> PSWM_W<24> {
        PSWM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope generation control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
