#[doc = "Writer for register SWS"]
pub type W = crate::W<u32, super::SWS>;
#[doc = "Register SWS `reset()`'s with value 0"]
impl crate::ResetValue for super::SWS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SVLS1`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `SVLS2`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `STRGS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `SSTRS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `SSTPS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `SSTD`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `SCRS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `SCFS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `SCSS`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
}
