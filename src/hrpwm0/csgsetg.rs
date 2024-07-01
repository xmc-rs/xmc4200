#[doc = "Register `CSGSETG` writer"]
pub type W = crate::W<CSGSETG_SPEC>;
#[doc = "Field `SD0R` writer - DAC0 run bit set"]
pub type SD0R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC0R` writer - CMP0 run bit set"]
pub type SC0R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC0P` writer - CMP0 passive level set"]
pub type SC0P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1R` writer - DAC1 run bit set"]
pub type SD1R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC1R` writer - CMP1 run bit set"]
pub type SC1R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC1P` writer - CMP1 passive level set"]
pub type SC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD2R` writer - DAC2 run bit set"]
pub type SD2R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC2R` writer - CMP2 run bit set"]
pub type SC2R_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC2P` writer - CMP2 passive level set"]
pub type SC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd0r(&mut self) -> SD0R_W<CSGSETG_SPEC> {
        SD0R_W::new(self, 0)
    }
    #[doc = "Bit 1 - CMP0 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc0r(&mut self) -> SC0R_W<CSGSETG_SPEC> {
        SC0R_W::new(self, 1)
    }
    #[doc = "Bit 2 - CMP0 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc0p(&mut self) -> SC0P_W<CSGSETG_SPEC> {
        SC0P_W::new(self, 2)
    }
    #[doc = "Bit 4 - DAC1 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd1r(&mut self) -> SD1R_W<CSGSETG_SPEC> {
        SD1R_W::new(self, 4)
    }
    #[doc = "Bit 5 - CMP1 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc1r(&mut self) -> SC1R_W<CSGSETG_SPEC> {
        SC1R_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMP1 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc1p(&mut self) -> SC1P_W<CSGSETG_SPEC> {
        SC1P_W::new(self, 6)
    }
    #[doc = "Bit 8 - DAC2 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd2r(&mut self) -> SD2R_W<CSGSETG_SPEC> {
        SD2R_W::new(self, 8)
    }
    #[doc = "Bit 9 - CMP2 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc2r(&mut self) -> SC2R_W<CSGSETG_SPEC> {
        SC2R_W::new(self, 9)
    }
    #[doc = "Bit 10 - CMP2 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc2p(&mut self) -> SC2P_W<CSGSETG_SPEC> {
        SC2P_W::new(self, 10)
    }
}
#[doc = "Global CSG run bit set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgsetg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGSETG_SPEC;
impl crate::RegisterSpec for CSGSETG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgsetg::W`](W) writer structure"]
impl crate::Writable for CSGSETG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGSETG to value 0"]
impl crate::Resettable for CSGSETG_SPEC {
    const RESET_VALUE: u32 = 0;
}
