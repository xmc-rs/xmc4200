#[doc = "Register `MIRRALLSTAT` reader"]
pub type R = crate::R<MirrallstatSpec>;
#[doc = "Mirror All Execution Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: No update is pening"]
    Value1 = 0,
    #[doc = "1: Update is pending"]
    Value2 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Mirror All Execution Status"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Value1,
            true => Busy::Value2,
        }
    }
    #[doc = "No update is pening"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Busy::Value1
    }
    #[doc = "Update is pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Busy::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Mirror All Execution Status"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Mirror All Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mirrallstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MirrallstatSpec;
impl crate::RegisterSpec for MirrallstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mirrallstat::R`](R) reader structure"]
impl crate::Readable for MirrallstatSpec {}
#[doc = "`reset()` method sets MIRRALLSTAT to value 0"]
impl crate::Resettable for MirrallstatSpec {
    const RESET_VALUE: u32 = 0;
}
