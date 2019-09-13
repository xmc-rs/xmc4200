#[doc = "Reader of register DSV2"]
pub type R = crate::R<u32, super::DSV2>;
#[doc = "Writer for register DSV2"]
pub type W = crate::W<u32, super::DSV2>;
#[doc = "Register DSV2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSV2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSV2`"]
pub type DSV2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DSV2`"]
pub struct DSV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DSV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DAC reference value 2"]
    #[inline(always)]
    pub fn dsv2(&self) -> DSV2_R {
        DSV2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC reference value 2"]
    #[inline(always)]
    pub fn dsv2(&mut self) -> DSV2_W {
        DSV2_W { w: self }
    }
}
