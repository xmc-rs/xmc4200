#[doc = "Register `CSGCLRG` writer"]
pub type W = crate::W<CSGCLRG_SPEC>;
#[doc = "Field `CD0R` writer - DAC0 run bit clear"]
pub type CD0R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0R` writer - CMP0 run bit clear"]
pub type CC0R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0P` writer - CMP0 passive level clear"]
pub type CC0P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD1R` writer - DAC1 run bit clear"]
pub type CD1R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1R` writer - CMP1 run bit clear"]
pub type CC1R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` writer - CMP1 passive level clear"]
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD2R` writer - DAC2 run bit clear"]
pub type CD2R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2R` writer - CMP2 run bit clear"]
pub type CC2R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` writer - CMP2 passive level clear"]
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd0r(&mut self) -> CD0R_W<CSGCLRG_SPEC> {
        CD0R_W::new(self, 0)
    }
    #[doc = "Bit 1 - CMP0 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc0r(&mut self) -> CC0R_W<CSGCLRG_SPEC> {
        CC0R_W::new(self, 1)
    }
    #[doc = "Bit 2 - CMP0 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc0p(&mut self) -> CC0P_W<CSGCLRG_SPEC> {
        CC0P_W::new(self, 2)
    }
    #[doc = "Bit 4 - DAC1 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd1r(&mut self) -> CD1R_W<CSGCLRG_SPEC> {
        CD1R_W::new(self, 4)
    }
    #[doc = "Bit 5 - CMP1 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc1r(&mut self) -> CC1R_W<CSGCLRG_SPEC> {
        CC1R_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMP1 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<CSGCLRG_SPEC> {
        CC1P_W::new(self, 6)
    }
    #[doc = "Bit 8 - DAC2 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd2r(&mut self) -> CD2R_W<CSGCLRG_SPEC> {
        CD2R_W::new(self, 8)
    }
    #[doc = "Bit 9 - CMP2 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc2r(&mut self) -> CC2R_W<CSGCLRG_SPEC> {
        CC2R_W::new(self, 9)
    }
    #[doc = "Bit 10 - CMP2 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<CSGCLRG_SPEC> {
        CC2P_W::new(self, 10)
    }
}
#[doc = "Global CSG run bit clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgclrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCLRG_SPEC;
impl crate::RegisterSpec for CSGCLRG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgclrg::W`](W) writer structure"]
impl crate::Writable for CSGCLRG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCLRG to value 0"]
impl crate::Resettable for CSGCLRG_SPEC {
    const RESET_VALUE: u32 = 0;
}
