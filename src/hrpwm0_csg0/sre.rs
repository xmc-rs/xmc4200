#[doc = "Reader of register SRE"]
pub type R = crate::R<u32, super::SRE>;
#[doc = "Writer for register SRE"]
pub type W = crate::W<u32, super::SRE>;
#[doc = "Register SRE `reset()`'s with value 0"]
impl crate::ResetValue for super::SRE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLS1E`"]
pub type VLS1E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLS1E`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `VLS2E`"]
pub type VLS2E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLS2E`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TRGSE`"]
pub type TRGSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGSE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `STRSE`"]
pub type STRSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STRSE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `STPSE`"]
pub type STPSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPSE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `STDE`"]
pub type STDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STDE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CRSE`"]
pub type CRSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CFSE`"]
pub type CFSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFSE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CSEE`"]
pub type CSEE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSEE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
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
}
