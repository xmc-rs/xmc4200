#[doc = "Writer for register SWC"]
pub type W = crate::W<u32, super::SWC>;
#[doc = "Register SWC `reset()`'s with value 0"]
impl crate::ResetValue for super::SWC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CVLS1`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `CVLS2`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `CTRGS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CSTRS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `CSTPS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `CSTD`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `CCRS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `CCFS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `CCSS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
}
