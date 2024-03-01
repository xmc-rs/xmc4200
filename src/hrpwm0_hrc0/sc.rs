#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Source selector for the shadow transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St {
    #[doc = "0: Shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    Value1 = 0,
    #[doc = "1: Shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    Value2 = 1,
}
impl From<St> for bool {
    #[inline(always)]
    fn from(variant: St) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST` reader - Source selector for the shadow transfer"]
pub type StR = crate::BitReader<St>;
impl StR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> St {
        match self.bits {
            false => St::Value1,
            true => St::Value2,
        }
    }
    #[doc = "Shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == St::Value1
    }
    #[doc = "Shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == St::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new((self.bits & 1) != 0)
    }
}
#[doc = "HRC current source for shadow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {
    const RESET_VALUE: u32 = 0;
}
