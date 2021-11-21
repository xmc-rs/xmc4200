#[doc = "Register `PLC` reader"]
pub struct R(crate::R<PLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLC` writer"]
pub struct W(crate::W<PLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLC_SPEC>;
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
impl From<crate::W<PLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clamping control signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IPLS_A {
    #[doc = "0: HRPWMx.BLyA"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.BLyB"]
    VALUE2 = 1,
    #[doc = "2: HRPWMx.BLyC"]
    VALUE3 = 2,
    #[doc = "3: HRPWMx.BLyD"]
    VALUE4 = 3,
    #[doc = "4: HRPWMx.BLyE"]
    VALUE5 = 4,
    #[doc = "5: HRPWMx.BLyF"]
    VALUE6 = 5,
    #[doc = "6: HRPWMx.BLyG"]
    VALUE7 = 6,
    #[doc = "7: HRPWMx.BLyH"]
    VALUE8 = 7,
    #[doc = "8: HRPWMx.BLyI"]
    VALUE9 = 8,
    #[doc = "9: HRPWMx.BLyJ"]
    VALUE10 = 9,
    #[doc = "10: HRPWMx.BLyK"]
    VALUE11 = 10,
    #[doc = "11: HRPWMx.BLyL"]
    VALUE12 = 11,
    #[doc = "12: HRPWMx.BLyM"]
    VALUE13 = 12,
    #[doc = "13: HRPWMx.BLyN"]
    VALUE14 = 13,
    #[doc = "14: HRPWMx.BLyO"]
    VALUE15 = 14,
    #[doc = "15: HRPWMx.BLyP"]
    VALUE16 = 15,
}
impl From<IPLS_A> for u8 {
    #[inline(always)]
    fn from(variant: IPLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IPLS` reader - Clamping control signal selector"]
pub struct IPLS_R(crate::FieldReader<u8, IPLS_A>);
impl IPLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IPLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPLS_A {
        match self.bits {
            0 => IPLS_A::VALUE1,
            1 => IPLS_A::VALUE2,
            2 => IPLS_A::VALUE3,
            3 => IPLS_A::VALUE4,
            4 => IPLS_A::VALUE5,
            5 => IPLS_A::VALUE6,
            6 => IPLS_A::VALUE7,
            7 => IPLS_A::VALUE8,
            8 => IPLS_A::VALUE9,
            9 => IPLS_A::VALUE10,
            10 => IPLS_A::VALUE11,
            11 => IPLS_A::VALUE12,
            12 => IPLS_A::VALUE13,
            13 => IPLS_A::VALUE14,
            14 => IPLS_A::VALUE15,
            15 => IPLS_A::VALUE16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == IPLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == IPLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == IPLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == IPLS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == IPLS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == IPLS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == IPLS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        **self == IPLS_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        **self == IPLS_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        **self == IPLS_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        **self == IPLS_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        **self == IPLS_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        **self == IPLS_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        **self == IPLS_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        **self == IPLS_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        **self == IPLS_A::VALUE16
    }
}
impl core::ops::Deref for IPLS_R {
    type Target = crate::FieldReader<u8, IPLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPLS` writer - Clamping control signal selector"]
pub struct IPLS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HRPWMx.BLyA"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE1)
    }
    #[doc = "HRPWMx.BLyB"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE2)
    }
    #[doc = "HRPWMx.BLyC"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE3)
    }
    #[doc = "HRPWMx.BLyD"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE4)
    }
    #[doc = "HRPWMx.BLyE"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE5)
    }
    #[doc = "HRPWMx.BLyF"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE6)
    }
    #[doc = "HRPWMx.BLyG"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE7)
    }
    #[doc = "HRPWMx.BLyH"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE8)
    }
    #[doc = "HRPWMx.BLyI"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE9)
    }
    #[doc = "HRPWMx.BLyJ"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE10)
    }
    #[doc = "HRPWMx.BLyK"]
    #[inline(always)]
    pub fn value11(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE11)
    }
    #[doc = "HRPWMx.BLyL"]
    #[inline(always)]
    pub fn value12(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE12)
    }
    #[doc = "HRPWMx.BLyM"]
    #[inline(always)]
    pub fn value13(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE13)
    }
    #[doc = "HRPWMx.BLyN"]
    #[inline(always)]
    pub fn value14(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE14)
    }
    #[doc = "HRPWMx.BLyO"]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE15)
    }
    #[doc = "HRPWMx.BLyP"]
    #[inline(always)]
    pub fn value16(self) -> &'a mut W {
        self.variant(IPLS_A::VALUE16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Clamping control signal level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLCL_A {
    #[doc = "0: Clamping control disabled"]
    VALUE1 = 0,
    #[doc = "1: Output is set to clamped level when the control signal is HIGH"]
    VALUE2 = 1,
    #[doc = "2: Output is set to clamped level when the control signal is LOW"]
    VALUE3 = 2,
}
impl From<PLCL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLCL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLCL` reader - Clamping control signal level selection"]
pub struct PLCL_R(crate::FieldReader<u8, PLCL_A>);
impl PLCL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLCL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLCL_A> {
        match self.bits {
            0 => Some(PLCL_A::VALUE1),
            1 => Some(PLCL_A::VALUE2),
            2 => Some(PLCL_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PLCL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PLCL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PLCL_A::VALUE3
    }
}
impl core::ops::Deref for PLCL_R {
    type Target = crate::FieldReader<u8, PLCL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLCL` writer - Clamping control signal level selection"]
pub struct PLCL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLCL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clamping control disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLCL_A::VALUE1)
    }
    #[doc = "Output is set to clamped level when the control signal is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLCL_A::VALUE2)
    }
    #[doc = "Output is set to clamped level when the control signal is LOW"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLCL_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Output passive level value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL_A {
    #[doc = "0: Output clamped level is LOW"]
    VALUE1 = 0,
    #[doc = "1: Output clamped level is HIGH"]
    VALUE2 = 1,
}
impl From<PSL_A> for bool {
    #[inline(always)]
    fn from(variant: PSL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL` reader - Output passive level value"]
pub struct PSL_R(crate::FieldReader<bool, PSL_A>);
impl PSL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL_A {
        match self.bits {
            false => PSL_A::VALUE1,
            true => PSL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PSL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PSL_A::VALUE2
    }
}
impl core::ops::Deref for PSL_R {
    type Target = crate::FieldReader<bool, PSL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSL` writer - Output passive level value"]
pub struct PSL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Output clamped level is LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL_A::VALUE1)
    }
    #[doc = "Output clamped level is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL_A::VALUE2)
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
#[doc = "Clamped state exit SW configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLSW_A {
    #[doc = "0: External signal and SW can remove the output from the clamped state"]
    VALUE1 = 0,
    #[doc = "1: Only SW can remove the output from the clamped state"]
    VALUE2 = 1,
}
impl From<PLSW_A> for bool {
    #[inline(always)]
    fn from(variant: PLSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLSW` reader - Clamped state exit SW configuration"]
pub struct PLSW_R(crate::FieldReader<bool, PLSW_A>);
impl PLSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLSW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLSW_A {
        match self.bits {
            false => PLSW_A::VALUE1,
            true => PLSW_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PLSW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PLSW_A::VALUE2
    }
}
impl core::ops::Deref for PLSW_R {
    type Target = crate::FieldReader<bool, PLSW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLSW` writer - Clamped state exit SW configuration"]
pub struct PLSW_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLSW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External signal and SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLSW_A::VALUE1)
    }
    #[doc = "Only SW can remove the output from the clamped state"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLSW_A::VALUE2)
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
#[doc = "Passive level enter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLEC_A {
    #[doc = "0: Passive level is entered immediately"]
    VALUE1 = 0,
    #[doc = "1: Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2 = 1,
    #[doc = "2: Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3 = 2,
}
impl From<PLEC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLEC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLEC` reader - Passive level enter configuration"]
pub struct PLEC_R(crate::FieldReader<u8, PLEC_A>);
impl PLEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLEC_A> {
        match self.bits {
            0 => Some(PLEC_A::VALUE1),
            1 => Some(PLEC_A::VALUE2),
            2 => Some(PLEC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PLEC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PLEC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PLEC_A::VALUE3
    }
}
impl core::ops::Deref for PLEC_R {
    type Target = crate::FieldReader<u8, PLEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLEC` writer - Passive level enter configuration"]
pub struct PLEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLEC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Passive level is entered immediately"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLEC_A::VALUE1)
    }
    #[doc = "Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLEC_A::VALUE2)
    }
    #[doc = "Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLEC_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Passive level exit configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLXC_A {
    #[doc = "0: Passive level is exit immediately"]
    VALUE1 = 0,
    #[doc = "1: Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2 = 1,
    #[doc = "2: Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3 = 2,
}
impl From<PLXC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLXC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLXC` reader - Passive level exit configuration"]
pub struct PLXC_R(crate::FieldReader<u8, PLXC_A>);
impl PLXC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLXC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLXC_A> {
        match self.bits {
            0 => Some(PLXC_A::VALUE1),
            1 => Some(PLXC_A::VALUE2),
            2 => Some(PLXC_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PLXC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PLXC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == PLXC_A::VALUE3
    }
}
impl core::ops::Deref for PLXC_R {
    type Target = crate::FieldReader<u8, PLXC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLXC` writer - Passive level exit configuration"]
pub struct PLXC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLXC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLXC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Passive level is exit immediately"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLXC_A::VALUE1)
    }
    #[doc = "Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLXC_A::VALUE2)
    }
    #[doc = "Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLXC_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline(always)]
    pub fn ipls(&self) -> IPLS_R {
        IPLS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline(always)]
    pub fn plcl(&self) -> PLCL_R {
        PLCL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline(always)]
    pub fn psl(&self) -> PSL_R {
        PSL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline(always)]
    pub fn plsw(&self) -> PLSW_R {
        PLSW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline(always)]
    pub fn plec(&self) -> PLEC_R {
        PLEC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline(always)]
    pub fn plxc(&self) -> PLXC_R {
        PLXC_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clamping control signal selector"]
    #[inline(always)]
    pub fn ipls(&mut self) -> IPLS_W {
        IPLS_W { w: self }
    }
    #[doc = "Bits 8:9 - Clamping control signal level selection"]
    #[inline(always)]
    pub fn plcl(&mut self) -> PLCL_W {
        PLCL_W { w: self }
    }
    #[doc = "Bit 10 - Output passive level value"]
    #[inline(always)]
    pub fn psl(&mut self) -> PSL_W {
        PSL_W { w: self }
    }
    #[doc = "Bit 11 - Clamped state exit SW configuration"]
    #[inline(always)]
    pub fn plsw(&mut self) -> PLSW_W {
        PLSW_W { w: self }
    }
    #[doc = "Bits 12:13 - Passive level enter configuration"]
    #[inline(always)]
    pub fn plec(&mut self) -> PLEC_W {
        PLEC_W { w: self }
    }
    #[doc = "Bits 14:15 - Passive level exit configuration"]
    #[inline(always)]
    pub fn plxc(&mut self) -> PLXC_W {
        PLXC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Passive level configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plc](index.html) module"]
pub struct PLC_SPEC;
impl crate::RegisterSpec for PLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plc::R](R) reader structure"]
impl crate::Readable for PLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plc::W](W) writer structure"]
impl crate::Writable for PLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLC to value 0"]
impl crate::Resettable for PLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
