#[doc = "Register `DTEMPALARM` reader"]
pub struct R(crate::R<DTEMPALARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTEMPALARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTEMPALARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTEMPALARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNDERFL` reader - Lower Limit Underflow"]
pub type UNDERFL_R = crate::BitReader<UNDERFL_A>;
#[doc = "Lower Limit Underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNDERFL_A {
    #[doc = "0: No temperature underflow was detected"]
    VALUE1 = 0,
    #[doc = "1: A temperature underflow was detected"]
    VALUE2 = 1,
}
impl From<UNDERFL_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFL_A) -> Self {
        variant as u8 != 0
    }
}
impl UNDERFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFL_A {
        match self.bits {
            false => UNDERFL_A::VALUE1,
            true => UNDERFL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UNDERFL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UNDERFL_A::VALUE2
    }
}
#[doc = "Field `OVERFL` reader - Upper Limit Overflow"]
pub type OVERFL_R = crate::BitReader<OVERFL_A>;
#[doc = "Upper Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERFL_A {
    #[doc = "0: No temperature overflow was detected"]
    VALUE1 = 0,
    #[doc = "1: A temperature overflow was detected"]
    VALUE2 = 1,
}
impl From<OVERFL_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFL_A) -> Self {
        variant as u8 != 0
    }
}
impl OVERFL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFL_A {
        match self.bits {
            false => OVERFL_A::VALUE1,
            true => OVERFL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OVERFL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OVERFL_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Lower Limit Underflow"]
    #[inline(always)]
    pub fn underfl(&self) -> UNDERFL_R {
        UNDERFL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Upper Limit Overflow"]
    #[inline(always)]
    pub fn overfl(&self) -> OVERFL_R {
        OVERFL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Die Temperature Sensor Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtempalarm](index.html) module"]
pub struct DTEMPALARM_SPEC;
impl crate::RegisterSpec for DTEMPALARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtempalarm::R](R) reader structure"]
impl crate::Readable for DTEMPALARM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTEMPALARM to value 0"]
impl crate::Resettable for DTEMPALARM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
