#[doc = "Register `CSGFCG` writer"]
pub type W = crate::W<CsgfcgSpec>;
#[doc = "Field `S0STR` writer - Slope 0 start"]
pub type S0strW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0STP` writer - Slope 0 stop"]
pub type S0stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS0STR` writer - Prescaler 0 start"]
pub type Ps0strW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS0STP` writer - Prescaler 0 stop"]
pub type Ps0stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS0CLR` writer - Prescaler 0 clear"]
pub type Ps0clrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1STR` writer - Slope 1 start"]
pub type S1strW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1STP` writer - Slope 1 stop"]
pub type S1stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1STR` writer - Prescaler 1 start"]
pub type Ps1strW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1STP` writer - Prescaler 1 stop"]
pub type Ps1stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1CLR` writer - Prescaler 1 clear"]
pub type Ps1clrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2STR` writer - Slope 2 start"]
pub type S2strW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2STP` writer - Slope 2 stop"]
pub type S2stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2STR` writer - Prescaler 2 start"]
pub type Ps2strW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2STP` writer - Prescaler 2 stop"]
pub type Ps2stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2CLR` writer - Prescaler 2 clear"]
pub type Ps2clrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Slope 0 start"]
    #[inline(always)]
    #[must_use]
    pub fn s0str(&mut self) -> S0strW<CsgfcgSpec> {
        S0strW::new(self, 0)
    }
    #[doc = "Bit 1 - Slope 0 stop"]
    #[inline(always)]
    #[must_use]
    pub fn s0stp(&mut self) -> S0stpW<CsgfcgSpec> {
        S0stpW::new(self, 1)
    }
    #[doc = "Bit 2 - Prescaler 0 start"]
    #[inline(always)]
    #[must_use]
    pub fn ps0str(&mut self) -> Ps0strW<CsgfcgSpec> {
        Ps0strW::new(self, 2)
    }
    #[doc = "Bit 3 - Prescaler 0 stop"]
    #[inline(always)]
    #[must_use]
    pub fn ps0stp(&mut self) -> Ps0stpW<CsgfcgSpec> {
        Ps0stpW::new(self, 3)
    }
    #[doc = "Bit 4 - Prescaler 0 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ps0clr(&mut self) -> Ps0clrW<CsgfcgSpec> {
        Ps0clrW::new(self, 4)
    }
    #[doc = "Bit 8 - Slope 1 start"]
    #[inline(always)]
    #[must_use]
    pub fn s1str(&mut self) -> S1strW<CsgfcgSpec> {
        S1strW::new(self, 8)
    }
    #[doc = "Bit 9 - Slope 1 stop"]
    #[inline(always)]
    #[must_use]
    pub fn s1stp(&mut self) -> S1stpW<CsgfcgSpec> {
        S1stpW::new(self, 9)
    }
    #[doc = "Bit 10 - Prescaler 1 start"]
    #[inline(always)]
    #[must_use]
    pub fn ps1str(&mut self) -> Ps1strW<CsgfcgSpec> {
        Ps1strW::new(self, 10)
    }
    #[doc = "Bit 11 - Prescaler 1 stop"]
    #[inline(always)]
    #[must_use]
    pub fn ps1stp(&mut self) -> Ps1stpW<CsgfcgSpec> {
        Ps1stpW::new(self, 11)
    }
    #[doc = "Bit 12 - Prescaler 1 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ps1clr(&mut self) -> Ps1clrW<CsgfcgSpec> {
        Ps1clrW::new(self, 12)
    }
    #[doc = "Bit 16 - Slope 2 start"]
    #[inline(always)]
    #[must_use]
    pub fn s2str(&mut self) -> S2strW<CsgfcgSpec> {
        S2strW::new(self, 16)
    }
    #[doc = "Bit 17 - Slope 2 stop"]
    #[inline(always)]
    #[must_use]
    pub fn s2stp(&mut self) -> S2stpW<CsgfcgSpec> {
        S2stpW::new(self, 17)
    }
    #[doc = "Bit 18 - Prescaler 2 start"]
    #[inline(always)]
    #[must_use]
    pub fn ps2str(&mut self) -> Ps2strW<CsgfcgSpec> {
        Ps2strW::new(self, 18)
    }
    #[doc = "Bit 19 - Prescaler 2 stop"]
    #[inline(always)]
    #[must_use]
    pub fn ps2stp(&mut self) -> Ps2stpW<CsgfcgSpec> {
        Ps2stpW::new(self, 19)
    }
    #[doc = "Bit 20 - Prescaler 2 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ps2clr(&mut self) -> Ps2clrW<CsgfcgSpec> {
        Ps2clrW::new(self, 20)
    }
}
#[doc = "Global CSG slope/prescaler control\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgfcg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgfcgSpec;
impl crate::RegisterSpec for CsgfcgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgfcg::W`](W) writer structure"]
impl crate::Writable for CsgfcgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGFCG to value 0"]
impl crate::Resettable for CsgfcgSpec {
    const RESET_VALUE: u32 = 0;
}
