#[doc = "Writer for register CSGTRC"]
pub type W = crate::W<u32, super::CSGTRC>;
#[doc = "Register CSGTRC `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGTRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `D0SEC`"]
pub struct D0SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> D0SEC_W<'a> {
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
#[doc = "Write proxy for field `D1SEC`"]
pub struct D1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> D1SEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `D2SEC`"]
pub struct D2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> D2SEC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable clear"]
    #[inline(always)]
    pub fn d0sec(&mut self) -> D0SEC_W {
        D0SEC_W { w: self }
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable clear"]
    #[inline(always)]
    pub fn d1sec(&mut self) -> D1SEC_W {
        D1SEC_W { w: self }
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable clear"]
    #[inline(always)]
    pub fn d2sec(&mut self) -> D2SEC_W {
        D2SEC_W { w: self }
    }
}
