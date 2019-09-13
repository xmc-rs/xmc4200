#[doc = "Reader of register SRS"]
pub type R = crate::R<u32, super::SRS>;
#[doc = "Writer for register SRS"]
pub type W = crate::W<u32, super::SRS>;
#[doc = "Register SRS `reset()`'s with value 0"]
impl crate::ResetValue for super::SRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS1S_A {
    #[doc = "0: CSGySR0"]
    VALUE1,
    #[doc = "1: CSGySR1"]
    VALUE2,
    #[doc = "2: CSGySR2"]
    VALUE3,
    #[doc = "3: CSGySR3"]
    VALUE4,
}
impl From<VLS1S_A> for u8 {
    #[inline(always)]
    fn from(variant: VLS1S_A) -> Self {
        match variant {
            VLS1S_A::VALUE1 => 0,
            VLS1S_A::VALUE2 => 1,
            VLS1S_A::VALUE3 => 2,
            VLS1S_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `VLS1S`"]
pub type VLS1S_R = crate::R<u8, VLS1S_A>;
impl VLS1S_R {
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
        *self == VLS1S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VLS1S_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VLS1S_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VLS1S_A::VALUE4
    }
}
#[doc = "Write proxy for field `VLS1S`"]
pub struct VLS1S_W<'a> {
    w: &'a mut W,
}
impl<'a> VLS1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLS1S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLS2S_A {
    #[doc = "0: CSGySR0"]
    VALUE1,
    #[doc = "1: CSGySR1"]
    VALUE2,
    #[doc = "2: CSGySR2"]
    VALUE3,
    #[doc = "3: CSGySR3"]
    VALUE4,
}
impl From<VLS2S_A> for u8 {
    #[inline(always)]
    fn from(variant: VLS2S_A) -> Self {
        match variant {
            VLS2S_A::VALUE1 => 0,
            VLS2S_A::VALUE2 => 1,
            VLS2S_A::VALUE3 => 2,
            VLS2S_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `VLS2S`"]
pub type VLS2S_R = crate::R<u8, VLS2S_A>;
impl VLS2S_R {
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
        *self == VLS2S_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VLS2S_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VLS2S_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VLS2S_A::VALUE4
    }
}
#[doc = "Write proxy for field `VLS2S`"]
pub struct VLS2S_W<'a> {
    w: &'a mut W,
}
impl<'a> VLS2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLS2S_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Conversion trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1,
    #[doc = "1: CSGySR1"]
    VALUE2,
    #[doc = "2: CSGySR2"]
    VALUE3,
    #[doc = "3: CSGySR3"]
    VALUE4,
}
impl From<TRLS_A> for u8 {
    #[inline(always)]
    fn from(variant: TRLS_A) -> Self {
        match variant {
            TRLS_A::VALUE1 => 0,
            TRLS_A::VALUE2 => 1,
            TRLS_A::VALUE3 => 2,
            TRLS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `TRLS`"]
pub type TRLS_R = crate::R<u8, TRLS_A>;
impl TRLS_R {
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
        *self == TRLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRLS_A::VALUE4
    }
}
#[doc = "Write proxy for field `TRLS`"]
pub struct TRLS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Start/Stop trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1,
    #[doc = "1: CSGySR1"]
    VALUE2,
    #[doc = "2: CSGySR2"]
    VALUE3,
    #[doc = "3: CSGySR3"]
    VALUE4,
}
impl From<SSLS_A> for u8 {
    #[inline(always)]
    fn from(variant: SSLS_A) -> Self {
        match variant {
            SSLS_A::VALUE1 => 0,
            SSLS_A::VALUE2 => 1,
            SSLS_A::VALUE3 => 2,
            SSLS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `SSLS`"]
pub type SSLS_R = crate::R<u8, SSLS_A>;
impl SSLS_R {
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
        *self == SSLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SSLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SSLS_A::VALUE4
    }
}
#[doc = "Write proxy for field `SSLS`"]
pub struct SSLS_W<'a> {
    w: &'a mut W,
}
impl<'a> SSLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Shadow transfer done interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1,
    #[doc = "1: CSGySR1"]
    VALUE2,
    #[doc = "2: CSGySR2"]
    VALUE3,
    #[doc = "3: CSGySR3"]
    VALUE4,
}
impl From<STLS_A> for u8 {
    #[inline(always)]
    fn from(variant: STLS_A) -> Self {
        match variant {
            STLS_A::VALUE1 => 0,
            STLS_A::VALUE2 => 1,
            STLS_A::VALUE3 => 2,
            STLS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `STLS`"]
pub type STLS_R = crate::R<u8, STLS_A>;
impl STLS_R {
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
        *self == STLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STLS_A::VALUE4
    }
}
#[doc = "Write proxy for field `STLS`"]
pub struct STLS_W<'a> {
    w: &'a mut W,
}
impl<'a> STLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Comparator rise/fall interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRFLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1,
    #[doc = "1: CSGySR1"]
    VALUE2,
    #[doc = "2: CSGySR2"]
    VALUE3,
    #[doc = "3: CSGySR3"]
    VALUE4,
}
impl From<CRFLS_A> for u8 {
    #[inline(always)]
    fn from(variant: CRFLS_A) -> Self {
        match variant {
            CRFLS_A::VALUE1 => 0,
            CRFLS_A::VALUE2 => 1,
            CRFLS_A::VALUE3 => 2,
            CRFLS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `CRFLS`"]
pub type CRFLS_R = crate::R<u8, CRFLS_A>;
impl CRFLS_R {
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
        *self == CRFLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CRFLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CRFLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CRFLS_A::VALUE4
    }
}
#[doc = "Write proxy for field `CRFLS`"]
pub struct CRFLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CRFLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRFLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Comparator clamped state interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSLS_A {
    #[doc = "0: CSGySR0"]
    VALUE1,
    #[doc = "1: CSGySR1"]
    VALUE2,
    #[doc = "2: CSGySR2"]
    VALUE3,
    #[doc = "3: CSGySR3"]
    VALUE4,
}
impl From<CSLS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSLS_A) -> Self {
        match variant {
            CSLS_A::VALUE1 => 0,
            CSLS_A::VALUE2 => 1,
            CSLS_A::VALUE3 => 2,
            CSLS_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `CSLS`"]
pub type CSLS_R = crate::R<u8, CSLS_A>;
impl CSLS_R {
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
        *self == CSLS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSLS_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CSLS_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CSLS_A::VALUE4
    }
}
#[doc = "Write proxy for field `CSLS`"]
pub struct CSLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSLS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSLS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
}
