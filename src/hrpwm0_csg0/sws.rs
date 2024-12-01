#[doc = "Register `SWS` writer"]
pub type W = crate::W<SWS_SPEC>;
#[doc = "Field `SVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
pub type SVLS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
pub type SVLS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRGS` writer - Conversion trigger status set"]
pub type STRGS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTRS` writer - Start trigger status set"]
pub type SSTRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTPS` writer - Stop trigger status set"]
pub type SSTPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTD` writer - Shadow transfer status set"]
pub type SSTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRS` writer - Comparator rise status set"]
pub type SCRS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFS` writer - Comparator fall status set"]
pub type SCFS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCSS` writer - Comparator clamped state status set"]
pub type SCSS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
    #[inline(always)]
    pub fn svls1(&mut self) -> SVLS1_W<SWS_SPEC> {
        SVLS1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
    #[inline(always)]
    pub fn svls2(&mut self) -> SVLS2_W<SWS_SPEC> {
        SVLS2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Conversion trigger status set"]
    #[inline(always)]
    pub fn strgs(&mut self) -> STRGS_W<SWS_SPEC> {
        STRGS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Start trigger status set"]
    #[inline(always)]
    pub fn sstrs(&mut self) -> SSTRS_W<SWS_SPEC> {
        SSTRS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stop trigger status set"]
    #[inline(always)]
    pub fn sstps(&mut self) -> SSTPS_W<SWS_SPEC> {
        SSTPS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Shadow transfer status set"]
    #[inline(always)]
    pub fn sstd(&mut self) -> SSTD_W<SWS_SPEC> {
        SSTD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparator rise status set"]
    #[inline(always)]
    pub fn scrs(&mut self) -> SCRS_W<SWS_SPEC> {
        SCRS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator fall status set"]
    #[inline(always)]
    pub fn scfs(&mut self) -> SCFS_W<SWS_SPEC> {
        SCFS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Comparator clamped state status set"]
    #[inline(always)]
    pub fn scss(&mut self) -> SCSS_W<SWS_SPEC> {
        SCSS_W::new(self, 8)
    }
}
#[doc = "Service request SW set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sws::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWS_SPEC;
impl crate::RegisterSpec for SWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sws::W`](W) writer structure"]
impl crate::Writable for SWS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SWS_SPEC {
    const RESET_VALUE: u32 = 0;
}
