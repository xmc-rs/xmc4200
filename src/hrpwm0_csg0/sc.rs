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
#[doc = "Prescaler external start configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PSRM` reader - Prescaler external start configuration"]
pub struct PSRM_R(crate::FieldReader<u8, PSRM_A>);
impl PSRM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PSRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSRM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PSRM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PSRM_A::VALUE4
    }
}
impl core::ops::Deref for PSRM_R {
    type Target = crate::FieldReader<u8, PSRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSRM` writer - Prescaler external start configuration"]
pub struct PSRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSRM_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Prescaler external stop configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PSTM` reader - Prescaler external stop configuration"]
pub struct PSTM_R(crate::FieldReader<u8, PSTM_A>);
impl PSTM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSTM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PSTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSTM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PSTM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PSTM_A::VALUE4
    }
}
impl core::ops::Deref for PSTM_R {
    type Target = crate::FieldReader<u8, PSTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSTM` writer - Prescaler external stop configuration"]
pub struct PSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSTM_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Fixed division disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FPD` reader - Fixed division disable"]
pub struct FPD_R(crate::FieldReader<bool, FPD_A>);
impl FPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == FPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FPD_A::VALUE2
    }
}
impl core::ops::Deref for FPD_R {
    type Target = crate::FieldReader<bool, FPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPD` writer - Fixed division disable"]
pub struct FPD_W<'a> {
    w: &'a mut W,
}
impl<'a> FPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Prescaler division factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PSV` reader - Prescaler division factor"]
pub struct PSV_R(crate::FieldReader<u8, PSV_A>);
impl PSV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PSV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSV_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PSV_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == PSV_A::VALUE4
    }
}
impl core::ops::Deref for PSV_R {
    type Target = crate::FieldReader<u8, PSV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSV` writer - Prescaler division factor"]
pub struct PSV_W<'a> {
    w: &'a mut W,
}
impl<'a> PSV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSV_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Slope control mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SCM` reader - Slope control mode"]
pub struct SCM_R(crate::FieldReader<u8, SCM_A>);
impl SCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SCM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SCM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SCM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SCM_A::VALUE4
    }
}
impl core::ops::Deref for SCM_R {
    type Target = crate::FieldReader<u8, SCM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCM` writer - Slope control mode"]
pub struct SCM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCM_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Slope external start configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SSRM` reader - Slope external start configuration"]
pub struct SSRM_R(crate::FieldReader<u8, SSRM_A>);
impl SSRM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSRM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SSRM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SSRM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SSRM_A::VALUE3
    }
}
impl core::ops::Deref for SSRM_R {
    type Target = crate::FieldReader<u8, SSRM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSRM` writer - Slope external start configuration"]
pub struct SSRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSRM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Slope external stop configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SSTM` reader - Slope external stop configuration"]
pub struct SSTM_R(crate::FieldReader<u8, SSTM_A>);
impl SSTM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSTM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SSTM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SSTM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SSTM_A::VALUE3
    }
}
impl core::ops::Deref for SSTM_R {
    type Target = crate::FieldReader<u8, SSTM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTM` writer - Slope external stop configuration"]
pub struct SSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSTM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Slope reference value mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SVSC` reader - Slope reference value mode"]
pub struct SVSC_R(crate::FieldReader<u8, SVSC_A>);
impl SVSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SVSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SVSC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SVSC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SVSC_A::VALUE3
    }
}
impl core::ops::Deref for SVSC_R {
    type Target = crate::FieldReader<u8, SVSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSC` writer - Slope reference value mode"]
pub struct SVSC_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Initial DAC start mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SWSM` reader - Initial DAC start mode"]
pub struct SWSM_R(crate::FieldReader<u8, SWSM_A>);
impl SWSM_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWSM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SWSM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SWSM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SWSM_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SWSM_A::VALUE4
    }
}
impl core::ops::Deref for SWSM_R {
    type Target = crate::FieldReader<u8, SWSM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWSM` writer - Initial DAC start mode"]
pub struct SWSM_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWSM_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Slope step gain configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `GCFG` reader - Slope step gain configuration"]
pub struct GCFG_R(crate::FieldReader<u8, GCFG_A>);
impl GCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        GCFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == GCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == GCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == GCFG_A::VALUE4
    }
}
impl core::ops::Deref for GCFG_R {
    type Target = crate::FieldReader<u8, GCFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCFG` writer - Slope step gain configuration"]
pub struct GCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCFG_A) -> &'a mut W {
        self.bits(variant.into())
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `IST` reader - Immediate shadow transfer"]
pub struct IST_R(crate::FieldReader<bool, bool>);
impl IST_R {
    pub(crate) fn new(bits: bool) -> Self {
        IST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IST` writer - Immediate shadow transfer"]
pub struct IST_W<'a> {
    w: &'a mut W,
}
impl<'a> IST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Pulse swallow enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PSE` reader - Pulse swallow enable"]
pub struct PSE_R(crate::FieldReader<bool, PSE_A>);
impl PSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PSE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSE_A::VALUE2
    }
}
impl core::ops::Deref for PSE_R {
    type Target = crate::FieldReader<bool, PSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSE` writer - Pulse swallow enable"]
pub struct PSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSE_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Pulse swallow window mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `PSWM` reader - Pulse swallow window mode"]
pub struct PSWM_R(crate::FieldReader<u8, PSWM_A>);
impl PSWM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSWM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == PSWM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSWM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PSWM_A::VALUE3
    }
}
impl core::ops::Deref for PSWM_R {
    type Target = crate::FieldReader<u8, PSWM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSWM` writer - Pulse swallow window mode"]
pub struct PSWM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSWM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline(always)]
    pub fn psrm(&self) -> PSRM_R {
        PSRM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline(always)]
    pub fn pstm(&self) -> PSTM_R {
        PSTM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline(always)]
    pub fn fpd(&self) -> FPD_R {
        FPD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline(always)]
    pub fn psv(&self) -> PSV_R {
        PSV_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline(always)]
    pub fn scm(&self) -> SCM_R {
        SCM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline(always)]
    pub fn ssrm(&self) -> SSRM_R {
        SSRM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline(always)]
    pub fn sstm(&self) -> SSTM_R {
        SSTM_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline(always)]
    pub fn svsc(&self) -> SVSC_R {
        SVSC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline(always)]
    pub fn swsm(&self) -> SWSM_R {
        SWSM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline(always)]
    pub fn gcfg(&self) -> GCFG_R {
        GCFG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline(always)]
    pub fn ist(&self) -> IST_R {
        IST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline(always)]
    pub fn pswm(&self) -> PSWM_R {
        PSWM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaler external start configuration"]
    #[inline(always)]
    pub fn psrm(&mut self) -> PSRM_W {
        PSRM_W { w: self }
    }
    #[doc = "Bits 2:3 - Prescaler external stop configuration"]
    #[inline(always)]
    pub fn pstm(&mut self) -> PSTM_W {
        PSTM_W { w: self }
    }
    #[doc = "Bit 4 - Fixed division disable"]
    #[inline(always)]
    pub fn fpd(&mut self) -> FPD_W {
        FPD_W { w: self }
    }
    #[doc = "Bits 5:6 - Prescaler division factor"]
    #[inline(always)]
    pub fn psv(&mut self) -> PSV_W {
        PSV_W { w: self }
    }
    #[doc = "Bits 8:9 - Slope control mode"]
    #[inline(always)]
    pub fn scm(&mut self) -> SCM_W {
        SCM_W { w: self }
    }
    #[doc = "Bits 10:11 - Slope external start configuration"]
    #[inline(always)]
    pub fn ssrm(&mut self) -> SSRM_W {
        SSRM_W { w: self }
    }
    #[doc = "Bits 12:13 - Slope external stop configuration"]
    #[inline(always)]
    pub fn sstm(&mut self) -> SSTM_W {
        SSTM_W { w: self }
    }
    #[doc = "Bits 14:15 - Slope reference value mode"]
    #[inline(always)]
    pub fn svsc(&mut self) -> SVSC_W {
        SVSC_W { w: self }
    }
    #[doc = "Bits 16:17 - Initial DAC start mode"]
    #[inline(always)]
    pub fn swsm(&mut self) -> SWSM_W {
        SWSM_W { w: self }
    }
    #[doc = "Bits 18:19 - Slope step gain configuration"]
    #[inline(always)]
    pub fn gcfg(&mut self) -> GCFG_W {
        GCFG_W { w: self }
    }
    #[doc = "Bit 20 - Immediate shadow transfer"]
    #[inline(always)]
    pub fn ist(&mut self) -> IST_W {
        IST_W { w: self }
    }
    #[doc = "Bit 21 - Pulse swallow enable"]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W {
        PSE_W { w: self }
    }
    #[doc = "Bits 24:25 - Pulse swallow window mode"]
    #[inline(always)]
    pub fn pswm(&mut self) -> PSWM_W {
        PSWM_W { w: self }
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
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
