#[doc = "Register `SWS` writer"]
pub struct W(crate::W<SWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWS_SPEC>;
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
impl From<crate::W<SWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
pub type SVLS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
pub type SVLS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `STRGS` writer - Conversion trigger status set"]
pub type STRGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SSTRS` writer - Start trigger status set"]
pub type SSTRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SSTPS` writer - Stop trigger status set"]
pub type SSTPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SSTD` writer - Shadow transfer status set"]
pub type SSTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SCRS` writer - Comparator rise status set"]
pub type SCRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SCFS` writer - Comparator fall status set"]
pub type SCFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
#[doc = "Field `SCSS` writer - Comparator clamped state status set"]
pub type SCSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
    #[inline(always)]
    #[must_use]
    pub fn svls1(&mut self) -> SVLS1_W<0> {
        SVLS1_W::new(self)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
    #[inline(always)]
    #[must_use]
    pub fn svls2(&mut self) -> SVLS2_W<1> {
        SVLS2_W::new(self)
    }
    #[doc = "Bit 2 - Conversion trigger status set"]
    #[inline(always)]
    #[must_use]
    pub fn strgs(&mut self) -> STRGS_W<2> {
        STRGS_W::new(self)
    }
    #[doc = "Bit 3 - Start trigger status set"]
    #[inline(always)]
    #[must_use]
    pub fn sstrs(&mut self) -> SSTRS_W<3> {
        SSTRS_W::new(self)
    }
    #[doc = "Bit 4 - Stop trigger status set"]
    #[inline(always)]
    #[must_use]
    pub fn sstps(&mut self) -> SSTPS_W<4> {
        SSTPS_W::new(self)
    }
    #[doc = "Bit 5 - Shadow transfer status set"]
    #[inline(always)]
    #[must_use]
    pub fn sstd(&mut self) -> SSTD_W<5> {
        SSTD_W::new(self)
    }
    #[doc = "Bit 6 - Comparator rise status set"]
    #[inline(always)]
    #[must_use]
    pub fn scrs(&mut self) -> SCRS_W<6> {
        SCRS_W::new(self)
    }
    #[doc = "Bit 7 - Comparator fall status set"]
    #[inline(always)]
    #[must_use]
    pub fn scfs(&mut self) -> SCFS_W<7> {
        SCFS_W::new(self)
    }
    #[doc = "Bit 8 - Comparator clamped state status set"]
    #[inline(always)]
    #[must_use]
    pub fn scss(&mut self) -> SCSS_W<8> {
        SCSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service request SW set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sws](index.html) module"]
pub struct SWS_SPEC;
impl crate::RegisterSpec for SWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sws::W](W) writer structure"]
impl crate::Writable for SWS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SWS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
