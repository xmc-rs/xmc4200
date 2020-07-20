#[doc = "Reader of register PEEN"]
pub type R = crate::R<u32, super::PEEN>;
#[doc = "Writer for register PEEN"]
pub type W = crate::W<u32, super::PEEN>;
#[doc = "Register PEEN `reset()`'s with value 0"]
impl crate::ResetValue for super::PEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Error Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENPS_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEENPS`"]
pub type PEENPS_R = crate::R<bool, PEENPS_A>;
impl PEENPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENPS_A {
        match self.bits {
            false => PEENPS_A::VALUE1,
            true => PEENPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENPS_A::VALUE2
    }
}
#[doc = "Write proxy for field `PEENPS`"]
pub struct PEENPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENPS_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENPS_A::VALUE2)
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
#[doc = "Parity Error Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENDS1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEENDS1`"]
pub type PEENDS1_R = crate::R<bool, PEENDS1_A>;
impl PEENDS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENDS1_A {
        match self.bits {
            false => PEENDS1_A::VALUE1,
            true => PEENDS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENDS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENDS1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PEENDS1`"]
pub struct PEENDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENDS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENDS1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENDS1_A::VALUE2)
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
#[doc = "Parity Error Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENU0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEENU0`"]
pub type PEENU0_R = crate::R<bool, PEENU0_A>;
impl PEENU0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENU0_A {
        match self.bits {
            false => PEENU0_A::VALUE1,
            true => PEENU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENU0_A::VALUE2
    }
}
#[doc = "Write proxy for field `PEENU0`"]
pub struct PEENU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENU0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENU0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENU0_A::VALUE2)
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
#[doc = "Parity Error Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENU1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEENU1`"]
pub type PEENU1_R = crate::R<bool, PEENU1_A>;
impl PEENU1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENU1_A {
        match self.bits {
            false => PEENU1_A::VALUE1,
            true => PEENU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENU1_A::VALUE2
    }
}
#[doc = "Write proxy for field `PEENU1`"]
pub struct PEENU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENU1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENU1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENU1_A::VALUE2)
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
#[doc = "Parity Error Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENMC_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEENMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEENMC`"]
pub type PEENMC_R = crate::R<bool, PEENMC_A>;
impl PEENMC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENMC_A {
        match self.bits {
            false => PEENMC_A::VALUE1,
            true => PEENMC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENMC_A::VALUE2
    }
}
#[doc = "Write proxy for field `PEENMC`"]
pub struct PEENMC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENMC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENMC_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENMC_A::VALUE2)
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
#[doc = "Parity Error Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENPPRF_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEENPPRF`"]
pub type PEENPPRF_R = crate::R<bool, PEENPPRF_A>;
impl PEENPPRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENPPRF_A {
        match self.bits {
            false => PEENPPRF_A::VALUE1,
            true => PEENPPRF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENPPRF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENPPRF_A::VALUE2
    }
}
#[doc = "Write proxy for field `PEENPPRF`"]
pub struct PEENPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENPPRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENPPRF_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENPPRF_A::VALUE2)
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
#[doc = "Parity Error Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEENUSB_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEENUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEENUSB`"]
pub type PEENUSB_R = crate::R<bool, PEENUSB_A>;
impl PEENUSB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENUSB_A {
        match self.bits {
            false => PEENUSB_A::VALUE1,
            true => PEENUSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENUSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENUSB_A::VALUE2
    }
}
#[doc = "Write proxy for field `PEENUSB`"]
pub struct PEENUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PEENUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEENUSB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENUSB_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENUSB_A::VALUE2)
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
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&self) -> PEENPS_R {
        PEENPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&self) -> PEENDS1_R {
        PEENDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&self) -> PEENU0_R {
        PEENU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&self) -> PEENU1_R {
        PEENU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&self) -> PEENMC_R {
        PEENMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&self) -> PEENPPRF_R {
        PEENPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&self) -> PEENUSB_R {
        PEENUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&mut self) -> PEENPS_W {
        PEENPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&mut self) -> PEENDS1_W {
        PEENDS1_W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&mut self) -> PEENU0_W {
        PEENU0_W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&mut self) -> PEENU1_W {
        PEENU1_W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&mut self) -> PEENMC_W {
        PEENMC_W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&mut self) -> PEENPPRF_W {
        PEENPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&mut self) -> PEENUSB_W {
        PEENUSB_W { w: self }
    }
}
