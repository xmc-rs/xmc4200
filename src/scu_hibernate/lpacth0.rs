#[doc = "Register `LPACTH0` reader"]
pub struct R(crate::R<LPACTH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPACTH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPACTH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPACTH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPACTH0` writer"]
pub struct W(crate::W<LPACTH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPACTH0_SPEC>;
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
impl From<crate::W<LPACTH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPACTH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBATLO` reader - VBAT Lower Threshold Value"]
pub struct VBATLO_R(crate::FieldReader<u8, u8>);
impl VBATLO_R {
    pub(crate) fn new(bits: u8) -> Self {
        VBATLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBATLO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATLO` writer - VBAT Lower Threshold Value"]
pub struct VBATLO_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `VBATHI` reader - VBAT Upper Threshold Value"]
pub struct VBATHI_R(crate::FieldReader<u8, u8>);
impl VBATHI_R {
    pub(crate) fn new(bits: u8) -> Self {
        VBATHI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBATHI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBATHI` writer - VBAT Upper Threshold Value"]
pub struct VBATHI_W<'a> {
    w: &'a mut W,
}
impl<'a> VBATHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    pub fn vbatlo(&self) -> VBATLO_R {
        VBATLO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    pub fn vbathi(&self) -> VBATHI_R {
        VBATHI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VBAT Lower Threshold Value"]
    #[inline(always)]
    pub fn vbatlo(&mut self) -> VBATLO_W {
        VBATLO_W { w: self }
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    pub fn vbathi(&mut self) -> VBATHI_W {
        VBATHI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPAC Threshold Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacth0](index.html) module"]
pub struct LPACTH0_SPEC;
impl crate::RegisterSpec for LPACTH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpacth0::R](R) reader structure"]
impl crate::Readable for LPACTH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpacth0::W](W) writer structure"]
impl crate::Writable for LPACTH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPACTH0 to value 0"]
impl crate::Resettable for LPACTH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
