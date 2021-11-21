#[doc = "Register `SRE` reader"]
pub struct R(crate::R<SRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRE` writer"]
pub struct W(crate::W<SRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRE_SPEC>;
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
impl From<crate::W<SRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLS1E` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub struct VLS1E_R(crate::FieldReader<bool, bool>);
impl VLS1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLS1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLS1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLS1E` writer - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
pub struct VLS1E_W<'a> {
    w: &'a mut W,
}
impl<'a> VLS1E_W<'a> {
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
#[doc = "Field `VLS2E` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub struct VLS2E_R(crate::FieldReader<bool, bool>);
impl VLS2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLS2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VLS2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLS2E` writer - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
pub struct VLS2E_W<'a> {
    w: &'a mut W,
}
impl<'a> VLS2E_W<'a> {
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
#[doc = "Field `TRGSE` reader - Conversion trigger interrupt enable"]
pub struct TRGSE_R(crate::FieldReader<bool, bool>);
impl TRGSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSE` writer - Conversion trigger interrupt enable"]
pub struct TRGSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSE_W<'a> {
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
#[doc = "Field `STRSE` reader - Start trigger interrupt enable"]
pub struct STRSE_R(crate::FieldReader<bool, bool>);
impl STRSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRSE` writer - Start trigger interrupt enable"]
pub struct STRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSE_W<'a> {
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
#[doc = "Field `STPSE` reader - Stop trigger interrupt enable"]
pub struct STPSE_R(crate::FieldReader<bool, bool>);
impl STPSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPSE` writer - Stop trigger interrupt enable"]
pub struct STPSE_W<'a> {
    w: &'a mut W,
}
impl<'a> STPSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `STDE` reader - Shadow transfer done interrupt enable"]
pub struct STDE_R(crate::FieldReader<bool, bool>);
impl STDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STDE` writer - Shadow transfer done interrupt enable"]
pub struct STDE_W<'a> {
    w: &'a mut W,
}
impl<'a> STDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CRSE` reader - Comparator rise interrupt enable"]
pub struct CRSE_R(crate::FieldReader<bool, bool>);
impl CRSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRSE` writer - Comparator rise interrupt enable"]
pub struct CRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CFSE` reader - Comparator fall interrupt enable"]
pub struct CFSE_R(crate::FieldReader<bool, bool>);
impl CFSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFSE` writer - Comparator fall interrupt enable"]
pub struct CFSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CSEE` reader - Clamped state interrupt enable"]
pub struct CSEE_R(crate::FieldReader<bool, bool>);
impl CSEE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSEE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSEE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSEE` writer - Clamped state interrupt enable"]
pub struct CSEE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
    #[inline(always)]
    pub fn vls1e(&self) -> VLS1E_R {
        VLS1E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
    #[inline(always)]
    pub fn vls2e(&self) -> VLS2E_R {
        VLS2E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Conversion trigger interrupt enable"]
    #[inline(always)]
    pub fn trgse(&self) -> TRGSE_R {
        TRGSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Start trigger interrupt enable"]
    #[inline(always)]
    pub fn strse(&self) -> STRSE_R {
        STRSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stop trigger interrupt enable"]
    #[inline(always)]
    pub fn stpse(&self) -> STPSE_R {
        STPSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shadow transfer done interrupt enable"]
    #[inline(always)]
    pub fn stde(&self) -> STDE_R {
        STDE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Comparator rise interrupt enable"]
    #[inline(always)]
    pub fn crse(&self) -> CRSE_R {
        CRSE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Comparator fall interrupt enable"]
    #[inline(always)]
    pub fn cfse(&self) -> CFSE_R {
        CFSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clamped state interrupt enable"]
    #[inline(always)]
    pub fn csee(&self) -> CSEE_R {
        CSEE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt enable"]
    #[inline(always)]
    pub fn vls1e(&mut self) -> VLS1E_W {
        VLS1E_W { w: self }
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt enable"]
    #[inline(always)]
    pub fn vls2e(&mut self) -> VLS2E_W {
        VLS2E_W { w: self }
    }
    #[doc = "Bit 2 - Conversion trigger interrupt enable"]
    #[inline(always)]
    pub fn trgse(&mut self) -> TRGSE_W {
        TRGSE_W { w: self }
    }
    #[doc = "Bit 3 - Start trigger interrupt enable"]
    #[inline(always)]
    pub fn strse(&mut self) -> STRSE_W {
        STRSE_W { w: self }
    }
    #[doc = "Bit 4 - Stop trigger interrupt enable"]
    #[inline(always)]
    pub fn stpse(&mut self) -> STPSE_W {
        STPSE_W { w: self }
    }
    #[doc = "Bit 5 - Shadow transfer done interrupt enable"]
    #[inline(always)]
    pub fn stde(&mut self) -> STDE_W {
        STDE_W { w: self }
    }
    #[doc = "Bit 6 - Comparator rise interrupt enable"]
    #[inline(always)]
    pub fn crse(&mut self) -> CRSE_W {
        CRSE_W { w: self }
    }
    #[doc = "Bit 7 - Comparator fall interrupt enable"]
    #[inline(always)]
    pub fn cfse(&mut self) -> CFSE_W {
        CFSE_W { w: self }
    }
    #[doc = "Bit 8 - Clamped state interrupt enable"]
    #[inline(always)]
    pub fn csee(&mut self) -> CSEE_W {
        CSEE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service request enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sre](index.html) module"]
pub struct SRE_SPEC;
impl crate::RegisterSpec for SRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sre::R](R) reader structure"]
impl crate::Readable for SRE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sre::W](W) writer structure"]
impl crate::Writable for SRE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRE to value 0"]
impl crate::Resettable for SRE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
