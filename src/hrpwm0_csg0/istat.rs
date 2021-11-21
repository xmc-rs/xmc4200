#[doc = "Register `ISTAT` reader"]
pub struct R(crate::R<ISTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Value switch from CSGyDSV1 to CSGyDSV2 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS1S_A {
    #[doc = "0: Value switch not detected"]
    VALUE1 = 0,
    #[doc = "1: Value switch detected"]
    VALUE2 = 1,
}
impl From<VLS1S_A> for bool {
    #[inline(always)]
    fn from(variant: VLS1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLS1S` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt status"]
pub struct VLS1S_R(crate::FieldReader<bool, VLS1S_A>);
impl VLS1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLS1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLS1S_A {
        match self.bits {
            false => VLS1S_A::VALUE1,
            true => VLS1S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VLS1S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VLS1S_A::VALUE2
    }
}
impl core::ops::Deref for VLS1S_R {
    type Target = crate::FieldReader<bool, VLS1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Value switch from CSGyDSV2 to CSGyDSV1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS2S_A {
    #[doc = "0: Value switch not detected"]
    VALUE1 = 0,
    #[doc = "1: Value switch detected"]
    VALUE2 = 1,
}
impl From<VLS2S_A> for bool {
    #[inline(always)]
    fn from(variant: VLS2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLS2S` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt status"]
pub struct VLS2S_R(crate::FieldReader<bool, VLS2S_A>);
impl VLS2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLS2S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLS2S_A {
        match self.bits {
            false => VLS2S_A::VALUE1,
            true => VLS2S_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VLS2S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VLS2S_A::VALUE2
    }
}
impl core::ops::Deref for VLS2S_R {
    type Target = crate::FieldReader<bool, VLS2S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Conversion trigger status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSS_A {
    #[doc = "0: Conversion trigger was not generated"]
    VALUE1 = 0,
    #[doc = "1: Conversion trigger was generated"]
    VALUE2 = 1,
}
impl From<TRGSS_A> for bool {
    #[inline(always)]
    fn from(variant: TRGSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSS` reader - Conversion trigger status"]
pub struct TRGSS_R(crate::FieldReader<bool, TRGSS_A>);
impl TRGSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSS_A {
        match self.bits {
            false => TRGSS_A::VALUE1,
            true => TRGSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRGSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRGSS_A::VALUE2
    }
}
impl core::ops::Deref for TRGSS_R {
    type Target = crate::FieldReader<bool, TRGSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Start trigger interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRSS_A {
    #[doc = "0: Start trigger not detected"]
    VALUE1 = 0,
    #[doc = "1: Start trigger detected"]
    VALUE2 = 1,
}
impl From<STRSS_A> for bool {
    #[inline(always)]
    fn from(variant: STRSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRSS` reader - Start trigger interrupt status"]
pub struct STRSS_R(crate::FieldReader<bool, STRSS_A>);
impl STRSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRSS_A {
        match self.bits {
            false => STRSS_A::VALUE1,
            true => STRSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STRSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STRSS_A::VALUE2
    }
}
impl core::ops::Deref for STRSS_R {
    type Target = crate::FieldReader<bool, STRSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Stop trigger interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPSS_A {
    #[doc = "0: Stop trigger not detected"]
    VALUE1 = 0,
    #[doc = "1: Stop trigger detected"]
    VALUE2 = 1,
}
impl From<STPSS_A> for bool {
    #[inline(always)]
    fn from(variant: STPSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPSS` reader - Stop trigger interrupt status"]
pub struct STPSS_R(crate::FieldReader<bool, STPSS_A>);
impl STPSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPSS_A {
        match self.bits {
            false => STPSS_A::VALUE1,
            true => STPSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STPSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STPSS_A::VALUE2
    }
}
impl core::ops::Deref for STPSS_R {
    type Target = crate::FieldReader<bool, STPSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shadow transfer interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STDS_A {
    #[doc = "0: Shadow transfer was not performed"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer was performed"]
    VALUE2 = 1,
}
impl From<STDS_A> for bool {
    #[inline(always)]
    fn from(variant: STDS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDS` reader - Shadow transfer interrupt status"]
pub struct STDS_R(crate::FieldReader<bool, STDS_A>);
impl STDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STDS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STDS_A {
        match self.bits {
            false => STDS_A::VALUE1,
            true => STDS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STDS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STDS_A::VALUE2
    }
}
impl core::ops::Deref for STDS_R {
    type Target = crate::FieldReader<bool, STDS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator rise interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRSS_A {
    #[doc = "0: Comparator output LOW to HIGH transition not detected"]
    VALUE1 = 0,
    #[doc = "1: Comparator output LOW to HIGH transition detected"]
    VALUE2 = 1,
}
impl From<CRSS_A> for bool {
    #[inline(always)]
    fn from(variant: CRSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSS` reader - Comparator rise interrupt status"]
pub struct CRSS_R(crate::FieldReader<bool, CRSS_A>);
impl CRSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRSS_A {
        match self.bits {
            false => CRSS_A::VALUE1,
            true => CRSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CRSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CRSS_A::VALUE2
    }
}
impl core::ops::Deref for CRSS_R {
    type Target = crate::FieldReader<bool, CRSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator fall interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFSS_A {
    #[doc = "0: Comparator output HIGH to LOW transition not detected"]
    VALUE1 = 0,
    #[doc = "1: Comparator output HIGH to LOW transition detected"]
    VALUE2 = 1,
}
impl From<CFSS_A> for bool {
    #[inline(always)]
    fn from(variant: CFSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFSS` reader - Comparator fall interrupt status"]
pub struct CFSS_R(crate::FieldReader<bool, CFSS_A>);
impl CFSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFSS_A {
        match self.bits {
            false => CFSS_A::VALUE1,
            true => CFSS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CFSS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CFSS_A::VALUE2
    }
}
impl core::ops::Deref for CFSS_R {
    type Target = crate::FieldReader<bool, CFSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Comparator clamped interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSES_A {
    #[doc = "0: Comparator output has been set to the clamped state"]
    VALUE1 = 0,
    #[doc = "1: Comparator output has not been set to the clamped state"]
    VALUE2 = 1,
}
impl From<CSES_A> for bool {
    #[inline(always)]
    fn from(variant: CSES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSES` reader - Comparator clamped interrupt status"]
pub struct CSES_R(crate::FieldReader<bool, CSES_A>);
impl CSES_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSES_A {
        match self.bits {
            false => CSES_A::VALUE1,
            true => CSES_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CSES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CSES_A::VALUE2
    }
}
impl core::ops::Deref for CSES_R {
    type Target = crate::FieldReader<bool, CSES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt status"]
    #[inline(always)]
    pub fn vls1s(&self) -> VLS1S_R {
        VLS1S_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt status"]
    #[inline(always)]
    pub fn vls2s(&self) -> VLS2S_R {
        VLS2S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Conversion trigger status"]
    #[inline(always)]
    pub fn trgss(&self) -> TRGSS_R {
        TRGSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start trigger interrupt status"]
    #[inline(always)]
    pub fn strss(&self) -> STRSS_R {
        STRSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stop trigger interrupt status"]
    #[inline(always)]
    pub fn stpss(&self) -> STPSS_R {
        STPSS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shadow transfer interrupt status"]
    #[inline(always)]
    pub fn stds(&self) -> STDS_R {
        STDS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comparator rise interrupt status"]
    #[inline(always)]
    pub fn crss(&self) -> CRSS_R {
        CRSS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparator fall interrupt status"]
    #[inline(always)]
    pub fn cfss(&self) -> CFSS_R {
        CFSS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparator clamped interrupt status"]
    #[inline(always)]
    pub fn cses(&self) -> CSES_R {
        CSES_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Service request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istat](index.html) module"]
pub struct ISTAT_SPEC;
impl crate::RegisterSpec for ISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [istat::R](R) reader structure"]
impl crate::Readable for ISTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for ISTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
