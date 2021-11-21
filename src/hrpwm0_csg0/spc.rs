#[doc = "Register `SPC` reader"]
pub struct R(crate::R<SPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPC` writer"]
pub struct W(crate::W<SPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPC_SPEC>;
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
impl From<crate::W<SPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPSWV` reader - Shadow pulse swallow value"]
pub struct SPSWV_R(crate::FieldReader<u8, u8>);
impl SPSWV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPSWV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPSWV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPSWV` writer - Shadow pulse swallow value"]
pub struct SPSWV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPSWV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    pub fn spswv(&self) -> SPSWV_R {
        SPSWV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Shadow pulse swallow value"]
    #[inline(always)]
    pub fn spswv(&mut self) -> SPSWV_W {
        SPSWV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shadow Pulse swallow value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spc](index.html) module"]
pub struct SPC_SPEC;
impl crate::RegisterSpec for SPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spc::R](R) reader structure"]
impl crate::Readable for SPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spc::W](W) writer structure"]
impl crate::Writable for SPC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPC to value 0"]
impl crate::Resettable for SPC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
