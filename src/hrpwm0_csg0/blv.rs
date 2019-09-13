#[doc = "Reader of register BLV"]
pub type R = crate::R<u32, super::BLV>;
#[doc = "Writer for register BLV"]
pub type W = crate::W<u32, super::BLV>;
#[doc = "Register BLV `reset()`'s with value 0"]
impl crate::ResetValue for super::BLV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLV`"]
pub type BLV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLV`"]
pub struct BLV_W<'a> {
    w: &'a mut W,
}
impl<'a> BLV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    pub fn blv(&self) -> BLV_R {
        BLV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    pub fn blv(&mut self) -> BLV_W {
        BLV_W { w: self }
    }
}
