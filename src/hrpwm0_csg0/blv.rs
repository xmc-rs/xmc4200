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
pub type BLV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLV` writer - Blanking value"]
pub type BLV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLV_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn blv(&mut self) -> BLV_W<0> {
        BLV_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLV to value 0"]
impl crate::Resettable for BLV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
