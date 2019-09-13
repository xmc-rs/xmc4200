#[doc = "Reader of register SDCF"]
pub type R = crate::R<u32, super::SDCF>;
#[doc = "Writer for register SDCF"]
pub type W = crate::W<u32, super::SDCF>;
#[doc = "Register SDCF `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SDCF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `SDTFV`"]
pub type SDTFV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDTFV`"]
pub struct SDTFV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTFV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow dead time falling value"]
    #[inline(always)]
    pub fn sdtfv(&self) -> SDTFV_R {
        SDTFV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow dead time falling value"]
    #[inline(always)]
    pub fn sdtfv(&mut self) -> SDTFV_W {
        SDTFV_W { w: self }
    }
}
