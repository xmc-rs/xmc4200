#[doc = "Reader of register HRGHRS"]
pub type R = crate::R<u32, super::HRGHRS>;
#[doc = "High Resolution Generation Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRGR_A {
    #[doc = "0: High resolution logic is not working"]
    VALUE1 = 0,
    #[doc = "1: High resolution logic is working"]
    VALUE2 = 1,
}
impl From<HRGR_A> for bool {
    #[inline(always)]
    fn from(variant: HRGR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRGR`"]
pub type HRGR_R = crate::R<bool, HRGR_A>;
impl HRGR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRGR_A {
        match self.bits {
            false => HRGR_A::VALUE1,
            true => HRGR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRGR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRGR_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Generation Ready"]
    #[inline(always)]
    pub fn hrgr(&self) -> HRGR_R {
        HRGR_R::new((self.bits & 0x01) != 0)
    }
}
