#[doc = "Register `CSGTRSG` reader"]
pub type R = crate::R<CsgtrsgSpec>;
#[doc = "DAC0 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D0ste {
    #[doc = "0: Shadow transfer has been performed."]
    Value1 = 0,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    Value2 = 1,
}
impl From<D0ste> for bool {
    #[inline(always)]
    fn from(variant: D0ste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D0STE` reader - DAC0 shadow transfer enable"]
pub type D0steR = crate::BitReader<D0ste>;
impl D0steR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D0ste {
        match self.bits {
            false => D0ste::Value1,
            true => D0ste::Value2,
        }
    }
    #[doc = "Shadow transfer has been performed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D0ste::Value1
    }
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D0ste::Value2
    }
}
#[doc = "CMP0 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sw0st {
    #[doc = "0: Inverting input connected to HRPWMx.C0I\\[A\\]"]
    Value1 = 0,
    #[doc = "1: Inverting input connected to HRPWMx.C0I\\[B\\]"]
    Value2 = 1,
}
impl From<Sw0st> for bool {
    #[inline(always)]
    fn from(variant: Sw0st) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW0ST` reader - CMP0 inverting input connection status"]
pub type Sw0stR = crate::BitReader<Sw0st>;
impl Sw0stR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sw0st {
        match self.bits {
            false => Sw0st::Value1,
            true => Sw0st::Value2,
        }
    }
    #[doc = "Inverting input connected to HRPWMx.C0I\\[A\\]"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sw0st::Value1
    }
    #[doc = "Inverting input connected to HRPWMx.C0I\\[B\\]"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sw0st::Value2
    }
}
#[doc = "DAC1 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D1ste {
    #[doc = "0: Shadow transfer has been performed."]
    Value1 = 0,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    Value2 = 1,
}
impl From<D1ste> for bool {
    #[inline(always)]
    fn from(variant: D1ste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D1STE` reader - DAC1 shadow transfer enable"]
pub type D1steR = crate::BitReader<D1ste>;
impl D1steR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D1ste {
        match self.bits {
            false => D1ste::Value1,
            true => D1ste::Value2,
        }
    }
    #[doc = "Shadow transfer has been performed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D1ste::Value1
    }
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D1ste::Value2
    }
}
#[doc = "CMP1 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sw1st {
    #[doc = "0: Inverting input connected to HRPWMx.C1I\\[A\\]"]
    Value1 = 0,
    #[doc = "1: Inverting input connected to HRPWMx.C1I\\[B\\]"]
    Value2 = 1,
}
impl From<Sw1st> for bool {
    #[inline(always)]
    fn from(variant: Sw1st) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW1ST` reader - CMP1 inverting input connection status"]
pub type Sw1stR = crate::BitReader<Sw1st>;
impl Sw1stR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sw1st {
        match self.bits {
            false => Sw1st::Value1,
            true => Sw1st::Value2,
        }
    }
    #[doc = "Inverting input connected to HRPWMx.C1I\\[A\\]"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sw1st::Value1
    }
    #[doc = "Inverting input connected to HRPWMx.C1I\\[B\\]"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sw1st::Value2
    }
}
#[doc = "DAC2 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D2ste {
    #[doc = "0: Shadow transfer has been performed."]
    Value1 = 0,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    Value2 = 1,
}
impl From<D2ste> for bool {
    #[inline(always)]
    fn from(variant: D2ste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D2STE` reader - DAC2 shadow transfer enable"]
pub type D2steR = crate::BitReader<D2ste>;
impl D2steR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D2ste {
        match self.bits {
            false => D2ste::Value1,
            true => D2ste::Value2,
        }
    }
    #[doc = "Shadow transfer has been performed."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D2ste::Value1
    }
    #[doc = "Shadow transfer has been requested but is still pending completion."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D2ste::Value2
    }
}
#[doc = "CMP2 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sw2st {
    #[doc = "0: Inverting input connected to HRPWMx.C2I\\[A\\]"]
    Value1 = 0,
    #[doc = "1: Inverting input connected to HRPWMx.C2I\\[B\\]"]
    Value2 = 1,
}
impl From<Sw2st> for bool {
    #[inline(always)]
    fn from(variant: Sw2st) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW2ST` reader - CMP2 inverting input connection status"]
pub type Sw2stR = crate::BitReader<Sw2st>;
impl Sw2stR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sw2st {
        match self.bits {
            false => Sw2st::Value1,
            true => Sw2st::Value2,
        }
    }
    #[doc = "Inverting input connected to HRPWMx.C2I\\[A\\]"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Sw2st::Value1
    }
    #[doc = "Inverting input connected to HRPWMx.C2I\\[B\\]"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Sw2st::Value2
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 shadow transfer enable"]
    #[inline(always)]
    pub fn d0ste(&self) -> D0steR {
        D0steR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMP0 inverting input connection status"]
    #[inline(always)]
    pub fn sw0st(&self) -> Sw0stR {
        Sw0stR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable"]
    #[inline(always)]
    pub fn d1ste(&self) -> D1steR {
        D1steR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMP1 inverting input connection status"]
    #[inline(always)]
    pub fn sw1st(&self) -> Sw1stR {
        Sw1stR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable"]
    #[inline(always)]
    pub fn d2ste(&self) -> D2steR {
        D2steR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMP2 inverting input connection status"]
    #[inline(always)]
    pub fn sw2st(&self) -> Sw2stR {
        Sw2stR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Global CSG shadow/switch status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgtrsg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgtrsgSpec;
impl crate::RegisterSpec for CsgtrsgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgtrsg::R`](R) reader structure"]
impl crate::Readable for CsgtrsgSpec {}
#[doc = "`reset()` method sets CSGTRSG to value 0"]
impl crate::Resettable for CsgtrsgSpec {
    const RESET_VALUE: u32 = 0;
}
