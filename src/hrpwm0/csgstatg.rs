#[doc = "Register `CSGSTATG` reader"]
pub type R = crate::R<CsgstatgSpec>;
#[doc = "DAC0 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D0rb {
    #[doc = "0: DAC0 is not running (control logic is disabled)"]
    Value1 = 0,
    #[doc = "1: DAC0 is running"]
    Value2 = 1,
}
impl From<D0rb> for bool {
    #[inline(always)]
    fn from(variant: D0rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D0RB` reader - DAC0 run bit status"]
pub type D0rbR = crate::BitReader<D0rb>;
impl D0rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D0rb {
        match self.bits {
            false => D0rb::Value1,
            true => D0rb::Value2,
        }
    }
    #[doc = "DAC0 is not running (control logic is disabled)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D0rb::Value1
    }
    #[doc = "DAC0 is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D0rb::Value2
    }
}
#[doc = "CMP0 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0rb {
    #[doc = "0: CMP0 functionality is disabled"]
    Value1 = 0,
    #[doc = "1: CMP0 functionality is enabled"]
    Value2 = 1,
}
impl From<C0rb> for bool {
    #[inline(always)]
    fn from(variant: C0rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0RB` reader - CMP0 run bit status"]
pub type C0rbR = crate::BitReader<C0rb>;
impl C0rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0rb {
        match self.bits {
            false => C0rb::Value1,
            true => C0rb::Value2,
        }
    }
    #[doc = "CMP0 functionality is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0rb::Value1
    }
    #[doc = "CMP0 functionality is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0rb::Value2
    }
}
#[doc = "CMP0 output passive status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psls0 {
    #[doc = "0: CMP0 output is not clamped"]
    Value1 = 0,
    #[doc = "1: CMP0 output is clamped"]
    Value2 = 1,
}
impl From<Psls0> for bool {
    #[inline(always)]
    fn from(variant: Psls0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSLS0` reader - CMP0 output passive status"]
pub type Psls0R = crate::BitReader<Psls0>;
impl Psls0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psls0 {
        match self.bits {
            false => Psls0::Value1,
            true => Psls0::Value2,
        }
    }
    #[doc = "CMP0 output is not clamped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psls0::Value1
    }
    #[doc = "CMP0 output is clamped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psls0::Value2
    }
}
#[doc = "DAC1 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D1rb {
    #[doc = "0: DAC1 is not running (control logic is disabled)"]
    Value1 = 0,
    #[doc = "1: DAC1 is running"]
    Value2 = 1,
}
impl From<D1rb> for bool {
    #[inline(always)]
    fn from(variant: D1rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D1RB` reader - DAC1 run bit status"]
pub type D1rbR = crate::BitReader<D1rb>;
impl D1rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D1rb {
        match self.bits {
            false => D1rb::Value1,
            true => D1rb::Value2,
        }
    }
    #[doc = "DAC1 is not running (control logic is disabled)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D1rb::Value1
    }
    #[doc = "DAC1 is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D1rb::Value2
    }
}
#[doc = "CMP1 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1rb {
    #[doc = "0: CMP1 functionality is disabled"]
    Value1 = 0,
    #[doc = "1: CMP1 functionality is enabled"]
    Value2 = 1,
}
impl From<C1rb> for bool {
    #[inline(always)]
    fn from(variant: C1rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1RB` reader - CMP1 run bit status"]
pub type C1rbR = crate::BitReader<C1rb>;
impl C1rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1rb {
        match self.bits {
            false => C1rb::Value1,
            true => C1rb::Value2,
        }
    }
    #[doc = "CMP1 functionality is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1rb::Value1
    }
    #[doc = "CMP1 functionality is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1rb::Value2
    }
}
#[doc = "CMP1 output passive status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psls1 {
    #[doc = "0: CMP1 output is not clamped"]
    Value1 = 0,
    #[doc = "1: CMP1 output is clamped"]
    Value2 = 1,
}
impl From<Psls1> for bool {
    #[inline(always)]
    fn from(variant: Psls1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSLS1` reader - CMP1 output passive status"]
pub type Psls1R = crate::BitReader<Psls1>;
impl Psls1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psls1 {
        match self.bits {
            false => Psls1::Value1,
            true => Psls1::Value2,
        }
    }
    #[doc = "CMP1 output is not clamped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psls1::Value1
    }
    #[doc = "CMP1 output is clamped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psls1::Value2
    }
}
#[doc = "DAC2 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D2rb {
    #[doc = "0: DAC2 is not running (control logic is disabled)"]
    Value1 = 0,
    #[doc = "1: DAC1 is running"]
    Value2 = 1,
}
impl From<D2rb> for bool {
    #[inline(always)]
    fn from(variant: D2rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D2RB` reader - DAC2 run bit status"]
pub type D2rbR = crate::BitReader<D2rb>;
impl D2rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D2rb {
        match self.bits {
            false => D2rb::Value1,
            true => D2rb::Value2,
        }
    }
    #[doc = "DAC2 is not running (control logic is disabled)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D2rb::Value1
    }
    #[doc = "DAC1 is running"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D2rb::Value2
    }
}
#[doc = "CMP2 run bit status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2rb {
    #[doc = "0: CMP2 functionality is disabled"]
    Value1 = 0,
    #[doc = "1: CMP2 functionality is enabled"]
    Value2 = 1,
}
impl From<C2rb> for bool {
    #[inline(always)]
    fn from(variant: C2rb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C2RB` reader - CMP2 run bit status"]
pub type C2rbR = crate::BitReader<C2rb>;
impl C2rbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C2rb {
        match self.bits {
            false => C2rb::Value1,
            true => C2rb::Value2,
        }
    }
    #[doc = "CMP2 functionality is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C2rb::Value1
    }
    #[doc = "CMP2 functionality is enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C2rb::Value2
    }
}
#[doc = "CMP2 output passive status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psls2 {
    #[doc = "0: CMP2 output is not clamped"]
    Value1 = 0,
    #[doc = "1: CMP2 output is clamped"]
    Value2 = 1,
}
impl From<Psls2> for bool {
    #[inline(always)]
    fn from(variant: Psls2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSLS2` reader - CMP2 output passive status"]
pub type Psls2R = crate::BitReader<Psls2>;
impl Psls2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psls2 {
        match self.bits {
            false => Psls2::Value1,
            true => Psls2::Value2,
        }
    }
    #[doc = "CMP2 output is not clamped"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psls2::Value1
    }
    #[doc = "CMP2 output is clamped"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psls2::Value2
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 run bit status"]
    #[inline(always)]
    pub fn d0rb(&self) -> D0rbR {
        D0rbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP0 run bit status"]
    #[inline(always)]
    pub fn c0rb(&self) -> C0rbR {
        C0rbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMP0 output passive status"]
    #[inline(always)]
    pub fn psls0(&self) -> Psls0R {
        Psls0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC1 run bit status"]
    #[inline(always)]
    pub fn d1rb(&self) -> D1rbR {
        D1rbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP1 run bit status"]
    #[inline(always)]
    pub fn c1rb(&self) -> C1rbR {
        C1rbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMP1 output passive status"]
    #[inline(always)]
    pub fn psls1(&self) -> Psls1R {
        Psls1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC2 run bit status"]
    #[inline(always)]
    pub fn d2rb(&self) -> D2rbR {
        D2rbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMP2 run bit status"]
    #[inline(always)]
    pub fn c2rb(&self) -> C2rbR {
        C2rbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CMP2 output passive status"]
    #[inline(always)]
    pub fn psls2(&self) -> Psls2R {
        Psls2R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Global CSG run bit status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgstatg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgstatgSpec;
impl crate::RegisterSpec for CsgstatgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgstatg::R`](R) reader structure"]
impl crate::Readable for CsgstatgSpec {}
#[doc = "`reset()` method sets CSGSTATG to value 0"]
impl crate::Resettable for CsgstatgSpec {
    const RESET_VALUE: u32 = 0;
}
