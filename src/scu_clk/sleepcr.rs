#[doc = "Register `SLEEPCR` reader"]
pub type R = crate::R<SLEEPCR_SPEC>;
#[doc = "Register `SLEEPCR` writer"]
pub type W = crate::W<SLEEPCR_SPEC>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    VALUE1 = 0,
    #[doc = "1: fPLL clock"]
    VALUE2 = 1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SYSSEL_R = crate::BitReader<SYSSEL_A>;
impl SYSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::VALUE1,
            true => SYSSEL_A::VALUE2,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SYSSEL_A::VALUE1
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SYSSEL_A::VALUE2
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SYSSEL_W<'a, REG> = crate::BitWriter<'a, REG, SYSSEL_A>;
impl<'a, REG> SYSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE1)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::VALUE2)
    }
}
#[doc = "USB Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<USBCR_A> for bool {
    #[inline(always)]
    fn from(variant: USBCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCR` reader - USB Clock Control"]
pub type USBCR_R = crate::BitReader<USBCR_A>;
impl USBCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBCR_A {
        match self.bits {
            false => USBCR_A::VALUE1,
            true => USBCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USBCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USBCR_A::VALUE2
    }
}
#[doc = "Field `USBCR` writer - USB Clock Control"]
pub type USBCR_W<'a, REG> = crate::BitWriter<'a, REG, USBCR_A>;
impl<'a, REG> USBCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USBCR_A::VALUE2)
    }
}
#[doc = "CCU Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<CCUCR_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCR` reader - CCU Clock Control"]
pub type CCUCR_R = crate::BitReader<CCUCR_A>;
impl CCUCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUCR_A {
        match self.bits {
            false => CCUCR_A::VALUE1,
            true => CCUCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCUCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCUCR_A::VALUE2
    }
}
#[doc = "Field `CCUCR` writer - CCU Clock Control"]
pub type CCUCR_W<'a, REG> = crate::BitWriter<'a, REG, CCUCR_A>;
impl<'a, REG> CCUCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCR_A::VALUE2)
    }
}
#[doc = "WDT Clock Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCR_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WDTCR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCR` reader - WDT Clock Control"]
pub type WDTCR_R = crate::BitReader<WDTCR_A>;
impl WDTCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTCR_A {
        match self.bits {
            false => WDTCR_A::VALUE1,
            true => WDTCR_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WDTCR_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WDTCR_A::VALUE2
    }
}
#[doc = "Field `WDTCR` writer - WDT Clock Control"]
pub type WDTCR_W<'a, REG> = crate::BitWriter<'a, REG, WDTCR_A>;
impl<'a, REG> WDTCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCR_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCR_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    pub fn usbcr(&self) -> USBCR_R {
        USBCR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    pub fn ccucr(&self) -> CCUCR_R {
        CCUCR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WDTCR_R {
        WDTCR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn syssel(&mut self) -> SYSSEL_W<SLEEPCR_SPEC> {
        SYSSEL_W::new(self, 0)
    }
    #[doc = "Bit 16 - USB Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbcr(&mut self) -> USBCR_W<SLEEPCR_SPEC> {
        USBCR_W::new(self, 16)
    }
    #[doc = "Bit 20 - CCU Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn ccucr(&mut self) -> CCUCR_W<SLEEPCR_SPEC> {
        CCUCR_W::new(self, 20)
    }
    #[doc = "Bit 21 - WDT Clock Control"]
    #[inline(always)]
    #[must_use]
    pub fn wdtcr(&mut self) -> WDTCR_W<SLEEPCR_SPEC> {
        WDTCR_W::new(self, 21)
    }
}
#[doc = "Sleep Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEPCR_SPEC;
impl crate::RegisterSpec for SLEEPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleepcr::R`](R) reader structure"]
impl crate::Readable for SLEEPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleepcr::W`](W) writer structure"]
impl crate::Writable for SLEEPCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEPCR to value 0"]
impl crate::Resettable for SLEEPCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
