#[doc = "Register `HRGHRS` reader"]
pub struct R(crate::R<HRGHRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRGHRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRGHRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRGHRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "High Resolution Generation Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub struct HRGR_R(crate::FieldReader<bool, HRGR_A>);
impl HRGR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRGR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRGR_A {
        match self.bits {
            false => HRGR_A::VALUE1,
            true => HRGR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HRGR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HRGR_A::VALUE2
    }
}
impl core::ops::Deref for HRGR_R {
    type Target = crate::FieldReader<bool, HRGR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Generation Ready"]
    #[inline(always)]
    pub fn hrgr(&self) -> HRGR_R {
        HRGR_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "High Resolution Generation Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrghrs](index.html) module"]
pub struct HRGHRS_SPEC;
impl crate::RegisterSpec for HRGHRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrghrs::R](R) reader structure"]
impl crate::Readable for HRGHRS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HRGHRS to value 0"]
impl crate::Resettable for HRGHRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
