#[doc = "Register `PEFLAG` reader"]
pub type R = crate::R<PEFLAG_SPEC>;
#[doc = "Register `PEFLAG` writer"]
pub type W = crate::W<PEFLAG_SPEC>;
#[doc = "Parity Error Flag for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEFPS_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEFPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFPS` reader - Parity Error Flag for PSRAM"]
pub type PEFPS_R = crate::BitReader<PEFPS_A>;
impl PEFPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEFPS_A {
        match self.bits {
            false => PEFPS_A::VALUE1,
            true => PEFPS_A::VALUE2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEFPS_A::VALUE1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEFPS_A::VALUE2
    }
}
#[doc = "Field `PEFPS` writer - Parity Error Flag for PSRAM"]
pub type PEFPS_W<'a, REG> = crate::BitWriter<'a, REG, PEFPS_A>;
impl<'a, REG> PEFPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEFPS_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEFPS_A::VALUE2)
    }
}
#[doc = "Parity Error Flag for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEFDS1_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEFDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFDS1` reader - Parity Error Flag for DSRAM1"]
pub type PEFDS1_R = crate::BitReader<PEFDS1_A>;
impl PEFDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEFDS1_A {
        match self.bits {
            false => PEFDS1_A::VALUE1,
            true => PEFDS1_A::VALUE2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEFDS1_A::VALUE1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEFDS1_A::VALUE2
    }
}
#[doc = "Field `PEFDS1` writer - Parity Error Flag for DSRAM1"]
pub type PEFDS1_W<'a, REG> = crate::BitWriter<'a, REG, PEFDS1_A>;
impl<'a, REG> PEFDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEFDS1_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEFDS1_A::VALUE2)
    }
}
#[doc = "Parity Error Flag for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEFU0_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEFU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU0` reader - Parity Error Flag for USIC0 Memory"]
pub type PEFU0_R = crate::BitReader<PEFU0_A>;
impl PEFU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEFU0_A {
        match self.bits {
            false => PEFU0_A::VALUE1,
            true => PEFU0_A::VALUE2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEFU0_A::VALUE1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEFU0_A::VALUE2
    }
}
#[doc = "Field `PEFU0` writer - Parity Error Flag for USIC0 Memory"]
pub type PEFU0_W<'a, REG> = crate::BitWriter<'a, REG, PEFU0_A>;
impl<'a, REG> PEFU0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEFU0_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEFU0_A::VALUE2)
    }
}
#[doc = "Parity Error Flag for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEFU1_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEFU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFU1` reader - Parity Error Flag for USIC1 Memory"]
pub type PEFU1_R = crate::BitReader<PEFU1_A>;
impl PEFU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEFU1_A {
        match self.bits {
            false => PEFU1_A::VALUE1,
            true => PEFU1_A::VALUE2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEFU1_A::VALUE1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEFU1_A::VALUE2
    }
}
#[doc = "Field `PEFU1` writer - Parity Error Flag for USIC1 Memory"]
pub type PEFU1_W<'a, REG> = crate::BitWriter<'a, REG, PEFU1_A>;
impl<'a, REG> PEFU1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEFU1_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEFU1_A::VALUE2)
    }
}
#[doc = "Parity Error Flag for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEFMC_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEFMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFMC` reader - Parity Error Flag for MultiCAN Memory"]
pub type PEFMC_R = crate::BitReader<PEFMC_A>;
impl PEFMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEFMC_A {
        match self.bits {
            false => PEFMC_A::VALUE1,
            true => PEFMC_A::VALUE2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEFMC_A::VALUE1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEFMC_A::VALUE2
    }
}
#[doc = "Field `PEFMC` writer - Parity Error Flag for MultiCAN Memory"]
pub type PEFMC_W<'a, REG> = crate::BitWriter<'a, REG, PEFMC_A>;
impl<'a, REG> PEFMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEFMC_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEFMC_A::VALUE2)
    }
}
#[doc = "Parity Error Flag for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEFPPRF_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEFPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEFPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEFPPRF` reader - Parity Error Flag for PMU Prefetch Memory"]
pub type PEFPPRF_R = crate::BitReader<PEFPPRF_A>;
impl PEFPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEFPPRF_A {
        match self.bits {
            false => PEFPPRF_A::VALUE1,
            true => PEFPPRF_A::VALUE2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEFPPRF_A::VALUE1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEFPPRF_A::VALUE2
    }
}
#[doc = "Field `PEFPPRF` writer - Parity Error Flag for PMU Prefetch Memory"]
pub type PEFPPRF_W<'a, REG> = crate::BitWriter<'a, REG, PEFPPRF_A>;
impl<'a, REG> PEFPPRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEFPPRF_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEFPPRF_A::VALUE2)
    }
}
#[doc = "Parity Error Flag for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEUSB_A {
    #[doc = "0: No parity error detected"]
    VALUE1 = 0,
    #[doc = "1: Parity error detected"]
    VALUE2 = 1,
}
impl From<PEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEUSB` reader - Parity Error Flag for USB Memory"]
pub type PEUSB_R = crate::BitReader<PEUSB_A>;
impl PEUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEUSB_A {
        match self.bits {
            false => PEUSB_A::VALUE1,
            true => PEUSB_A::VALUE2,
        }
    }
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEUSB_A::VALUE1
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEUSB_A::VALUE2
    }
}
#[doc = "Field `PEUSB` writer - Parity Error Flag for USB Memory"]
pub type PEUSB_W<'a, REG> = crate::BitWriter<'a, REG, PEUSB_A>;
impl<'a, REG> PEUSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error detected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PEUSB_A::VALUE1)
    }
    #[doc = "Parity error detected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PEUSB_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    pub fn pefps(&self) -> PEFPS_R {
        PEFPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    pub fn pefds1(&self) -> PEFDS1_R {
        PEFDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    pub fn pefu0(&self) -> PEFU0_R {
        PEFU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    pub fn pefu1(&self) -> PEFU1_R {
        PEFU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    pub fn pefmc(&self) -> PEFMC_R {
        PEFMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn pefpprf(&self) -> PEFPPRF_R {
        PEFPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    pub fn peusb(&self) -> PEUSB_R {
        PEUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Flag for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn pefps(&mut self) -> PEFPS_W<PEFLAG_SPEC> {
        PEFPS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Flag for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn pefds1(&mut self) -> PEFDS1_W<PEFLAG_SPEC> {
        PEFDS1_W::new(self, 1)
    }
    #[doc = "Bit 8 - Parity Error Flag for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefu0(&mut self) -> PEFU0_W<PEFLAG_SPEC> {
        PEFU0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Flag for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefu1(&mut self) -> PEFU1_W<PEFLAG_SPEC> {
        PEFU1_W::new(self, 9)
    }
    #[doc = "Bit 12 - Parity Error Flag for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefmc(&mut self) -> PEFMC_W<PEFLAG_SPEC> {
        PEFMC_W::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Flag for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn pefpprf(&mut self) -> PEFPPRF_W<PEFLAG_SPEC> {
        PEFPPRF_W::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Flag for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peusb(&mut self) -> PEUSB_W<PEFLAG_SPEC> {
        PEUSB_W::new(self, 16)
    }
}
#[doc = "Parity Error Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`peflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEFLAG_SPEC;
impl crate::RegisterSpec for PEFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peflag::R`](R) reader structure"]
impl crate::Readable for PEFLAG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peflag::W`](W) writer structure"]
impl crate::Writable for PEFLAG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEFLAG to value 0"]
impl crate::Resettable for PEFLAG_SPEC {
    const RESET_VALUE: u32 = 0;
}
