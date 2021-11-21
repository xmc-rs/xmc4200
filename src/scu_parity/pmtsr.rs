#[doc = "Register `PMTSR` reader"]
pub struct R(crate::R<PMTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMTSR` writer"]
pub struct W(crate::W<PMTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMTSR_SPEC>;
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
impl From<crate::W<PMTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Test Enable Control for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENPS_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTENPS_A> for bool {
    #[inline(always)]
    fn from(variant: MTENPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTENPS` reader - Test Enable Control for PSRAM"]
pub struct MTENPS_R(crate::FieldReader<bool, MTENPS_A>);
impl MTENPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENPS_A {
        match self.bits {
            false => MTENPS_A::VALUE1,
            true => MTENPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MTENPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MTENPS_A::VALUE2
    }
}
impl core::ops::Deref for MTENPS_R {
    type Target = crate::FieldReader<bool, MTENPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENPS` writer - Test Enable Control for PSRAM"]
pub struct MTENPS_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTENPS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENPS_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENPS_A::VALUE2)
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
#[doc = "Test Enable Control for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTENDS1_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: MTENDS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTENDS1` reader - Test Enable Control for DSRAM1"]
pub struct MTENDS1_R(crate::FieldReader<bool, MTENDS1_A>);
impl MTENDS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTENDS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTENDS1_A {
        match self.bits {
            false => MTENDS1_A::VALUE1,
            true => MTENDS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MTENDS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MTENDS1_A::VALUE2
    }
}
impl core::ops::Deref for MTENDS1_R {
    type Target = crate::FieldReader<bool, MTENDS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTENDS1` writer - Test Enable Control for DSRAM1"]
pub struct MTENDS1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTENDS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTENDS1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTENDS1_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTENDS1_A::VALUE2)
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
#[doc = "Test Enable Control for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU0_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEU0_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEU0` reader - Test Enable Control for USIC0 Memory"]
pub struct MTEU0_R(crate::FieldReader<bool, MTEU0_A>);
impl MTEU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEU0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEU0_A {
        match self.bits {
            false => MTEU0_A::VALUE1,
            true => MTEU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MTEU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MTEU0_A::VALUE2
    }
}
impl core::ops::Deref for MTEU0_R {
    type Target = crate::FieldReader<bool, MTEU0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEU0` writer - Test Enable Control for USIC0 Memory"]
pub struct MTEU0_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEU0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEU0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEU0_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEU0_A::VALUE2)
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
#[doc = "Test Enable Control for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEU1_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEU1_A> for bool {
    #[inline(always)]
    fn from(variant: MTEU1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEU1` reader - Test Enable Control for USIC1 Memory"]
pub struct MTEU1_R(crate::FieldReader<bool, MTEU1_A>);
impl MTEU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEU1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEU1_A {
        match self.bits {
            false => MTEU1_A::VALUE1,
            true => MTEU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MTEU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MTEU1_A::VALUE2
    }
}
impl core::ops::Deref for MTEU1_R {
    type Target = crate::FieldReader<bool, MTEU1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEU1` writer - Test Enable Control for USIC1 Memory"]
pub struct MTEU1_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEU1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEU1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEU1_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEU1_A::VALUE2)
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
#[doc = "Test Enable Control for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEMC_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEMC_A> for bool {
    #[inline(always)]
    fn from(variant: MTEMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEMC` reader - Test Enable Control for MultiCAN Memory"]
pub struct MTEMC_R(crate::FieldReader<bool, MTEMC_A>);
impl MTEMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEMC_A {
        match self.bits {
            false => MTEMC_A::VALUE1,
            true => MTEMC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MTEMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MTEMC_A::VALUE2
    }
}
impl core::ops::Deref for MTEMC_R {
    type Target = crate::FieldReader<bool, MTEMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEMC` writer - Test Enable Control for MultiCAN Memory"]
pub struct MTEMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEMC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEMC_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEMC_A::VALUE2)
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
#[doc = "Test Enable Control for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTEPPRF_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTEPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: MTEPPRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTEPPRF` reader - Test Enable Control for PMU Prefetch Memory"]
pub struct MTEPPRF_R(crate::FieldReader<bool, MTEPPRF_A>);
impl MTEPPRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTEPPRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTEPPRF_A {
        match self.bits {
            false => MTEPPRF_A::VALUE1,
            true => MTEPPRF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MTEPPRF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MTEPPRF_A::VALUE2
    }
}
impl core::ops::Deref for MTEPPRF_R {
    type Target = crate::FieldReader<bool, MTEPPRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTEPPRF` writer - Test Enable Control for PMU Prefetch Memory"]
pub struct MTEPPRF_W<'a> {
    w: &'a mut W,
}
impl<'a> MTEPPRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTEPPRF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTEPPRF_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTEPPRF_A::VALUE2)
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
#[doc = "Test Enable Control for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTUSB_A {
    #[doc = "0: Standard operation"]
    VALUE1 = 0,
    #[doc = "1: Parity bits under test"]
    VALUE2 = 1,
}
impl From<MTUSB_A> for bool {
    #[inline(always)]
    fn from(variant: MTUSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTUSB` reader - Test Enable Control for USB Memory"]
pub struct MTUSB_R(crate::FieldReader<bool, MTUSB_A>);
impl MTUSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTUSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTUSB_A {
        match self.bits {
            false => MTUSB_A::VALUE1,
            true => MTUSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == MTUSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == MTUSB_A::VALUE2
    }
}
impl core::ops::Deref for MTUSB_R {
    type Target = crate::FieldReader<bool, MTUSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTUSB` writer - Test Enable Control for USB Memory"]
pub struct MTUSB_W<'a> {
    w: &'a mut W,
}
impl<'a> MTUSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTUSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MTUSB_A::VALUE1)
    }
    #[doc = "Parity bits under test"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MTUSB_A::VALUE2)
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
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    pub fn mtenps(&self) -> MTENPS_R {
        MTENPS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    pub fn mtends1(&self) -> MTENDS1_R {
        MTENDS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    pub fn mteu0(&self) -> MTEU0_R {
        MTEU0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    pub fn mteu1(&self) -> MTEU1_R {
        MTEU1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    pub fn mtemc(&self) -> MTEMC_R {
        MTEMC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn mtepprf(&self) -> MTEPPRF_R {
        MTEPPRF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    pub fn mtusb(&self) -> MTUSB_R {
        MTUSB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Test Enable Control for PSRAM"]
    #[inline(always)]
    pub fn mtenps(&mut self) -> MTENPS_W {
        MTENPS_W { w: self }
    }
    #[doc = "Bit 1 - Test Enable Control for DSRAM1"]
    #[inline(always)]
    pub fn mtends1(&mut self) -> MTENDS1_W {
        MTENDS1_W { w: self }
    }
    #[doc = "Bit 8 - Test Enable Control for USIC0 Memory"]
    #[inline(always)]
    pub fn mteu0(&mut self) -> MTEU0_W {
        MTEU0_W { w: self }
    }
    #[doc = "Bit 9 - Test Enable Control for USIC1 Memory"]
    #[inline(always)]
    pub fn mteu1(&mut self) -> MTEU1_W {
        MTEU1_W { w: self }
    }
    #[doc = "Bit 12 - Test Enable Control for MultiCAN Memory"]
    #[inline(always)]
    pub fn mtemc(&mut self) -> MTEMC_W {
        MTEMC_W { w: self }
    }
    #[doc = "Bit 13 - Test Enable Control for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn mtepprf(&mut self) -> MTEPPRF_W {
        MTEPPRF_W { w: self }
    }
    #[doc = "Bit 16 - Test Enable Control for USB Memory"]
    #[inline(always)]
    pub fn mtusb(&mut self) -> MTUSB_W {
        MTUSB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Memory Test Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmtsr](index.html) module"]
pub struct PMTSR_SPEC;
impl crate::RegisterSpec for PMTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmtsr::R](R) reader structure"]
impl crate::Readable for PMTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmtsr::W](W) writer structure"]
impl crate::Writable for PMTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMTSR to value 0"]
impl crate::Resettable for PMTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
