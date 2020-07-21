#[doc = "Reader of register SC"]
pub type R = crate::R<u32, super::SC>;
#[doc = "Source selector for the shadow transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST_A {
    #[doc = "0: Shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    VALUE2 = 1,
}
impl From<ST_A> for bool {
    #[inline(always)]
    fn from(variant: ST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ST`"]
pub type ST_R = crate::R<bool, ST_A>;
impl ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST_A {
        match self.bits {
            false => ST_A::VALUE1,
            true => ST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ST_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 0x01) != 0)
    }
}
