#[doc = "Register `GLBANA` reader"]
pub struct R(crate::R<GLBANA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLBANA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLBANA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLBANA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GLBANA` writer"]
pub struct W(crate::W<GLBANA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLBANA_SPEC>;
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
impl From<crate::W<GLBANA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLBANA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLDLY` reader - Delay of lock detection"]
pub struct SLDLY_R(crate::FieldReader<u8, u8>);
impl SLDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLDLY` writer - Delay of lock detection"]
pub struct SLDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SLDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `FUP` reader - Force chargepump up"]
pub struct FUP_R(crate::FieldReader<bool, bool>);
impl FUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUP` writer - Force chargepump up"]
pub struct FUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FDN` reader - Force chargepump down"]
pub struct FDN_R(crate::FieldReader<bool, bool>);
impl FDN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FDN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDN` writer - Force chargepump down"]
pub struct FDN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SLCP` reader - HRCs chargepump current selection"]
pub struct SLCP_R(crate::FieldReader<u8, u8>);
impl SLCP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLCP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLCP` writer - HRCs chargepump current selection"]
pub struct SLCP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLCP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `SLIBLDO` reader - HRCs LDO bias current"]
pub struct SLIBLDO_R(crate::FieldReader<u8, u8>);
impl SLIBLDO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLIBLDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLIBLDO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLIBLDO` writer - HRCs LDO bias current"]
pub struct SLIBLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLIBLDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `SLIBLF` reader - HRCs loop filter bias current"]
pub struct SLIBLF_R(crate::FieldReader<u8, u8>);
impl SLIBLF_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLIBLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLIBLF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLIBLF` writer - HRCs loop filter bias current"]
pub struct SLIBLF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLIBLF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `SLVREF` reader - Reference voltage for chargepump and loop filter"]
pub struct SLVREF_R(crate::FieldReader<u8, u8>);
impl SLVREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        SLVREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLVREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVREF` writer - Reference voltage for chargepump and loop filter"]
pub struct SLVREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `TRIBIAS` reader - Bias trimming"]
pub struct TRIBIAS_R(crate::FieldReader<u8, u8>);
impl TRIBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIBIAS` writer - Bias trimming"]
pub struct TRIBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Force chargepump down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GHREN_A {
    #[doc = "0: Global high resolution generation is enabled"]
    VALUE1 = 0,
    #[doc = "1: Global high resolution generation is disabled"]
    VALUE2 = 1,
}
impl From<GHREN_A> for bool {
    #[inline(always)]
    fn from(variant: GHREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GHREN` reader - Force chargepump down"]
pub struct GHREN_R(crate::FieldReader<bool, GHREN_A>);
impl GHREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GHREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GHREN_A {
        match self.bits {
            false => GHREN_A::VALUE1,
            true => GHREN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GHREN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GHREN_A::VALUE2
    }
}
impl core::ops::Deref for GHREN_R {
    type Target = crate::FieldReader<bool, GHREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GHREN` writer - Force chargepump down"]
pub struct GHREN_W<'a> {
    w: &'a mut W,
}
impl<'a> GHREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GHREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Global high resolution generation is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GHREN_A::VALUE1)
    }
    #[doc = "Global high resolution generation is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GHREN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    pub fn sldly(&self) -> SLDLY_R {
        SLDLY_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    pub fn fup(&self) -> FUP_R {
        FUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    pub fn fdn(&self) -> FDN_R {
        FDN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    pub fn slcp(&self) -> SLCP_R {
        SLCP_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    pub fn slibldo(&self) -> SLIBLDO_R {
        SLIBLDO_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    pub fn sliblf(&self) -> SLIBLF_R {
        SLIBLF_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    pub fn slvref(&self) -> SLVREF_R {
        SLVREF_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    pub fn tribias(&self) -> TRIBIAS_R {
        TRIBIAS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    pub fn ghren(&self) -> GHREN_R {
        GHREN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Delay of lock detection"]
    #[inline(always)]
    pub fn sldly(&mut self) -> SLDLY_W {
        SLDLY_W { w: self }
    }
    #[doc = "Bit 2 - Force chargepump up"]
    #[inline(always)]
    pub fn fup(&mut self) -> FUP_W {
        FUP_W { w: self }
    }
    #[doc = "Bit 3 - Force chargepump down"]
    #[inline(always)]
    pub fn fdn(&mut self) -> FDN_W {
        FDN_W { w: self }
    }
    #[doc = "Bits 6:8 - HRCs chargepump current selection"]
    #[inline(always)]
    pub fn slcp(&mut self) -> SLCP_W {
        SLCP_W { w: self }
    }
    #[doc = "Bits 9:10 - HRCs LDO bias current"]
    #[inline(always)]
    pub fn slibldo(&mut self) -> SLIBLDO_W {
        SLIBLDO_W { w: self }
    }
    #[doc = "Bits 11:12 - HRCs loop filter bias current"]
    #[inline(always)]
    pub fn sliblf(&mut self) -> SLIBLF_W {
        SLIBLF_W { w: self }
    }
    #[doc = "Bits 13:15 - Reference voltage for chargepump and loop filter"]
    #[inline(always)]
    pub fn slvref(&mut self) -> SLVREF_W {
        SLVREF_W { w: self }
    }
    #[doc = "Bits 16:17 - Bias trimming"]
    #[inline(always)]
    pub fn tribias(&mut self) -> TRIBIAS_W {
        TRIBIAS_W { w: self }
    }
    #[doc = "Bit 18 - Force chargepump down"]
    #[inline(always)]
    pub fn ghren(&mut self) -> GHREN_W {
        GHREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Analog Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glbana](index.html) module"]
pub struct GLBANA_SPEC;
impl crate::RegisterSpec for GLBANA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glbana::R](R) reader structure"]
impl crate::Readable for GLBANA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glbana::W](W) writer structure"]
impl crate::Writable for GLBANA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GLBANA to value 0x4b8c"]
impl crate::Resettable for GLBANA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4b8c
    }
}
