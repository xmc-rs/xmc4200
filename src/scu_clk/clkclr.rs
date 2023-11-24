#[doc = "Register `CLKCLR` writer"]
pub type W = crate::W<CLKCLR_SPEC>;
#[doc = "USB Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCDI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<USBCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: USBCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCDI` writer - USB Clock Disable"]
pub type USBCDI_W<'a, REG> = crate::BitWriter<'a, REG, USBCDI_AW>;
impl<'a, REG> USBCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBCDI_AW::VALUE2)
    }
}
#[doc = "CCU Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCDI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<CCUCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: CCUCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCDI` writer - CCU Clock Disable"]
pub type CCUCDI_W<'a, REG> = crate::BitWriter<'a, REG, CCUCDI_AW>;
impl<'a, REG> CCUCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCDI_AW::VALUE2)
    }
}
#[doc = "WDT Clock Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCDI_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable clock"]
    VALUE2 = 1,
}
impl From<WDTCDI_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCDI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCDI` writer - WDT Clock Disable"]
pub type WDTCDI_W<'a, REG> = crate::BitWriter<'a, REG, WDTCDI_AW>;
impl<'a, REG> WDTCDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCDI_AW::VALUE1)
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCDI_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - USB Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn usbcdi(&mut self) -> USBCDI_W<CLKCLR_SPEC> {
        USBCDI_W::new(self, 0)
    }
    #[doc = "Bit 4 - CCU Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ccucdi(&mut self) -> CCUCDI_W<CLKCLR_SPEC> {
        CCUCDI_W::new(self, 4)
    }
    #[doc = "Bit 5 - WDT Clock Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcdi(&mut self) -> WDTCDI_W<CLKCLR_SPEC> {
        WDTCDI_W::new(self, 5)
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
#[doc = "CLK Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCLR_SPEC;
impl crate::RegisterSpec for CLKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clkclr::W`](W) writer structure"]
impl crate::Writable for CLKCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCLR to value 0"]
impl crate::Resettable for CLKCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
