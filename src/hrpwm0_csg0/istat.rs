#[doc = "Register `ISTAT` reader"]
pub type R = crate::R<ISTAT_SPEC>;
#[doc = "Field `VLS1S` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt status"]
pub type VLS1S_R = crate::BitReader<VLS1S_A>;
#[doc = "Value switch from CSGyDSV1 to CSGyDSV2 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl VLS1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLS1S_A {
        match self.bits {
            false => VLS1S_A::VALUE1,
            true => VLS1S_A::VALUE2,
        }
    }
    #[doc = "Value switch not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VLS1S_A::VALUE1
    }
    #[doc = "Value switch detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VLS1S_A::VALUE2
    }
}
#[doc = "Field `VLS2S` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt status"]
pub type VLS2S_R = crate::BitReader<VLS2S_A>;
#[doc = "Value switch from CSGyDSV2 to CSGyDSV1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl VLS2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLS2S_A {
        match self.bits {
            false => VLS2S_A::VALUE1,
            true => VLS2S_A::VALUE2,
        }
    }
    #[doc = "Value switch not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VLS2S_A::VALUE1
    }
    #[doc = "Value switch detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VLS2S_A::VALUE2
    }
}
#[doc = "Field `TRGSS` reader - Conversion trigger status"]
pub type TRGSS_R = crate::BitReader<TRGSS_A>;
#[doc = "Conversion trigger status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TRGSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRGSS_A {
        match self.bits {
            false => TRGSS_A::VALUE1,
            true => TRGSS_A::VALUE2,
        }
    }
    #[doc = "Conversion trigger was not generated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRGSS_A::VALUE1
    }
    #[doc = "Conversion trigger was generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRGSS_A::VALUE2
    }
}
#[doc = "Field `STRSS` reader - Start trigger interrupt status"]
pub type STRSS_R = crate::BitReader<STRSS_A>;
#[doc = "Start trigger interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STRSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STRSS_A {
        match self.bits {
            false => STRSS_A::VALUE1,
            true => STRSS_A::VALUE2,
        }
    }
    #[doc = "Start trigger not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STRSS_A::VALUE1
    }
    #[doc = "Start trigger detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STRSS_A::VALUE2
    }
}
#[doc = "Field `STPSS` reader - Stop trigger interrupt status"]
pub type STPSS_R = crate::BitReader<STPSS_A>;
#[doc = "Stop trigger interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STPSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STPSS_A {
        match self.bits {
            false => STPSS_A::VALUE1,
            true => STPSS_A::VALUE2,
        }
    }
    #[doc = "Stop trigger not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STPSS_A::VALUE1
    }
    #[doc = "Stop trigger detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STPSS_A::VALUE2
    }
}
#[doc = "Field `STDS` reader - Shadow transfer interrupt status"]
pub type STDS_R = crate::BitReader<STDS_A>;
#[doc = "Shadow transfer interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl STDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STDS_A {
        match self.bits {
            false => STDS_A::VALUE1,
            true => STDS_A::VALUE2,
        }
    }
    #[doc = "Shadow transfer was not performed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STDS_A::VALUE1
    }
    #[doc = "Shadow transfer was performed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STDS_A::VALUE2
    }
}
#[doc = "Field `CRSS` reader - Comparator rise interrupt status"]
pub type CRSS_R = crate::BitReader<CRSS_A>;
#[doc = "Comparator rise interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CRSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRSS_A {
        match self.bits {
            false => CRSS_A::VALUE1,
            true => CRSS_A::VALUE2,
        }
    }
    #[doc = "Comparator output LOW to HIGH transition not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CRSS_A::VALUE1
    }
    #[doc = "Comparator output LOW to HIGH transition detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CRSS_A::VALUE2
    }
}
#[doc = "Field `CFSS` reader - Comparator fall interrupt status"]
pub type CFSS_R = crate::BitReader<CFSS_A>;
#[doc = "Comparator fall interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CFSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFSS_A {
        match self.bits {
            false => CFSS_A::VALUE1,
            true => CFSS_A::VALUE2,
        }
    }
    #[doc = "Comparator output HIGH to LOW transition not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CFSS_A::VALUE1
    }
    #[doc = "Comparator output HIGH to LOW transition detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CFSS_A::VALUE2
    }
}
#[doc = "Field `CSES` reader - Comparator clamped interrupt status"]
pub type CSES_R = crate::BitReader<CSES_A>;
#[doc = "Comparator clamped interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl CSES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSES_A {
        match self.bits {
            false => CSES_A::VALUE1,
            true => CSES_A::VALUE2,
        }
    }
    #[doc = "Comparator output has been set to the clamped state"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CSES_A::VALUE1
    }
    #[doc = "Comparator output has not been set to the clamped state"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSES_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt status"]
    #[inline(always)]
    pub fn vls1s(&self) -> VLS1S_R {
        VLS1S_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt status"]
    #[inline(always)]
    pub fn vls2s(&self) -> VLS2S_R {
        VLS2S_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Conversion trigger status"]
    #[inline(always)]
    pub fn trgss(&self) -> TRGSS_R {
        TRGSS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start trigger interrupt status"]
    #[inline(always)]
    pub fn strss(&self) -> STRSS_R {
        STRSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop trigger interrupt status"]
    #[inline(always)]
    pub fn stpss(&self) -> STPSS_R {
        STPSS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shadow transfer interrupt status"]
    #[inline(always)]
    pub fn stds(&self) -> STDS_R {
        STDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator rise interrupt status"]
    #[inline(always)]
    pub fn crss(&self) -> CRSS_R {
        CRSS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator fall interrupt status"]
    #[inline(always)]
    pub fn cfss(&self) -> CFSS_R {
        CFSS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparator clamped interrupt status"]
    #[inline(always)]
    pub fn cses(&self) -> CSES_R {
        CSES_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Service request status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTAT_SPEC;
impl crate::RegisterSpec for ISTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for ISTAT_SPEC {}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for ISTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
