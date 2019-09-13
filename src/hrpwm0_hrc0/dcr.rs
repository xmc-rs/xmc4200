#[doc = "Reader of register DCR"]
pub type R = crate::R<u32, super::DCR>;
#[doc = "Reader of field `DTRV`"]
pub type DTRV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Dead time rising value"]
    #[inline(always)]
    pub fn dtrv(&self) -> DTRV_R {
        DTRV_R::new((self.bits & 0xffff) as u16)
    }
}
