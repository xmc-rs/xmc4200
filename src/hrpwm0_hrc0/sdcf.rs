#[doc = "Register `SDCF` reader"]
pub struct R(crate::R<SDCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCF` writer"]
pub struct W(crate::W<SDCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCF_SPEC>;
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
impl From<crate::W<SDCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDTFV` reader - Shadow dead time falling value"]
pub struct SDTFV_R(crate::FieldReader<u16, u16>);
impl SDTFV_R {
    pub(crate) fn new(bits: u16) -> Self {
        SDTFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDTFV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDTFV` writer - Shadow dead time falling value"]
pub struct SDTFV_W<'a> {
    w: &'a mut W,
}
impl<'a> SDTFV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Shadow dead time falling value"]
    #[inline(always)]
    pub fn sdtfv(&self) -> SDTFV_R {
        SDTFV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shadow dead time falling value"]
    #[inline(always)]
    pub fn sdtfv(&mut self) -> SDTFV_W {
        SDTFV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC shadow dead time falling\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcf](index.html) module"]
pub struct SDCF_SPEC;
impl crate::RegisterSpec for SDCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcf::R](R) reader structure"]
impl crate::Readable for SDCF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcf::W](W) writer structure"]
impl crate::Writable for SDCF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDCF to value 0x01"]
impl crate::Resettable for SDCF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
