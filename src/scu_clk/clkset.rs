#[doc = "Register `CLKSET` writer"]
pub type W = crate::W<ClksetSpec>;
#[doc = "USB Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbcen {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Usbcen> for bool {
    #[inline(always)]
    fn from(variant: Usbcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCEN` writer - USB Clock Enable"]
pub type UsbcenW<'a, REG> = crate::BitWriter<'a, REG, Usbcen>;
impl<'a, REG> UsbcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcen::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usbcen::Value2)
    }
}
#[doc = "CCU Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccucen {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Ccucen> for bool {
    #[inline(always)]
    fn from(variant: Ccucen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCEN` writer - CCU Clock Enable"]
pub type CcucenW<'a, REG> = crate::BitWriter<'a, REG, Ccucen>;
impl<'a, REG> CcucenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucen::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccucen::Value2)
    }
}
#[doc = "WDT Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdtcen {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Wdtcen> for bool {
    #[inline(always)]
    fn from(variant: Wdtcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCEN` writer - WDT Clock Enable"]
pub type WdtcenW<'a, REG> = crate::BitWriter<'a, REG, Wdtcen>;
impl<'a, REG> WdtcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcen::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Wdtcen::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcen(&mut self) -> UsbcenW<ClksetSpec> {
        UsbcenW::new(self, 0)
    }
    #[doc = "Bit 4 - CCU Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucen(&mut self) -> CcucenW<ClksetSpec> {
        CcucenW::new(self, 4)
    }
    #[doc = "Bit 5 - WDT Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcen(&mut self) -> WdtcenW<ClksetSpec> {
        WdtcenW::new(self, 5)
    }
}
#[doc = "CLK Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClksetSpec;
impl crate::RegisterSpec for ClksetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkset::W`](W) writer structure"]
impl crate::Writable for ClksetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSET to value 0"]
impl crate::Resettable for ClksetSpec {
    const RESET_VALUE: u32 = 0;
}
