#[doc = "Reader of register DSV1"]
pub type R = crate::R<u32, super::DSV1>;
#[doc = "Reader of field `DSV1`"]
pub type DSV1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC reference value 1"]
    #[inline(always)]
    pub fn dsv1(&self) -> DSV1_R {
        DSV1_R::new((self.bits & 0x03ff) as u16)
    }
}
