#[doc = "Register `SWC` writer"]
pub struct W(crate::W<SWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWC_SPEC>;
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
impl From<crate::W<SWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVLS1` writer - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
pub struct CVLS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CVLS1_W<'a> {
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
#[doc = "Field `CVLS2` writer - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
pub struct CVLS2_W<'a> {
    w: &'a mut W,
}
impl<'a> CVLS2_W<'a> {
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
#[doc = "Field `CTRGS` writer - Conversion trigger status clear"]
pub struct CTRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRGS_W<'a> {
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
#[doc = "Field `CSTRS` writer - Start trigger status clear"]
pub struct CSTRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTRS_W<'a> {
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
#[doc = "Field `CSTPS` writer - Stop trigger status clear"]
pub struct CSTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTPS_W<'a> {
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
#[doc = "Field `CSTD` writer - Shadow transfer status clear"]
pub struct CSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTD_W<'a> {
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
#[doc = "Field `CCRS` writer - Comparator rise status clear"]
pub struct CCRS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRS_W<'a> {
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
#[doc = "Field `CCFS` writer - Comparator fall status clear"]
pub struct CCFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCFS_W<'a> {
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
#[doc = "Field `CCSS` writer - Comparator clamped status clear"]
pub struct CCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CCSS_W<'a> {
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
    #[doc = "Bit 0 - Value switch from CSGyDSV1 to CSGyDSV2 status clear"]
    #[inline(always)]
    pub fn cvls1(&mut self) -> CVLS1_W {
        CVLS1_W { w: self }
    }
    #[doc = "Bit 1 - Value switch from CSGyDSV2 to CSGyDSV1 status clear"]
    #[inline(always)]
    pub fn cvls2(&mut self) -> CVLS2_W {
        CVLS2_W { w: self }
    }
    #[doc = "Bit 2 - Conversion trigger status clear"]
    #[inline(always)]
    pub fn ctrgs(&mut self) -> CTRGS_W {
        CTRGS_W { w: self }
    }
    #[doc = "Bit 3 - Start trigger status clear"]
    #[inline(always)]
    pub fn cstrs(&mut self) -> CSTRS_W {
        CSTRS_W { w: self }
    }
    #[doc = "Bit 4 - Stop trigger status clear"]
    #[inline(always)]
    pub fn cstps(&mut self) -> CSTPS_W {
        CSTPS_W { w: self }
    }
    #[doc = "Bit 5 - Shadow transfer status clear"]
    #[inline(always)]
    pub fn cstd(&mut self) -> CSTD_W {
        CSTD_W { w: self }
    }
    #[doc = "Bit 6 - Comparator rise status clear"]
    #[inline(always)]
    pub fn ccrs(&mut self) -> CCRS_W {
        CCRS_W { w: self }
    }
    #[doc = "Bit 7 - Comparator fall status clear"]
    #[inline(always)]
    pub fn ccfs(&mut self) -> CCFS_W {
        CCFS_W { w: self }
    }
    #[doc = "Bit 8 - Comparator clamped status clear"]
    #[inline(always)]
    pub fn ccss(&mut self) -> CCSS_W {
        CCSS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service request SW clear\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swc](index.html) module"]
pub struct SWC_SPEC;
impl crate::RegisterSpec for SWC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swc::W](W) writer structure"]
impl crate::Writable for SWC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWC to value 0"]
impl crate::Resettable for SWC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
