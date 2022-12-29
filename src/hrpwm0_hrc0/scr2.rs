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
pub type SCR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCR2` writer - High resolution rising edge value"]
pub type SCR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCR2_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn scr2(&mut self) -> SCR2_W<0> {
        SCR2_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR2 to value 0"]
impl crate::Resettable for SCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
