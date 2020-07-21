#[doc = "Reader of register MCHKCON"]
pub type R = crate::R<u32, super::MCHKCON>;
#[doc = "Writer for register MCHKCON"]
pub type W = crate::W<u32, super::MCHKCON>;
#[doc = "Register MCHKCON `reset()`'s with value 0"]
impl crate::ResetValue for super::MCHKCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select Memory Check for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELPS_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELPS_A> for bool {
    #[inline(always)]
    fn from(variant: SELPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SELPS`"]
pub type SELPS_R = crate::R<bool, SELPS_A>;
impl SELPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELPS_A {
        match self.bits {
            false => SELPS_A::VALUE1,
            true => SELPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELPS_A::VALUE2
    }
}
#[doc = "Write proxy for field `SELPS`"]
pub struct SELPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SELPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELPS_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELPS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Select Memory Check for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELDS1_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELDS1_A> for bool {
    #[inline(always)]
    fn from(variant: SELDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SELDS1`"]
pub type SELDS1_R = crate::R<bool, SELDS1_A>;
impl SELDS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELDS1_A {
        match self.bits {
            false => SELDS1_A::VALUE1,
            true => SELDS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELDS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELDS1_A::VALUE2
    }
}
#[doc = "Write proxy for field `SELDS1`"]
pub struct SELDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SELDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELDS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELDS1_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELDS1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Select Memory Check for USIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC0DRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<USIC0DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC0DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USIC0DRA`"]
pub type USIC0DRA_R = crate::R<bool, USIC0DRA_A>;
impl USIC0DRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC0DRA_A {
        match self.bits {
            false => USIC0DRA_A::VALUE1,
            true => USIC0DRA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC0DRA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC0DRA_A::VALUE2
    }
}
#[doc = "Write proxy for field `USIC0DRA`"]
pub struct USIC0DRA_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC0DRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC0DRA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC0DRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC0DRA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Select Memory Check for USIC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USIC1DRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<USIC1DRA_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1DRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USIC1DRA`"]
pub type USIC1DRA_R = crate::R<bool, USIC1DRA_A>;
impl USIC1DRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USIC1DRA_A {
        match self.bits {
            false => USIC1DRA_A::VALUE1,
            true => USIC1DRA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == USIC1DRA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == USIC1DRA_A::VALUE2
    }
}
#[doc = "Write proxy for field `USIC1DRA`"]
pub struct USIC1DRA_W<'a> {
    w: &'a mut W,
}
impl<'a> USIC1DRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USIC1DRA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(USIC1DRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(USIC1DRA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Select Memory Check for MultiCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCANDRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<MCANDRA_A> for bool {
    #[inline(always)]
    fn from(variant: MCANDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCANDRA`"]
pub type MCANDRA_R = crate::R<bool, MCANDRA_A>;
impl MCANDRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCANDRA_A {
        match self.bits {
            false => MCANDRA_A::VALUE1,
            true => MCANDRA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCANDRA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCANDRA_A::VALUE2
    }
}
#[doc = "Write proxy for field `MCANDRA`"]
pub struct MCANDRA_W<'a> {
    w: &'a mut W,
}
impl<'a> MCANDRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCANDRA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCANDRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCANDRA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Select Memory Check for PMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRFDRA_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<PPRFDRA_A> for bool {
    #[inline(always)]
    fn from(variant: PPRFDRA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PPRFDRA`"]
pub type PPRFDRA_R = crate::R<bool, PPRFDRA_A>;
impl PPRFDRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPRFDRA_A {
        match self.bits {
            false => PPRFDRA_A::VALUE1,
            true => PPRFDRA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PPRFDRA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PPRFDRA_A::VALUE2
    }
}
#[doc = "Write proxy for field `PPRFDRA`"]
pub struct PPRFDRA_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRFDRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPRFDRA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PPRFDRA_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PPRFDRA_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Select Memory Check for USB SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELUSB_A {
    #[doc = "0: Not selected"]
    VALUE1 = 0,
    #[doc = "1: Selected"]
    VALUE2 = 1,
}
impl From<SELUSB_A> for bool {
    #[inline(always)]
    fn from(variant: SELUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SELUSB`"]
pub type SELUSB_R = crate::R<bool, SELUSB_A>;
impl SELUSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELUSB_A {
        match self.bits {
            false => SELUSB_A::VALUE1,
            true => SELUSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELUSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELUSB_A::VALUE2
    }
}
#[doc = "Write proxy for field `SELUSB`"]
pub struct SELUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SELUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELUSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELUSB_A::VALUE1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELUSB_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&self) -> SELPS_R {
        SELPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&self) -> SELDS1_R {
        SELDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&self) -> USIC0DRA_R {
        USIC0DRA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&self) -> USIC1DRA_R {
        USIC1DRA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&self) -> MCANDRA_R {
        MCANDRA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&self) -> PPRFDRA_R {
        PPRFDRA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&self) -> SELUSB_R {
        SELUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&mut self) -> SELPS_W {
        SELPS_W { w: self }
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&mut self) -> SELDS1_W {
        SELDS1_W { w: self }
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&mut self) -> USIC0DRA_W {
        USIC0DRA_W { w: self }
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&mut self) -> USIC1DRA_W {
        USIC1DRA_W { w: self }
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&mut self) -> MCANDRA_W {
        MCANDRA_W { w: self }
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&mut self) -> PPRFDRA_W {
        PPRFDRA_W { w: self }
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&mut self) -> SELUSB_W {
        SELUSB_W { w: self }
    }
}
