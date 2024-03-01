#[doc = "Register `SWS` writer"]
pub type W = crate::W<SwsSpec>;
#[doc = "Field `SVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
pub type Svls1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
pub type Svls2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRGS` writer - Conversion trigger status set"]
pub type StrgsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTRS` writer - Start trigger status set"]
pub type SstrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTPS` writer - Stop trigger status set"]
pub type SstpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTD` writer - Shadow transfer status set"]
pub type SstdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCRS` writer - Comparator rise status set"]
pub type ScrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCFS` writer - Comparator fall status set"]
pub type ScfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCSS` writer - Comparator clamped state status set"]
pub type ScssW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
    #[inline(always)]
    #[must_use]
    pub fn svls1(&mut self) -> Svls1W<SwsSpec> {
        Svls1W::new(self, 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
    #[inline(always)]
    #[must_use]
    pub fn svls2(&mut self) -> Svls2W<SwsSpec> {
        Svls2W::new(self, 1)
    }
    #[doc = "Bit 2 - Conversion trigger status set"]
    #[inline(always)]
    #[must_use]
    pub fn strgs(&mut self) -> StrgsW<SwsSpec> {
        StrgsW::new(self, 2)
    }
    #[doc = "Bit 3 - Start trigger status set"]
    #[inline(always)]
    #[must_use]
    pub fn sstrs(&mut self) -> SstrsW<SwsSpec> {
        SstrsW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop trigger status set"]
    #[inline(always)]
    #[must_use]
    pub fn sstps(&mut self) -> SstpsW<SwsSpec> {
        SstpsW::new(self, 4)
    }
    #[doc = "Bit 5 - Shadow transfer status set"]
    #[inline(always)]
    #[must_use]
    pub fn sstd(&mut self) -> SstdW<SwsSpec> {
        SstdW::new(self, 5)
    }
    #[doc = "Bit 6 - Comparator rise status set"]
    #[inline(always)]
    #[must_use]
    pub fn scrs(&mut self) -> ScrsW<SwsSpec> {
        ScrsW::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator fall status set"]
    #[inline(always)]
    #[must_use]
    pub fn scfs(&mut self) -> ScfsW<SwsSpec> {
        ScfsW::new(self, 7)
    }
    #[doc = "Bit 8 - Comparator clamped state status set"]
    #[inline(always)]
    #[must_use]
    pub fn scss(&mut self) -> ScssW<SwsSpec> {
        ScssW::new(self, 8)
    }
}
#[doc = "Service request SW set\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sws::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwsSpec;
impl crate::RegisterSpec for SwsSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sws::W`](W) writer structure"]
impl crate::Writable for SwsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SwsSpec {
    const RESET_VALUE: u32 = 0;
}
