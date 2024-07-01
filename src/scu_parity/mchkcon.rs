#[doc = "Register `MCHKCON` reader"]
pub type R = crate::R<MCHKCON_SPEC>;
#[doc = "Register `MCHKCON` writer"]
pub type W = crate::W<MCHKCON_SPEC>;
#[doc = "Select Memory Check for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELPS_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELPS_A> for bool {
    #[inline(always)]
    fn from(variant: SELPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELPS` reader - Select Memory Check for PSRAM"]
pub type SELPS_R = crate::BitReader<SELPS_A>;
impl SELPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELPS_A {
        match self.bits {
            false => SELPS_A::VALUE1,
            true => SELPS_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELPS_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELPS_A::VALUE2
    }
}
#[doc = "Field `SELPS` writer - Select Memory Check for PSRAM"]
pub type SELPS_W<'a, REG> = crate::BitWriter<'a, REG, SELPS_A>;
impl<'a, REG> SELPS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELPS_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELPS_A::VALUE2)
    }
}
#[doc = "Select Memory Check for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELDS1_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELDS1_A> for bool {
    #[inline(always)]
    fn from(variant: SELDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELDS1` reader - Select Memory Check for DSRAM1"]
pub type SELDS1_R = crate::BitReader<SELDS1_A>;
impl SELDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELDS1_A {
        match self.bits {
            false => SELDS1_A::VALUE1,
            true => SELDS1_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELDS1_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELDS1_A::VALUE2
    }
}
#[doc = "Field `SELDS1` writer - Select Memory Check for DSRAM1"]
pub type SELDS1_W<'a, REG> = crate::BitWriter<'a, REG, SELDS1_A>;
impl<'a, REG> SELDS1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELDS1_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELDS1_A::VALUE2)
    }
}
#[doc = "Select Memory Check for USIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0DRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<USIC0DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0DRA` reader - Select Memory Check for USIC0"]
pub type USIC0DRA_R = crate::BitReader<USIC0DRA_A>;
impl USIC0DRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC0DRA_A {
        match self.bits {
            false => USIC0DRA_A::VALUE1,
            true => USIC0DRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0DRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0DRA_A::VALUE2
    }
}
#[doc = "Field `USIC0DRA` writer - Select Memory Check for USIC0"]
pub type USIC0DRA_W<'a, REG> = crate::BitWriter<'a, REG, USIC0DRA_A>;
impl<'a, REG> USIC0DRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0DRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0DRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for USIC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1DRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<USIC1DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1DRA` reader - Select Memory Check for USIC1"]
pub type USIC1DRA_R = crate::BitReader<USIC1DRA_A>;
impl USIC1DRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USIC1DRA_A {
        match self.bits {
            false => USIC1DRA_A::VALUE1,
            true => USIC1DRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC1DRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1DRA_A::VALUE2
    }
}
#[doc = "Field `USIC1DRA` writer - Select Memory Check for USIC1"]
pub type USIC1DRA_W<'a, REG> = crate::BitWriter<'a, REG, USIC1DRA_A>;
impl<'a, REG> USIC1DRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1DRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1DRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for MultiCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCANDRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<MCANDRA_A> for bool {
    #[inline(always)]
    fn from(variant: MCANDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANDRA` reader - Select Memory Check for MultiCAN"]
pub type MCANDRA_R = crate::BitReader<MCANDRA_A>;
impl MCANDRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCANDRA_A {
        match self.bits {
            false => MCANDRA_A::VALUE1,
            true => MCANDRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCANDRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCANDRA_A::VALUE2
    }
}
#[doc = "Field `MCANDRA` writer - Select Memory Check for MultiCAN"]
pub type MCANDRA_W<'a, REG> = crate::BitWriter<'a, REG, MCANDRA_A>;
impl<'a, REG> MCANDRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCANDRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCANDRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for PMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPRFDRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<PPRFDRA_A> for bool {
    #[inline(always)]
    fn from(variant: PPRFDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPRFDRA` reader - Select Memory Check for PMU"]
pub type PPRFDRA_R = crate::BitReader<PPRFDRA_A>;
impl PPRFDRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PPRFDRA_A {
        match self.bits {
            false => PPRFDRA_A::VALUE1,
            true => PPRFDRA_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPRFDRA_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPRFDRA_A::VALUE2
    }
}
#[doc = "Field `PPRFDRA` writer - Select Memory Check for PMU"]
pub type PPRFDRA_W<'a, REG> = crate::BitWriter<'a, REG, PPRFDRA_A>;
impl<'a, REG> PPRFDRA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PPRFDRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PPRFDRA_A::VALUE2)
    }
}
#[doc = "Select Memory Check for USB SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELUSB_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELUSB_A> for bool {
    #[inline(always)]
    fn from(variant: SELUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELUSB` reader - Select Memory Check for USB SRAM"]
pub type SELUSB_R = crate::BitReader<SELUSB_A>;
impl SELUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELUSB_A {
        match self.bits {
            false => SELUSB_A::VALUE1,
            true => SELUSB_A::VALUE2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELUSB_A::VALUE1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELUSB_A::VALUE2
    }
}
#[doc = "Field `SELUSB` writer - Select Memory Check for USB SRAM"]
pub type SELUSB_W<'a, REG> = crate::BitWriter<'a, REG, SELUSB_A>;
impl<'a, REG> SELUSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELUSB_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELUSB_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&self) -> SELPS_R {
        SELPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&self) -> SELDS1_R {
        SELDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&self) -> USIC0DRA_R {
        USIC0DRA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&self) -> USIC1DRA_R {
        USIC1DRA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&self) -> MCANDRA_R {
        MCANDRA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&self) -> PPRFDRA_R {
        PPRFDRA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&self) -> SELUSB_R {
        SELUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selps(&mut self) -> SELPS_W<MCHKCON_SPEC> {
        SELPS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn selds1(&mut self) -> SELDS1_W<MCHKCON_SPEC> {
        SELDS1_W::new(self, 1)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    #[must_use]
    pub fn usic0dra(&mut self) -> USIC0DRA_W<MCHKCON_SPEC> {
        USIC0DRA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    #[must_use]
    pub fn usic1dra(&mut self) -> USIC1DRA_W<MCHKCON_SPEC> {
        USIC1DRA_W::new(self, 9)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    #[must_use]
    pub fn mcandra(&mut self) -> MCANDRA_W<MCHKCON_SPEC> {
        MCANDRA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    #[must_use]
    pub fn pprfdra(&mut self) -> PPRFDRA_W<MCHKCON_SPEC> {
        PPRFDRA_W::new(self, 13)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selusb(&mut self) -> SELUSB_W<MCHKCON_SPEC> {
        SELUSB_W::new(self, 16)
    }
}
#[doc = "Memory Checking Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mchkcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mchkcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCHKCON_SPEC;
impl crate::RegisterSpec for MCHKCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mchkcon::R`](R) reader structure"]
impl crate::Readable for MCHKCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mchkcon::W`](W) writer structure"]
impl crate::Writable for MCHKCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCHKCON to value 0"]
impl crate::Resettable for MCHKCON_SPEC {
    const RESET_VALUE: u32 = 0;
}
