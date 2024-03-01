#[doc = "Register `LPACST` reader"]
pub type R = crate::R<LpacstSpec>;
#[doc = "Trigger VBAT Single Compare Operation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatscmp {
    #[doc = "0: Ready to start new compare operation"]
    Value1 = 0,
    #[doc = "1: Compare operation completed"]
    Value2 = 1,
}
impl From<Vbatscmp> for bool {
    #[inline(always)]
    fn from(variant: Vbatscmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATSCMP` reader - Trigger VBAT Single Compare Operation Status"]
pub type VbatscmpR = crate::BitReader<Vbatscmp>;
impl VbatscmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatscmp {
        match self.bits {
            false => Vbatscmp::Value1,
            true => Vbatscmp::Value2,
        }
    }
    #[doc = "Ready to start new compare operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vbatscmp::Value1
    }
    #[doc = "Compare operation completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vbatscmp::Value2
    }
}
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0scmp {
    #[doc = "0: Ready to start new compare operation"]
    Value1 = 0,
    #[doc = "1: Compare operation completed"]
    Value2 = 1,
}
impl From<Ahibio0scmp> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0scmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0SCMP` reader - Trigger HIB_IO_0 Input Single Compare Operation Status"]
pub type Ahibio0scmpR = crate::BitReader<Ahibio0scmp>;
impl Ahibio0scmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahibio0scmp {
        match self.bits {
            false => Ahibio0scmp::Value1,
            true => Ahibio0scmp::Value2,
        }
    }
    #[doc = "Ready to start new compare operation"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ahibio0scmp::Value1
    }
    #[doc = "Compare operation completed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ahibio0scmp::Value2
    }
}
#[doc = "VBAT Compare Operation Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatval {
    #[doc = "0: Below programmed threshold"]
    Value1 = 0,
    #[doc = "1: Above programmed threshold"]
    Value2 = 1,
}
impl From<Vbatval> for bool {
    #[inline(always)]
    fn from(variant: Vbatval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATVAL` reader - VBAT Compare Operation Result"]
pub type VbatvalR = crate::BitReader<Vbatval>;
impl VbatvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatval {
        match self.bits {
            false => Vbatval::Value1,
            true => Vbatval::Value2,
        }
    }
    #[doc = "Below programmed threshold"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vbatval::Value1
    }
    #[doc = "Above programmed threshold"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vbatval::Value2
    }
}
#[doc = "HIB_IO_0 Input Compare Operation Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0val {
    #[doc = "0: Below programmed threshold"]
    Value1 = 0,
    #[doc = "1: Above programmed threshold"]
    Value2 = 1,
}
impl From<Ahibio0val> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0VAL` reader - HIB_IO_0 Input Compare Operation Result"]
pub type Ahibio0valR = crate::BitReader<Ahibio0val>;
impl Ahibio0valR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahibio0val {
        match self.bits {
            false => Ahibio0val::Value1,
            true => Ahibio0val::Value2,
        }
    }
    #[doc = "Below programmed threshold"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ahibio0val::Value1
    }
    #[doc = "Above programmed threshold"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ahibio0val::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Status"]
    #[inline(always)]
    pub fn vbatscmp(&self) -> VbatscmpR {
        VbatscmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Status"]
    #[inline(always)]
    pub fn ahibio0scmp(&self) -> Ahibio0scmpR {
        Ahibio0scmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - VBAT Compare Operation Result"]
    #[inline(always)]
    pub fn vbatval(&self) -> VbatvalR {
        VbatvalR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Operation Result"]
    #[inline(always)]
    pub fn ahibio0val(&self) -> Ahibio0valR {
        Ahibio0valR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Hibernate Analog Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpacst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpacstSpec;
impl crate::RegisterSpec for LpacstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpacst::R`](R) reader structure"]
impl crate::Readable for LpacstSpec {}
#[doc = "`reset()` method sets LPACST to value 0"]
impl crate::Resettable for LpacstSpec {
    const RESET_VALUE: u32 = 0;
}
