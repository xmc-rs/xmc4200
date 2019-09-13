#[doc = "Writer for register CSGSETG"]
pub type W = crate::W<u32, super::CSGSETG>;
#[doc = "Register CSGSETG `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGSETG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SD0R`"]
pub struct SD0R_W<'a> {
    w: &'a mut W,
}
impl<'a> SD0R_W<'a> {
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
#[doc = "Write proxy for field `SC0R`"]
pub struct SC0R_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0R_W<'a> {
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
#[doc = "Write proxy for field `SC0P`"]
pub struct SC0P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC0P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `SD1R`"]
pub struct SD1R_W<'a> {
    w: &'a mut W,
}
impl<'a> SD1R_W<'a> {
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
#[doc = "Write proxy for field `SC1R`"]
pub struct SC1R_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1R_W<'a> {
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
#[doc = "Write proxy for field `SC1P`"]
pub struct SC1P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC1P_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `SD2R`"]
pub struct SD2R_W<'a> {
    w: &'a mut W,
}
impl<'a> SD2R_W<'a> {
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
#[doc = "Write proxy for field `SC2R`"]
pub struct SC2R_W<'a> {
    w: &'a mut W,
}
impl<'a> SC2R_W<'a> {
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
#[doc = "Write proxy for field `SC2P`"]
pub struct SC2P_W<'a> {
    w: &'a mut W,
}
impl<'a> SC2P_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DAC0 run bit set"]
    #[inline(always)]
    pub fn sd0r(&mut self) -> SD0R_W {
        SD0R_W { w: self }
    }
    #[doc = "Bit 1 - CMP0 run bit set"]
    #[inline(always)]
    pub fn sc0r(&mut self) -> SC0R_W {
        SC0R_W { w: self }
    }
    #[doc = "Bit 2 - CMP0 passive level set"]
    #[inline(always)]
    pub fn sc0p(&mut self) -> SC0P_W {
        SC0P_W { w: self }
    }
    #[doc = "Bit 4 - DAC1 run bit set"]
    #[inline(always)]
    pub fn sd1r(&mut self) -> SD1R_W {
        SD1R_W { w: self }
    }
    #[doc = "Bit 5 - CMP1 run bit set"]
    #[inline(always)]
    pub fn sc1r(&mut self) -> SC1R_W {
        SC1R_W { w: self }
    }
    #[doc = "Bit 6 - CMP1 passive level set"]
    #[inline(always)]
    pub fn sc1p(&mut self) -> SC1P_W {
        SC1P_W { w: self }
    }
    #[doc = "Bit 8 - DAC2 run bit set"]
    #[inline(always)]
    pub fn sd2r(&mut self) -> SD2R_W {
        SD2R_W { w: self }
    }
    #[doc = "Bit 9 - CMP2 run bit set"]
    #[inline(always)]
    pub fn sc2r(&mut self) -> SC2R_W {
        SC2R_W { w: self }
    }
    #[doc = "Bit 10 - CMP2 passive level set"]
    #[inline(always)]
    pub fn sc2p(&mut self) -> SC2P_W {
        SC2P_W { w: self }
    }
}
