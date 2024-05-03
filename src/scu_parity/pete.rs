#[doc = "Register `PETE` reader"]
pub type R = crate::R<PETE_SPEC>;
#[doc = "Register `PETE` writer"]
pub type W = crate::W<PETE_SPEC>;
#[doc = "Parity Error Trap Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEPS_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEPS_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPS` reader - Parity Error Trap Enable for PSRAM"]
pub type PETEPS_R = crate::BitReader<PETEPS_A>;
impl PETEPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEPS_A {
        match self.bits {
            false => PETEPS_A::VALUE1,
            true => PETEPS_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEPS_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEPS_A::VALUE2
    }
}
#[doc = "Field `PETEPS` writer - Parity Error Trap Enable for PSRAM"]
pub type PETEPS_W<'a, REG> = crate::BitWriter<'a, REG, PETEPS_A>;
impl<'a, REG> PETEPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPS_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPS_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEDS1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEDS1` reader - Parity Error Trap Enable for DSRAM1"]
pub type PETEDS1_R = crate::BitReader<PETEDS1_A>;
impl PETEDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEDS1_A {
        match self.bits {
            false => PETEDS1_A::VALUE1,
            true => PETEDS1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEDS1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEDS1_A::VALUE2
    }
}
#[doc = "Field `PETEDS1` writer - Parity Error Trap Enable for DSRAM1"]
pub type PETEDS1_W<'a, REG> = crate::BitWriter<'a, REG, PETEDS1_A>;
impl<'a, REG> PETEDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEDS1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEDS1_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEU0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEU0_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU0` reader - Parity Error Trap Enable for USIC0 Memory"]
pub type PETEU0_R = crate::BitReader<PETEU0_A>;
impl PETEU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEU0_A {
        match self.bits {
            false => PETEU0_A::VALUE1,
            true => PETEU0_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU0_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU0_A::VALUE2
    }
}
#[doc = "Field `PETEU0` writer - Parity Error Trap Enable for USIC0 Memory"]
pub type PETEU0_W<'a, REG> = crate::BitWriter<'a, REG, PETEU0_A>;
impl<'a, REG> PETEU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU0_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEU1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEU1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU1` reader - Parity Error Trap Enable for USIC1 Memory"]
pub type PETEU1_R = crate::BitReader<PETEU1_A>;
impl PETEU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEU1_A {
        match self.bits {
            false => PETEU1_A::VALUE1,
            true => PETEU1_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEU1_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEU1_A::VALUE2
    }
}
#[doc = "Field `PETEU1` writer - Parity Error Trap Enable for USIC1 Memory"]
pub type PETEU1_W<'a, REG> = crate::BitWriter<'a, REG, PETEU1_A>;
impl<'a, REG> PETEU1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEU1_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEMC_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEMC_A> for bool {
    #[inline(always)]
    fn from(variant: PETEMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEMC` reader - Parity Error Trap Enable for MultiCAN Memory"]
pub type PETEMC_R = crate::BitReader<PETEMC_A>;
impl PETEMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEMC_A {
        match self.bits {
            false => PETEMC_A::VALUE1,
            true => PETEMC_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEMC_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEMC_A::VALUE2
    }
}
#[doc = "Field `PETEMC` writer - Parity Error Trap Enable for MultiCAN Memory"]
pub type PETEMC_W<'a, REG> = crate::BitWriter<'a, REG, PETEMC_A>;
impl<'a, REG> PETEMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEMC_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEMC_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEPPRF_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPPRF` reader - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PETEPPRF_R = crate::BitReader<PETEPPRF_A>;
impl PETEPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEPPRF_A {
        match self.bits {
            false => PETEPPRF_A::VALUE1,
            true => PETEPPRF_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEPPRF_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEPPRF_A::VALUE2
    }
}
#[doc = "Field `PETEPPRF` writer - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PETEPPRF_W<'a, REG> = crate::BitWriter<'a, REG, PETEPPRF_A>;
impl<'a, REG> PETEPPRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPPRF_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEPPRF_A::VALUE2)
    }
}
#[doc = "Parity Error Trap Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PETEUSB_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PETEUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEUSB` reader - Parity Error Trap Enable for USB Memory"]
pub type PETEUSB_R = crate::BitReader<PETEUSB_A>;
impl PETEUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PETEUSB_A {
        match self.bits {
            false => PETEUSB_A::VALUE1,
            true => PETEUSB_A::VALUE2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PETEUSB_A::VALUE1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PETEUSB_A::VALUE2
    }
}
#[doc = "Field `PETEUSB` writer - Parity Error Trap Enable for USB Memory"]
pub type PETEUSB_W<'a, REG> = crate::BitWriter<'a, REG, PETEUSB_A>;
impl<'a, REG> PETEUSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PETEUSB_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PETEUSB_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&self) -> PETEPS_R {
        PETEPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&self) -> PETEDS1_R {
        PETEDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&self) -> PETEU0_R {
        PETEU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&self) -> PETEU1_R {
        PETEU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&self) -> PETEMC_R {
        PETEMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&self) -> PETEPPRF_R {
        PETEPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&self) -> PETEUSB_R {
        PETEUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn peteps(&mut self) -> PETEPS_W<PETE_SPEC> {
        PETEPS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn peteds1(&mut self) -> PETEDS1_W<PETE_SPEC> {
        PETEDS1_W::new(self, 1)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu0(&mut self) -> PETEU0_W<PETE_SPEC> {
        PETEU0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu1(&mut self) -> PETEU1_W<PETE_SPEC> {
        PETEU1_W::new(self, 9)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petemc(&mut self) -> PETEMC_W<PETE_SPEC> {
        PETEMC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petepprf(&mut self) -> PETEPPRF_W<PETE_SPEC> {
        PETEPPRF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteusb(&mut self) -> PETEUSB_W<PETE_SPEC> {
        PETEUSB_W::new(self, 16)
    }
}
#[doc = "Parity Error Trap Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pete::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pete::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PETE_SPEC;
impl crate::RegisterSpec for PETE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pete::R`](R) reader structure"]
impl crate::Readable for PETE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pete::W`](W) writer structure"]
impl crate::Writable for PETE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PETE to value 0"]
impl crate::Resettable for PETE_SPEC {
    const RESET_VALUE: u32 = 0;
}
