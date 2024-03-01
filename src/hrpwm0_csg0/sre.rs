#[doc = "Register `SRE` reader"]
pub type R = crate::R<SreSpec>;
#[doc = "Register `SRE` writer"]
pub type W = crate::W<SreSpec>;
#[doc = "Field `VLS1E` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub type Vls1eR = crate::BitReader;
#[doc = "Field `VLS1E` writer - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub type Vls1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLS2E` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub type Vls2eR = crate::BitReader;
#[doc = "Field `VLS2E` writer - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub type Vls2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSE` reader - Conversion trigger interrupt enable"]
pub type TrgseR = crate::BitReader;
#[doc = "Field `TRGSE` writer - Conversion trigger interrupt enable"]
pub type TrgseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRSE` reader - Start trigger interrupt enable"]
pub type StrseR = crate::BitReader;
#[doc = "Field `STRSE` writer - Start trigger interrupt enable"]
pub type StrseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPSE` reader - Stop trigger interrupt enable"]
pub type StpseR = crate::BitReader;
#[doc = "Field `STPSE` writer - Stop trigger interrupt enable"]
pub type StpseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STDE` reader - Shadow transfer done interrupt enable"]
pub type StdeR = crate::BitReader;
#[doc = "Field `STDE` writer - Shadow transfer done interrupt enable"]
pub type StdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSE` reader - Comparator rise interrupt enable"]
pub type CrseR = crate::BitReader;
#[doc = "Field `CRSE` writer - Comparator rise interrupt enable"]
pub type CrseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFSE` reader - Comparator fall interrupt enable"]
pub type CfseR = crate::BitReader;
#[doc = "Field `CFSE` writer - Comparator fall interrupt enable"]
pub type CfseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEE` reader - Clamped state interrupt enable"]
pub type CseeR = crate::BitReader;
#[doc = "Field `CSEE` writer - Clamped state interrupt enable"]
pub type CseeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
    #[inline(always)]
    pub fn vls1e(&self) -> Vls1eR {
        Vls1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
    #[inline(always)]
    pub fn vls2e(&self) -> Vls2eR {
        Vls2eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Conversion trigger interrupt enable"]
    #[inline(always)]
    pub fn trgse(&self) -> TrgseR {
        TrgseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start trigger interrupt enable"]
    #[inline(always)]
    pub fn strse(&self) -> StrseR {
        StrseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop trigger interrupt enable"]
    #[inline(always)]
    pub fn stpse(&self) -> StpseR {
        StpseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shadow transfer done interrupt enable"]
    #[inline(always)]
    pub fn stde(&self) -> StdeR {
        StdeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator rise interrupt enable"]
    #[inline(always)]
    pub fn crse(&self) -> CrseR {
        CrseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator fall interrupt enable"]
    #[inline(always)]
    pub fn cfse(&self) -> CfseR {
        CfseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clamped state interrupt enable"]
    #[inline(always)]
    pub fn csee(&self) -> CseeR {
        CseeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vls1e(&mut self) -> Vls1eW<SreSpec> {
        Vls1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vls2e(&mut self) -> Vls2eW<SreSpec> {
        Vls2eW::new(self, 1)
    }
    #[doc = "Bit 2 - Conversion trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgse(&mut self) -> TrgseW<SreSpec> {
        TrgseW::new(self, 2)
    }
    #[doc = "Bit 3 - Start trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn strse(&mut self) -> StrseW<SreSpec> {
        StrseW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpse(&mut self) -> StpseW<SreSpec> {
        StpseW::new(self, 4)
    }
    #[doc = "Bit 5 - Shadow transfer done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stde(&mut self) -> StdeW<SreSpec> {
        StdeW::new(self, 5)
    }
    #[doc = "Bit 6 - Comparator rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crse(&mut self) -> CrseW<SreSpec> {
        CrseW::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfse(&mut self) -> CfseW<SreSpec> {
        CfseW::new(self, 7)
    }
    #[doc = "Bit 8 - Clamped state interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn csee(&mut self) -> CseeW<SreSpec> {
        CseeW::new(self, 8)
    }
}
#[doc = "Service request enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SreSpec;
impl crate::RegisterSpec for SreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sre::R`](R) reader structure"]
impl crate::Readable for SreSpec {}
#[doc = "`write(|w| ..)` method takes [`sre::W`](W) writer structure"]
impl crate::Writable for SreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRE to value 0"]
impl crate::Resettable for SreSpec {
    const RESET_VALUE: u32 = 0;
}
