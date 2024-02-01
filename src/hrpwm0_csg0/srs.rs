#[doc = "Register `SRS` reader"]
pub type R = crate::R<SRS_SPEC>;
#[doc = "Register `SRS` writer"]
pub type W = crate::W<SRS_SPEC>;
#[doc = "Field `VLS1S` reader - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
pub type VLS1S_R = crate::FieldReader<VLS1S_A>;
#[doc = "Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for VLS1S_A {
    type Ux = u8;
}
impl VLS1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLS1S_A {
        match self.bits {
            0 => VLS1S_A::VALUE1,
            1 => VLS1S_A::VALUE2,
            2 => VLS1S_A::VALUE3,
            3 => VLS1S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VLS1S_A::VALUE1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VLS1S_A::VALUE2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VLS1S_A::VALUE3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VLS1S_A::VALUE4
    }
}
#[doc = "Field `VLS1S` writer - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
pub type VLS1S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VLS1S_A>;
impl<'a, REG> VLS1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VLS1S_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VLS1S_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(VLS1S_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(VLS1S_A::VALUE4)
    }
}
#[doc = "Field `VLS2S` reader - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
pub type VLS2S_R = crate::FieldReader<VLS2S_A>;
#[doc = "Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for VLS2S_A {
    type Ux = u8;
}
impl VLS2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VLS2S_A {
        match self.bits {
            0 => VLS2S_A::VALUE1,
            1 => VLS2S_A::VALUE2,
            2 => VLS2S_A::VALUE3,
            3 => VLS2S_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VLS2S_A::VALUE1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VLS2S_A::VALUE2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == VLS2S_A::VALUE3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == VLS2S_A::VALUE4
    }
}
#[doc = "Field `VLS2S` writer - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
pub type VLS2S_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VLS2S_A>;
impl<'a, REG> VLS2S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VLS2S_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VLS2S_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(VLS2S_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(VLS2S_A::VALUE4)
    }
}
#[doc = "Field `TRLS` reader - Conversion trigger interrupt line selection"]
pub type TRLS_R = crate::FieldReader<TRLS_A>;
#[doc = "Conversion trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for TRLS_A {
    type Ux = u8;
}
impl TRLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRLS_A {
        match self.bits {
            0 => TRLS_A::VALUE1,
            1 => TRLS_A::VALUE2,
            2 => TRLS_A::VALUE3,
            3 => TRLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TRLS_A::VALUE1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TRLS_A::VALUE2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TRLS_A::VALUE3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TRLS_A::VALUE4
    }
}
#[doc = "Field `TRLS` writer - Conversion trigger interrupt line selection"]
pub type TRLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TRLS_A>;
impl<'a, REG> TRLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TRLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TRLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TRLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TRLS_A::VALUE4)
    }
}
#[doc = "Field `SSLS` reader - Start/Stop trigger interrupt line selection"]
pub type SSLS_R = crate::FieldReader<SSLS_A>;
#[doc = "Start/Stop trigger interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for SSLS_A {
    type Ux = u8;
}
impl SSLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSLS_A {
        match self.bits {
            0 => SSLS_A::VALUE1,
            1 => SSLS_A::VALUE2,
            2 => SSLS_A::VALUE3,
            3 => SSLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SSLS_A::VALUE1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SSLS_A::VALUE2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SSLS_A::VALUE3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == SSLS_A::VALUE4
    }
}
#[doc = "Field `SSLS` writer - Start/Stop trigger interrupt line selection"]
pub type SSLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SSLS_A>;
impl<'a, REG> SSLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SSLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SSLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SSLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(SSLS_A::VALUE4)
    }
}
#[doc = "Field `STLS` reader - Shadow transfer done interrupt line selection"]
pub type STLS_R = crate::FieldReader<STLS_A>;
#[doc = "Shadow transfer done interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for STLS_A {
    type Ux = u8;
}
impl STLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STLS_A {
        match self.bits {
            0 => STLS_A::VALUE1,
            1 => STLS_A::VALUE2,
            2 => STLS_A::VALUE3,
            3 => STLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == STLS_A::VALUE1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == STLS_A::VALUE2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == STLS_A::VALUE3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == STLS_A::VALUE4
    }
}
#[doc = "Field `STLS` writer - Shadow transfer done interrupt line selection"]
pub type STLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, STLS_A>;
impl<'a, REG> STLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(STLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(STLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(STLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(STLS_A::VALUE4)
    }
}
#[doc = "Field `CRFLS` reader - Comparator rise/fall interrupt line selection"]
pub type CRFLS_R = crate::FieldReader<CRFLS_A>;
#[doc = "Comparator rise/fall interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CRFLS_A {
    type Ux = u8;
}
impl CRFLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRFLS_A {
        match self.bits {
            0 => CRFLS_A::VALUE1,
            1 => CRFLS_A::VALUE2,
            2 => CRFLS_A::VALUE3,
            3 => CRFLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CRFLS_A::VALUE1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CRFLS_A::VALUE2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CRFLS_A::VALUE3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CRFLS_A::VALUE4
    }
}
#[doc = "Field `CRFLS` writer - Comparator rise/fall interrupt line selection"]
pub type CRFLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CRFLS_A>;
impl<'a, REG> CRFLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRFLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRFLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CRFLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CRFLS_A::VALUE4)
    }
}
#[doc = "Field `CSLS` reader - Comparator clamped state interrupt line selection"]
pub type CSLS_R = crate::FieldReader<CSLS_A>;
#[doc = "Comparator clamped state interrupt line selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl crate::FieldSpec for CSLS_A {
    type Ux = u8;
}
impl CSLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSLS_A {
        match self.bits {
            0 => CSLS_A::VALUE1,
            1 => CSLS_A::VALUE2,
            2 => CSLS_A::VALUE3,
            3 => CSLS_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CSLS_A::VALUE1
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CSLS_A::VALUE2
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == CSLS_A::VALUE3
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == CSLS_A::VALUE4
    }
}
#[doc = "Field `CSLS` writer - Comparator clamped state interrupt line selection"]
pub type CSLS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CSLS_A>;
impl<'a, REG> CSLS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSGySR0"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CSLS_A::VALUE1)
    }
    #[doc = "CSGySR1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CSLS_A::VALUE2)
    }
    #[doc = "CSGySR2"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(CSLS_A::VALUE3)
    }
    #[doc = "CSGySR3"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(CSLS_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline(always)]
    pub fn vls1s(&self) -> VLS1S_R {
        VLS1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline(always)]
    pub fn vls2s(&self) -> VLS2S_R {
        VLS2S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline(always)]
    pub fn trls(&self) -> TRLS_R {
        TRLS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline(always)]
    pub fn ssls(&self) -> SSLS_R {
        SSLS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline(always)]
    pub fn stls(&self) -> STLS_R {
        STLS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline(always)]
    pub fn crfls(&self) -> CRFLS_R {
        CRFLS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline(always)]
    pub fn csls(&self) -> CSLS_R {
        CSLS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Value switch from CSGyDSV1 to CSGyDSV2 interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn vls1s(&mut self) -> VLS1S_W<SRS_SPEC> {
        VLS1S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Value switch from CSGyDSV2 to CSGyDSV1 interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn vls2s(&mut self) -> VLS2S_W<SRS_SPEC> {
        VLS2S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Conversion trigger interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn trls(&mut self) -> TRLS_W<SRS_SPEC> {
        TRLS_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Start/Stop trigger interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn ssls(&mut self) -> SSLS_W<SRS_SPEC> {
        SSLS_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Shadow transfer done interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn stls(&mut self) -> STLS_W<SRS_SPEC> {
        STLS_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Comparator rise/fall interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn crfls(&mut self) -> CRFLS_W<SRS_SPEC> {
        CRFLS_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Comparator clamped state interrupt line selection"]
    #[inline(always)]
    #[must_use]
    pub fn csls(&mut self) -> CSLS_W<SRS_SPEC> {
        CSLS_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Service request line selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRS_SPEC;
impl crate::RegisterSpec for SRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srs::R`](R) reader structure"]
impl crate::Readable for SRS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srs::W`](W) writer structure"]
impl crate::Writable for SRS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRS to value 0"]
impl crate::Resettable for SRS_SPEC {
    const RESET_VALUE: u32 = 0;
}
