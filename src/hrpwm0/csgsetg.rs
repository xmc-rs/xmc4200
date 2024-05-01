#[doc = "Register `CSGSETG` writer"]
pub type W = crate::W<CsgsetgSpec>;
#[doc = "Field `SD0R` writer - DAC0 run bit set"]
pub type Sd0rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC0R` writer - CMP0 run bit set"]
pub type Sc0rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC0P` writer - CMP0 passive level set"]
pub type Sc0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1R` writer - DAC1 run bit set"]
pub type Sd1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC1R` writer - CMP1 run bit set"]
pub type Sc1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC1P` writer - CMP1 passive level set"]
pub type Sc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD2R` writer - DAC2 run bit set"]
pub type Sd2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC2R` writer - CMP2 run bit set"]
pub type Sc2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC2P` writer - CMP2 passive level set"]
pub type Sc2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd0r(&mut self) -> Sd0rW<CsgsetgSpec> {
        Sd0rW::new(self, 0)
    }
    #[doc = "Bit 1 - CMP0 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc0r(&mut self) -> Sc0rW<CsgsetgSpec> {
        Sc0rW::new(self, 1)
    }
    #[doc = "Bit 2 - CMP0 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc0p(&mut self) -> Sc0pW<CsgsetgSpec> {
        Sc0pW::new(self, 2)
    }
    #[doc = "Bit 4 - DAC1 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd1r(&mut self) -> Sd1rW<CsgsetgSpec> {
        Sd1rW::new(self, 4)
    }
    #[doc = "Bit 5 - CMP1 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc1r(&mut self) -> Sc1rW<CsgsetgSpec> {
        Sc1rW::new(self, 5)
    }
    #[doc = "Bit 6 - CMP1 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc1p(&mut self) -> Sc1pW<CsgsetgSpec> {
        Sc1pW::new(self, 6)
    }
    #[doc = "Bit 8 - DAC2 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sd2r(&mut self) -> Sd2rW<CsgsetgSpec> {
        Sd2rW::new(self, 8)
    }
    #[doc = "Bit 9 - CMP2 run bit set"]
    #[inline(always)]
    #[must_use]
    pub fn sc2r(&mut self) -> Sc2rW<CsgsetgSpec> {
        Sc2rW::new(self, 9)
    }
    #[doc = "Bit 10 - CMP2 passive level set"]
    #[inline(always)]
    #[must_use]
    pub fn sc2p(&mut self) -> Sc2pW<CsgsetgSpec> {
        Sc2pW::new(self, 10)
    }
}
#[doc = "Global CSG run bit set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgsetg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgsetgSpec;
impl crate::RegisterSpec for CsgsetgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgsetg::W`](W) writer structure"]
impl crate::Writable for CsgsetgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGSETG to value 0"]
impl crate::Resettable for CsgsetgSpec {
    const RESET_VALUE: u32 = 0;
}
