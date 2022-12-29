#[doc = "Register `SRE` reader"]
pub struct R(crate::R<SRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRE` writer"]
pub struct W(crate::W<SRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRE_SPEC>;
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
impl From<crate::W<SRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLS1E` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub type VLS1E_R = crate::BitReader<bool>;
#[doc = "Field `VLS1E` writer - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub type VLS1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `VLS2E` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub type VLS2E_R = crate::BitReader<bool>;
#[doc = "Field `VLS2E` writer - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub type VLS2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `TRGSE` reader - Conversion trigger interrupt enable"]
pub type TRGSE_R = crate::BitReader<bool>;
#[doc = "Field `TRGSE` writer - Conversion trigger interrupt enable"]
pub type TRGSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `STRSE` reader - Start trigger interrupt enable"]
pub type STRSE_R = crate::BitReader<bool>;
#[doc = "Field `STRSE` writer - Start trigger interrupt enable"]
pub type STRSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `STPSE` reader - Stop trigger interrupt enable"]
pub type STPSE_R = crate::BitReader<bool>;
#[doc = "Field `STPSE` writer - Stop trigger interrupt enable"]
pub type STPSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `STDE` reader - Shadow transfer done interrupt enable"]
pub type STDE_R = crate::BitReader<bool>;
#[doc = "Field `STDE` writer - Shadow transfer done interrupt enable"]
pub type STDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `CRSE` reader - Comparator rise interrupt enable"]
pub type CRSE_R = crate::BitReader<bool>;
#[doc = "Field `CRSE` writer - Comparator rise interrupt enable"]
pub type CRSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `CFSE` reader - Comparator fall interrupt enable"]
pub type CFSE_R = crate::BitReader<bool>;
#[doc = "Field `CFSE` writer - Comparator fall interrupt enable"]
pub type CFSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
#[doc = "Field `CSEE` reader - Clamped state interrupt enable"]
pub type CSEE_R = crate::BitReader<bool>;
#[doc = "Field `CSEE` writer - Clamped state interrupt enable"]
pub type CSEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRE_SPEC, bool, O>;
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
    #[must_use]
    pub fn vls1e(&mut self) -> VLS1E_W<0> {
        VLS1E_W::new(self)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vls2e(&mut self) -> VLS2E_W<1> {
        VLS2E_W::new(self)
    }
    #[doc = "Bit 2 - Conversion trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgse(&mut self) -> TRGSE_W<2> {
        TRGSE_W::new(self)
    }
    #[doc = "Bit 3 - Start trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn strse(&mut self) -> STRSE_W<3> {
        STRSE_W::new(self)
    }
    #[doc = "Bit 4 - Stop trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stpse(&mut self) -> STPSE_W<4> {
        STPSE_W::new(self)
    }
    #[doc = "Bit 5 - Shadow transfer done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stde(&mut self) -> STDE_W<5> {
        STDE_W::new(self)
    }
    #[doc = "Bit 6 - Comparator rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn crse(&mut self) -> CRSE_W<6> {
        CRSE_W::new(self)
    }
    #[doc = "Bit 7 - Comparator fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfse(&mut self) -> CFSE_W<7> {
        CFSE_W::new(self)
    }
    #[doc = "Bit 8 - Clamped state interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn csee(&mut self) -> CSEE_W<8> {
        CSEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service request enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sre](index.html) module"]
pub struct SRE_SPEC;
impl crate::RegisterSpec for SRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sre::R](R) reader structure"]
impl crate::Readable for SRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sre::W](W) writer structure"]
impl crate::Writable for SRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRE to value 0"]
impl crate::Resettable for SRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
