#[doc = "Reader of register SCR2"]
pub type R = crate::R<u32, super::SCR2>;
#[doc = "Writer for register SCR2"]
pub type W = crate::W<u32, super::SCR2>;
#[doc = "Register SCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCR2`"]
pub type SCR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCR2`"]
pub struct SCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn scr2(&self) -> SCR2_R {
        SCR2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn scr2(&mut self) -> SCR2_W {
        SCR2_W { w: self }
    }
}
