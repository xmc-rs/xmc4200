#[doc = "Register `SRE` reader"]
pub type R = crate::R<SRE_SPEC>;
#[doc = "Register `SRE` writer"]
pub type W = crate::W<SRE_SPEC>;
#[doc = "Field `VLS1E` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub type VLS1E_R = crate::BitReader;
#[doc = "Field `VLS1E` writer - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub type VLS1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLS2E` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub type VLS2E_R = crate::BitReader;
#[doc = "Field `VLS2E` writer - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub type VLS2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGSE` reader - Conversion trigger interrupt enable"]
pub type TRGSE_R = crate::BitReader;
#[doc = "Field `TRGSE` writer - Conversion trigger interrupt enable"]
pub type TRGSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRSE` reader - Start trigger interrupt enable"]
pub type STRSE_R = crate::BitReader;
#[doc = "Field `STRSE` writer - Start trigger interrupt enable"]
pub type STRSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPSE` reader - Stop trigger interrupt enable"]
pub type STPSE_R = crate::BitReader;
#[doc = "Field `STPSE` writer - Stop trigger interrupt enable"]
pub type STPSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STDE` reader - Shadow transfer done interrupt enable"]
pub type STDE_R = crate::BitReader;
#[doc = "Field `STDE` writer - Shadow transfer done interrupt enable"]
pub type STDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSE` reader - Comparator rise interrupt enable"]
pub type CRSE_R = crate::BitReader;
#[doc = "Field `CRSE` writer - Comparator rise interrupt enable"]
pub type CRSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFSE` reader - Comparator fall interrupt enable"]
pub type CFSE_R = crate::BitReader;
#[doc = "Field `CFSE` writer - Comparator fall interrupt enable"]
pub type CFSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEE` reader - Clamped state interrupt enable"]
pub type CSEE_R = crate::BitReader;
#[doc = "Field `CSEE` writer - Clamped state interrupt enable"]
pub type CSEE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
    #[inline(always)]
    pub fn vls1e(&self) -> VLS1E_R {
        VLS1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
    #[inline(always)]
    pub fn vls2e(&self) -> VLS2E_R {
        VLS2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Conversion trigger interrupt enable"]
    #[inline(always)]
    pub fn trgse(&self) -> TRGSE_R {
        TRGSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start trigger interrupt enable"]
    #[inline(always)]
    pub fn strse(&self) -> STRSE_R {
        STRSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop trigger interrupt enable"]
    #[inline(always)]
    pub fn stpse(&self) -> STPSE_R {
        STPSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shadow transfer done interrupt enable"]
    #[inline(always)]
    pub fn stde(&self) -> STDE_R {
        STDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator rise interrupt enable"]
    #[inline(always)]
    pub fn crse(&self) -> CRSE_R {
        CRSE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator fall interrupt enable"]
    #[inline(always)]
    pub fn cfse(&self) -> CFSE_R {
        CFSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clamped state interrupt enable"]
    #[inline(always)]
    pub fn csee(&self) -> CSEE_R {
        CSEE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
    #[inline(always)]
    pub fn vls1e(&mut self) -> VLS1E_W<SRE_SPEC> {
        VLS1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
    #[inline(always)]
    pub fn vls2e(&mut self) -> VLS2E_W<SRE_SPEC> {
        VLS2E_W::new(self, 1)
    }
    #[doc = "Bit 2 - Conversion trigger interrupt enable"]
    #[inline(always)]
    pub fn trgse(&mut self) -> TRGSE_W<SRE_SPEC> {
        TRGSE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start trigger interrupt enable"]
    #[inline(always)]
    pub fn strse(&mut self) -> STRSE_W<SRE_SPEC> {
        STRSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stop trigger interrupt enable"]
    #[inline(always)]
    pub fn stpse(&mut self) -> STPSE_W<SRE_SPEC> {
        STPSE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Shadow transfer done interrupt enable"]
    #[inline(always)]
    pub fn stde(&mut self) -> STDE_W<SRE_SPEC> {
        STDE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparator rise interrupt enable"]
    #[inline(always)]
    pub fn crse(&mut self) -> CRSE_W<SRE_SPEC> {
        CRSE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator fall interrupt enable"]
    #[inline(always)]
    pub fn cfse(&mut self) -> CFSE_W<SRE_SPEC> {
        CFSE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clamped state interrupt enable"]
    #[inline(always)]
    pub fn csee(&mut self) -> CSEE_W<SRE_SPEC> {
        CSEE_W::new(self, 8)
    }
}
#[doc = "Service request enable\n\nYou can [`read`](crate::Reg::read) this register and get [`sre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRE_SPEC;
impl crate::RegisterSpec for SRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sre::R`](R) reader structure"]
impl crate::Readable for SRE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sre::W`](W) writer structure"]
impl crate::Writable for SRE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRE to value 0"]
impl crate::Resettable for SRE_SPEC {
    const RESET_VALUE: u32 = 0;
}
