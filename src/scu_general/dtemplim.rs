#[doc = "Register `DTEMPLIM` reader"]
pub struct R(crate::R<DTEMPLIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTEMPLIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTEMPLIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTEMPLIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTEMPLIM` writer"]
pub struct W(crate::W<DTEMPLIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTEMPLIM_SPEC>;
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
impl From<crate::W<DTEMPLIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTEMPLIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWER` reader - Lower Limit"]
pub struct LOWER_R(crate::FieldReader<u16, u16>);
impl LOWER_R {
    pub(crate) fn new(bits: u16) -> Self {
        LOWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWER` writer - Lower Limit"]
pub struct LOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `UPPER` reader - Upper Limit"]
pub struct UPPER_R(crate::FieldReader<u16, u16>);
impl UPPER_R {
    pub(crate) fn new(bits: u16) -> Self {
        UPPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPPER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPPER` writer - Upper Limit"]
pub struct UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    pub fn lower(&self) -> LOWER_R {
        LOWER_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    pub fn upper(&self) -> UPPER_R {
        UPPER_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Lower Limit"]
    #[inline(always)]
    pub fn lower(&mut self) -> LOWER_W {
        LOWER_W { w: self }
    }
    #[doc = "Bits 16:25 - Upper Limit"]
    #[inline(always)]
    pub fn upper(&mut self) -> UPPER_W {
        UPPER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Die Temperature Sensor Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtemplim](index.html) module"]
pub struct DTEMPLIM_SPEC;
impl crate::RegisterSpec for DTEMPLIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtemplim::R](R) reader structure"]
impl crate::Readable for DTEMPLIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtemplim::W](W) writer structure"]
impl crate::Writable for DTEMPLIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTEMPLIM to value 0"]
impl crate::Resettable for DTEMPLIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
