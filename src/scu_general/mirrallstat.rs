#[doc = "Register `MIRRALLSTAT` reader"]
pub type R = crate::R<MIRRALLSTAT_SPEC>;
#[doc = "Field `BUSY` reader - Mirror All Execution Status"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Mirror All Execution Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: No update is pening"]
    VALUE1 = 0,
    #[doc = "1: Update is pending"]
    VALUE2 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::VALUE1,
            true => BUSY_A::VALUE2,
        }
    }
    #[doc = "No update is pening"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BUSY_A::VALUE1
    }
    #[doc = "Update is pending"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BUSY_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Mirror All Execution Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Mirror All Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mirrallstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIRRALLSTAT_SPEC;
impl crate::RegisterSpec for MIRRALLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mirrallstat::R`](R) reader structure"]
impl crate::Readable for MIRRALLSTAT_SPEC {}
#[doc = "`reset()` method sets MIRRALLSTAT to value 0"]
impl crate::Resettable for MIRRALLSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
