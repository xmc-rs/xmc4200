#[doc = "Register `IES` reader"]
pub struct R(crate::R<IES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IES` writer"]
pub struct W(crate::W<IES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IES_SPEC>;
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
impl From<crate::W<IES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External value switch function level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SVLS_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active when input is HIGH"]
    VALUE2 = 1,
    #[doc = "2: Active when input is LOW"]
    VALUE3 = 2,
}
impl From<SVLS_A> for u8 {
    #[inline(always)]
    fn from(variant: SVLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SVLS` reader - External value switch function level selection"]
pub struct SVLS_R(crate::FieldReader<u8, SVLS_A>);
impl SVLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SVLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SVLS_A> {
        match self.bits {
            0 => Some(SVLS_A::VALUE1),
            1 => Some(SVLS_A::VALUE2),
            2 => Some(SVLS_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SVLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SVLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SVLS_A::VALUE3
    }
}
impl core::ops::Deref for SVLS_R {
    type Target = crate::FieldReader<u8, SVLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVLS` writer - External value switch function level selection"]
pub struct SVLS_W<'a> {
    w: &'a mut W,
}
impl<'a> SVLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVLS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVLS_A::VALUE1)
    }
    #[doc = "Active when input is HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVLS_A::VALUE2)
    }
    #[doc = "Active when input is LOW"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SVLS_A::VALUE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "External start function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STRES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<STRES_A> for u8 {
    #[inline(always)]
    fn from(variant: STRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STRES` reader - External start function edge selection"]
pub struct STRES_R(crate::FieldReader<u8, STRES_A>);
impl STRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        STRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRES_A {
        match self.bits {
            0 => STRES_A::VALUE1,
            1 => STRES_A::VALUE2,
            2 => STRES_A::VALUE3,
            3 => STRES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STRES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STRES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == STRES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == STRES_A::VALUE4
    }
}
impl core::ops::Deref for STRES_R {
    type Target = crate::FieldReader<u8, STRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRES` writer - External start function edge selection"]
pub struct STRES_W<'a> {
    w: &'a mut W,
}
impl<'a> STRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STRES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STRES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STRES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STRES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "External stop function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STPES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<STPES_A> for u8 {
    #[inline(always)]
    fn from(variant: STPES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STPES` reader - External stop function edge selection"]
pub struct STPES_R(crate::FieldReader<u8, STPES_A>);
impl STPES_R {
    pub(crate) fn new(bits: u8) -> Self {
        STPES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPES_A {
        match self.bits {
            0 => STPES_A::VALUE1,
            1 => STPES_A::VALUE2,
            2 => STPES_A::VALUE3,
            3 => STPES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STPES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STPES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == STPES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == STPES_A::VALUE4
    }
}
impl core::ops::Deref for STPES_R {
    type Target = crate::FieldReader<u8, STPES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPES` writer - External stop function edge selection"]
pub struct STPES_W<'a> {
    w: &'a mut W,
}
impl<'a> STPES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STPES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STPES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STPES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STPES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "External trigger function edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<TRGES_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGES` reader - External trigger function edge selection"]
pub struct TRGES_R(crate::FieldReader<u8, TRGES_A>);
impl TRGES_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGES_A {
        match self.bits {
            0 => TRGES_A::VALUE1,
            1 => TRGES_A::VALUE2,
            2 => TRGES_A::VALUE3,
            3 => TRGES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRGES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRGES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TRGES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TRGES_A::VALUE4
    }
}
impl core::ops::Deref for TRGES_R {
    type Target = crate::FieldReader<u8, TRGES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGES` writer - External trigger function edge selection"]
pub struct TRGES_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRGES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "External shadow transfer enable edge selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STES_A {
    #[doc = "0: Function disabled"]
    VALUE1 = 0,
    #[doc = "1: Active on rising edge"]
    VALUE2 = 1,
    #[doc = "2: Active on falling edge"]
    VALUE3 = 2,
    #[doc = "3: Active on both edges"]
    VALUE4 = 3,
}
impl From<STES_A> for u8 {
    #[inline(always)]
    fn from(variant: STES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STES` reader - External shadow transfer enable edge selection"]
pub struct STES_R(crate::FieldReader<u8, STES_A>);
impl STES_R {
    pub(crate) fn new(bits: u8) -> Self {
        STES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STES_A {
        match self.bits {
            0 => STES_A::VALUE1,
            1 => STES_A::VALUE2,
            2 => STES_A::VALUE3,
            3 => STES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STES_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STES_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == STES_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == STES_A::VALUE4
    }
}
impl core::ops::Deref for STES_R {
    type Target = crate::FieldReader<u8, STES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STES` writer - External shadow transfer enable edge selection"]
pub struct STES_W<'a> {
    w: &'a mut W,
}
impl<'a> STES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Function disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STES_A::VALUE1)
    }
    #[doc = "Active on rising edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STES_A::VALUE2)
    }
    #[doc = "Active on falling edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STES_A::VALUE3)
    }
    #[doc = "Active on both edges"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STES_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    pub fn svls(&self) -> SVLS_R {
        SVLS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    pub fn stres(&self) -> STRES_R {
        STRES_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    pub fn stpes(&self) -> STPES_R {
        STPES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    pub fn trges(&self) -> TRGES_R {
        TRGES_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    pub fn stes(&self) -> STES_R {
        STES_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External value switch function level selection"]
    #[inline(always)]
    pub fn svls(&mut self) -> SVLS_W {
        SVLS_W { w: self }
    }
    #[doc = "Bits 2:3 - External start function edge selection"]
    #[inline(always)]
    pub fn stres(&mut self) -> STRES_W {
        STRES_W { w: self }
    }
    #[doc = "Bits 4:5 - External stop function edge selection"]
    #[inline(always)]
    pub fn stpes(&mut self) -> STPES_W {
        STPES_W { w: self }
    }
    #[doc = "Bits 6:7 - External trigger function edge selection"]
    #[inline(always)]
    pub fn trges(&mut self) -> TRGES_W {
        TRGES_W { w: self }
    }
    #[doc = "Bits 8:9 - External shadow transfer enable edge selection"]
    #[inline(always)]
    pub fn stes(&mut self) -> STES_W {
        STES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ies](index.html) module"]
pub struct IES_SPEC;
impl crate::RegisterSpec for IES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ies::R](R) reader structure"]
impl crate::Readable for IES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ies::W](W) writer structure"]
impl crate::Writable for IES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IES to value 0"]
impl crate::Resettable for IES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
