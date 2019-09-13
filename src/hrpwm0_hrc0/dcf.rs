#[doc = "Reader of register DCF"]
pub type R = crate::R<u32, super::DCF>;
#[doc = "Reader of field `DTFV`"]
pub type DTFV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Dead time falling value"]
    #[inline(always)]
    pub fn dtfv(&self) -> DTFV_R {
        DTFV_R::new((self.bits & 0xffff) as u16)
    }
}
