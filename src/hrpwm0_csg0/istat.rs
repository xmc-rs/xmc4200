#[doc = "Register `ISTAT` reader"]
pub type R = crate::R<IstatSpec>;
#[doc = "Value switch from CSGyDSV1 to CSGyDSV2 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vls1s {
    #[doc = "0: Value switch not detected"]
    Value1 = 0,
    #[doc = "1: Value switch detected"]
    Value2 = 1,
}
impl From<Vls1s> for bool {
    #[inline(always)]
    fn from(variant: Vls1s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLS1S` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt status"]
pub type Vls1sR = crate::BitReader<Vls1s>;
impl Vls1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vls1s {
        match self.bits {
            false => Vls1s::Value1,
            true => Vls1s::Value2,
        }
    }
    #[doc = "Value switch not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vls1s::Value1
    }
    #[doc = "Value switch detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vls1s::Value2
    }
}
#[doc = "Value switch from CSGyDSV2 to CSGyDSV1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vls2s {
    #[doc = "0: Value switch not detected"]
    Value1 = 0,
    #[doc = "1: Value switch detected"]
    Value2 = 1,
}
impl From<Vls2s> for bool {
    #[inline(always)]
    fn from(variant: Vls2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLS2S` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt status"]
pub type Vls2sR = crate::BitReader<Vls2s>;
impl Vls2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vls2s {
        match self.bits {
            false => Vls2s::Value1,
            true => Vls2s::Value2,
        }
    }
    #[doc = "Value switch not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vls2s::Value1
    }
    #[doc = "Value switch detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vls2s::Value2
    }
}
#[doc = "Conversion trigger status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgss {
    #[doc = "0: Conversion trigger was not generated"]
    Value1 = 0,
    #[doc = "1: Conversion trigger was generated"]
    Value2 = 1,
}
impl From<Trgss> for bool {
    #[inline(always)]
    fn from(variant: Trgss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSS` reader - Conversion trigger status"]
pub type TrgssR = crate::BitReader<Trgss>;
impl TrgssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgss {
        match self.bits {
            false => Trgss::Value1,
            true => Trgss::Value2,
        }
    }
    #[doc = "Conversion trigger was not generated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Trgss::Value1
    }
    #[doc = "Conversion trigger was generated"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Trgss::Value2
    }
}
#[doc = "Start trigger interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Strss {
    #[doc = "0: Start trigger not detected"]
    Value1 = 0,
    #[doc = "1: Start trigger detected"]
    Value2 = 1,
}
impl From<Strss> for bool {
    #[inline(always)]
    fn from(variant: Strss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STRSS` reader - Start trigger interrupt status"]
pub type StrssR = crate::BitReader<Strss>;
impl StrssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Strss {
        match self.bits {
            false => Strss::Value1,
            true => Strss::Value2,
        }
    }
    #[doc = "Start trigger not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Strss::Value1
    }
    #[doc = "Start trigger detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Strss::Value2
    }
}
#[doc = "Stop trigger interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stpss {
    #[doc = "0: Stop trigger not detected"]
    Value1 = 0,
    #[doc = "1: Stop trigger detected"]
    Value2 = 1,
}
impl From<Stpss> for bool {
    #[inline(always)]
    fn from(variant: Stpss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPSS` reader - Stop trigger interrupt status"]
pub type StpssR = crate::BitReader<Stpss>;
impl StpssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stpss {
        match self.bits {
            false => Stpss::Value1,
            true => Stpss::Value2,
        }
    }
    #[doc = "Stop trigger not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stpss::Value1
    }
    #[doc = "Stop trigger detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stpss::Value2
    }
}
#[doc = "Shadow transfer interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stds {
    #[doc = "0: Shadow transfer was not performed"]
    Value1 = 0,
    #[doc = "1: Shadow transfer was performed"]
    Value2 = 1,
}
impl From<Stds> for bool {
    #[inline(always)]
    fn from(variant: Stds) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDS` reader - Shadow transfer interrupt status"]
pub type StdsR = crate::BitReader<Stds>;
impl StdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stds {
        match self.bits {
            false => Stds::Value1,
            true => Stds::Value2,
        }
    }
    #[doc = "Shadow transfer was not performed"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stds::Value1
    }
    #[doc = "Shadow transfer was performed"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stds::Value2
    }
}
#[doc = "Comparator rise interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crss {
    #[doc = "0: Comparator output LOW to HIGH transition not detected"]
    Value1 = 0,
    #[doc = "1: Comparator output LOW to HIGH transition detected"]
    Value2 = 1,
}
impl From<Crss> for bool {
    #[inline(always)]
    fn from(variant: Crss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSS` reader - Comparator rise interrupt status"]
pub type CrssR = crate::BitReader<Crss>;
impl CrssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crss {
        match self.bits {
            false => Crss::Value1,
            true => Crss::Value2,
        }
    }
    #[doc = "Comparator output LOW to HIGH transition not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Crss::Value1
    }
    #[doc = "Comparator output LOW to HIGH transition detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Crss::Value2
    }
}
#[doc = "Comparator fall interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfss {
    #[doc = "0: Comparator output HIGH to LOW transition not detected"]
    Value1 = 0,
    #[doc = "1: Comparator output HIGH to LOW transition detected"]
    Value2 = 1,
}
impl From<Cfss> for bool {
    #[inline(always)]
    fn from(variant: Cfss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFSS` reader - Comparator fall interrupt status"]
pub type CfssR = crate::BitReader<Cfss>;
impl CfssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfss {
        match self.bits {
            false => Cfss::Value1,
            true => Cfss::Value2,
        }
    }
    #[doc = "Comparator output HIGH to LOW transition not detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cfss::Value1
    }
    #[doc = "Comparator output HIGH to LOW transition detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cfss::Value2
    }
}
#[doc = "Comparator clamped interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cses {
    #[doc = "0: Comparator output has been set to the clamped state"]
    Value1 = 0,
    #[doc = "1: Comparator output has not been set to the clamped state"]
    Value2 = 1,
}
impl From<Cses> for bool {
    #[inline(always)]
    fn from(variant: Cses) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSES` reader - Comparator clamped interrupt status"]
pub type CsesR = crate::BitReader<Cses>;
impl CsesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cses {
        match self.bits {
            false => Cses::Value1,
            true => Cses::Value2,
        }
    }
    #[doc = "Comparator output has been set to the clamped state"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Cses::Value1
    }
    #[doc = "Comparator output has not been set to the clamped state"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Cses::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt status"]
    #[inline(always)]
    pub fn vls1s(&self) -> Vls1sR {
        Vls1sR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt status"]
    #[inline(always)]
    pub fn vls2s(&self) -> Vls2sR {
        Vls2sR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Conversion trigger status"]
    #[inline(always)]
    pub fn trgss(&self) -> TrgssR {
        TrgssR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start trigger interrupt status"]
    #[inline(always)]
    pub fn strss(&self) -> StrssR {
        StrssR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop trigger interrupt status"]
    #[inline(always)]
    pub fn stpss(&self) -> StpssR {
        StpssR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shadow transfer interrupt status"]
    #[inline(always)]
    pub fn stds(&self) -> StdsR {
        StdsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator rise interrupt status"]
    #[inline(always)]
    pub fn crss(&self) -> CrssR {
        CrssR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator fall interrupt status"]
    #[inline(always)]
    pub fn cfss(&self) -> CfssR {
        CfssR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparator clamped interrupt status"]
    #[inline(always)]
    pub fn cses(&self) -> CsesR {
        CsesR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Service request status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`istat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstatSpec;
impl crate::RegisterSpec for IstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for IstatSpec {}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for IstatSpec {
    const RESET_VALUE: u32 = 0;
}
