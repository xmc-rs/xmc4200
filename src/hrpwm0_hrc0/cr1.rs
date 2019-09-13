#[doc = "Reader of register CR1"]
pub type R = crate::R<u32, super::CR1>;
#[doc = "Reader of field `CR1`"]
pub type CR1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn cr1(&self) -> CR1_R {
        CR1_R::new((self.bits & 0xff) as u8)
    }
}
