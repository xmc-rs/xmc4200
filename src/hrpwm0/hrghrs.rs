#[doc = "Register `HRGHRS` reader"]
pub type R = crate::R<HrghrsSpec>;
#[doc = "High Resolution Generation Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hrgr {
    #[doc = "0: High resolution logic is not working"]
    Value1 = 0,
    #[doc = "1: High resolution logic is working"]
    Value2 = 1,
}
impl From<Hrgr> for bool {
    #[inline(always)]
    fn from(variant: Hrgr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRGR` reader - High Resolution Generation Ready"]
pub type HrgrR = crate::BitReader<Hrgr>;
impl HrgrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hrgr {
        match self.bits {
            false => Hrgr::Value1,
            true => Hrgr::Value2,
        }
    }
    #[doc = "High resolution logic is not working"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hrgr::Value1
    }
    #[doc = "High resolution logic is working"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hrgr::Value2
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Generation Ready"]
    #[inline(always)]
    pub fn hrgr(&self) -> HrgrR {
        HrgrR::new((self.bits & 1) != 0)
    }
}
#[doc = "High Resolution Generation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrghrs::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrghrsSpec;
impl crate::RegisterSpec for HrghrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrghrs::R`](R) reader structure"]
impl crate::Readable for HrghrsSpec {}
#[doc = "`reset()` method sets HRGHRS to value 0"]
impl crate::Resettable for HrghrsSpec {
    const RESET_VALUE: u32 = 0;
}
