#[doc = "Register `HDCLR` writer"]
pub type W = crate::W<HdclrSpec>;
#[doc = "Wake-up Pin Event Positive Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear wake-up event"]
    Value2 = 1,
}
impl From<Epev> for bool {
    #[inline(always)]
    fn from(variant: Epev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPEV` writer - Wake-up Pin Event Positive Edge Clear"]
pub type EpevW<'a, REG> = crate::BitWriter<'a, REG, Epev>;
impl<'a, REG> EpevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Epev::Value1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Epev::Value2)
    }
}
#[doc = "Wake-up Pin Event Negative Edge Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear wake-up event"]
    Value2 = 1,
}
impl From<Enev> for bool {
    #[inline(always)]
    fn from(variant: Enev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENEV` writer - Wake-up Pin Event Negative Edge Clear"]
pub type EnevW<'a, REG> = crate::BitWriter<'a, REG, Enev>;
impl<'a, REG> EnevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Enev::Value1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Enev::Value2)
    }
}
#[doc = "RTC Event Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear wake-up event"]
    Value2 = 1,
}
impl From<Rtcev> for bool {
    #[inline(always)]
    fn from(variant: Rtcev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEV` writer - RTC Event Clear"]
pub type RtcevW<'a, REG> = crate::BitWriter<'a, REG, Rtcev>;
impl<'a, REG> RtcevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcev::Value1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcev::Value2)
    }
}
#[doc = "ULP WDG Alarm Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulpwdg {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear watchdog alarm"]
    Value2 = 1,
}
impl From<Ulpwdg> for bool {
    #[inline(always)]
    fn from(variant: Ulpwdg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDG` writer - ULP WDG Alarm Clear"]
pub type UlpwdgW<'a, REG> = crate::BitWriter<'a, REG, Ulpwdg>;
impl<'a, REG> UlpwdgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdg::Value1)
    }
    #[doc = "Clear watchdog alarm"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ulpwdg::Value2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatpev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear wake-up event"]
    Value2 = 1,
}
impl From<Vbatpev> for bool {
    #[inline(always)]
    fn from(variant: Vbatpev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATPEV` writer - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear"]
pub type VbatpevW<'a, REG> = crate::BitWriter<'a, REG, Vbatpev>;
impl<'a, REG> VbatpevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatpev::Value1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatpev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatnev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear wake-up event"]
    Value2 = 1,
}
impl From<Vbatnev> for bool {
    #[inline(always)]
    fn from(variant: Vbatnev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATNEV` writer - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear"]
pub type VbatnevW<'a, REG> = crate::BitWriter<'a, REG, Vbatnev>;
impl<'a, REG> VbatnevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatnev::Value1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatnev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0pev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear wake-up event"]
    Value2 = 1,
}
impl From<Ahibio0pev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0pev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0PEV` writer - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear"]
pub type Ahibio0pevW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0pev>;
impl<'a, REG> Ahibio0pevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0pev::Value1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0pev::Value2)
    }
}
#[doc = "Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0nev {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear wake-up event"]
    Value2 = 1,
}
impl From<Ahibio0nev> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0nev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0NEV` writer - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear"]
pub type Ahibio0nevW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0nev>;
impl<'a, REG> Ahibio0nevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0nev::Value1)
    }
    #[doc = "Clear wake-up event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0nev::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Wake-up Pin Event Positive Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn epev(&mut self) -> EpevW<HdclrSpec> {
        EpevW::new(self, 0)
    }
    #[doc = "Bit 1 - Wake-up Pin Event Negative Edge Clear"]
    #[inline(always)]
    #[must_use]
    pub fn enev(&mut self) -> EnevW<HdclrSpec> {
        EnevW::new(self, 1)
    }
    #[doc = "Bit 2 - RTC Event Clear"]
    #[inline(always)]
    #[must_use]
    pub fn rtcev(&mut self) -> RtcevW<HdclrSpec> {
        RtcevW::new(self, 2)
    }
    #[doc = "Bit 3 - ULP WDG Alarm Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulpwdg(&mut self) -> UlpwdgW<HdclrSpec> {
        UlpwdgW::new(self, 3)
    }
    #[doc = "Bit 8 - Wake-Up on LPAC Positive Edge of VBAT Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatpev(&mut self) -> VbatpevW<HdclrSpec> {
        VbatpevW::new(self, 8)
    }
    #[doc = "Bit 9 - Wake-Up on LPAC Negative Edge of VBAT Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatnev(&mut self) -> VbatnevW<HdclrSpec> {
        VbatnevW::new(self, 9)
    }
    #[doc = "Bit 10 - Wake-Up on LPAC Positive Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0pev(&mut self) -> Ahibio0pevW<HdclrSpec> {
        Ahibio0pevW::new(self, 10)
    }
    #[doc = "Bit 11 - Wake-Up on LPAC Negative Edge of HIB_IO_0 Threshold Crossing Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0nev(&mut self) -> Ahibio0nevW<HdclrSpec> {
        Ahibio0nevW::new(self, 11)
    }
}
#[doc = "Hibernate Domain Status Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdclrSpec;
impl crate::RegisterSpec for HdclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdclr::W`](W) writer structure"]
impl crate::Writable for HdclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HDCLR to value 0"]
impl crate::Resettable for HdclrSpec {
    const RESET_VALUE: u32 = 0;
}
