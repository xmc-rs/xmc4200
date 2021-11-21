#[doc = "Register `SDCR` reader"]
pub struct R(crate::R<SDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCR` writer"]
pub struct W(crate::W<SDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCR_SPEC>;
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
impl From<crate::W<SDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDTRV` reader - Shadow dead time rising value"]
pub struct SDTRV_R(crate::FieldReader<u16, u16>);
impl SDTRV_R {
    pub(crate) fn new(bits: u16) -> Self {
        SDTRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDTRV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDTRV` writer - Shadow dead time rising value"]
pub struct SDTRV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    pub fn sdtrv(&self) -> SDTRV_R {
        SDTRV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow dead time rising value"]
    #[inline(always)]
    pub fn sdtrv(&mut self) -> SDTRV_W {
        SDTRV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC shadow dead time rising\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcr](index.html) module"]
pub struct SDCR_SPEC;
impl crate::RegisterSpec for SDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcr::R](R) reader structure"]
impl crate::Readable for SDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcr::W](W) writer structure"]
impl crate::Writable for SDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDCR to value 0x01"]
impl crate::Resettable for SDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
