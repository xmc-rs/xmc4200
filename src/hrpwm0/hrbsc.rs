#[doc = "Reader of register HRBSC"]
pub type R = crate::R<u32, super::HRBSC>;
#[doc = "Writer for register HRBSC"]
pub type W = crate::W<u32, super::HRBSC>;
#[doc = "Register HRBSC `reset()`'s with value 0"]
impl crate::ResetValue for super::HRBSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Suspend configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUSCFG_A {
    #[doc = "0: Suspend is ignored."]
    VALUE1,
    #[doc = "1: CSGy and HRCy units are halted."]
    VALUE2,
    #[doc = "2: Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE3,
    #[doc = "3: CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    VALUE4,
}
impl From<SUSCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: SUSCFG_A) -> Self {
        match variant {
            SUSCFG_A::VALUE1 => 0,
            SUSCFG_A::VALUE2 => 1,
            SUSCFG_A::VALUE3 => 2,
            SUSCFG_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `SUSCFG`"]
pub type SUSCFG_R = crate::R<u8, SUSCFG_A>;
impl SUSCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUSCFG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SUSCFG_A::VALUE1),
            1 => Val(SUSCFG_A::VALUE2),
            2 => Val(SUSCFG_A::VALUE3),
            3 => Val(SUSCFG_A::VALUE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SUSCFG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SUSCFG_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SUSCFG_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SUSCFG_A::VALUE4
    }
}
#[doc = "Write proxy for field `SUSCFG`"]
pub struct SUSCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SUSCFG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Suspend is ignored."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE1)
    }
    #[doc = "CSGy and HRCy units are halted."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE2)
    }
    #[doc = "Comparator outputs, HRPWMx.CyO are clamped to passive level and the CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE3)
    }
    #[doc = "CSGy units are halted. High resolution channel outputs, HRPWMx.HROUTy0 and HRPWMx.HROUTy1, are clamped to passive state and the HRCy units are halted."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SUSCFG_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `HRBE`"]
pub type HRBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRBE`"]
pub struct HRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> HRBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    pub fn suscfg(&self) -> SUSCFG_R {
        SUSCFG_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    pub fn hrbe(&self) -> HRBE_R {
        HRBE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Suspend configuration"]
    #[inline(always)]
    pub fn suscfg(&mut self) -> SUSCFG_W {
        SUSCFG_W { w: self }
    }
    #[doc = "Bit 8 - HRPWM bias enable"]
    #[inline(always)]
    pub fn hrbe(&mut self) -> HRBE_W {
        HRBE_W { w: self }
    }
}
