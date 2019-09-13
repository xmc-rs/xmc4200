#[doc = "Reader of register LPACTH0"]
pub type R = crate::R<u32, super::LPACTH0>;
#[doc = "Writer for register LPACTH0"]
pub type W = crate::W<u32, super::LPACTH0>;
#[doc = "Register LPACTH0 `reset()`'s with value 0"]
impl crate::ResetValue for super::LPACTH0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBATLO`"]
pub type VBATLO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBATLO`"]
pub struct VBATLO_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `VBATHI`"]
pub type VBATHI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VBATHI`"]
pub struct VBATHI_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    pub fn vbatlo(&self) -> VBATLO_R {
        VBATLO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    pub fn vbathi(&self) -> VBATHI_R {
        VBATHI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    pub fn vbatlo(&mut self) -> VBATLO_W {
        VBATLO_W { w: self }
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    pub fn vbathi(&mut self) -> VBATHI_W {
        VBATHI_W { w: self }
    }
}
