#[doc = "Reader of register SCR1"]
pub type R = crate::R<u32, super::SCR1>;
#[doc = "Writer for register SCR1"]
pub type W = crate::W<u32, super::SCR1>;
#[doc = "Register SCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCR1`"]
pub type SCR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCR1`"]
pub struct SCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn scr1(&self) -> SCR1_R {
        SCR1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn scr1(&mut self) -> SCR1_W {
        SCR1_W { w: self }
    }
}
