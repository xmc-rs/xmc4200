#[doc = "Reader of register SDSV1"]
pub type R = crate::R<u32, super::SDSV1>;
#[doc = "Writer for register SDSV1"]
pub type W = crate::W<u32, super::SDSV1>;
#[doc = "Register SDSV1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SDSV1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDSV1`"]
pub type SDSV1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDSV1`"]
pub struct SDSV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SDSV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Shadow DAC reference value 1"]
    #[inline(always)]
    pub fn sdsv1(&self) -> SDSV1_R {
        SDSV1_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Shadow DAC reference value 1"]
    #[inline(always)]
    pub fn sdsv1(&mut self) -> SDSV1_W {
        SDSV1_W { w: self }
    }
}
