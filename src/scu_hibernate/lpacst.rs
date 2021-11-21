#[doc = "Register `LPACST` reader"]
pub struct R(crate::R<LPACST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPACST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPACST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPACST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Trigger VBAT Single Compare Operation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSCMP_A {
    #[doc = "0: Ready to start new compare operation"]
    VALUE1 = 0,
    #[doc = "1: Compare operation completed"]
    VALUE2 = 1,
}
impl From<VBATSCMP_A> for bool {
    #[inline(always)]
    fn from(variant: VBATSCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATSCMP` reader - Trigger VBAT Single Compare Operation Status"]
pub struct VBATSCMP_R(crate::FieldReader<bool, VBATSCMP_A>);
impl VBATSCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATSCMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATSCMP_A {
        match self.bits {
            false => VBATSCMP_A::VALUE1,
            true => VBATSCMP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBATSCMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VBATSCMP_A::VALUE2
    }
}
impl core::ops::Deref for VBATSCMP_R {
    type Target = crate::FieldReader<bool, VBATSCMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0SCMP_A {
    #[doc = "0: Ready to start new compare operation"]
    VALUE1 = 0,
    #[doc = "1: Compare operation completed"]
    VALUE2 = 1,
}
impl From<AHIBIO0SCMP_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0SCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0SCMP` reader - Trigger HIB_IO_0 Input Single Compare Operation Status"]
pub struct AHIBIO0SCMP_R(crate::FieldReader<bool, AHIBIO0SCMP_A>);
impl AHIBIO0SCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHIBIO0SCMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHIBIO0SCMP_A {
        match self.bits {
            false => AHIBIO0SCMP_A::VALUE1,
            true => AHIBIO0SCMP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AHIBIO0SCMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AHIBIO0SCMP_A::VALUE2
    }
}
impl core::ops::Deref for AHIBIO0SCMP_R {
    type Target = crate::FieldReader<bool, AHIBIO0SCMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "VBAT Compare Operation Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATVAL_A {
    #[doc = "0: Below programmed threshold"]
    VALUE1 = 0,
    #[doc = "1: Above programmed threshold"]
    VALUE2 = 1,
}
impl From<VBATVAL_A> for bool {
    #[inline(always)]
    fn from(variant: VBATVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATVAL` reader - VBAT Compare Operation Result"]
pub struct VBATVAL_R(crate::FieldReader<bool, VBATVAL_A>);
impl VBATVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBATVAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATVAL_A {
        match self.bits {
            false => VBATVAL_A::VALUE1,
            true => VBATVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VBATVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VBATVAL_A::VALUE2
    }
}
impl core::ops::Deref for VBATVAL_R {
    type Target = crate::FieldReader<bool, VBATVAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HIB_IO_0 Input Compare Operation Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0VAL_A {
    #[doc = "0: Below programmed threshold"]
    VALUE1 = 0,
    #[doc = "1: Above programmed threshold"]
    VALUE2 = 1,
}
impl From<AHIBIO0VAL_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0VAL` reader - HIB_IO_0 Input Compare Operation Result"]
pub struct AHIBIO0VAL_R(crate::FieldReader<bool, AHIBIO0VAL_A>);
impl AHIBIO0VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHIBIO0VAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHIBIO0VAL_A {
        match self.bits {
            false => AHIBIO0VAL_A::VALUE1,
            true => AHIBIO0VAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AHIBIO0VAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AHIBIO0VAL_A::VALUE2
    }
}
impl core::ops::Deref for AHIBIO0VAL_R {
    type Target = crate::FieldReader<bool, AHIBIO0VAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Status"]
    #[inline(always)]
    pub fn vbatscmp(&self) -> VBATSCMP_R {
        VBATSCMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Status"]
    #[inline(always)]
    pub fn ahibio0scmp(&self) -> AHIBIO0SCMP_R {
        AHIBIO0SCMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - VBAT Compare Operation Result"]
    #[inline(always)]
    pub fn vbatval(&self) -> VBATVAL_R {
        VBATVAL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Operation Result"]
    #[inline(always)]
    pub fn ahibio0val(&self) -> AHIBIO0VAL_R {
        AHIBIO0VAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Hibernate Analog Control State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacst](index.html) module"]
pub struct LPACST_SPEC;
impl crate::RegisterSpec for LPACST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpacst::R](R) reader structure"]
impl crate::Readable for LPACST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPACST to value 0"]
impl crate::Resettable for LPACST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
