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
#[doc = "Source selector for the shadow transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `SST` reader - Source selector for the shadow transfer"]
pub struct SST_R(crate::FieldReader<bool, SST_A>);
impl SST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == SST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SST_A::VALUE2
    }
}
impl core::ops::Deref for SST_R {
    type Target = crate::FieldReader<bool, SST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SST` writer - Source selector for the shadow transfer"]
pub struct SST_W<'a> {
    w: &'a mut W,
}
impl<'a> SST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SST_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn sst(&self) -> SST_R {
        SST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source selector for the shadow transfer"]
    #[inline(always)]
    pub fn sst(&mut self) -> SST_W {
        SST_W { w: self }
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
}
#[doc = "`reset()` method sets SSC to value 0"]
impl crate::Resettable for SSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
