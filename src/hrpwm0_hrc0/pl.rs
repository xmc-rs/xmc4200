#[doc = "Reader of register PL"]
pub type R = crate::R<u32, super::PL>;
#[doc = "Writer for register PL"]
pub type W = crate::W<u32, super::PL>;
#[doc = "Register PL `reset()`'s with value 0"]
impl crate::ResetValue for super::PL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HRPWMx.OUTy0 passive level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL0_A {
    #[doc = "0: HRPWMx.OUTy0 output passive level is set to LOW"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.OUTy0 output passive level is set to HIGH"]
    VALUE2 = 1,
}
impl From<PSL0_A> for bool {
    #[inline(always)]
    fn from(variant: PSL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSL0`"]
pub type PSL0_R = crate::R<bool, PSL0_A>;
impl PSL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL0_A {
        match self.bits {
            false => PSL0_A::VALUE1,
            true => PSL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL0_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL0`"]
pub struct PSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL0_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL0_A::VALUE2)
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
#[doc = "HRPWMx.OUTy1 passive level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSL1_A {
    #[doc = "0: HRPWMx.OUTy1 output passive level is set to LOW"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.OUTy1 output passive level is set to HIGH"]
    VALUE2 = 1,
}
impl From<PSL1_A> for bool {
    #[inline(always)]
    fn from(variant: PSL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PSL1`"]
pub type PSL1_R = crate::R<bool, PSL1_A>;
impl PSL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL1_A {
        match self.bits {
            false => PSL1_A::VALUE1,
            true => PSL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PSL1`"]
pub struct PSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PSL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSL1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL1_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline(always)]
    pub fn psl0(&self) -> PSL0_R {
        PSL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline(always)]
    pub fn psl1(&self) -> PSL1_R {
        PSL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline(always)]
    pub fn psl0(&mut self) -> PSL0_W {
        PSL0_W { w: self }
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline(always)]
    pub fn psl1(&mut self) -> PSL1_W {
        PSL1_W { w: self }
    }
}
