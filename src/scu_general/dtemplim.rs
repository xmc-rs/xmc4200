#[doc = "Reader of register DTEMPLIM"]
pub type R = crate::R<u32, super::DTEMPLIM>;
#[doc = "Writer for register DTEMPLIM"]
pub type W = crate::W<u32, super::DTEMPLIM>;
#[doc = "Register DTEMPLIM `reset()`'s with value 0"]
impl crate::ResetValue for super::DTEMPLIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOWER`"]
pub type LOWER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LOWER`"]
pub struct LOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `UPPER`"]
pub type UPPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UPPER`"]
pub struct UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    pub fn lower(&self) -> LOWER_R {
        LOWER_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    pub fn upper(&self) -> UPPER_R {
        UPPER_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    pub fn lower(&mut self) -> LOWER_W {
        LOWER_W { w: self }
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    pub fn upper(&mut self) -> UPPER_W {
        UPPER_W { w: self }
    }
}
