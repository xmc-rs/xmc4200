#[doc = "Reader of register PLC"]
pub type R = crate::R<u32, super::PLC>;
#[doc = "Writer for register PLC"]
pub type W = crate::W<u32, super::PLC>;
#[doc = "Register PLC `reset()`'s with value 0"]
impl crate::ResetValue for super::PLC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clamping control signal selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPLS_A {
    #[doc = "0: HRPWMx.BLyA"]
    VALUE1,
    #[doc = "1: HRPWMx.BLyB"]
    VALUE2,
    #[doc = "2: HRPWMx.BLyC"]
    VALUE3,
    #[doc = "3: HRPWMx.BLyD"]
    VALUE4,
    #[doc = "4: HRPWMx.BLyE"]
    VALUE5,
    #[doc = "5: HRPWMx.BLyF"]
    VALUE6,
    #[doc = "6: HRPWMx.BLyG"]
    VALUE7,
    #[doc = "7: HRPWMx.BLyH"]
    VALUE8,
    #[doc = "8: HRPWMx.BLyI"]
    VALUE9,
    #[doc = "9: HRPWMx.BLyJ"]
    VALUE10,
    #[doc = "10: HRPWMx.BLyK"]
    VALUE11,
    #[doc = "11: HRPWMx.BLyL"]
    VALUE12,
    #[doc = "12: HRPWMx.BLyM"]
    VALUE13,
    #[doc = "13: HRPWMx.BLyN"]
    VALUE14,
    #[doc = "14: HRPWMx.BLyO"]
    VALUE15,
    #[doc = "15: HRPWMx.BLyP"]
    VALUE16,
}
impl From<IPLS_A> for u8 {
    #[inline(always)]
    fn from(variant: IPLS_A) -> Self {
        match variant {
            IPLS_A::VALUE1 => 0,
            IPLS_A::VALUE2 => 1,
            IPLS_A::VALUE3 => 2,
            IPLS_A::VALUE4 => 3,
            IPLS_A::VALUE5 => 4,
            IPLS_A::VALUE6 => 5,
            IPLS_A::VALUE7 => 6,
            IPLS_A::VALUE8 => 7,
            IPLS_A::VALUE9 => 8,
            IPLS_A::VALUE10 => 9,
            IPLS_A::VALUE11 => 10,
            IPLS_A::VALUE12 => 11,
            IPLS_A::VALUE13 => 12,
            IPLS_A::VALUE14 => 13,
            IPLS_A::VALUE15 => 14,
            IPLS_A::VALUE16 => 15,
        }
    }
}
#[doc = "Reader of field `IPLS`"]
pub type IPLS_R = crate::R<u8, IPLS_A>;
impl IPLS_R {
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
        *self == IPLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IPLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == IPLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == IPLS_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == IPLS_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == IPLS_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == IPLS_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE8`"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == IPLS_A::VALUE8
    }
    #[doc = "Checks if the value of the field is `VALUE9`"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == IPLS_A::VALUE9
    }
    #[doc = "Checks if the value of the field is `VALUE10`"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == IPLS_A::VALUE10
    }
    #[doc = "Checks if the value of the field is `VALUE11`"]
    #[inline(always)]
    pub fn is_value11(&self) -> bool {
        *self == IPLS_A::VALUE11
    }
    #[doc = "Checks if the value of the field is `VALUE12`"]
    #[inline(always)]
    pub fn is_value12(&self) -> bool {
        *self == IPLS_A::VALUE12
    }
    #[doc = "Checks if the value of the field is `VALUE13`"]
    #[inline(always)]
    pub fn is_value13(&self) -> bool {
        *self == IPLS_A::VALUE13
    }
    #[doc = "Checks if the value of the field is `VALUE14`"]
    #[inline(always)]
    pub fn is_value14(&self) -> bool {
        *self == IPLS_A::VALUE14
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == IPLS_A::VALUE15
    }
    #[doc = "Checks if the value of the field is `VALUE16`"]
    #[inline(always)]
    pub fn is_value16(&self) -> bool {
        *self == IPLS_A::VALUE16
    }
}
#[doc = "Write proxy for field `IPLS`"]
pub struct IPLS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Clamping control signal level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLCL_A {
    #[doc = "0: Clamping control disabled"]
    VALUE1,
    #[doc = "1: Output is set to clamped level when the control signal is HIGH"]
    VALUE2,
    #[doc = "2: Output is set to clamped level when the control signal is LOW"]
    VALUE3,
}
impl From<PLCL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLCL_A) -> Self {
        match variant {
            PLCL_A::VALUE1 => 0,
            PLCL_A::VALUE2 => 1,
            PLCL_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `PLCL`"]
pub type PLCL_R = crate::R<u8, PLCL_A>;
impl PLCL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLCL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLCL_A::VALUE1),
            1 => Val(PLCL_A::VALUE2),
            2 => Val(PLCL_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLCL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLCL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLCL_A::VALUE3
    }
}
#[doc = "Write proxy for field `PLCL`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Output passive level value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL_A {
    #[doc = "0: Output clamped level is LOW"]
    VALUE1,
    #[doc = "1: Output clamped level is HIGH"]
    VALUE2,
}
impl From<PSL_A> for bool {
    #[inline(always)]
    fn from(variant: PSL_A) -> Self {
        match variant {
            PSL_A::VALUE1 => false,
            PSL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PSL`"]
pub type PSL_R = crate::R<bool, PSL_A>;
impl PSL_R {
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
        *self == PSL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL`"]
pub struct PSL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Clamped state exit SW configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLSW_A {
    #[doc = "0: External signal and SW can remove the output from the clamped state"]
    VALUE1,
    #[doc = "1: Only SW can remove the output from the clamped state"]
    VALUE2,
}
impl From<PLSW_A> for bool {
    #[inline(always)]
    fn from(variant: PLSW_A) -> Self {
        match variant {
            PLSW_A::VALUE1 => false,
            PLSW_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PLSW`"]
pub type PLSW_R = crate::R<bool, PLSW_A>;
impl PLSW_R {
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
        *self == PLSW_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLSW_A::VALUE2
    }
}
#[doc = "Write proxy for field `PLSW`"]
pub struct PLSW_W<'a> {
    w: &'a mut W,
}
impl<'a> PLSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Passive level enter configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLEC_A {
    #[doc = "0: Passive level is entered immediately"]
    VALUE1,
    #[doc = "1: Passive level is entered only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2,
    #[doc = "2: Passive level is entered only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3,
}
impl From<PLEC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLEC_A) -> Self {
        match variant {
            PLEC_A::VALUE1 => 0,
            PLEC_A::VALUE2 => 1,
            PLEC_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `PLEC`"]
pub type PLEC_R = crate::R<u8, PLEC_A>;
impl PLEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLEC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLEC_A::VALUE1),
            1 => Val(PLEC_A::VALUE2),
            2 => Val(PLEC_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLEC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLEC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLEC_A::VALUE3
    }
}
#[doc = "Write proxy for field `PLEC`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Passive level exit configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLXC_A {
    #[doc = "0: Passive level is exit immediately"]
    VALUE1,
    #[doc = "1: Passive level is exit only after the comparator output passes to LOW (output from the blanking stage)"]
    VALUE2,
    #[doc = "2: Passive level is exit only after the comparator output passes to HIGH (output from the blanking stage)"]
    VALUE3,
}
impl From<PLXC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLXC_A) -> Self {
        match variant {
            PLXC_A::VALUE1 => 0,
            PLXC_A::VALUE2 => 1,
            PLXC_A::VALUE3 => 2,
        }
    }
}
#[doc = "Reader of field `PLXC`"]
pub type PLXC_R = crate::R<u8, PLXC_A>;
impl PLXC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLXC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLXC_A::VALUE1),
            1 => Val(PLXC_A::VALUE2),
            2 => Val(PLXC_A::VALUE3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLXC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLXC_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLXC_A::VALUE3
    }
}
#[doc = "Write proxy for field `PLXC`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
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
}
