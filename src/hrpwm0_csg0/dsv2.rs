#[doc = "Register `DSV2` reader"]
pub struct R(crate::R<DSV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSV2` writer"]
pub struct W(crate::W<DSV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSV2_SPEC>;
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
impl From<crate::W<DSV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSV2` reader - DAC reference value 2"]
pub struct DSV2_R(crate::FieldReader<u16, u16>);
impl DSV2_R {
    pub(crate) fn new(bits: u16) -> Self {
        DSV2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSV2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSV2` writer - DAC reference value 2"]
pub struct DSV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DSV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DAC reference value 2"]
    #[inline(always)]
    pub fn dsv2(&self) -> DSV2_R {
        DSV2_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC reference value 2"]
    #[inline(always)]
    pub fn dsv2(&mut self) -> DSV2_W {
        DSV2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC reference value 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsv2](index.html) module"]
pub struct DSV2_SPEC;
impl crate::RegisterSpec for DSV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsv2::R](R) reader structure"]
impl crate::Readable for DSV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsv2::W](W) writer structure"]
impl crate::Writable for DSV2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSV2 to value 0"]
impl crate::Resettable for DSV2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
