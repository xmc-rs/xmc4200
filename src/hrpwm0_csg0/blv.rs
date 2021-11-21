#[doc = "Register `BLV` reader"]
pub struct R(crate::R<BLV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLV` writer"]
pub struct W(crate::W<BLV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLV_SPEC>;
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
impl From<crate::W<BLV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLV` reader - Blanking value"]
pub struct BLV_R(crate::FieldReader<u8, u8>);
impl BLV_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLV` writer - Blanking value"]
pub struct BLV_W<'a> {
    w: &'a mut W,
}
impl<'a> BLV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    pub fn blv(&self) -> BLV_R {
        BLV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Blanking value"]
    #[inline(always)]
    pub fn blv(&mut self) -> BLV_W {
        BLV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator blanking value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blv](index.html) module"]
pub struct BLV_SPEC;
impl crate::RegisterSpec for BLV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blv::R](R) reader structure"]
impl crate::Readable for BLV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blv::W](W) writer structure"]
impl crate::Writable for BLV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLV to value 0"]
impl crate::Resettable for BLV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
