#[doc = "Writer for register CSGTRG"]
pub type W = crate::W<u32, super::CSGTRG>;
#[doc = "Register CSGTRG `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGTRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `D0SES`"]
pub struct D0SES_W<'a> {
    w: &'a mut W,
}
impl<'a> D0SES_W<'a> {
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
#[doc = "Write proxy for field `D0SVS`"]
pub struct D0SVS_W<'a> {
    w: &'a mut W,
}
impl<'a> D0SVS_W<'a> {
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
#[doc = "Write proxy for field `D1SES`"]
pub struct D1SES_W<'a> {
    w: &'a mut W,
}
impl<'a> D1SES_W<'a> {
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
#[doc = "Write proxy for field `D1SVS`"]
pub struct D1SVS_W<'a> {
    w: &'a mut W,
}
impl<'a> D1SVS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `D2SES`"]
pub struct D2SES_W<'a> {
    w: &'a mut W,
}
impl<'a> D2SES_W<'a> {
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
#[doc = "Write proxy for field `D2SVS`"]
pub struct D2SVS_W<'a> {
    w: &'a mut W,
}
impl<'a> D2SVS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DAC0 shadow transfer enable set"]
    #[inline(always)]
    pub fn d0ses(&mut self) -> D0SES_W {
        D0SES_W { w: self }
    }
    #[doc = "Bit 1 - CMP0 inverting input switch request"]
    #[inline(always)]
    pub fn d0svs(&mut self) -> D0SVS_W {
        D0SVS_W { w: self }
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable set"]
    #[inline(always)]
    pub fn d1ses(&mut self) -> D1SES_W {
        D1SES_W { w: self }
    }
    #[doc = "Bit 5 - CMP1 inverting input switch request"]
    #[inline(always)]
    pub fn d1svs(&mut self) -> D1SVS_W {
        D1SVS_W { w: self }
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable set"]
    #[inline(always)]
    pub fn d2ses(&mut self) -> D2SES_W {
        D2SES_W { w: self }
    }
    #[doc = "Bit 9 - CMP2 inverting input switch request"]
    #[inline(always)]
    pub fn d2svs(&mut self) -> D2SVS_W {
        D2SVS_W { w: self }
    }
}
