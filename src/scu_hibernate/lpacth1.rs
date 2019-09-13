#[doc = "Reader of register LPACTH1"]
pub type R = crate::R<u32, super::LPACTH1>;
#[doc = "Writer for register LPACTH1"]
pub type W = crate::W<u32, super::LPACTH1>;
#[doc = "Register LPACTH1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LPACTH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AHIBIO0LO`"]
pub type AHIBIO0LO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHIBIO0LO`"]
pub struct AHIBIO0LO_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `AHIBIO0HI`"]
pub type AHIBIO0HI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AHIBIO0HI`"]
pub struct AHIBIO0HI_W<'a> {
    w: &'a mut W,
}
impl<'a> AHIBIO0HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio0lo(&self) -> AHIBIO0LO_R {
        AHIBIO0LO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio0hi(&self) -> AHIBIO0HI_R {
        AHIBIO0HI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio0lo(&mut self) -> AHIBIO0LO_W {
        AHIBIO0LO_W { w: self }
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio0hi(&mut self) -> AHIBIO0HI_W {
        AHIBIO0HI_W { w: self }
    }
}
