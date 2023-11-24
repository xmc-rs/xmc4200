#[doc = "Register `CSGSTATG` reader"]
pub type R = crate::R<CSGSTATG_SPEC>;
#[doc = "Field `D0RB` reader - DAC0 run bit status"]
pub type D0RB_R = crate::BitReader<D0RB_A>;
#[doc = "DAC0 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D0RB_A {
    #[doc = "0: DAC0 is not running (control logic is disabled)"]
    VALUE1 = 0,
    #[doc = "1: DAC0 is running"]
    VALUE2 = 1,
}
impl From<D0RB_A> for bool {
    #[inline(always)]
    fn from(variant: D0RB_A) -> Self {
        variant as u8 != 0
    }
}
impl D0RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D0RB_A {
        match self.bits {
            false => D0RB_A::VALUE1,
            true => D0RB_A::VALUE2,
        }
    }
    #[doc = "DAC0 is not running (control logic is disabled)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D0RB_A::VALUE1
    }
    #[doc = "DAC0 is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D0RB_A::VALUE2
    }
}
#[doc = "Field `C0RB` reader - CMP0 run bit status"]
pub type C0RB_R = crate::BitReader<C0RB_A>;
#[doc = "CMP0 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0RB_A {
    #[doc = "0: CMP0 functionality is disabled"]
    VALUE1 = 0,
    #[doc = "1: CMP0 functionality is enabled"]
    VALUE2 = 1,
}
impl From<C0RB_A> for bool {
    #[inline(always)]
    fn from(variant: C0RB_A) -> Self {
        variant as u8 != 0
    }
}
impl C0RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0RB_A {
        match self.bits {
            false => C0RB_A::VALUE1,
            true => C0RB_A::VALUE2,
        }
    }
    #[doc = "CMP0 functionality is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0RB_A::VALUE1
    }
    #[doc = "CMP0 functionality is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0RB_A::VALUE2
    }
}
#[doc = "Field `PSLS0` reader - CMP0 output passive status"]
pub type PSLS0_R = crate::BitReader<PSLS0_A>;
#[doc = "CMP0 output passive status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSLS0_A {
    #[doc = "0: CMP0 output is not clamped"]
    VALUE1 = 0,
    #[doc = "1: CMP0 output is clamped"]
    VALUE2 = 1,
}
impl From<PSLS0_A> for bool {
    #[inline(always)]
    fn from(variant: PSLS0_A) -> Self {
        variant as u8 != 0
    }
}
impl PSLS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSLS0_A {
        match self.bits {
            false => PSLS0_A::VALUE1,
            true => PSLS0_A::VALUE2,
        }
    }
    #[doc = "CMP0 output is not clamped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSLS0_A::VALUE1
    }
    #[doc = "CMP0 output is clamped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSLS0_A::VALUE2
    }
}
#[doc = "Field `D1RB` reader - DAC1 run bit status"]
pub type D1RB_R = crate::BitReader<D1RB_A>;
#[doc = "DAC1 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D1RB_A {
    #[doc = "0: DAC1 is not running (control logic is disabled)"]
    VALUE1 = 0,
    #[doc = "1: DAC1 is running"]
    VALUE2 = 1,
}
impl From<D1RB_A> for bool {
    #[inline(always)]
    fn from(variant: D1RB_A) -> Self {
        variant as u8 != 0
    }
}
impl D1RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D1RB_A {
        match self.bits {
            false => D1RB_A::VALUE1,
            true => D1RB_A::VALUE2,
        }
    }
    #[doc = "DAC1 is not running (control logic is disabled)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D1RB_A::VALUE1
    }
    #[doc = "DAC1 is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D1RB_A::VALUE2
    }
}
#[doc = "Field `C1RB` reader - CMP1 run bit status"]
pub type C1RB_R = crate::BitReader<C1RB_A>;
#[doc = "CMP1 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1RB_A {
    #[doc = "0: CMP1 functionality is disabled"]
    VALUE1 = 0,
    #[doc = "1: CMP1 functionality is enabled"]
    VALUE2 = 1,
}
impl From<C1RB_A> for bool {
    #[inline(always)]
    fn from(variant: C1RB_A) -> Self {
        variant as u8 != 0
    }
}
impl C1RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1RB_A {
        match self.bits {
            false => C1RB_A::VALUE1,
            true => C1RB_A::VALUE2,
        }
    }
    #[doc = "CMP1 functionality is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1RB_A::VALUE1
    }
    #[doc = "CMP1 functionality is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1RB_A::VALUE2
    }
}
#[doc = "Field `PSLS1` reader - CMP1 output passive status"]
pub type PSLS1_R = crate::BitReader<PSLS1_A>;
#[doc = "CMP1 output passive status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSLS1_A {
    #[doc = "0: CMP1 output is not clamped"]
    VALUE1 = 0,
    #[doc = "1: CMP1 output is clamped"]
    VALUE2 = 1,
}
impl From<PSLS1_A> for bool {
    #[inline(always)]
    fn from(variant: PSLS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PSLS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSLS1_A {
        match self.bits {
            false => PSLS1_A::VALUE1,
            true => PSLS1_A::VALUE2,
        }
    }
    #[doc = "CMP1 output is not clamped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSLS1_A::VALUE1
    }
    #[doc = "CMP1 output is clamped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSLS1_A::VALUE2
    }
}
#[doc = "Field `D2RB` reader - DAC2 run bit status"]
pub type D2RB_R = crate::BitReader<D2RB_A>;
#[doc = "DAC2 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D2RB_A {
    #[doc = "0: DAC2 is not running (control logic is disabled)"]
    VALUE1 = 0,
    #[doc = "1: DAC1 is running"]
    VALUE2 = 1,
}
impl From<D2RB_A> for bool {
    #[inline(always)]
    fn from(variant: D2RB_A) -> Self {
        variant as u8 != 0
    }
}
impl D2RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D2RB_A {
        match self.bits {
            false => D2RB_A::VALUE1,
            true => D2RB_A::VALUE2,
        }
    }
    #[doc = "DAC2 is not running (control logic is disabled)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D2RB_A::VALUE1
    }
    #[doc = "DAC1 is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D2RB_A::VALUE2
    }
}
#[doc = "Field `C2RB` reader - CMP2 run bit status"]
pub type C2RB_R = crate::BitReader<C2RB_A>;
#[doc = "CMP2 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2RB_A {
    #[doc = "0: CMP2 functionality is disabled"]
    VALUE1 = 0,
    #[doc = "1: CMP2 functionality is enabled"]
    VALUE2 = 1,
}
impl From<C2RB_A> for bool {
    #[inline(always)]
    fn from(variant: C2RB_A) -> Self {
        variant as u8 != 0
    }
}
impl C2RB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C2RB_A {
        match self.bits {
            false => C2RB_A::VALUE1,
            true => C2RB_A::VALUE2,
        }
    }
    #[doc = "CMP2 functionality is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C2RB_A::VALUE1
    }
    #[doc = "CMP2 functionality is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C2RB_A::VALUE2
    }
}
#[doc = "Field `PSLS2` reader - CMP2 output passive status"]
pub type PSLS2_R = crate::BitReader<PSLS2_A>;
#[doc = "CMP2 output passive status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSLS2_A {
    #[doc = "0: CMP2 output is not clamped"]
    VALUE1 = 0,
    #[doc = "1: CMP2 output is clamped"]
    VALUE2 = 1,
}
impl From<PSLS2_A> for bool {
    #[inline(always)]
    fn from(variant: PSLS2_A) -> Self {
        variant as u8 != 0
    }
}
impl PSLS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSLS2_A {
        match self.bits {
            false => PSLS2_A::VALUE1,
            true => PSLS2_A::VALUE2,
        }
    }
    #[doc = "CMP2 output is not clamped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSLS2_A::VALUE1
    }
    #[doc = "CMP2 output is clamped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSLS2_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 run bit status"]
    #[inline(always)]
    pub fn d0rb(&self) -> D0RB_R {
        D0RB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP0 run bit status"]
    #[inline(always)]
    pub fn c0rb(&self) -> C0RB_R {
        C0RB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP0 output passive status"]
    #[inline(always)]
    pub fn psls0(&self) -> PSLS0_R {
        PSLS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC1 run bit status"]
    #[inline(always)]
    pub fn d1rb(&self) -> D1RB_R {
        D1RB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP1 run bit status"]
    #[inline(always)]
    pub fn c1rb(&self) -> C1RB_R {
        C1RB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMP1 output passive status"]
    #[inline(always)]
    pub fn psls1(&self) -> PSLS1_R {
        PSLS1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC2 run bit status"]
    #[inline(always)]
    pub fn d2rb(&self) -> D2RB_R {
        D2RB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMP2 run bit status"]
    #[inline(always)]
    pub fn c2rb(&self) -> C2RB_R {
        C2RB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CMP2 output passive status"]
    #[inline(always)]
    pub fn psls2(&self) -> PSLS2_R {
        PSLS2_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Global CSG run bit status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgstatg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGSTATG_SPEC;
impl crate::RegisterSpec for CSGSTATG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgstatg::R`](R) reader structure"]
impl crate::Readable for CSGSTATG_SPEC {}
#[doc = "`reset()` method sets CSGSTATG to value 0"]
impl crate::Resettable for CSGSTATG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
