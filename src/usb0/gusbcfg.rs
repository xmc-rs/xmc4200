#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GUSBCFG_SPEC>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GUSBCFG_SPEC>;
#[doc = "Field `TOutCal` reader - FS Timeout Calibration"]
pub type TOUT_CAL_R = crate::FieldReader;
#[doc = "Field `TOutCal` writer - FS Timeout Calibration"]
pub type TOUT_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYSel` reader - USB 1.1 Full-Speed Serial Transceiver Select"]
pub type PHYSEL_R = crate::BitReader<PHYSEL_A>;
#[doc = "USB 1.1 Full-Speed Serial Transceiver Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PHYSEL_A {
    #[doc = "1: USB 1.1 full-speed serial transceiver"]
    VALUE2 = 1,
}
impl From<PHYSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PHYSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl PHYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PHYSEL_A> {
        match self.bits {
            true => Some(PHYSEL_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "USB 1.1 full-speed serial transceiver"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PHYSEL_A::VALUE2
    }
}
#[doc = "Field `USBTrdTim` reader - USB Turnaround Time"]
pub type USBTRD_TIM_R = crate::FieldReader;
#[doc = "Field `USBTrdTim` writer - USB Turnaround Time"]
pub type USBTRD_TIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TxEndDelay` reader - Tx End Delay"]
pub type TX_END_DELAY_R = crate::BitReader<TX_END_DELAY_A>;
#[doc = "Tx End Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_END_DELAY_A {
    #[doc = "0: Normal mode"]
    VALUE1 = 0,
    #[doc = "1: Introduce Tx end delay timers"]
    VALUE2 = 1,
}
impl From<TX_END_DELAY_A> for bool {
    #[inline(always)]
    fn from(variant: TX_END_DELAY_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_END_DELAY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TX_END_DELAY_A {
        match self.bits {
            false => TX_END_DELAY_A::VALUE1,
            true => TX_END_DELAY_A::VALUE2,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TX_END_DELAY_A::VALUE1
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TX_END_DELAY_A::VALUE2
    }
}
#[doc = "Field `TxEndDelay` writer - Tx End Delay"]
pub type TX_END_DELAY_W<'a, REG> = crate::BitWriter<'a, REG, TX_END_DELAY_A>;
impl<'a, REG> TX_END_DELAY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TX_END_DELAY_A::VALUE1)
    }
    #[doc = "Introduce Tx end delay timers"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TX_END_DELAY_A::VALUE2)
    }
}
#[doc = "Field `ForceDevMode` reader - Force Device Mode"]
pub type FORCE_DEV_MODE_R = crate::BitReader<FORCE_DEV_MODE_A>;
#[doc = "Force Device Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_DEV_MODE_A {
    #[doc = "0: Normal Mode"]
    VALUE1 = 0,
    #[doc = "1: Force Device Mode"]
    VALUE2 = 1,
}
impl From<FORCE_DEV_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_DEV_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_DEV_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FORCE_DEV_MODE_A {
        match self.bits {
            false => FORCE_DEV_MODE_A::VALUE1,
            true => FORCE_DEV_MODE_A::VALUE2,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FORCE_DEV_MODE_A::VALUE1
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FORCE_DEV_MODE_A::VALUE2
    }
}
#[doc = "Field `ForceDevMode` writer - Force Device Mode"]
pub type FORCE_DEV_MODE_W<'a, REG> = crate::BitWriter<'a, REG, FORCE_DEV_MODE_A>;
impl<'a, REG> FORCE_DEV_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_DEV_MODE_A::VALUE1)
    }
    #[doc = "Force Device Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FORCE_DEV_MODE_A::VALUE2)
    }
}
#[doc = "Field `CTP` reader - Corrupt Tx packet"]
pub type CTP_R = crate::BitReader;
#[doc = "Field `CTP` writer - Corrupt Tx packet"]
pub type CTP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    pub fn tout_cal(&self) -> TOUT_CAL_R {
        TOUT_CAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrd_tim(&self) -> USBTRD_TIM_R {
        USBTRD_TIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn tx_end_delay(&self) -> TX_END_DELAY_R {
        TX_END_DELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    pub fn force_dev_mode(&self) -> FORCE_DEV_MODE_R {
        FORCE_DEV_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctp(&self) -> CTP_R {
        CTP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS Timeout Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cal(&mut self) -> TOUT_CAL_W<GUSBCFG_SPEC> {
        TOUT_CAL_W::new(self, 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrd_tim(&mut self) -> USBTRD_TIM_W<GUSBCFG_SPEC> {
        USBTRD_TIM_W::new(self, 10)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tx_end_delay(&mut self) -> TX_END_DELAY_W<GUSBCFG_SPEC> {
        TX_END_DELAY_W::new(self, 28)
    }
    #[doc = "Bit 30 - Force Device Mode"]
    #[inline(always)]
    #[must_use]
    pub fn force_dev_mode(&mut self) -> FORCE_DEV_MODE_W<GUSBCFG_SPEC> {
        FORCE_DEV_MODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    #[must_use]
    pub fn ctp(&mut self) -> CTP_W<GUSBCFG_SPEC> {
        CTP_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x1440;
}
