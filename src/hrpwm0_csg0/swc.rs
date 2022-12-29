#[doc = "Register `SWC` writer"]
pub struct W(crate::W<SWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
pub type CVLS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
pub type CVLS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CTRGS` writer - Conversion trigger status clear"]
pub type CTRGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CSTRS` writer - Start trigger status clear"]
pub type CSTRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CSTPS` writer - Stop trigger status clear"]
pub type CSTPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CSTD` writer - Shadow transfer status clear"]
pub type CSTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CCRS` writer - Comparator rise status clear"]
pub type CCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CCFS` writer - Comparator fall status clear"]
pub type CCFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
#[doc = "Field `CCSS` writer - Comparator clamped status clear"]
pub type CCSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cvls1(&mut self) -> CVLS1_W<0> {
        CVLS1_W::new(self)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cvls2(&mut self) -> CVLS2_W<1> {
        CVLS2_W::new(self)
    }
    #[doc = "Bit 2 - Conversion trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ctrgs(&mut self) -> CTRGS_W<2> {
        CTRGS_W::new(self)
    }
    #[doc = "Bit 3 - Start trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstrs(&mut self) -> CSTRS_W<3> {
        CSTRS_W::new(self)
    }
    #[doc = "Bit 4 - Stop trigger status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstps(&mut self) -> CSTPS_W<4> {
        CSTPS_W::new(self)
    }
    #[doc = "Bit 5 - Shadow transfer status clear"]
    #[inline(always)]
    #[must_use]
    pub fn cstd(&mut self) -> CSTD_W<5> {
        CSTD_W::new(self)
    }
    #[doc = "Bit 6 - Comparator rise status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccrs(&mut self) -> CCRS_W<6> {
        CCRS_W::new(self)
    }
    #[doc = "Bit 7 - Comparator fall status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CCFS_W<7> {
        CCFS_W::new(self)
    }
    #[doc = "Bit 8 - Comparator clamped status clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccss(&mut self) -> CCSS_W<8> {
        CCSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service request SW clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swc](index.html) module"]
pub struct SWC_SPEC;
impl crate::RegisterSpec for SWC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swc::W](W) writer structure"]
impl crate::Writable for SWC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWC to value 0"]
impl crate::Resettable for SWC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
