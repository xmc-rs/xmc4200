#[doc = "Writer for register MIRRALLREQ"]
pub type W = crate::W<u32, super::MIRRALLREQ>;
#[doc = "Register MIRRALLREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::MIRRALLREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mirror All Execution Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REQ_AW {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: Start mirror update"]
    VALUE2 = 1,
}
impl From<REQ_AW> for bool {
    #[inline(always)]
    fn from(variant: REQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `REQ`"]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REQ_AW::VALUE1)
    }
    #[doc = "Start mirror update"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REQ_AW::VALUE2)
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
impl W {
    #[doc = "Bit 0 - Mirror All Execution Request"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
}
