#[doc = "Register `HDSTAT` reader"]
pub type R = crate::R<HdstatSpec>;
#[doc = "Wake-up Pin Event Positive Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epev {
    #[doc = "0: Wake-up on positive edge pin event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on positive edge pin event active"]
    Value2 = 1,
}
impl From<Epev> for bool {
    #[inline(always)]
    fn from(variant: Epev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` reader - Wake-up Pin Event Positive Edge"]
pub type EpevR = crate::BitReader<Epev>;
impl EpevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epev {
        match self.bits {
            false => Epev::Value1,
            true => Epev::Value2,
        }
    }
    #[doc = "Wake-up on positive edge pin event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Epev::Value1
    }
    #[doc = "Wake-up on positive edge pin event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Epev::Value2
    }
}
#[doc = "Wake-up Pin Event Negative Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enev {
    #[doc = "0: Wake-up on negative edge pin event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on negative edge pin event active"]
    Value2 = 1,
}
impl From<Enev> for bool {
    #[inline(always)]
    fn from(variant: Enev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` reader - Wake-up Pin Event Negative Edge"]
pub type EnevR = crate::BitReader<Enev>;
impl EnevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enev {
        match self.bits {
            false => Enev::Value1,
            true => Enev::Value2,
        }
    }
    #[doc = "Wake-up on negative edge pin event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Enev::Value1
    }
    #[doc = "Wake-up on negative edge pin event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Enev::Value2
    }
}
#[doc = "RTC Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcev {
    #[doc = "0: Wake-up on RTC event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on RTC event active"]
    Value2 = 1,
}
impl From<Rtcev> for bool {
    #[inline(always)]
    fn from(variant: Rtcev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` reader - RTC Event"]
pub type RtcevR = crate::BitReader<Rtcev>;
impl RtcevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcev {
        match self.bits {
            false => Rtcev::Value1,
            true => Rtcev::Value2,
        }
    }
    #[doc = "Wake-up on RTC event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Rtcev::Value1
    }
    #[doc = "Wake-up on RTC event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Rtcev::Value2
    }
}
#[doc = "ULP WDG Alarm Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdg {
    #[doc = "0: Watchdog alarm did not occur"]
    Value1 = 0,
    #[doc = "1: Watchdog alarm occurred"]
    Value2 = 1,
}
impl From<Ulpwdg> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` reader - ULP WDG Alarm Status"]
pub type UlpwdgR = crate::BitReader<Ulpwdg>;
impl UlpwdgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulpwdg {
        match self.bits {
            false => Ulpwdg::Value1,
            true => Ulpwdg::Value2,
        }
    }
    #[doc = "Watchdog alarm did not occur"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ulpwdg::Value1
    }
    #[doc = "Watchdog alarm occurred"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ulpwdg::Value2
    }
}
#[doc = "Hibernate Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibnout {
    #[doc = "0: Hibernate not driven active to pads"]
    Value1 = 0,
    #[doc = "1: Hibernate driven active to pads"]
    Value2 = 1,
}
impl From<Hibnout> for bool {
    #[inline(always)]
    fn from(variant: Hibnout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNOUT` reader - Hibernate Control Status"]
pub type HibnoutR = crate::BitReader<Hibnout>;
impl HibnoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibnout {
        match self.bits {
            false => Hibnout::Value1,
            true => Hibnout::Value2,
        }
    }
    #[doc = "Hibernate not driven active to pads"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hibnout::Value1
    }
    #[doc = "Hibernate driven active to pads"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hibnout::Value2
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatpev {
    #[doc = "0: Wake-up on rising above threshold event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on rising above threshold event active"]
    Value2 = 1,
}
impl From<Vbatpev> for bool {
    #[inline(always)]
    fn from(variant: Vbatpev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPEV` reader - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing"]
pub type VbatpevR = crate::BitReader<Vbatpev>;
impl VbatpevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatpev {
        match self.bits {
            false => Vbatpev::Value1,
            true => Vbatpev::Value2,
        }
    }
    #[doc = "Wake-up on rising above threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vbatpev::Value1
    }
    #[doc = "Wake-up on rising above threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vbatpev::Value2
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatnev {
    #[doc = "0: Wake-up on falling below threshold event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on falling below threshold event active"]
    Value2 = 1,
}
impl From<Vbatnev> for bool {
    #[inline(always)]
    fn from(variant: Vbatnev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATNEV` reader - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing"]
pub type VbatnevR = crate::BitReader<Vbatnev>;
impl VbatnevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbatnev {
        match self.bits {
            false => Vbatnev::Value1,
            true => Vbatnev::Value2,
        }
    }
    #[doc = "Wake-up on falling below threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Vbatnev::Value1
    }
    #[doc = "Wake-up on falling below threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Vbatnev::Value2
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0pev {
    #[doc = "0: Wake-up on rising above threshold event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on rising above threshold event active"]
    Value2 = 1,
}
impl From<Ahibio0pev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0pev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0PEV` reader - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing"]
pub type Ahibio0pevR = crate::BitReader<Ahibio0pev>;
impl Ahibio0pevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahibio0pev {
        match self.bits {
            false => Ahibio0pev::Value1,
            true => Ahibio0pev::Value2,
        }
    }
    #[doc = "Wake-up on rising above threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ahibio0pev::Value1
    }
    #[doc = "Wake-up on rising above threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ahibio0pev::Value2
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0nev {
    #[doc = "0: Wake-up on falling below threshold event inactive"]
    Value1 = 0,
    #[doc = "1: Wake-up on falling below threshold event active"]
    Value2 = 1,
}
impl From<Ahibio0nev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0nev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0NEV` reader - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing"]
pub type Ahibio0nevR = crate::BitReader<Ahibio0nev>;
impl Ahibio0nevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahibio0nev {
        match self.bits {
            false => Ahibio0nev::Value1,
            true => Ahibio0nev::Value2,
        }
    }
    #[doc = "Wake-up on falling below threshold event inactive"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ahibio0nev::Value1
    }
    #[doc = "Wake-up on falling below threshold event active"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ahibio0nev::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge"]
    #[inline(always)]
    pub fn epev(&self) -> EpevR {
        EpevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge"]
    #[inline(always)]
    pub fn enev(&self) -> EnevR {
        EnevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Event"]
    #[inline(always)]
    pub fn rtcev(&self) -> RtcevR {
        RtcevR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Status"]
    #[inline(always)]
    pub fn ulpwdg(&self) -> UlpwdgR {
        UlpwdgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Hibernate Control Status"]
    #[inline(always)]
    pub fn hibnout(&self) -> HibnoutR {
        HibnoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing"]
    #[inline(always)]
    pub fn vbatpev(&self) -> VbatpevR {
        VbatpevR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing"]
    #[inline(always)]
    pub fn vbatnev(&self) -> VbatnevR {
        VbatnevR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio0pev(&self) -> Ahibio0pevR {
        Ahibio0pevR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio0nev(&self) -> Ahibio0nevR {
        Ahibio0nevR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Hibernate Domain Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdstatSpec;
impl crate::RegisterSpec for HdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hdstat::R`](R) reader structure"]
impl crate::Readable for HdstatSpec {}
#[doc = "`reset()` method sets HDSTAT to value 0"]
impl crate::Resettable for HdstatSpec {
    const RESET_VALUE: u32 = 0;
}
