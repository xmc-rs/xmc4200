#[doc = "Reader of register HDSTAT"]
pub type R = crate::R<u32, super::HDSTAT>;
#[doc = "Wake-up Pin Event Positive Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPEV_A {
    #[doc = "0: Wake-up on positive edge pin event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on positive edge pin event active"]
    VALUE2 = 1,
}
impl From<EPEV_A> for bool {
    #[inline(always)]
    fn from(variant: EPEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPEV`"]
pub type EPEV_R = crate::R<bool, EPEV_A>;
impl EPEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPEV_A {
        match self.bits {
            false => EPEV_A::VALUE1,
            true => EPEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPEV_A::VALUE2
    }
}
#[doc = "Wake-up Pin Event Negative Edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENEV_A {
    #[doc = "0: Wake-up on negative edge pin event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on negative edge pin event active"]
    VALUE2 = 1,
}
impl From<ENEV_A> for bool {
    #[inline(always)]
    fn from(variant: ENEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENEV`"]
pub type ENEV_R = crate::R<bool, ENEV_A>;
impl ENEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENEV_A {
        match self.bits {
            false => ENEV_A::VALUE1,
            true => ENEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENEV_A::VALUE2
    }
}
#[doc = "RTC Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEV_A {
    #[doc = "0: Wake-up on RTC event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on RTC event active"]
    VALUE2 = 1,
}
impl From<RTCEV_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCEV`"]
pub type RTCEV_R = crate::R<bool, RTCEV_A>;
impl RTCEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEV_A {
        match self.bits {
            false => RTCEV_A::VALUE1,
            true => RTCEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RTCEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RTCEV_A::VALUE2
    }
}
#[doc = "ULP WDG Alarm Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDG_A {
    #[doc = "0: Watchdog alarm did not occur"]
    VALUE1 = 0,
    #[doc = "1: Watchdog alarm occurred"]
    VALUE2 = 1,
}
impl From<ULPWDG_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ULPWDG`"]
pub type ULPWDG_R = crate::R<bool, ULPWDG_A>;
impl ULPWDG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDG_A {
        match self.bits {
            false => ULPWDG_A::VALUE1,
            true => ULPWDG_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDG_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDG_A::VALUE2
    }
}
#[doc = "Hibernate Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBNOUT_A {
    #[doc = "0: Hibernate not driven active to pads"]
    VALUE1 = 0,
    #[doc = "1: Hibernate driven active to pads"]
    VALUE2 = 1,
}
impl From<HIBNOUT_A> for bool {
    #[inline(always)]
    fn from(variant: HIBNOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HIBNOUT`"]
pub type HIBNOUT_R = crate::R<bool, HIBNOUT_A>;
impl HIBNOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBNOUT_A {
        match self.bits {
            false => HIBNOUT_A::VALUE1,
            true => HIBNOUT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBNOUT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBNOUT_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATPEV_A {
    #[doc = "0: Wake-up on rising above threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on rising above threshold event active"]
    VALUE2 = 1,
}
impl From<VBATPEV_A> for bool {
    #[inline(always)]
    fn from(variant: VBATPEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBATPEV`"]
pub type VBATPEV_R = crate::R<bool, VBATPEV_A>;
impl VBATPEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATPEV_A {
        match self.bits {
            false => VBATPEV_A::VALUE1,
            true => VBATPEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATPEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATPEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATNEV_A {
    #[doc = "0: Wake-up on falling below threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on falling below threshold event active"]
    VALUE2 = 1,
}
impl From<VBATNEV_A> for bool {
    #[inline(always)]
    fn from(variant: VBATNEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VBATNEV`"]
pub type VBATNEV_R = crate::R<bool, VBATNEV_A>;
impl VBATNEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATNEV_A {
        match self.bits {
            false => VBATNEV_A::VALUE1,
            true => VBATNEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATNEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATNEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0PEV_A {
    #[doc = "0: Wake-up on rising above threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on rising above threshold event active"]
    VALUE2 = 1,
}
impl From<AHIBIO0PEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0PEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHIBIO0PEV`"]
pub type AHIBIO0PEV_R = crate::R<bool, AHIBIO0PEV_A>;
impl AHIBIO0PEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHIBIO0PEV_A {
        match self.bits {
            false => AHIBIO0PEV_A::VALUE1,
            true => AHIBIO0PEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0PEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0PEV_A::VALUE2
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0NEV_A {
    #[doc = "0: Wake-up on falling below threshold event inactive"]
    VALUE1 = 0,
    #[doc = "1: Wake-up on falling below threshold event active"]
    VALUE2 = 1,
}
impl From<AHIBIO0NEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0NEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHIBIO0NEV`"]
pub type AHIBIO0NEV_R = crate::R<bool, AHIBIO0NEV_A>;
impl AHIBIO0NEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHIBIO0NEV_A {
        match self.bits {
            false => AHIBIO0NEV_A::VALUE1,
            true => AHIBIO0NEV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0NEV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0NEV_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge"]
    #[inline(always)]
    pub fn epev(&self) -> EPEV_R {
        EPEV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge"]
    #[inline(always)]
    pub fn enev(&self) -> ENEV_R {
        ENEV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Event"]
    #[inline(always)]
    pub fn rtcev(&self) -> RTCEV_R {
        RTCEV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Status"]
    #[inline(always)]
    pub fn ulpwdg(&self) -> ULPWDG_R {
        ULPWDG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hibernate Control Status"]
    #[inline(always)]
    pub fn hibnout(&self) -> HIBNOUT_R {
        HIBNOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing"]
    #[inline(always)]
    pub fn vbatpev(&self) -> VBATPEV_R {
        VBATPEV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing"]
    #[inline(always)]
    pub fn vbatnev(&self) -> VBATNEV_R {
        VBATNEV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio0pev(&self) -> AHIBIO0PEV_R {
        AHIBIO0PEV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing"]
    #[inline(always)]
    pub fn ahibio0nev(&self) -> AHIBIO0NEV_R {
        AHIBIO0NEV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
