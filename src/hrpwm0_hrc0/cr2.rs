#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Reader of field `CR2`"]
pub type CR2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn cr2(&self) -> CR2_R {
        CR2_R::new((self.bits & 0xff) as u8)
    }
}
