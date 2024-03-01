#[doc = "Register `CSGCLRG` writer"]
pub type W = crate::W<CsgclrgSpec>;
#[doc = "Field `CD0R` writer - DAC0 run bit clear"]
pub type Cd0rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0R` writer - CMP0 run bit clear"]
pub type Cc0rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0P` writer - CMP0 passive level clear"]
pub type Cc0pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD1R` writer - DAC1 run bit clear"]
pub type Cd1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1R` writer - CMP1 run bit clear"]
pub type Cc1rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` writer - CMP1 passive level clear"]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CD2R` writer - DAC2 run bit clear"]
pub type Cd2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2R` writer - CMP2 run bit clear"]
pub type Cc2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` writer - CMP2 passive level clear"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC0 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd0r(&mut self) -> Cd0rW<CsgclrgSpec> {
        Cd0rW::new(self, 0)
    }
    #[doc = "Bit 1 - CMP0 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc0r(&mut self) -> Cc0rW<CsgclrgSpec> {
        Cc0rW::new(self, 1)
    }
    #[doc = "Bit 2 - CMP0 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc0p(&mut self) -> Cc0pW<CsgclrgSpec> {
        Cc0pW::new(self, 2)
    }
    #[doc = "Bit 4 - DAC1 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd1r(&mut self) -> Cd1rW<CsgclrgSpec> {
        Cd1rW::new(self, 4)
    }
    #[doc = "Bit 5 - CMP1 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc1r(&mut self) -> Cc1rW<CsgclrgSpec> {
        Cc1rW::new(self, 5)
    }
    #[doc = "Bit 6 - CMP1 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> Cc1pW<CsgclrgSpec> {
        Cc1pW::new(self, 6)
    }
    #[doc = "Bit 8 - DAC2 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cd2r(&mut self) -> Cd2rW<CsgclrgSpec> {
        Cd2rW::new(self, 8)
    }
    #[doc = "Bit 9 - CMP2 run bit clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc2r(&mut self) -> Cc2rW<CsgclrgSpec> {
        Cc2rW::new(self, 9)
    }
    #[doc = "Bit 10 - CMP2 passive level clear"]
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> Cc2pW<CsgclrgSpec> {
        Cc2pW::new(self, 10)
    }
}
#[doc = "Global CSG run bit clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgclrg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsgclrgSpec;
impl crate::RegisterSpec for CsgclrgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`csgclrg::W`](W) writer structure"]
impl crate::Writable for CsgclrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCLRG to value 0"]
impl crate::Resettable for CsgclrgSpec {
    const RESET_VALUE: u32 = 0;
}
