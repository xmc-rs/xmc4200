#[doc = "Register `SWC` writer"]
pub type W = crate::W<SwcSpec>;
#[doc = "Field `CVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
pub type Cvls1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
pub type Cvls2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRGS` writer - Conversion trigger status clear"]
pub type CtrgsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTRS` writer - Start trigger status clear"]
pub type CstrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTPS` writer - Stop trigger status clear"]
pub type CstpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTD` writer - Shadow transfer status clear"]
pub type CstdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCRS` writer - Comparator rise status clear"]
pub type CcrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCFS` writer - Comparator fall status clear"]
pub type CcfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCSS` writer - Comparator clamped status clear"]
pub type CcssW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cvls1(&mut self) -> Cvls1W<SwcSpec> {
        Cvls1W::new(self, 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cvls2(&mut self) -> Cvls2W<SwcSpec> {
        Cvls2W::new(self, 1)
    }
    #[doc = "Bit 2 - Conversion trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctrgs(&mut self) -> CtrgsW<SwcSpec> {
        CtrgsW::new(self, 2)
    }
    #[doc = "Bit 3 - Start trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstrs(&mut self) -> CstrsW<SwcSpec> {
        CstrsW::new(self, 3)
    }
    #[doc = "Bit 4 - Stop trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstps(&mut self) -> CstpsW<SwcSpec> {
        CstpsW::new(self, 4)
    }
    #[doc = "Bit 5 - Shadow transfer status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstd(&mut self) -> CstdW<SwcSpec> {
        CstdW::new(self, 5)
    }
    #[doc = "Bit 6 - Comparator rise status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccrs(&mut self) -> CcrsW<SwcSpec> {
        CcrsW::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator fall status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CcfsW<SwcSpec> {
        CcfsW::new(self, 7)
    }
    #[doc = "Bit 8 - Comparator clamped status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccss(&mut self) -> CcssW<SwcSpec> {
        CcssW::new(self, 8)
    }
}
#[doc = "Service request SW clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwcSpec;
impl crate::RegisterSpec for SwcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swc::W`](W) writer structure"]
impl crate::Writable for SwcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWC to value 0"]
impl crate::Resettable for SwcSpec {
    const RESET_VALUE: u32 = 0;
}
