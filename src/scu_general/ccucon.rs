#[doc = "Register `CCUCON` reader"]
pub struct R(crate::R<CCUCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCUCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCUCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCUCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCUCON` writer"]
pub struct W(crate::W<CCUCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCUCON_SPEC>;
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
impl From<crate::W<CCUCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCUCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Global Start Control CCU40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC40_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC40_A> for bool {
    #[inline(always)]
    fn from(variant: GSC40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC40` reader - Global Start Control CCU40"]
pub struct GSC40_R(crate::FieldReader<bool, GSC40_A>);
impl GSC40_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSC40_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC40_A {
        match self.bits {
            false => GSC40_A::VALUE1,
            true => GSC40_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GSC40_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GSC40_A::VALUE2
    }
}
impl core::ops::Deref for GSC40_R {
    type Target = crate::FieldReader<bool, GSC40_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSC40` writer - Global Start Control CCU40"]
pub struct GSC40_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC40_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC40_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC40_A::VALUE2)
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
#[doc = "Global Start Control CCU41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC41_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC41_A> for bool {
    #[inline(always)]
    fn from(variant: GSC41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC41` reader - Global Start Control CCU41"]
pub struct GSC41_R(crate::FieldReader<bool, GSC41_A>);
impl GSC41_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSC41_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC41_A {
        match self.bits {
            false => GSC41_A::VALUE1,
            true => GSC41_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GSC41_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GSC41_A::VALUE2
    }
}
impl core::ops::Deref for GSC41_R {
    type Target = crate::FieldReader<bool, GSC41_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSC41` writer - Global Start Control CCU41"]
pub struct GSC41_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC41_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC41_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC41_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Global Start Control CCU80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSC80_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC80_A> for bool {
    #[inline(always)]
    fn from(variant: GSC80_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC80` reader - Global Start Control CCU80"]
pub struct GSC80_R(crate::FieldReader<bool, GSC80_A>);
impl GSC80_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSC80_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSC80_A {
        match self.bits {
            false => GSC80_A::VALUE1,
            true => GSC80_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GSC80_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GSC80_A::VALUE2
    }
}
impl core::ops::Deref for GSC80_R {
    type Target = crate::FieldReader<bool, GSC80_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSC80` writer - Global Start Control CCU80"]
pub struct GSC80_W<'a> {
    w: &'a mut W,
}
impl<'a> GSC80_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSC80_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSC80_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSC80_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Global Start Control HRPWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSHR0_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSHR0_A> for bool {
    #[inline(always)]
    fn from(variant: GSHR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSHR0` reader - Global Start Control HRPWM0"]
pub struct GSHR0_R(crate::FieldReader<bool, GSHR0_A>);
impl GSHR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSHR0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSHR0_A {
        match self.bits {
            false => GSHR0_A::VALUE1,
            true => GSHR0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GSHR0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GSHR0_A::VALUE2
    }
}
impl core::ops::Deref for GSHR0_R {
    type Target = crate::FieldReader<bool, GSHR0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSHR0` writer - Global Start Control HRPWM0"]
pub struct GSHR0_W<'a> {
    w: &'a mut W,
}
impl<'a> GSHR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSHR0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GSHR0_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GSHR0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> GSC41_R {
        GSC41_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> GSC80_R {
        GSC80_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Global Start Control HRPWM0"]
    #[inline(always)]
    pub fn gshr0(&self) -> GSHR0_R {
        GSHR0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&mut self) -> GSC40_W {
        GSC40_W { w: self }
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&mut self) -> GSC41_W {
        GSC41_W { w: self }
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&mut self) -> GSC80_W {
        GSC80_W { w: self }
    }
    #[doc = "Bit 24 - Global Start Control HRPWM0"]
    #[inline(always)]
    pub fn gshr0(&mut self) -> GSHR0_W {
        GSHR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccucon](index.html) module"]
pub struct CCUCON_SPEC;
impl crate::RegisterSpec for CCUCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccucon::R](R) reader structure"]
impl crate::Readable for CCUCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccucon::W](W) writer structure"]
impl crate::Writable for CCUCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCUCON to value 0"]
impl crate::Resettable for CCUCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
