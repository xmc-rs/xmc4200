#[doc = "Register `SSC` reader"]
pub struct R(crate::R<SSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSC` writer"]
pub struct W(crate::W<SSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSC_SPEC>;
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
impl From<crate::W<SSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SST` reader - Source selector for the shadow transfer"]
pub type SST_R = crate::BitReader<SST_A>;
#[doc = "Source selector for the shadow transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SST_A {
    #[doc = "0: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    VALUE1 = 0,
    #[doc = "1: Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    VALUE2 = 1,
}
impl From<SST_A> for bool {
    #[inline(always)]
    fn from(variant: SST_A) -> Self {
        variant as u8 != 0
    }
}
impl SST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SST_A {
        match self.bits {
            false => SST_A::VALUE1,
            true => SST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SST_A::VALUE2
    }
}
#[doc = "Field `SST` writer - Source selector for the shadow transfer"]
pub type SST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSC_SPEC, SST_A, O>;
impl<'a, const O: u8> SST_W<'a, O> {
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SST_A::VALUE1)
    }
    #[doc = "Next shadow transfer signals (shadow transfer trigger and shadow transfer enable) are linked with the timer CC8y connected to the Source Selector 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SST_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    #[must_use]
    pub fn sst(&mut self) -> SST_W<0> {
        SST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC next source for shadow\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc](index.html) module"]
pub struct SSC_SPEC;
impl crate::RegisterSpec for SSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssc::R](R) reader structure"]
impl crate::Readable for SSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssc::W](W) writer structure"]
impl crate::Writable for SSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSC to value 0"]
impl crate::Resettable for SSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
