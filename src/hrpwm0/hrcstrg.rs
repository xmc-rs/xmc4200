#[doc = "Writer for register HRCSTRG"]
pub type W = crate::W<u32, super::HRCSTRG>;
#[doc = "Register HRCSTRG `reset()`'s with value 0"]
impl crate::ResetValue for super::HRCSTRG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `H0ES`"]
pub struct H0ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H0ES_W<'a> {
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
#[doc = "Write proxy for field `H0DES`"]
pub struct H0DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H0DES_W<'a> {
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
#[doc = "Write proxy for field `H1ES`"]
pub struct H1ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H1ES_W<'a> {
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
#[doc = "Write proxy for field `H1DES`"]
pub struct H1DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H1DES_W<'a> {
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
#[doc = "Write proxy for field `H2ES`"]
pub struct H2ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H2ES_W<'a> {
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
#[doc = "Write proxy for field `H2DES`"]
pub struct H2DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H2DES_W<'a> {
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
#[doc = "Write proxy for field `H3ES`"]
pub struct H3ES_W<'a> {
    w: &'a mut W,
}
impl<'a> H3ES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `H3DES`"]
pub struct H3DES_W<'a> {
    w: &'a mut W,
}
impl<'a> H3DES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h0es(&mut self) -> H0ES_W {
        H0ES_W { w: self }
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h0des(&mut self) -> H0DES_W {
        H0DES_W { w: self }
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h1es(&mut self) -> H1ES_W {
        H1ES_W { w: self }
    }
    #[doc = "Bit 5 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h1des(&mut self) -> H1DES_W {
        H1DES_W { w: self }
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h2es(&mut self) -> H2ES_W {
        H2ES_W { w: self }
    }
    #[doc = "Bit 9 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h2des(&mut self) -> H2DES_W {
        H2DES_W { w: self }
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer Enable Set"]
    #[inline(always)]
    pub fn h3es(&mut self) -> H3ES_W {
        H3ES_W { w: self }
    }
    #[doc = "Bit 13 - HRC0 dead time value shadow transfer enable set"]
    #[inline(always)]
    pub fn h3des(&mut self) -> H3DES_W {
        H3DES_W { w: self }
    }
}
