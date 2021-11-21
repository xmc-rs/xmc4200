#[doc = "Register `SCR2` reader"]
pub struct R(crate::R<SCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR2` writer"]
pub struct W(crate::W<SCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR2_SPEC>;
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
impl From<crate::W<SCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCR2` reader - High resolution rising edge value"]
pub struct SCR2_R(crate::FieldReader<u8, u8>);
impl SCR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCR2` writer - High resolution rising edge value"]
pub struct SCR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn scr2(&self) -> SCR2_R {
        SCR2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution rising edge value"]
    #[inline(always)]
    pub fn scr2(&mut self) -> SCR2_W {
        SCR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC shadow falling edge value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr2](index.html) module"]
pub struct SCR2_SPEC;
impl crate::RegisterSpec for SCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr2::R](R) reader structure"]
impl crate::Readable for SCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr2::W](W) writer structure"]
impl crate::Writable for SCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR2 to value 0"]
impl crate::Resettable for SCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
