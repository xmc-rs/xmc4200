#[doc = "Register `HRGHRS` reader"]
pub type R = crate::R<HRGHRS_SPEC>;
#[doc = "High Resolution Generation Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRGR_A {
    #[doc = "0: High resolution logic is not working"]
    VALUE1 = 0,
    #[doc = "1: High resolution logic is working"]
    VALUE2 = 1,
}
impl From<HRGR_A> for bool {
    #[inline(always)]
    fn from(variant: HRGR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRGR` reader - High Resolution Generation Ready"]
pub type HRGR_R = crate::BitReader<HRGR_A>;
impl HRGR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HRGR_A {
        match self.bits {
            false => HRGR_A::VALUE1,
            true => HRGR_A::VALUE2,
        }
    }
    #[doc = "High resolution logic is not working"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HRGR_A::VALUE1
    }
    #[doc = "High resolution logic is working"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HRGR_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Generation Ready"]
    #[inline(always)]
    pub fn hrgr(&self) -> HRGR_R {
        HRGR_R::new((self.bits & 1) != 0)
    }
}
#[doc = "High Resolution Generation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrghrs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRGHRS_SPEC;
impl crate::RegisterSpec for HRGHRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrghrs::R`](R) reader structure"]
impl crate::Readable for HRGHRS_SPEC {}
#[doc = "`reset()` method sets HRGHRS to value 0"]
impl crate::Resettable for HRGHRS_SPEC {
    const RESET_VALUE: u32 = 0;
}
