#[doc = "Register `CSGFSG` reader"]
pub type R = crate::R<CsgfsgSpec>;
#[doc = "DAC0 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0rb {
    #[doc = "0: Slope generation is stopped."]
    Value1 = 0,
    #[doc = "1: Slope generation is running."]
    Value2 = 1,
}
impl From<S0rb> for bool {
    #[inline(always)]
    fn from(variant: S0rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S0RB` reader - DAC0 slope generation status"]
pub type S0rbR = crate::BitReader<S0rb>;
impl S0rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0rb {
        match self.bits {
            false => S0rb::Value1,
            true => S0rb::Value2,
        }
    }
    #[doc = "Slope generation is stopped."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0rb::Value1
    }
    #[doc = "Slope generation is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0rb::Value2
    }
}
#[doc = "CSG0 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P0rb {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    Value1 = 0,
    #[doc = "1: Prescaler is running."]
    Value2 = 1,
}
impl From<P0rb> for bool {
    #[inline(always)]
    fn from(variant: P0rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P0RB` reader - CSG0 prescaler status"]
pub type P0rbR = crate::BitReader<P0rb>;
impl P0rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P0rb {
        match self.bits {
            false => P0rb::Value1,
            true => P0rb::Value2,
        }
    }
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P0rb::Value1
    }
    #[doc = "Prescaler is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P0rb::Value2
    }
}
#[doc = "DAC1 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1rb {
    #[doc = "0: Slope generation is stopped."]
    Value1 = 0,
    #[doc = "1: Slope generation is running."]
    Value2 = 1,
}
impl From<S1rb> for bool {
    #[inline(always)]
    fn from(variant: S1rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S1RB` reader - DAC1 slope generation status"]
pub type S1rbR = crate::BitReader<S1rb>;
impl S1rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1rb {
        match self.bits {
            false => S1rb::Value1,
            true => S1rb::Value2,
        }
    }
    #[doc = "Slope generation is stopped."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1rb::Value1
    }
    #[doc = "Slope generation is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1rb::Value2
    }
}
#[doc = "CSG1 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P1rb {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    Value1 = 0,
    #[doc = "1: Prescaler is running."]
    Value2 = 1,
}
impl From<P1rb> for bool {
    #[inline(always)]
    fn from(variant: P1rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P1RB` reader - CSG1 prescaler status"]
pub type P1rbR = crate::BitReader<P1rb>;
impl P1rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P1rb {
        match self.bits {
            false => P1rb::Value1,
            true => P1rb::Value2,
        }
    }
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P1rb::Value1
    }
    #[doc = "Prescaler is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P1rb::Value2
    }
}
#[doc = "DAC2 slope generation status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S2rb {
    #[doc = "0: Slope generation is stopped."]
    Value1 = 0,
    #[doc = "1: Slope generation is running."]
    Value2 = 1,
}
impl From<S2rb> for bool {
    #[inline(always)]
    fn from(variant: S2rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S2RB` reader - DAC2 slope generation status"]
pub type S2rbR = crate::BitReader<S2rb>;
impl S2rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S2rb {
        match self.bits {
            false => S2rb::Value1,
            true => S2rb::Value2,
        }
    }
    #[doc = "Slope generation is stopped."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S2rb::Value1
    }
    #[doc = "Slope generation is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S2rb::Value2
    }
}
#[doc = "CSG2 prescaler status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P2rb {
    #[doc = "0: Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    Value1 = 0,
    #[doc = "1: Prescaler is running."]
    Value2 = 1,
}
impl From<P2rb> for bool {
    #[inline(always)]
    fn from(variant: P2rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P2RB` reader - CSG2 prescaler status"]
pub type P2rbR = crate::BitReader<P2rb>;
impl P2rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P2rb {
        match self.bits {
            false => P2rb::Value1,
            true => P2rb::Value2,
        }
    }
    #[doc = "Prescaler is stopped. The clock used for the slope generation is halted and therefore the slope is frozen."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == P2rb::Value1
    }
    #[doc = "Prescaler is running."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == P2rb::Value2
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 slope generation status"]
    #[inline(always)]
    pub fn s0rb(&self) -> S0rbR {
        S0rbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CSG0 prescaler status"]
    #[inline(always)]
    pub fn p0rb(&self) -> P0rbR {
        P0rbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC1 slope generation status"]
    #[inline(always)]
    pub fn s1rb(&self) -> S1rbR {
        S1rbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSG1 prescaler status"]
    #[inline(always)]
    pub fn p1rb(&self) -> P1rbR {
        P1rbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC2 slope generation status"]
    #[inline(always)]
    pub fn s2rb(&self) -> S2rbR {
        S2rbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CSG2 prescaler status"]
    #[inline(always)]
    pub fn p2rb(&self) -> P2rbR {
        P2rbR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Global CSG slope/prescaler status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgfsg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgfsgSpec;
impl crate::RegisterSpec for CsgfsgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgfsg::R`](R) reader structure"]
impl crate::Readable for CsgfsgSpec {}
#[doc = "`reset()` method sets CSGFSG to value 0"]
impl crate::Resettable for CsgfsgSpec {
    const RESET_VALUE: u32 = 0;
}
