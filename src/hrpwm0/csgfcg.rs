#[doc = "Writer for register CSGFCG"]
pub type W = crate::W<u32, super::CSGFCG>;
#[doc = "Register CSGFCG `reset()`'s with value 0"]
impl crate::ResetValue for super::CSGFCG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `S0STR`"]
pub struct S0STR_W<'a> {
    w: &'a mut W,
}
impl<'a> S0STR_W<'a> {
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
#[doc = "Write proxy for field `S0STP`"]
pub struct S0STP_W<'a> {
    w: &'a mut W,
}
impl<'a> S0STP_W<'a> {
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
#[doc = "Write proxy for field `PS0STR`"]
pub struct PS0STR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0STR_W<'a> {
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
#[doc = "Write proxy for field `PS0STP`"]
pub struct PS0STP_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0STP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `PS0CLR`"]
pub struct PS0CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS0CLR_W<'a> {
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
#[doc = "Write proxy for field `S1STR`"]
pub struct S1STR_W<'a> {
    w: &'a mut W,
}
impl<'a> S1STR_W<'a> {
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
#[doc = "Write proxy for field `S1STP`"]
pub struct S1STP_W<'a> {
    w: &'a mut W,
}
impl<'a> S1STP_W<'a> {
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
#[doc = "Write proxy for field `PS1STR`"]
pub struct PS1STR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1STR_W<'a> {
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
#[doc = "Write proxy for field `PS1STP`"]
pub struct PS1STP_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1STP_W<'a> {
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
#[doc = "Write proxy for field `PS1CLR`"]
pub struct PS1CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS1CLR_W<'a> {
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
#[doc = "Write proxy for field `S2STR`"]
pub struct S2STR_W<'a> {
    w: &'a mut W,
}
impl<'a> S2STR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `S2STP`"]
pub struct S2STP_W<'a> {
    w: &'a mut W,
}
impl<'a> S2STP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `PS2STR`"]
pub struct PS2STR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2STR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `PS2STP`"]
pub struct PS2STP_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2STP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `PS2CLR`"]
pub struct PS2CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Slope 0 start"]
    #[inline(always)]
    pub fn s0str(&mut self) -> S0STR_W {
        S0STR_W { w: self }
    }
    #[doc = "Bit 1 - Slope 0 stop"]
    #[inline(always)]
    pub fn s0stp(&mut self) -> S0STP_W {
        S0STP_W { w: self }
    }
    #[doc = "Bit 2 - Prescaler 0 start"]
    #[inline(always)]
    pub fn ps0str(&mut self) -> PS0STR_W {
        PS0STR_W { w: self }
    }
    #[doc = "Bit 3 - Prescaler 0 stop"]
    #[inline(always)]
    pub fn ps0stp(&mut self) -> PS0STP_W {
        PS0STP_W { w: self }
    }
    #[doc = "Bit 4 - Prescaler 0 clear"]
    #[inline(always)]
    pub fn ps0clr(&mut self) -> PS0CLR_W {
        PS0CLR_W { w: self }
    }
    #[doc = "Bit 8 - Slope 1 start"]
    #[inline(always)]
    pub fn s1str(&mut self) -> S1STR_W {
        S1STR_W { w: self }
    }
    #[doc = "Bit 9 - Slope 1 stop"]
    #[inline(always)]
    pub fn s1stp(&mut self) -> S1STP_W {
        S1STP_W { w: self }
    }
    #[doc = "Bit 10 - Prescaler 1 start"]
    #[inline(always)]
    pub fn ps1str(&mut self) -> PS1STR_W {
        PS1STR_W { w: self }
    }
    #[doc = "Bit 11 - Prescaler 1 stop"]
    #[inline(always)]
    pub fn ps1stp(&mut self) -> PS1STP_W {
        PS1STP_W { w: self }
    }
    #[doc = "Bit 12 - Prescaler 1 clear"]
    #[inline(always)]
    pub fn ps1clr(&mut self) -> PS1CLR_W {
        PS1CLR_W { w: self }
    }
    #[doc = "Bit 16 - Slope 2 start"]
    #[inline(always)]
    pub fn s2str(&mut self) -> S2STR_W {
        S2STR_W { w: self }
    }
    #[doc = "Bit 17 - Slope 2 stop"]
    #[inline(always)]
    pub fn s2stp(&mut self) -> S2STP_W {
        S2STP_W { w: self }
    }
    #[doc = "Bit 18 - Prescaler 2 start"]
    #[inline(always)]
    pub fn ps2str(&mut self) -> PS2STR_W {
        PS2STR_W { w: self }
    }
    #[doc = "Bit 19 - Prescaler 2 stop"]
    #[inline(always)]
    pub fn ps2stp(&mut self) -> PS2STP_W {
        PS2STP_W { w: self }
    }
    #[doc = "Bit 20 - Prescaler 2 clear"]
    #[inline(always)]
    pub fn ps2clr(&mut self) -> PS2CLR_W {
        PS2CLR_W { w: self }
    }
}
