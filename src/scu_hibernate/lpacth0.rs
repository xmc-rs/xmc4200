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
pub type VBATLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBATLO` writer - VBAT Lower Threshold Value"]
pub type VBATLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPACTH0_SPEC, u8, u8, 6, O>;
#[doc = "Field `VBATHI` reader - VBAT Upper Threshold Value"]
pub type VBATHI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VBATHI` writer - VBAT Upper Threshold Value"]
pub type VBATHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPACTH0_SPEC, u8, u8, 6, O>;
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
    #[must_use]
    pub fn vbatlo(&mut self) -> VBATLO_W<0> {
        VBATLO_W::new(self)
    }
    #[doc = "Bits 8:13 - VBAT Upper Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn vbathi(&mut self) -> VBATHI_W<8> {
        VBATHI_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPACTH0 to value 0"]
impl crate::Resettable for LPACTH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
