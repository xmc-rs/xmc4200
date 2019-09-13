#[doc = "Reader of register SPC"]
pub type R = crate::R<u32, super::SPC>;
#[doc = "Writer for register SPC"]
pub type W = crate::W<u32, super::SPC>;
#[doc = "Register SPC `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPSWV`"]
pub type SPSWV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPSWV`"]
pub struct SPSWV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPSWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    pub fn spswv(&self) -> SPSWV_R {
        SPSWV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    pub fn spswv(&mut self) -> SPSWV_W {
        SPSWV_W { w: self }
    }
}
