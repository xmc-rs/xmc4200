#[doc = "Register `CSGFCG` writer"]
pub type W = crate::W<CSGFCG_SPEC>;
#[doc = "Field `S0STR` writer - Slope 0 start"]
pub type S0STR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S0STP` writer - Slope 0 stop"]
pub type S0STP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS0STR` writer - Prescaler 0 start"]
pub type PS0STR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS0STP` writer - Prescaler 0 stop"]
pub type PS0STP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS0CLR` writer - Prescaler 0 clear"]
pub type PS0CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1STR` writer - Slope 1 start"]
pub type S1STR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S1STP` writer - Slope 1 stop"]
pub type S1STP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1STR` writer - Prescaler 1 start"]
pub type PS1STR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1STP` writer - Prescaler 1 stop"]
pub type PS1STP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS1CLR` writer - Prescaler 1 clear"]
pub type PS1CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2STR` writer - Slope 2 start"]
pub type S2STR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2STP` writer - Slope 2 stop"]
pub type S2STP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2STR` writer - Prescaler 2 start"]
pub type PS2STR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2STP` writer - Prescaler 2 stop"]
pub type PS2STP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2CLR` writer - Prescaler 2 clear"]
pub type PS2CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Slope 0 start"]
    #[inline(always)]
    pub fn s0str(&mut self) -> S0STR_W<CSGFCG_SPEC> {
        S0STR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Slope 0 stop"]
    #[inline(always)]
    pub fn s0stp(&mut self) -> S0STP_W<CSGFCG_SPEC> {
        S0STP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Prescaler 0 start"]
    #[inline(always)]
    pub fn ps0str(&mut self) -> PS0STR_W<CSGFCG_SPEC> {
        PS0STR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Prescaler 0 stop"]
    #[inline(always)]
    pub fn ps0stp(&mut self) -> PS0STP_W<CSGFCG_SPEC> {
        PS0STP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Prescaler 0 clear"]
    #[inline(always)]
    pub fn ps0clr(&mut self) -> PS0CLR_W<CSGFCG_SPEC> {
        PS0CLR_W::new(self, 4)
    }
    #[doc = "Bit 8 - Slope 1 start"]
    #[inline(always)]
    pub fn s1str(&mut self) -> S1STR_W<CSGFCG_SPEC> {
        S1STR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Slope 1 stop"]
    #[inline(always)]
    pub fn s1stp(&mut self) -> S1STP_W<CSGFCG_SPEC> {
        S1STP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Prescaler 1 start"]
    #[inline(always)]
    pub fn ps1str(&mut self) -> PS1STR_W<CSGFCG_SPEC> {
        PS1STR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Prescaler 1 stop"]
    #[inline(always)]
    pub fn ps1stp(&mut self) -> PS1STP_W<CSGFCG_SPEC> {
        PS1STP_W::new(self, 11)
    }
    #[doc = "Bit 12 - Prescaler 1 clear"]
    #[inline(always)]
    pub fn ps1clr(&mut self) -> PS1CLR_W<CSGFCG_SPEC> {
        PS1CLR_W::new(self, 12)
    }
    #[doc = "Bit 16 - Slope 2 start"]
    #[inline(always)]
    pub fn s2str(&mut self) -> S2STR_W<CSGFCG_SPEC> {
        S2STR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Slope 2 stop"]
    #[inline(always)]
    pub fn s2stp(&mut self) -> S2STP_W<CSGFCG_SPEC> {
        S2STP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Prescaler 2 start"]
    #[inline(always)]
    pub fn ps2str(&mut self) -> PS2STR_W<CSGFCG_SPEC> {
        PS2STR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Prescaler 2 stop"]
    #[inline(always)]
    pub fn ps2stp(&mut self) -> PS2STP_W<CSGFCG_SPEC> {
        PS2STP_W::new(self, 19)
    }
    #[doc = "Bit 20 - Prescaler 2 clear"]
    #[inline(always)]
    pub fn ps2clr(&mut self) -> PS2CLR_W<CSGFCG_SPEC> {
        PS2CLR_W::new(self, 20)
    }
}
#[doc = "Global CSG slope/prescaler control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgfcg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGFCG_SPEC;
impl crate::RegisterSpec for CSGFCG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgfcg::W`](W) writer structure"]
impl crate::Writable for CSGFCG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGFCG to value 0"]
impl crate::Resettable for CSGFCG_SPEC {
    const RESET_VALUE: u32 = 0;
}
