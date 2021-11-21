#[doc = "Register `SCR1` reader"]
pub struct R(crate::R<SCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR1` writer"]
pub struct W(crate::W<SCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR1_SPEC>;
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
impl From<crate::W<SCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCR1` reader - High resolution falling edge value"]
pub struct SCR1_R(crate::FieldReader<u8, u8>);
impl SCR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCR1` writer - High resolution falling edge value"]
pub struct SCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn scr1(&self) -> SCR1_R {
        SCR1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - High resolution falling edge value"]
    #[inline(always)]
    pub fn scr1(&mut self) -> SCR1_W {
        SCR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC shadow rising edge value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr1](index.html) module"]
pub struct SCR1_SPEC;
impl crate::RegisterSpec for SCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr1::R](R) reader structure"]
impl crate::Readable for SCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr1::W](W) writer structure"]
impl crate::Writable for SCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR1 to value 0"]
impl crate::Resettable for SCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
