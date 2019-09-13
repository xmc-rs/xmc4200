#[doc = "Reader of register SDCR"]
pub type R = crate::R<u32, super::SDCR>;
#[doc = "Writer for register SDCR"]
pub type W = crate::W<u32, super::SDCR>;
#[doc = "Register SDCR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SDTRV`"]
pub type SDTRV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDTRV`"]
pub struct SDTRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    pub fn sdtrv(&self) -> SDTRV_R {
        SDTRV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    pub fn sdtrv(&mut self) -> SDTRV_W {
        SDTRV_W { w: self }
    }
}
