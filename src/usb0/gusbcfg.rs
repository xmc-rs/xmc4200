#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GusbcfgSpec>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GusbcfgSpec>;
#[doc = "Field `TOutCal` reader - FS Timeout Calibration"]
pub type ToutCalR = crate::FieldReader;
#[doc = "Field `TOutCal` writer - FS Timeout Calibration"]
pub type ToutCalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "USB 1.1 Full-Speed Serial Transceiver Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Physel {
    #[doc = "1: USB 1.1 full-speed serial transceiver"]
    Value2 = 1,
}
impl From<Physel> for bool {
    #[inline(always)]
    fn from(variant: Physel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHYSel` reader - USB 1.1 Full-Speed Serial Transceiver Select"]
pub type PhyselR = crate::BitReader<Physel>;
impl PhyselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Physel> {
        match self.bits {
            true => Some(Physel::Value2),
            _ => None,
        }
    }
    #[doc = "USB 1.1 full-speed serial transceiver"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Physel::Value2
    }
}
#[doc = "Field `USBTrdTim` reader - USB Turnaround Time"]
pub type UsbtrdTimR = crate::FieldReader;
#[doc = "Field `USBTrdTim` writer - USB Turnaround Time"]
pub type UsbtrdTimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Tx End Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxEndDelay {
    #[doc = "0: Normal mode"]
    Value1 = 0,
    #[doc = "1: Introduce Tx end delay timers"]
    Value2 = 1,
}
impl From<TxEndDelay> for bool {
    #[inline(always)]
    fn from(variant: TxEndDelay) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxEndDelay` reader - Tx End Delay"]
pub type TxEndDelayR = crate::BitReader<TxEndDelay>;
impl TxEndDelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxEndDelay {
        match self.bits {
            false => TxEndDelay::Value1,
            true => TxEndDelay::Value2,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TxEndDelay::Value1
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TxEndDelay::Value2
    }
}
#[doc = "Field `TxEndDelay` writer - Tx End Delay"]
pub type TxEndDelayW<'a, REG> = crate::BitWriter<'a, REG, TxEndDelay>;
impl<'a, REG> TxEndDelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TxEndDelay::Value1)
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TxEndDelay::Value2)
    }
}
#[doc = "Force Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceDevMode {
    #[doc = "0: Normal Mode"]
    Value1 = 0,
    #[doc = "1: Force Device Mode"]
    Value2 = 1,
}
impl From<ForceDevMode> for bool {
    #[inline(always)]
    fn from(variant: ForceDevMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForceDevMode` reader - Force Device Mode"]
pub type ForceDevModeR = crate::BitReader<ForceDevMode>;
impl ForceDevModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForceDevMode {
        match self.bits {
            false => ForceDevMode::Value1,
            true => ForceDevMode::Value2,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ForceDevMode::Value1
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ForceDevMode::Value2
    }
}
#[doc = "Field `ForceDevMode` writer - Force Device Mode"]
pub type ForceDevModeW<'a, REG> = crate::BitWriter<'a, REG, ForceDevMode>;
impl<'a, REG> ForceDevModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDevMode::Value1)
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ForceDevMode::Value2)
    }
}
#[doc = "Field `CTP` reader - Corrupt Tx packet"]
pub type CtpR = crate::BitReader;
#[doc = "Field `CTP` writer - Corrupt Tx packet"]
pub type CtpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&self) -> ToutCalR {
        ToutCalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&self) -> PhyselR {
        PhyselR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&self) -> UsbtrdTimR {
        UsbtrdTimR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&self) -> TxEndDelayR {
        TxEndDelayR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&self) -> ForceDevModeR {
        ForceDevModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&self) -> CtpR {
        CtpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cal(&mut self) -> ToutCalW<GusbcfgSpec> {
        ToutCalW::new(self, 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrd_tim(&mut self) -> UsbtrdTimW<GusbcfgSpec> {
        UsbtrdTimW::new(self, 10)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tx_end_delay(&mut self) -> TxEndDelayW<GusbcfgSpec> {
        TxEndDelayW::new(self, 28)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn force_dev_mode(&mut self) -> ForceDevModeW<GusbcfgSpec> {
        ForceDevModeW::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CtpW<GusbcfgSpec> {
        CtpW::new(self, 31)
    }
}
#[doc = "USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GusbcfgSpec;
impl crate::RegisterSpec for GusbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GusbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GusbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GusbcfgSpec {
    const RESET_VALUE: u32 = 0x1440;
}
