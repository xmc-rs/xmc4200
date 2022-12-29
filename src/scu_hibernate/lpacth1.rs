#[doc = "Register `LPACTH1` reader"]
pub struct R(crate::R<LPACTH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPACTH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPACTH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPACTH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPACTH1` writer"]
pub struct W(crate::W<LPACTH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPACTH1_SPEC>;
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
impl From<crate::W<LPACTH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPACTH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHIBIO0LO` reader - Analog HIB_IO_0 Lower Threshold Value"]
pub type AHIBIO0LO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHIBIO0LO` writer - Analog HIB_IO_0 Lower Threshold Value"]
pub type AHIBIO0LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPACTH1_SPEC, u8, u8, 6, O>;
#[doc = "Field `AHIBIO0HI` reader - Analog HIB_IO_0 Upper Threshold Value"]
pub type AHIBIO0HI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHIBIO0HI` writer - Analog HIB_IO_0 Upper Threshold Value"]
pub type AHIBIO0HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPACTH1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    pub fn ahibio0lo(&self) -> AHIBIO0LO_R {
        AHIBIO0LO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    pub fn ahibio0hi(&self) -> AHIBIO0HI_R {
        AHIBIO0HI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Analog HIB_IO_0 Lower Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0lo(&mut self) -> AHIBIO0LO_W<0> {
        AHIBIO0LO_W::new(self)
    }
    #[doc = "Bits 8:13 - Analog HIB_IO_0 Upper Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0hi(&mut self) -> AHIBIO0HI_W<8> {
        AHIBIO0HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPAC Threshold Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacth1](index.html) module"]
pub struct LPACTH1_SPEC;
impl crate::RegisterSpec for LPACTH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpacth1::R](R) reader structure"]
impl crate::Readable for LPACTH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpacth1::W](W) writer structure"]
impl crate::Writable for LPACTH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPACTH1 to value 0"]
impl crate::Resettable for LPACTH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
