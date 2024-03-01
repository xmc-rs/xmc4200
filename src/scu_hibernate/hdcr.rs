#[doc = "Register `HDCR` reader"]
pub type R = crate::R<HdcrSpec>;
#[doc = "Register `HDCR` writer"]
pub type W = crate::W<HdcrSpec>;
#[doc = "Wake-Up on Pin Event Positive Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkpep {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Wkpep> for bool {
    #[inline(always)]
    fn from(variant: Wkpep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKPEP` reader - Wake-Up on Pin Event Positive Edge Enable"]
pub type WkpepR = crate::BitReader<Wkpep>;
impl WkpepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkpep {
        match self.bits {
            false => Wkpep::Value1,
            true => Wkpep::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wkpep::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wkpep::Value2
    }
}
#[doc = "Field `WKPEP` writer - Wake-Up on Pin Event Positive Edge Enable"]
pub type WkpepW<'a, REG> = crate::BitWriter<'a, REG, Wkpep>;
impl<'a, REG> WkpepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpep::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpep::Value2)
    }
}
#[doc = "Wake-up on Pin Event Negative Edge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkpen {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Wkpen> for bool {
    #[inline(always)]
    fn from(variant: Wkpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKPEN` reader - Wake-up on Pin Event Negative Edge Enable"]
pub type WkpenR = crate::BitReader<Wkpen>;
impl WkpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkpen {
        match self.bits {
            false => Wkpen::Value1,
            true => Wkpen::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wkpen::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wkpen::Value2
    }
}
#[doc = "Field `WKPEN` writer - Wake-up on Pin Event Negative Edge Enable"]
pub type WkpenW<'a, REG> = crate::BitWriter<'a, REG, Wkpen>;
impl<'a, REG> WkpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpen::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wkpen::Value2)
    }
}
#[doc = "Wake-up on RTC Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtce {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Rtce> for bool {
    #[inline(always)]
    fn from(variant: Rtce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCE` reader - Wake-up on RTC Event Enable"]
pub type RtceR = crate::BitReader<Rtce>;
impl RtceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtce {
        match self.bits {
            false => Rtce::Value1,
            true => Rtce::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rtce::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rtce::Value2
    }
}
#[doc = "Field `RTCE` writer - Wake-up on RTC Event Enable"]
pub type RtceW<'a, REG> = crate::BitWriter<'a, REG, Rtce>;
impl<'a, REG> RtceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtce::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtce::Value2)
    }
}
#[doc = "ULP WDG Alarm Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdgen {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Ulpwdgen> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGEN` reader - ULP WDG Alarm Enable"]
pub type UlpwdgenR = crate::BitReader<Ulpwdgen>;
impl UlpwdgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulpwdgen {
        match self.bits {
            false => Ulpwdgen::Value1,
            true => Ulpwdgen::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ulpwdgen::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ulpwdgen::Value2
    }
}
#[doc = "Field `ULPWDGEN` writer - ULP WDG Alarm Enable"]
pub type UlpwdgenW<'a, REG> = crate::BitWriter<'a, REG, Ulpwdgen>;
impl<'a, REG> UlpwdgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdgen::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdgen::Value2)
    }
}
#[doc = "Hibernate Request Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hib {
    #[doc = "0: External hibernate request inactive"]
    Value1 = 0,
    #[doc = "1: External hibernate request active"]
    Value2 = 1,
}
impl From<Hib> for bool {
    #[inline(always)]
    fn from(variant: Hib) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIB` reader - Hibernate Request Value Set"]
pub type HibR = crate::BitReader<Hib>;
impl HibR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hib {
        match self.bits {
            false => Hib::Value1,
            true => Hib::Value2,
        }
    }
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hib::Value1
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hib::Value2
    }
}
#[doc = "Field `HIB` writer - Hibernate Request Value Set"]
pub type HibW<'a, REG> = crate::BitWriter<'a, REG, Hib>;
impl<'a, REG> HibW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External hibernate request inactive"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Value1)
    }
    #[doc = "External hibernate request active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hib::Value2)
    }
}
#[doc = "Multiplex Control for RTC_XTAL_1 Select as GPI Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xtalgpi1sel {
    #[doc = "0: RTC_XTAL_1 input selected"]
    Value1 = 0,
    #[doc = "1: Analog comparator output for HIB_IO_1 or pre-selected digital IO input"]
    Value2 = 1,
}
impl From<Xtalgpi1sel> for bool {
    #[inline(always)]
    fn from(variant: Xtalgpi1sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTALGPI1SEL` reader - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
pub type Xtalgpi1selR = crate::BitReader<Xtalgpi1sel>;
impl Xtalgpi1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xtalgpi1sel {
        match self.bits {
            false => Xtalgpi1sel::Value1,
            true => Xtalgpi1sel::Value2,
        }
    }
    #[doc = "RTC_XTAL_1 input selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Xtalgpi1sel::Value1
    }
    #[doc = "Analog comparator output for HIB_IO_1 or pre-selected digital IO input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Xtalgpi1sel::Value2
    }
}
#[doc = "Field `XTALGPI1SEL` writer - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
pub type Xtalgpi1selW<'a, REG> = crate::BitWriter<'a, REG, Xtalgpi1sel>;
impl<'a, REG> Xtalgpi1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_XTAL_1 input selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalgpi1sel::Value1)
    }
    #[doc = "Analog comparator output for HIB_IO_1 or pre-selected digital IO input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Xtalgpi1sel::Value2)
    }
}
#[doc = "fRTC Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcs {
    #[doc = "0: fOSI selected"]
    Value1 = 0,
    #[doc = "1: fULP selected"]
    Value2 = 1,
}
impl From<Rcs> for bool {
    #[inline(always)]
    fn from(variant: Rcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCS` reader - fRTC Clock Selection"]
pub type RcsR = crate::BitReader<Rcs>;
impl RcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcs {
        match self.bits {
            false => Rcs::Value1,
            true => Rcs::Value2,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rcs::Value1
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rcs::Value2
    }
}
#[doc = "Field `RCS` writer - fRTC Clock Selection"]
pub type RcsW<'a, REG> = crate::BitWriter<'a, REG, Rcs>;
impl<'a, REG> RcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rcs::Value1)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rcs::Value2)
    }
}
#[doc = "fSTDBY Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stdbysel {
    #[doc = "0: fOSI selected"]
    Value1 = 0,
    #[doc = "1: fULP selected"]
    Value2 = 1,
}
impl From<Stdbysel> for bool {
    #[inline(always)]
    fn from(variant: Stdbysel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STDBYSEL` reader - fSTDBY Clock Selection"]
pub type StdbyselR = crate::BitReader<Stdbysel>;
impl StdbyselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stdbysel {
        match self.bits {
            false => Stdbysel::Value1,
            true => Stdbysel::Value2,
        }
    }
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Stdbysel::Value1
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Stdbysel::Value2
    }
}
#[doc = "Field `STDBYSEL` writer - fSTDBY Clock Selection"]
pub type StdbyselW<'a, REG> = crate::BitWriter<'a, REG, Stdbysel>;
impl<'a, REG> StdbyselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOSI selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Stdbysel::Value1)
    }
    #[doc = "fULP selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Stdbysel::Value2)
    }
}
#[doc = "Wake-Up from Hibernate Trigger Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupsel {
    #[doc = "0: HIB_IO_1 pin selected"]
    Value1 = 0,
    #[doc = "1: HIB_IO_0 pin selected"]
    Value2 = 1,
}
impl From<Wkupsel> for bool {
    #[inline(always)]
    fn from(variant: Wkupsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPSEL` reader - Wake-Up from Hibernate Trigger Input Selection"]
pub type WkupselR = crate::BitReader<Wkupsel>;
impl WkupselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupsel {
        match self.bits {
            false => Wkupsel::Value1,
            true => Wkupsel::Value2,
        }
    }
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Wkupsel::Value1
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Wkupsel::Value2
    }
}
#[doc = "Field `WKUPSEL` writer - Wake-Up from Hibernate Trigger Input Selection"]
pub type WkupselW<'a, REG> = crate::BitWriter<'a, REG, Wkupsel>;
impl<'a, REG> WkupselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIB_IO_1 pin selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupsel::Value1)
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupsel::Value2)
    }
}
#[doc = "General Purpose Input 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpi0sel {
    #[doc = "1: HIB_IO_0 pin selected"]
    Value2 = 1,
}
impl From<Gpi0sel> for bool {
    #[inline(always)]
    fn from(variant: Gpi0sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPI0SEL` reader - General Purpose Input 0 Selection"]
pub type Gpi0selR = crate::BitReader<Gpi0sel>;
impl Gpi0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpi0sel> {
        match self.bits {
            true => Some(Gpi0sel::Value2),
            _ => None,
        }
    }
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gpi0sel::Value2
    }
}
#[doc = "Field `GPI0SEL` writer - General Purpose Input 0 Selection"]
pub type Gpi0selW<'a, REG> = crate::BitWriter<'a, REG, Gpi0sel>;
impl<'a, REG> Gpi0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIB_IO_0 pin selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gpi0sel::Value2)
    }
}
#[doc = "HIBIO0 Polarity Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibio0pol {
    #[doc = "0: Direct value"]
    Value1 = 0,
    #[doc = "1: Inverted value"]
    Value2 = 1,
}
impl From<Hibio0pol> for bool {
    #[inline(always)]
    fn from(variant: Hibio0pol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBIO0POL` reader - HIBIO0 Polarity Set"]
pub type Hibio0polR = crate::BitReader<Hibio0pol>;
impl Hibio0polR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibio0pol {
        match self.bits {
            false => Hibio0pol::Value1,
            true => Hibio0pol::Value2,
        }
    }
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hibio0pol::Value1
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hibio0pol::Value2
    }
}
#[doc = "Field `HIBIO0POL` writer - HIBIO0 Polarity Set"]
pub type Hibio0polW<'a, REG> = crate::BitWriter<'a, REG, Hibio0pol>;
impl<'a, REG> Hibio0polW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Direct value"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0pol::Value1)
    }
    #[doc = "Inverted value"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0pol::Value2)
    }
}
#[doc = "Select Analog Channel 0 or Digital Output Path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adig0sel {
    #[doc = "0: Digital input"]
    Value1 = 0,
    #[doc = "1: Analog comparator result for HIB_IO_0"]
    Value2 = 1,
}
impl From<Adig0sel> for bool {
    #[inline(always)]
    fn from(variant: Adig0sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADIG0SEL` reader - Select Analog Channel 0 or Digital Output Path"]
pub type Adig0selR = crate::BitReader<Adig0sel>;
impl Adig0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adig0sel {
        match self.bits {
            false => Adig0sel::Value1,
            true => Adig0sel::Value2,
        }
    }
    #[doc = "Digital input"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Adig0sel::Value1
    }
    #[doc = "Analog comparator result for HIB_IO_0"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Adig0sel::Value2
    }
}
#[doc = "Field `ADIG0SEL` writer - Select Analog Channel 0 or Digital Output Path"]
pub type Adig0selW<'a, REG> = crate::BitWriter<'a, REG, Adig0sel>;
impl<'a, REG> Adig0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Digital input"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Adig0sel::Value1)
    }
    #[doc = "Analog comparator result for HIB_IO_0"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Adig0sel::Value2)
    }
}
#[doc = "HIB_IO_0 Pin I/O Control (default HIBOUT)\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hibio0sel {
    #[doc = "0: Direct input, No input pull device connected"]
    Value1 = 0,
    #[doc = "1: Direct input, Input pull-down device connected"]
    Value2 = 1,
    #[doc = "2: Direct input, Input pull-up device connected"]
    Value3 = 2,
    #[doc = "8: Push-pull HIB Control output"]
    Value4 = 8,
    #[doc = "9: Push-pull WDT service output"]
    Value5 = 9,
    #[doc = "10: Push-pull GPIO output"]
    Value6 = 10,
    #[doc = "12: Open-drain HIB Control output"]
    Value7 = 12,
    #[doc = "13: Open-drain WDT service output"]
    Value8 = 13,
    #[doc = "14: Open-drain GPIO output"]
    Value9 = 14,
    #[doc = "15: Analog input"]
    Value10 = 15,
}
impl From<Hibio0sel> for u8 {
    #[inline(always)]
    fn from(variant: Hibio0sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hibio0sel {
    type Ux = u8;
}
#[doc = "Field `HIBIO0SEL` reader - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type Hibio0selR = crate::FieldReader<Hibio0sel>;
impl Hibio0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hibio0sel> {
        match self.bits {
            0 => Some(Hibio0sel::Value1),
            1 => Some(Hibio0sel::Value2),
            2 => Some(Hibio0sel::Value3),
            8 => Some(Hibio0sel::Value4),
            9 => Some(Hibio0sel::Value5),
            10 => Some(Hibio0sel::Value6),
            12 => Some(Hibio0sel::Value7),
            13 => Some(Hibio0sel::Value8),
            14 => Some(Hibio0sel::Value9),
            15 => Some(Hibio0sel::Value10),
            _ => None,
        }
    }
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hibio0sel::Value1
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hibio0sel::Value2
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Hibio0sel::Value3
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Hibio0sel::Value4
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == Hibio0sel::Value5
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == Hibio0sel::Value6
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == Hibio0sel::Value7
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn is_value8(&self) -> bool {
        *self == Hibio0sel::Value8
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn is_value9(&self) -> bool {
        *self == Hibio0sel::Value9
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn is_value10(&self) -> bool {
        *self == Hibio0sel::Value10
    }
}
#[doc = "Field `HIBIO0SEL` writer - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
pub type Hibio0selW<'a, REG> = crate::FieldWriter<'a, REG, 4, Hibio0sel>;
impl<'a, REG> Hibio0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Direct input, No input pull device connected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value1)
    }
    #[doc = "Direct input, Input pull-down device connected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value2)
    }
    #[doc = "Direct input, Input pull-up device connected"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value3)
    }
    #[doc = "Push-pull HIB Control output"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value4)
    }
    #[doc = "Push-pull WDT service output"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value5)
    }
    #[doc = "Push-pull GPIO output"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value6)
    }
    #[doc = "Open-drain HIB Control output"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value7)
    }
    #[doc = "Open-drain WDT service output"]
    #[inline(always)]
    pub fn value8(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value8)
    }
    #[doc = "Open-drain GPIO output"]
    #[inline(always)]
    pub fn value9(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value9)
    }
    #[doc = "Analog input"]
    #[inline(always)]
    pub fn value10(self) -> &'a mut crate::W<REG> {
        self.variant(Hibio0sel::Value10)
    }
}
#[doc = "Wake-Up on VBAT Falling Below Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatlo {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Vbatlo> for bool {
    #[inline(always)]
    fn from(variant: Vbatlo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATLO` reader - Wake-Up on VBAT Falling Below Threshold Enable"]
pub type VbatloR = crate::BitReader<Vbatlo>;
impl VbatloR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatlo {
        match self.bits {
            false => Vbatlo::Value1,
            true => Vbatlo::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vbatlo::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vbatlo::Value2
    }
}
#[doc = "Field `VBATLO` writer - Wake-Up on VBAT Falling Below Threshold Enable"]
pub type VbatloW<'a, REG> = crate::BitWriter<'a, REG, Vbatlo>;
impl<'a, REG> VbatloW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatlo::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatlo::Value2)
    }
}
#[doc = "Wake-Up on VBAT Rising Above Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbathi {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Vbathi> for bool {
    #[inline(always)]
    fn from(variant: Vbathi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATHI` reader - Wake-Up on VBAT Rising Above Threshold Enable"]
pub type VbathiR = crate::BitReader<Vbathi>;
impl VbathiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbathi {
        match self.bits {
            false => Vbathi::Value1,
            true => Vbathi::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vbathi::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vbathi::Value2
    }
}
#[doc = "Field `VBATHI` writer - Wake-Up on VBAT Rising Above Threshold Enable"]
pub type VbathiW<'a, REG> = crate::BitWriter<'a, REG, Vbathi>;
impl<'a, REG> VbathiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbathi::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbathi::Value2)
    }
}
#[doc = "Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0lo {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Ahibio0lo> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0lo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0LO` reader - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
pub type Ahibio0loR = crate::BitReader<Ahibio0lo>;
impl Ahibio0loR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahibio0lo {
        match self.bits {
            false => Ahibio0lo::Value1,
            true => Ahibio0lo::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ahibio0lo::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ahibio0lo::Value2
    }
}
#[doc = "Field `AHIBIO0LO` writer - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
pub type Ahibio0loW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0lo>;
impl<'a, REG> Ahibio0loW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0lo::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0lo::Value2)
    }
}
#[doc = "Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0hi {
    #[doc = "0: Wake-up event disabled"]
    Value1 = 0,
    #[doc = "1: Wake-up event enabled"]
    Value2 = 1,
}
impl From<Ahibio0hi> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0hi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0HI` reader - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
pub type Ahibio0hiR = crate::BitReader<Ahibio0hi>;
impl Ahibio0hiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahibio0hi {
        match self.bits {
            false => Ahibio0hi::Value1,
            true => Ahibio0hi::Value2,
        }
    }
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ahibio0hi::Value1
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ahibio0hi::Value2
    }
}
#[doc = "Field `AHIBIO0HI` writer - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
pub type Ahibio0hiW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0hi>;
impl<'a, REG> Ahibio0hiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up event disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0hi::Value1)
    }
    #[doc = "Wake-up event enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0hi::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    pub fn wkpep(&self) -> WkpepR {
        WkpepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    pub fn wkpen(&self) -> WkpenR {
        WkpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    pub fn rtce(&self) -> RtceR {
        RtceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    pub fn ulpwdgen(&self) -> UlpwdgenR {
        UlpwdgenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    pub fn hib(&self) -> HibR {
        HibR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
    #[inline(always)]
    pub fn xtalgpi1sel(&self) -> Xtalgpi1selR {
        Xtalgpi1selR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    pub fn rcs(&self) -> RcsR {
        RcsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    pub fn stdbysel(&self) -> StdbyselR {
        StdbyselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    pub fn wkupsel(&self) -> WkupselR {
        WkupselR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    pub fn gpi0sel(&self) -> Gpi0selR {
        Gpi0selR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    pub fn hibio0pol(&self) -> Hibio0polR {
        Hibio0polR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Select Analog Channel 0 or Digital Output Path"]
    #[inline(always)]
    pub fn adig0sel(&self) -> Adig0selR {
        Adig0selR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    pub fn hibio0sel(&self) -> Hibio0selR {
        Hibio0selR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Wake-Up on VBAT Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn vbatlo(&self) -> VbatloR {
        VbatloR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake-Up on VBAT Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn vbathi(&self) -> VbathiR {
        VbathiR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
    #[inline(always)]
    pub fn ahibio0lo(&self) -> Ahibio0loR {
        Ahibio0loR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
    #[inline(always)]
    pub fn ahibio0hi(&self) -> Ahibio0hiR {
        Ahibio0hiR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-Up on Pin Event Positive Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkpep(&mut self) -> WkpepW<HdcrSpec> {
        WkpepW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up on Pin Event Negative Edge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkpen(&mut self) -> WkpenW<HdcrSpec> {
        WkpenW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-up on RTC Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtce(&mut self) -> RtceW<HdcrSpec> {
        RtceW::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdgen(&mut self) -> UlpwdgenW<HdcrSpec> {
        UlpwdgenW::new(self, 3)
    }
    #[doc = "Bit 4 - Hibernate Request Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn hib(&mut self) -> HibW<HdcrSpec> {
        HibW::new(self, 4)
    }
    #[doc = "Bit 5 - Multiplex Control for RTC_XTAL_1 Select as GPI Input"]
    #[inline(always)]
    #[must_use]
    pub fn xtalgpi1sel(&mut self) -> Xtalgpi1selW<HdcrSpec> {
        Xtalgpi1selW::new(self, 5)
    }
    #[doc = "Bit 6 - fRTC Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn rcs(&mut self) -> RcsW<HdcrSpec> {
        RcsW::new(self, 6)
    }
    #[doc = "Bit 7 - fSTDBY Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn stdbysel(&mut self) -> StdbyselW<HdcrSpec> {
        StdbyselW::new(self, 7)
    }
    #[doc = "Bit 8 - Wake-Up from Hibernate Trigger Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wkupsel(&mut self) -> WkupselW<HdcrSpec> {
        WkupselW::new(self, 8)
    }
    #[doc = "Bit 10 - General Purpose Input 0 Selection"]
    #[inline(always)]
    #[must_use]
    pub fn gpi0sel(&mut self) -> Gpi0selW<HdcrSpec> {
        Gpi0selW::new(self, 10)
    }
    #[doc = "Bit 12 - HIBIO0 Polarity Set"]
    #[inline(always)]
    #[must_use]
    pub fn hibio0pol(&mut self) -> Hibio0polW<HdcrSpec> {
        Hibio0polW::new(self, 12)
    }
    #[doc = "Bit 14 - Select Analog Channel 0 or Digital Output Path"]
    #[inline(always)]
    #[must_use]
    pub fn adig0sel(&mut self) -> Adig0selW<HdcrSpec> {
        Adig0selW::new(self, 14)
    }
    #[doc = "Bits 16:19 - HIB_IO_0 Pin I/O Control (default HIBOUT)"]
    #[inline(always)]
    #[must_use]
    pub fn hibio0sel(&mut self) -> Hibio0selW<HdcrSpec> {
        Hibio0selW::new(self, 16)
    }
    #[doc = "Bit 24 - Wake-Up on VBAT Falling Below Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbatlo(&mut self) -> VbatloW<HdcrSpec> {
        VbatloW::new(self, 24)
    }
    #[doc = "Bit 25 - Wake-Up on VBAT Rising Above Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbathi(&mut self) -> VbathiW<HdcrSpec> {
        VbathiW::new(self, 25)
    }
    #[doc = "Bit 26 - Wake-Up on Analog HIB_IO_0 Falling Below Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0lo(&mut self) -> Ahibio0loW<HdcrSpec> {
        Ahibio0loW::new(self, 26)
    }
    #[doc = "Bit 27 - Wake-Up on Analog HIB_IO_0 Rising Above Threshold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0hi(&mut self) -> Ahibio0hiW<HdcrSpec> {
        Ahibio0hiW::new(self, 27)
    }
}
#[doc = "Hibernate Domain Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcrSpec;
impl crate::RegisterSpec for HdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdcr::R`](R) reader structure"]
impl crate::Readable for HdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcr::W`](W) writer structure"]
impl crate::Writable for HdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDCR to value 0x000c_2000"]
impl crate::Resettable for HdcrSpec {
    const RESET_VALUE: u32 = 0x000c_2000;
}
