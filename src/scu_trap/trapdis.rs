#[doc = "Register `TRAPDIS` reader"]
pub struct R(crate::R<TRAPDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRAPDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRAPDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRAPDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRAPDIS` writer"]
pub struct W(crate::W<TRAPDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRAPDIS_SPEC>;
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
impl From<crate::W<TRAPDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRAPDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System OSC WDT Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGT_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOSCWDGT` reader - System OSC WDT Trap Disable"]
pub struct SOSCWDGT_R(crate::FieldReader<bool, SOSCWDGT_A>);
impl SOSCWDGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOSCWDGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::VALUE1,
            true => SOSCWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SOSCWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SOSCWDGT_A::VALUE2
    }
}
impl core::ops::Deref for SOSCWDGT_R {
    type Target = crate::FieldReader<bool, SOSCWDGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCWDGT` writer - System OSC WDT Trap Disable"]
pub struct SOSCWDGT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCWDGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCWDGT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SOSCWDGT_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SOSCWDGT_A::VALUE2)
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
#[doc = "System VCO Lock Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKT_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCOLCKT` reader - System VCO Lock Trap Disable"]
pub struct SVCOLCKT_R(crate::FieldReader<bool, SVCOLCKT_A>);
impl SVCOLCKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVCOLCKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::VALUE1,
            true => SVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SVCOLCKT_A::VALUE2
    }
}
impl core::ops::Deref for SVCOLCKT_R {
    type Target = crate::FieldReader<bool, SVCOLCKT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVCOLCKT` writer - System VCO Lock Trap Disable"]
pub struct SVCOLCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCOLCKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCOLCKT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SVCOLCKT_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SVCOLCKT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "USB VCO Lock Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKT_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UVCOLCKT` reader - USB VCO Lock Trap Disable"]
pub struct UVCOLCKT_R(crate::FieldReader<bool, UVCOLCKT_A>);
impl UVCOLCKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UVCOLCKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::VALUE1,
            true => UVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == UVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == UVCOLCKT_A::VALUE2
    }
}
impl core::ops::Deref for UVCOLCKT_R {
    type Target = crate::FieldReader<bool, UVCOLCKT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UVCOLCKT` writer - USB VCO Lock Trap Disable"]
pub struct UVCOLCKT_W<'a> {
    w: &'a mut W,
}
impl<'a> UVCOLCKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UVCOLCKT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(UVCOLCKT_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(UVCOLCKT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Parity Error Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PET_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PET` reader - Parity Error Trap Disable"]
pub struct PET_R(crate::FieldReader<bool, PET_A>);
impl PET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::VALUE1,
            true => PET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PET_A::VALUE2
    }
}
impl core::ops::Deref for PET_R {
    type Target = crate::FieldReader<bool, PET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PET` writer - Parity Error Trap Disable"]
pub struct PET_W<'a> {
    w: &'a mut W,
}
impl<'a> PET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PET_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PET_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Brown Out Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNT_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRWNT` reader - Brown Out Trap Disable"]
pub struct BRWNT_R(crate::FieldReader<bool, BRWNT_A>);
impl BRWNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRWNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::VALUE1,
            true => BRWNT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BRWNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BRWNT_A::VALUE2
    }
}
impl core::ops::Deref for BRWNT_R {
    type Target = crate::FieldReader<bool, BRWNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRWNT` writer - Brown Out Trap Disable"]
pub struct BRWNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BRWNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRWNT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BRWNT_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BRWNT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Wake-up Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGT_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULPWDGT` reader - Wake-up Trap Disable"]
pub struct ULPWDGT_R(crate::FieldReader<bool, ULPWDGT_A>);
impl ULPWDGT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ULPWDGT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::VALUE1,
            true => ULPWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == ULPWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == ULPWDGT_A::VALUE2
    }
}
impl core::ops::Deref for ULPWDGT_R {
    type Target = crate::FieldReader<bool, ULPWDGT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULPWDGT` writer - Wake-up Trap Disable"]
pub struct ULPWDGT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPWDGT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULPWDGT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ULPWDGT_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ULPWDGT_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Peripheral Bridge 0 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0T_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR0T` reader - Peripheral Bridge 0 Trap Disable"]
pub struct BWERR0T_R(crate::FieldReader<bool, BWERR0T_A>);
impl BWERR0T_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWERR0T_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::VALUE1,
            true => BWERR0T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BWERR0T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BWERR0T_A::VALUE2
    }
}
impl core::ops::Deref for BWERR0T_R {
    type Target = crate::FieldReader<bool, BWERR0T_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWERR0T` writer - Peripheral Bridge 0 Trap Disable"]
pub struct BWERR0T_W<'a> {
    w: &'a mut W,
}
impl<'a> BWERR0T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWERR0T_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWERR0T_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWERR0T_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Peripheral Bridge 1 Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1T_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWERR1T` reader - Peripheral Bridge 1 Trap Disable"]
pub struct BWERR1T_R(crate::FieldReader<bool, BWERR1T_A>);
impl BWERR1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        BWERR1T_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::VALUE1,
            true => BWERR1T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BWERR1T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BWERR1T_A::VALUE2
    }
}
impl core::ops::Deref for BWERR1T_R {
    type Target = crate::FieldReader<bool, BWERR1T_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BWERR1T` writer - Peripheral Bridge 1 Trap Disable"]
pub struct BWERR1T_W<'a> {
    w: &'a mut W,
}
impl<'a> BWERR1T_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWERR1T_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BWERR1T_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BWERR1T_A::VALUE2)
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
#[doc = "Die Temperature Too High Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPHIT_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<TEMPHIT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPHIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPHIT` reader - Die Temperature Too High Trap Disable"]
pub struct TEMPHIT_R(crate::FieldReader<bool, TEMPHIT_A>);
impl TEMPHIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMPHIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPHIT_A {
        match self.bits {
            false => TEMPHIT_A::VALUE1,
            true => TEMPHIT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TEMPHIT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TEMPHIT_A::VALUE2
    }
}
impl core::ops::Deref for TEMPHIT_R {
    type Target = crate::FieldReader<bool, TEMPHIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPHIT` writer - Die Temperature Too High Trap Disable"]
pub struct TEMPHIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPHIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMPHIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TEMPHIT_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TEMPHIT_A::VALUE2)
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
#[doc = "Die Temperature Too Low Trap Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPLOT_A {
    #[doc = "0: Trap request enabled"]
    VALUE1 = 0,
    #[doc = "1: Trap request disabled"]
    VALUE2 = 1,
}
impl From<TEMPLOT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPLOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMPLOT` reader - Die Temperature Too Low Trap Disable"]
pub struct TEMPLOT_R(crate::FieldReader<bool, TEMPLOT_A>);
impl TEMPLOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEMPLOT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPLOT_A {
        match self.bits {
            false => TEMPLOT_A::VALUE1,
            true => TEMPLOT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TEMPLOT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TEMPLOT_A::VALUE2
    }
}
impl core::ops::Deref for TEMPLOT_R {
    type Target = crate::FieldReader<bool, TEMPLOT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMPLOT` writer - Die Temperature Too Low Trap Disable"]
pub struct TEMPLOT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPLOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMPLOT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trap request enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TEMPLOT_A::VALUE1)
    }
    #[doc = "Trap request disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TEMPLOT_A::VALUE2)
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
impl R {
    #[doc = "Bit 0 - System OSC WDT Trap Disable"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-up Trap Disable"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Die Temperature Too High Trap Disable"]
    #[inline(always)]
    pub fn temphit(&self) -> TEMPHIT_R {
        TEMPHIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Die Temperature Too Low Trap Disable"]
    #[inline(always)]
    pub fn templot(&self) -> TEMPLOT_R {
        TEMPLOT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System OSC WDT Trap Disable"]
    #[inline(always)]
    pub fn soscwdgt(&mut self) -> SOSCWDGT_W {
        SOSCWDGT_W { w: self }
    }
    #[doc = "Bit 2 - System VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn svcolckt(&mut self) -> SVCOLCKT_W {
        SVCOLCKT_W { w: self }
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Disable"]
    #[inline(always)]
    pub fn uvcolckt(&mut self) -> UVCOLCKT_W {
        UVCOLCKT_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error Trap Disable"]
    #[inline(always)]
    pub fn pet(&mut self) -> PET_W {
        PET_W { w: self }
    }
    #[doc = "Bit 5 - Brown Out Trap Disable"]
    #[inline(always)]
    pub fn brwnt(&mut self) -> BRWNT_W {
        BRWNT_W { w: self }
    }
    #[doc = "Bit 6 - Wake-up Trap Disable"]
    #[inline(always)]
    pub fn ulpwdgt(&mut self) -> ULPWDGT_W {
        ULPWDGT_W { w: self }
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Disable"]
    #[inline(always)]
    pub fn bwerr0t(&mut self) -> BWERR0T_W {
        BWERR0T_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Disable"]
    #[inline(always)]
    pub fn bwerr1t(&mut self) -> BWERR1T_W {
        BWERR1T_W { w: self }
    }
    #[doc = "Bit 12 - Die Temperature Too High Trap Disable"]
    #[inline(always)]
    pub fn temphit(&mut self) -> TEMPHIT_W {
        TEMPHIT_W { w: self }
    }
    #[doc = "Bit 13 - Die Temperature Too Low Trap Disable"]
    #[inline(always)]
    pub fn templot(&mut self) -> TEMPLOT_W {
        TEMPLOT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trap Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trapdis](index.html) module"]
pub struct TRAPDIS_SPEC;
impl crate::RegisterSpec for TRAPDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trapdis::R](R) reader structure"]
impl crate::Readable for TRAPDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trapdis::W](W) writer structure"]
impl crate::Writable for TRAPDIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRAPDIS to value 0x31ff"]
impl crate::Resettable for TRAPDIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x31ff
    }
}
