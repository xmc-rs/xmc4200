#[doc = "Register `SWS` writer"]
pub struct W(crate::W<SWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWS_SPEC>;
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
impl From<crate::W<SWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
pub struct SVLS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SVLS1_W<'a> {
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
#[doc = "Field `SVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
pub struct SVLS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SVLS2_W<'a> {
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
#[doc = "Field `STRGS` writer - Conversion trigger status set"]
pub struct STRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> STRGS_W<'a> {
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
#[doc = "Field `SSTRS` writer - Start trigger status set"]
pub struct SSTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTRS_W<'a> {
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
#[doc = "Field `SSTPS` writer - Stop trigger status set"]
pub struct SSTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTPS_W<'a> {
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
#[doc = "Field `SSTD` writer - Shadow transfer status set"]
pub struct SSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTD_W<'a> {
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
#[doc = "Field `SCRS` writer - Comparator rise status set"]
pub struct SCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRS_W<'a> {
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
#[doc = "Field `SCFS` writer - Comparator fall status set"]
pub struct SCFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCFS_W<'a> {
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
#[doc = "Field `SCSS` writer - Comparator clamped state status set"]
pub struct SCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status set"]
    #[inline(always)]
    pub fn svls1(&mut self) -> SVLS1_W {
        SVLS1_W { w: self }
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status set"]
    #[inline(always)]
    pub fn svls2(&mut self) -> SVLS2_W {
        SVLS2_W { w: self }
    }
    #[doc = "Bit 2 - Conversion trigger status set"]
    #[inline(always)]
    pub fn strgs(&mut self) -> STRGS_W {
        STRGS_W { w: self }
    }
    #[doc = "Bit 3 - Start trigger status set"]
    #[inline(always)]
    pub fn sstrs(&mut self) -> SSTRS_W {
        SSTRS_W { w: self }
    }
    #[doc = "Bit 4 - Stop trigger status set"]
    #[inline(always)]
    pub fn sstps(&mut self) -> SSTPS_W {
        SSTPS_W { w: self }
    }
    #[doc = "Bit 5 - Shadow transfer status set"]
    #[inline(always)]
    pub fn sstd(&mut self) -> SSTD_W {
        SSTD_W { w: self }
    }
    #[doc = "Bit 6 - Comparator rise status set"]
    #[inline(always)]
    pub fn scrs(&mut self) -> SCRS_W {
        SCRS_W { w: self }
    }
    #[doc = "Bit 7 - Comparator fall status set"]
    #[inline(always)]
    pub fn scfs(&mut self) -> SCFS_W {
        SCFS_W { w: self }
    }
    #[doc = "Bit 8 - Comparator clamped state status set"]
    #[inline(always)]
    pub fn scss(&mut self) -> SCSS_W {
        SCSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service request SW set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sws](index.html) module"]
pub struct SWS_SPEC;
impl crate::RegisterSpec for SWS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sws::W](W) writer structure"]
impl crate::Writable for SWS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWS to value 0"]
impl crate::Resettable for SWS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
