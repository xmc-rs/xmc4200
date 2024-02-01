#[doc = "Register `SWC` writer"]
pub type W = crate::W<SWC_SPEC>;
#[doc = "Field `CVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
pub type CVLS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
pub type CVLS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRGS` writer - Conversion trigger status clear"]
pub type CTRGS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTRS` writer - Start trigger status clear"]
pub type CSTRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTPS` writer - Stop trigger status clear"]
pub type CSTPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTD` writer - Shadow transfer status clear"]
pub type CSTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRS` writer - Comparator rise status clear"]
pub type CCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFS` writer - Comparator fall status clear"]
pub type CCFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCSS` writer - Comparator clamped status clear"]
pub type CCSS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cvls1(&mut self) -> CVLS1_W<SWC_SPEC> {
        CVLS1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cvls2(&mut self) -> CVLS2_W<SWC_SPEC> {
        CVLS2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Conversion trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctrgs(&mut self) -> CTRGS_W<SWC_SPEC> {
        CTRGS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstrs(&mut self) -> CSTRS_W<SWC_SPEC> {
        CSTRS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stop trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstps(&mut self) -> CSTPS_W<SWC_SPEC> {
        CSTPS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Shadow transfer status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstd(&mut self) -> CSTD_W<SWC_SPEC> {
        CSTD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparator rise status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccrs(&mut self) -> CCRS_W<SWC_SPEC> {
        CCRS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator fall status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CCFS_W<SWC_SPEC> {
        CCFS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Comparator clamped status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccss(&mut self) -> CCSS_W<SWC_SPEC> {
        CCSS_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Service request SW clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWC_SPEC;
impl crate::RegisterSpec for SWC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swc::W`](W) writer structure"]
impl crate::Writable for SWC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWC to value 0"]
impl crate::Resettable for SWC_SPEC {
    const RESET_VALUE: u32 = 0;
}
