#[doc = "Register `DTEMPALARM` reader"]
pub type R = crate::R<DtempalarmSpec>;
#[doc = "Lower Limit Underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Underfl {
    #[doc = "0: No temperature underflow was detected"]
    Value1 = 0,
    #[doc = "1: A temperature underflow was detected"]
    Value2 = 1,
}
impl From<Underfl> for bool {
    #[inline(always)]
    fn from(variant: Underfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDERFL` reader - Lower Limit Underflow"]
pub type UnderflR = crate::BitReader<Underfl>;
impl UnderflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Underfl {
        match self.bits {
            false => Underfl::Value1,
            true => Underfl::Value2,
        }
    }
    #[doc = "No temperature underflow was detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Underfl::Value1
    }
    #[doc = "A temperature underflow was detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Underfl::Value2
    }
}
#[doc = "Upper Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overfl {
    #[doc = "0: No temperature overflow was detected"]
    Value1 = 0,
    #[doc = "1: A temperature overflow was detected"]
    Value2 = 1,
}
impl From<Overfl> for bool {
    #[inline(always)]
    fn from(variant: Overfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFL` reader - Upper Limit Overflow"]
pub type OverflR = crate::BitReader<Overfl>;
impl OverflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Overfl {
        match self.bits {
            false => Overfl::Value1,
            true => Overfl::Value2,
        }
    }
    #[doc = "No temperature overflow was detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Overfl::Value1
    }
    #[doc = "A temperature overflow was detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Overfl::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Lower Limit Underflow"]
    #[inline(always)]
    pub fn underfl(&self) -> UnderflR {
        UnderflR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Upper Limit Overflow"]
    #[inline(always)]
    pub fn overfl(&self) -> OverflR {
        OverflR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Die Temperature Sensor Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtempalarm::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtempalarmSpec;
impl crate::RegisterSpec for DtempalarmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtempalarm::R`](R) reader structure"]
impl crate::Readable for DtempalarmSpec {}
#[doc = "`reset()` method sets DTEMPALARM to value 0"]
impl crate::Resettable for DtempalarmSpec {
    const RESET_VALUE: u32 = 0;
}
