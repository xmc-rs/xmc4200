#[doc = "Reader of register PC"]
pub type R = crate::R<u32, super::PC>;
#[doc = "Reader of field `PSWV`"]
pub type PSWV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Pulse swallow configuration"]
    #[inline(always)]
    pub fn pswv(&self) -> PSWV_R {
        PSWV_R::new((self.bits & 0x3f) as u8)
    }
}
