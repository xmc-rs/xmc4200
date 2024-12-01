#[doc = "Register `HDCLR` writer"]
pub type W = crate::W<HDCLR_SPEC>;
#[doc = "Wake-up Pin Event Positive Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPEV_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<EPEV_A> for bool {
    #[inline(always)]
    fn from(variant: EPEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` writer - Wake-up Pin Event Positive Edge Clear"]
pub type EPEV_W<'a, REG> = crate::BitWriter<'a, REG, EPEV_A>;
impl<'a, REG> EPEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EPEV_A::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EPEV_A::VALUE2)
    }
}
#[doc = "Wake-up Pin Event Negative Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENEV_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<ENEV_A> for bool {
    #[inline(always)]
    fn from(variant: ENEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` writer - Wake-up Pin Event Negative Edge Clear"]
pub type ENEV_W<'a, REG> = crate::BitWriter<'a, REG, ENEV_A>;
impl<'a, REG> ENEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENEV_A::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENEV_A::VALUE2)
    }
}
#[doc = "RTC Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEV_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<RTCEV_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` writer - RTC Event Clear"]
pub type RTCEV_W<'a, REG> = crate::BitWriter<'a, REG, RTCEV_A>;
impl<'a, REG> RTCEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEV_A::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEV_A::VALUE2)
    }
}
#[doc = "ULP WDG Alarm Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULPWDG_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear watchdog alarm"]
    VALUE2 = 1,
}
impl From<ULPWDG_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` writer - ULP WDG Alarm Clear"]
pub type ULPWDG_W<'a, REG> = crate::BitWriter<'a, REG, ULPWDG_A>;
impl<'a, REG> ULPWDG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDG_A::VALUE1)
    }
    #[doc = "Clear watchdog alarm"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ULPWDG_A::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATPEV_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<VBATPEV_A> for bool {
    #[inline(always)]
    fn from(variant: VBATPEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPEV` writer - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear"]
pub type VBATPEV_W<'a, REG> = crate::BitWriter<'a, REG, VBATPEV_A>;
impl<'a, REG> VBATPEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATPEV_A::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATPEV_A::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATNEV_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<VBATNEV_A> for bool {
    #[inline(always)]
    fn from(variant: VBATNEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATNEV` writer - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear"]
pub type VBATNEV_W<'a, REG> = crate::BitWriter<'a, REG, VBATNEV_A>;
impl<'a, REG> VBATNEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATNEV_A::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATNEV_A::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0PEV_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO0PEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0PEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear"]
pub type AHIBIO0PEV_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0PEV_A>;
impl<'a, REG> AHIBIO0PEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0PEV_A::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0PEV_A::VALUE2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0NEV_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear wake-up event"]
    VALUE2 = 1,
}
impl From<AHIBIO0NEV_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0NEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear"]
pub type AHIBIO0NEV_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0NEV_A>;
impl<'a, REG> AHIBIO0NEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0NEV_A::VALUE1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0NEV_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Clear"]
    #[inline(always)]
    pub fn epev(&mut self) -> EPEV_W<HDCLR_SPEC> {
        EPEV_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Clear"]
    #[inline(always)]
    pub fn enev(&mut self) -> ENEV_W<HDCLR_SPEC> {
        ENEV_W::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Event Clear"]
    #[inline(always)]
    pub fn rtcev(&mut self) -> RTCEV_W<HDCLR_SPEC> {
        RTCEV_W::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Clear"]
    #[inline(always)]
    pub fn ulpwdg(&mut self) -> ULPWDG_W<HDCLR_SPEC> {
        ULPWDG_W::new(self, 3)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear"]
    #[inline(always)]
    pub fn vbatpev(&mut self) -> VBATPEV_W<HDCLR_SPEC> {
        VBATPEV_W::new(self, 8)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear"]
    #[inline(always)]
    pub fn vbatnev(&mut self) -> VBATNEV_W<HDCLR_SPEC> {
        VBATNEV_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline(always)]
    pub fn ahibio0pev(&mut self) -> AHIBIO0PEV_W<HDCLR_SPEC> {
        AHIBIO0PEV_W::new(self, 10)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline(always)]
    pub fn ahibio0nev(&mut self) -> AHIBIO0NEV_W<HDCLR_SPEC> {
        AHIBIO0NEV_W::new(self, 11)
    }
}
#[doc = "Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HDCLR_SPEC;
impl crate::RegisterSpec for HDCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdclr::W`](W) writer structure"]
impl crate::Writable for HDCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDCLR to value 0"]
impl crate::Resettable for HDCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
