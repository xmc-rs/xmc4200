#[doc = "Register `PETE` reader"]
pub struct R(crate::R<PETE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PETE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PETE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PETE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PETE` writer"]
pub struct W(crate::W<PETE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PETE_SPEC>;
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
impl From<crate::W<PETE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PETE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Parity Error Trap Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEPS_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEPS_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPS` reader - Parity Error Trap Enable for PSRAM"]
pub struct PETEPS_R(crate::FieldReader<bool, PETEPS_A>);
impl PETEPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PETEPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEPS_A {
        match self.bits {
            false => PETEPS_A::VALUE1,
            true => PETEPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PETEPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PETEPS_A::VALUE2
    }
}
impl core::ops::Deref for PETEPS_R {
    type Target = crate::FieldReader<bool, PETEPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PETEPS` writer - Parity Error Trap Enable for PSRAM"]
pub struct PETEPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEPS_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEPS_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEDS1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEDS1` reader - Parity Error Trap Enable for DSRAM1"]
pub struct PETEDS1_R(crate::FieldReader<bool, PETEDS1_A>);
impl PETEDS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PETEDS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEDS1_A {
        match self.bits {
            false => PETEDS1_A::VALUE1,
            true => PETEDS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PETEDS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PETEDS1_A::VALUE2
    }
}
impl core::ops::Deref for PETEDS1_R {
    type Target = crate::FieldReader<bool, PETEDS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PETEDS1` writer - Parity Error Trap Enable for DSRAM1"]
pub struct PETEDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEDS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEDS1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEDS1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEU0_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU0` reader - Parity Error Trap Enable for USIC0 Memory"]
pub struct PETEU0_R(crate::FieldReader<bool, PETEU0_A>);
impl PETEU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PETEU0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEU0_A {
        match self.bits {
            false => PETEU0_A::VALUE1,
            true => PETEU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PETEU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PETEU0_A::VALUE2
    }
}
impl core::ops::Deref for PETEU0_R {
    type Target = crate::FieldReader<bool, PETEU0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PETEU0` writer - Parity Error Trap Enable for USIC0 Memory"]
pub struct PETEU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEU0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU0_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEU1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEU1_A> for bool {
    #[inline(always)]
    fn from(variant: PETEU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU1` reader - Parity Error Trap Enable for USIC1 Memory"]
pub struct PETEU1_R(crate::FieldReader<bool, PETEU1_A>);
impl PETEU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PETEU1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEU1_A {
        match self.bits {
            false => PETEU1_A::VALUE1,
            true => PETEU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PETEU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PETEU1_A::VALUE2
    }
}
impl core::ops::Deref for PETEU1_R {
    type Target = crate::FieldReader<bool, PETEU1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PETEU1` writer - Parity Error Trap Enable for USIC1 Memory"]
pub struct PETEU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEU1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEU1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEU1_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEMC_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEMC_A> for bool {
    #[inline(always)]
    fn from(variant: PETEMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEMC` reader - Parity Error Trap Enable for MultiCAN Memory"]
pub struct PETEMC_R(crate::FieldReader<bool, PETEMC_A>);
impl PETEMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PETEMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEMC_A {
        match self.bits {
            false => PETEMC_A::VALUE1,
            true => PETEMC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PETEMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PETEMC_A::VALUE2
    }
}
impl core::ops::Deref for PETEMC_R {
    type Target = crate::FieldReader<bool, PETEMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PETEMC` writer - Parity Error Trap Enable for MultiCAN Memory"]
pub struct PETEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEMC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEMC_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEMC_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEPPRF_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PETEPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPPRF` reader - Parity Error Trap Enable for PMU Prefetch Memory"]
pub struct PETEPPRF_R(crate::FieldReader<bool, PETEPPRF_A>);
impl PETEPPRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PETEPPRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEPPRF_A {
        match self.bits {
            false => PETEPPRF_A::VALUE1,
            true => PETEPPRF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PETEPPRF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PETEPPRF_A::VALUE2
    }
}
impl core::ops::Deref for PETEPPRF_R {
    type Target = crate::FieldReader<bool, PETEPPRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PETEPPRF` writer - Parity Error Trap Enable for PMU Prefetch Memory"]
pub struct PETEPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEPPRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEPPRF_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEPPRF_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Parity Error Trap Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PETEUSB_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PETEUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PETEUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEUSB` reader - Parity Error Trap Enable for USB Memory"]
pub struct PETEUSB_R(crate::FieldReader<bool, PETEUSB_A>);
impl PETEUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PETEUSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PETEUSB_A {
        match self.bits {
            false => PETEUSB_A::VALUE1,
            true => PETEUSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PETEUSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PETEUSB_A::VALUE2
    }
}
impl core::ops::Deref for PETEUSB_R {
    type Target = crate::FieldReader<bool, PETEUSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PETEUSB` writer - Parity Error Trap Enable for USB Memory"]
pub struct PETEUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PETEUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PETEUSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PETEUSB_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PETEUSB_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&self) -> PETEPS_R {
        PETEPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&self) -> PETEDS1_R {
        PETEDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&self) -> PETEU0_R {
        PETEU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&self) -> PETEU1_R {
        PETEU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&self) -> PETEMC_R {
        PETEMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&self) -> PETEPPRF_R {
        PETEPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&self) -> PETEUSB_R {
        PETEUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&mut self) -> PETEPS_W {
        PETEPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&mut self) -> PETEDS1_W {
        PETEDS1_W { w: self }
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&mut self) -> PETEU0_W {
        PETEU0_W { w: self }
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&mut self) -> PETEU1_W {
        PETEU1_W { w: self }
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&mut self) -> PETEMC_W {
        PETEMC_W { w: self }
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&mut self) -> PETEPPRF_W {
        PETEPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&mut self) -> PETEUSB_W {
        PETEUSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Trap Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pete](index.html) module"]
pub struct PETE_SPEC;
impl crate::RegisterSpec for PETE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pete::R](R) reader structure"]
impl crate::Readable for PETE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pete::W](W) writer structure"]
impl crate::Writable for PETE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PETE to value 0"]
impl crate::Resettable for PETE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
