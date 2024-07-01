#[doc = "Register `CGATSET2` writer"]
pub type W = crate::W<CGATSET2_SPEC>;
#[doc = "WDT Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` writer - WDT Gating Set"]
pub type WDT_W<'a, REG> = crate::BitWriter<'a, REG, WDT_A>;
impl<'a, REG> WDT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_A::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDT_A::VALUE2)
    }
}
#[doc = "DMA0 Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA0_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<DMA0_A> for bool {
    #[inline(always)]
    fn from(variant: DMA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA0` writer - DMA0 Gating Set"]
pub type DMA0_W<'a, REG> = crate::BitWriter<'a, REG, DMA0_A>;
impl<'a, REG> DMA0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0_A::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMA0_A::VALUE2)
    }
}
#[doc = "FCE Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCE_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<FCE_A> for bool {
    #[inline(always)]
    fn from(variant: FCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCE` writer - FCE Gating Set"]
pub type FCE_W<'a, REG> = crate::BitWriter<'a, REG, FCE_A>;
impl<'a, REG> FCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FCE_A::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FCE_A::VALUE2)
    }
}
#[doc = "USB Gating Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USB_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Enable gating"]
    VALUE2 = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` writer - USB Gating Set"]
pub type USB_W<'a, REG> = crate::BitWriter<'a, REG, USB_A>;
impl<'a, REG> USB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USB_A::VALUE1)
    }
    #[doc = "Enable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USB_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 1 - WDT Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<CGATSET2_SPEC> {
        WDT_W::new(self, 1)
    }
    #[doc = "Bit 4 - DMA0 Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn dma0(&mut self) -> DMA0_W<CGATSET2_SPEC> {
        DMA0_W::new(self, 4)
    }
    #[doc = "Bit 6 - FCE Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn fce(&mut self) -> FCE_W<CGATSET2_SPEC> {
        FCE_W::new(self, 6)
    }
    #[doc = "Bit 7 - USB Gating Set"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<CGATSET2_SPEC> {
        USB_W::new(self, 7)
    }
}
#[doc = "Peripheral 2 Clock Gating Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgatset2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATSET2_SPEC;
impl crate::RegisterSpec for CGATSET2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatset2::W`](W) writer structure"]
impl crate::Writable for CGATSET2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATSET2 to value 0"]
impl crate::Resettable for CGATSET2_SPEC {
    const RESET_VALUE: u32 = 0;
}
