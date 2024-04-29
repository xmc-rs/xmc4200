#[doc = "Register `CSGFSG` reader"]
pub type R = crate::R<CSGFSG_SPEC>;
#[doc = "DAC0 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0RB_A {
    #[doc = "0: Slope generation is stopped."]
    VALUE1 = 0,
    #[doc = "1: Slope generation is running."]
    VALUE2 = 1,
}
impl From<S0RB_A> for bool {
    #[inline(always)]
    fn from(variant: S0RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RB` reader - DAC0 slope generation status"]
pub type S0RB_R = crate::BitReader<S0RB_A>;
impl S0RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0RB_A {
        match self.bits {
            false => S0RB_A::VALUE1,
            true => S0RB_A::VALUE2,
        }
    }
    #[doc = "Slope generation is stopped."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0RB_A::VALUE1
    }
    #[doc = "Slope generation is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0RB_A::VALUE2
    }
}
#[doc = "CSG0 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0RB_A {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running."]
    VALUE2 = 1,
}
impl From<P0RB_A> for bool {
    #[inline(always)]
    fn from(variant: P0RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0RB` reader - CSG0 prescaler status"]
pub type P0RB_R = crate::BitReader<P0RB_A>;
impl P0RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0RB_A {
        match self.bits {
            false => P0RB_A::VALUE1,
            true => P0RB_A::VALUE2,
        }
    }
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P0RB_A::VALUE1
    }
    #[doc = "Prescaler is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P0RB_A::VALUE2
    }
}
#[doc = "DAC1 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1RB_A {
    #[doc = "0: Slope generation is stopped."]
    VALUE1 = 0,
    #[doc = "1: Slope generation is running."]
    VALUE2 = 1,
}
impl From<S1RB_A> for bool {
    #[inline(always)]
    fn from(variant: S1RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RB` reader - DAC1 slope generation status"]
pub type S1RB_R = crate::BitReader<S1RB_A>;
impl S1RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1RB_A {
        match self.bits {
            false => S1RB_A::VALUE1,
            true => S1RB_A::VALUE2,
        }
    }
    #[doc = "Slope generation is stopped."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1RB_A::VALUE1
    }
    #[doc = "Slope generation is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1RB_A::VALUE2
    }
}
#[doc = "CSG1 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1RB_A {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running."]
    VALUE2 = 1,
}
impl From<P1RB_A> for bool {
    #[inline(always)]
    fn from(variant: P1RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1RB` reader - CSG1 prescaler status"]
pub type P1RB_R = crate::BitReader<P1RB_A>;
impl P1RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1RB_A {
        match self.bits {
            false => P1RB_A::VALUE1,
            true => P1RB_A::VALUE2,
        }
    }
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P1RB_A::VALUE1
    }
    #[doc = "Prescaler is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P1RB_A::VALUE2
    }
}
#[doc = "DAC2 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2RB_A {
    #[doc = "0: Slope generation is stopped."]
    VALUE1 = 0,
    #[doc = "1: Slope generation is running."]
    VALUE2 = 1,
}
impl From<S2RB_A> for bool {
    #[inline(always)]
    fn from(variant: S2RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RB` reader - DAC2 slope generation status"]
pub type S2RB_R = crate::BitReader<S2RB_A>;
impl S2RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2RB_A {
        match self.bits {
            false => S2RB_A::VALUE1,
            true => S2RB_A::VALUE2,
        }
    }
    #[doc = "Slope generation is stopped."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2RB_A::VALUE1
    }
    #[doc = "Slope generation is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2RB_A::VALUE2
    }
}
#[doc = "CSG2 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2RB_A {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    VALUE1 = 0,
    #[doc = "1: Prescaler is running."]
    VALUE2 = 1,
}
impl From<P2RB_A> for bool {
    #[inline(always)]
    fn from(variant: P2RB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2RB` reader - CSG2 prescaler status"]
pub type P2RB_R = crate::BitReader<P2RB_A>;
impl P2RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2RB_A {
        match self.bits {
            false => P2RB_A::VALUE1,
            true => P2RB_A::VALUE2,
        }
    }
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P2RB_A::VALUE1
    }
    #[doc = "Prescaler is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P2RB_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 slope generation status"]
    #[inline(always)]
    pub fn s0rb(&self) -> S0RB_R {
        S0RB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSG0 prescaler status"]
    #[inline(always)]
    pub fn p0rb(&self) -> P0RB_R {
        P0RB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC1 slope generation status"]
    #[inline(always)]
    pub fn s1rb(&self) -> S1RB_R {
        S1RB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSG1 prescaler status"]
    #[inline(always)]
    pub fn p1rb(&self) -> P1RB_R {
        P1RB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC2 slope generation status"]
    #[inline(always)]
    pub fn s2rb(&self) -> S2RB_R {
        S2RB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CSG2 prescaler status"]
    #[inline(always)]
    pub fn p2rb(&self) -> P2RB_R {
        P2RB_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Global CSG slope/prescaler status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgfsg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGFSG_SPEC;
impl crate::RegisterSpec for CSGFSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgfsg::R`](R) reader structure"]
impl crate::Readable for CSGFSG_SPEC {}
#[doc = "`reset()` method sets CSGFSG to value 0"]
impl crate::Resettable for CSGFSG_SPEC {
    const RESET_VALUE: u32 = 0;
}
