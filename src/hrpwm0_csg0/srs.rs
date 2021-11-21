#[doc = "Register `SRS` reader"]
pub struct R(crate::R<SRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRS` writer"]
pub struct W(crate::W<SRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRS_SPEC>;
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
impl From<crate::W<SRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLS1S_A {
    #[doc = "0: CSGySR0"]
    VALUE1 = 0,
    #[doc = "1: CSGySR1"]
    VALUE2 = 1,
    #[doc = "2: CSGySR2"]
    VALUE3 = 2,
    #[doc = "3: CSGySR3"]
    VALUE4 = 3,
}
impl From<VLS1S_A> for u8 {
    #[inline(always)]
    fn from(variant: VLS1S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VLS1S` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
pub struct VLS1S_R(crate::FieldReader<u8, VLS1S_A>);
impl VLS1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        VLS1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLS1S_A {
        match self.bits {
            0 => VLS1S_A::VALUE1,
            1 => VLS1S_A::VALUE2,
            2 => VLS1S_A::VALUE3,
            3 => VLS1S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VLS1S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VLS1S_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == VLS1S_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == VLS1S_A::VALUE4
    }
}
impl core::ops::Deref for VLS1S_R {
    type Target = crate::FieldReader<u8, VLS1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLS1S` writer - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
pub struct VLS1S_W<'a> {
    w: &'a mut W,
}
impl<'a> VLS1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLS1S_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VLS1S_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VLS1S_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(VLS1S_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(VLS1S_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLS2S_A {
    #[doc = "0: CSGySR0"]
    VALUE1 = 0,
    #[doc = "1: CSGySR1"]
    VALUE2 = 1,
    #[doc = "2: CSGySR2"]
    VALUE3 = 2,
    #[doc = "3: CSGySR3"]
    VALUE4 = 3,
}
impl From<VLS2S_A> for u8 {
    #[inline(always)]
    fn from(variant: VLS2S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VLS2S` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
pub struct VLS2S_R(crate::FieldReader<u8, VLS2S_A>);
impl VLS2S_R {
    pub(crate) fn new(bits: u8) -> Self {
        VLS2S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLS2S_A {
        match self.bits {
            0 => VLS2S_A::VALUE1,
            1 => VLS2S_A::VALUE2,
            2 => VLS2S_A::VALUE3,
            3 => VLS2S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == VLS2S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == VLS2S_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == VLS2S_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == VLS2S_A::VALUE4
    }
}
impl core::ops::Deref for VLS2S_R {
    type Target = crate::FieldReader<u8, VLS2S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLS2S` writer - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
pub struct VLS2S_W<'a> {
    w: &'a mut W,
}
impl<'a> VLS2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLS2S_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VLS2S_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VLS2S_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(VLS2S_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(VLS2S_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Conversion trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1 = 0,
    #[doc = "1: CSGySR1"]
    VALUE2 = 1,
    #[doc = "2: CSGySR2"]
    VALUE3 = 2,
    #[doc = "3: CSGySR3"]
    VALUE4 = 3,
}
impl From<TRLS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRLS` reader - Conversion trigger interrupt line selection"]
pub struct TRLS_R(crate::FieldReader<u8, TRLS_A>);
impl TRLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRLS_A {
        match self.bits {
            0 => TRLS_A::VALUE1,
            1 => TRLS_A::VALUE2,
            2 => TRLS_A::VALUE3,
            3 => TRLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TRLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TRLS_A::VALUE4
    }
}
impl core::ops::Deref for TRLS_R {
    type Target = crate::FieldReader<u8, TRLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRLS` writer - Conversion trigger interrupt line selection"]
pub struct TRLS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRLS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Start/Stop trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1 = 0,
    #[doc = "1: CSGySR1"]
    VALUE2 = 1,
    #[doc = "2: CSGySR2"]
    VALUE3 = 2,
    #[doc = "3: CSGySR3"]
    VALUE4 = 3,
}
impl From<SSLS_A> for u8 {
    #[inline(always)]
    fn from(variant: SSLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SSLS` reader - Start/Stop trigger interrupt line selection"]
pub struct SSLS_R(crate::FieldReader<u8, SSLS_A>);
impl SSLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSLS_A {
        match self.bits {
            0 => SSLS_A::VALUE1,
            1 => SSLS_A::VALUE2,
            2 => SSLS_A::VALUE3,
            3 => SSLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == SSLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == SSLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == SSLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == SSLS_A::VALUE4
    }
}
impl core::ops::Deref for SSLS_R {
    type Target = crate::FieldReader<u8, SSLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSLS` writer - Start/Stop trigger interrupt line selection"]
pub struct SSLS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(SSLS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Shadow transfer done interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1 = 0,
    #[doc = "1: CSGySR1"]
    VALUE2 = 1,
    #[doc = "2: CSGySR2"]
    VALUE3 = 2,
    #[doc = "3: CSGySR3"]
    VALUE4 = 3,
}
impl From<STLS_A> for u8 {
    #[inline(always)]
    fn from(variant: STLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STLS` reader - Shadow transfer done interrupt line selection"]
pub struct STLS_R(crate::FieldReader<u8, STLS_A>);
impl STLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        STLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STLS_A {
        match self.bits {
            0 => STLS_A::VALUE1,
            1 => STLS_A::VALUE2,
            2 => STLS_A::VALUE3,
            3 => STLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == STLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == STLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == STLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == STLS_A::VALUE4
    }
}
impl core::ops::Deref for STLS_R {
    type Target = crate::FieldReader<u8, STLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STLS` writer - Shadow transfer done interrupt line selection"]
pub struct STLS_W<'a> {
    w: &'a mut W,
}
impl<'a> STLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(STLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(STLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(STLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(STLS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Comparator rise/fall interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRFLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1 = 0,
    #[doc = "1: CSGySR1"]
    VALUE2 = 1,
    #[doc = "2: CSGySR2"]
    VALUE3 = 2,
    #[doc = "3: CSGySR3"]
    VALUE4 = 3,
}
impl From<CRFLS_A> for u8 {
    #[inline(always)]
    fn from(variant: CRFLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRFLS` reader - Comparator rise/fall interrupt line selection"]
pub struct CRFLS_R(crate::FieldReader<u8, CRFLS_A>);
impl CRFLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRFLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRFLS_A {
        match self.bits {
            0 => CRFLS_A::VALUE1,
            1 => CRFLS_A::VALUE2,
            2 => CRFLS_A::VALUE3,
            3 => CRFLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CRFLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CRFLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CRFLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CRFLS_A::VALUE4
    }
}
impl core::ops::Deref for CRFLS_R {
    type Target = crate::FieldReader<u8, CRFLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRFLS` writer - Comparator rise/fall interrupt line selection"]
pub struct CRFLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRFLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRFLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CRFLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CRFLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CRFLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CRFLS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Comparator clamped state interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1 = 0,
    #[doc = "1: CSGySR1"]
    VALUE2 = 1,
    #[doc = "2: CSGySR2"]
    VALUE3 = 2,
    #[doc = "3: CSGySR3"]
    VALUE4 = 3,
}
impl From<CSLS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSLS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSLS` reader - Comparator clamped state interrupt line selection"]
pub struct CSLS_R(crate::FieldReader<u8, CSLS_A>);
impl CSLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSLS_A {
        match self.bits {
            0 => CSLS_A::VALUE1,
            1 => CSLS_A::VALUE2,
            2 => CSLS_A::VALUE3,
            3 => CSLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CSLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CSLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CSLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CSLS_A::VALUE4
    }
}
impl core::ops::Deref for CSLS_R {
    type Target = crate::FieldReader<u8, CSLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSLS` writer - Comparator clamped state interrupt line selection"]
pub struct CSLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSLS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CSLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CSLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CSLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CSLS_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline(always)]
    pub fn vls1s(&self) -> VLS1S_R {
        VLS1S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline(always)]
    pub fn vls2s(&self) -> VLS2S_R {
        VLS2S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline(always)]
    pub fn trls(&self) -> TRLS_R {
        TRLS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline(always)]
    pub fn ssls(&self) -> SSLS_R {
        SSLS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline(always)]
    pub fn stls(&self) -> STLS_R {
        STLS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline(always)]
    pub fn crfls(&self) -> CRFLS_R {
        CRFLS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline(always)]
    pub fn csls(&self) -> CSLS_R {
        CSLS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline(always)]
    pub fn vls1s(&mut self) -> VLS1S_W {
        VLS1S_W { w: self }
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline(always)]
    pub fn vls2s(&mut self) -> VLS2S_W {
        VLS2S_W { w: self }
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline(always)]
    pub fn trls(&mut self) -> TRLS_W {
        TRLS_W { w: self }
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline(always)]
    pub fn ssls(&mut self) -> SSLS_W {
        SSLS_W { w: self }
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline(always)]
    pub fn stls(&mut self) -> STLS_W {
        STLS_W { w: self }
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline(always)]
    pub fn crfls(&mut self) -> CRFLS_W {
        CRFLS_W { w: self }
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline(always)]
    pub fn csls(&mut self) -> CSLS_W {
        CSLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Service request line selector\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs](index.html) module"]
pub struct SRS_SPEC;
impl crate::RegisterSpec for SRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srs::R](R) reader structure"]
impl crate::Readable for SRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srs::W](W) writer structure"]
impl crate::Writable for SRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRS to value 0"]
impl crate::Resettable for SRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
