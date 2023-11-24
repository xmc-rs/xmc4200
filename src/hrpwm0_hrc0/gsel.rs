#[doc = "Register `GSEL` reader"]
pub type R = crate::R<GSEL_SPEC>;
#[doc = "Register `GSEL` writer"]
pub type W = crate::W<GSEL_SPEC>;
#[doc = "Field `C0SS` reader - Source selector 0 comparator set configuration"]
pub type C0SS_R = crate::FieldReader<C0SS_A>;
#[doc = "Source selector 0 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0SS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG1 unit can be used as set source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3 = 2,
}
impl From<C0SS_A> for u8 {
    #[inline(always)]
    fn from(variant: C0SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0SS_A {
    type Ux = u8;
}
impl C0SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0SS_A> {
        match self.bits {
            0 => Some(C0SS_A::VALUE1),
            1 => Some(C0SS_A::VALUE2),
            2 => Some(C0SS_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0SS_A::VALUE1
    }
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0SS_A::VALUE2
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0SS_A::VALUE3
    }
}
#[doc = "Field `C0SS` writer - Source selector 0 comparator set configuration"]
pub type C0SS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, C0SS_A>;
impl<'a, REG> C0SS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0SS_A::VALUE1)
    }
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0SS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C0SS_A::VALUE3)
    }
}
#[doc = "Field `C0CS` reader - Source selector 0 comparator clear configuration"]
pub type C0CS_R = crate::FieldReader<C0CS_A>;
#[doc = "Source selector 0 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0CS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG1 unit can be used as clear source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3 = 2,
}
impl From<C0CS_A> for u8 {
    #[inline(always)]
    fn from(variant: C0CS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0CS_A {
    type Ux = u8;
}
impl C0CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0CS_A> {
        match self.bits {
            0 => Some(C0CS_A::VALUE1),
            1 => Some(C0CS_A::VALUE2),
            2 => Some(C0CS_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0CS_A::VALUE1
    }
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0CS_A::VALUE2
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0CS_A::VALUE3
    }
}
#[doc = "Field `C0CS` writer - Source selector 0 comparator clear configuration"]
pub type C0CS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, C0CS_A>;
impl<'a, REG> C0CS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0CS_A::VALUE1)
    }
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0CS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C0CS_A::VALUE3)
    }
}
#[doc = "Field `S0M` reader - Source selector 0 set configuration"]
pub type S0M_R = crate::FieldReader<S0M_A>;
#[doc = "Source selector 0 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0M_A {
    #[doc = "0: Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    VALUE2 = 1,
}
impl From<S0M_A> for u8 {
    #[inline(always)]
    fn from(variant: S0M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0M_A {
    type Ux = u8;
}
impl S0M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0M_A> {
        match self.bits {
            0 => Some(S0M_A::VALUE1),
            1 => Some(S0M_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0M_A::VALUE1
    }
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0M_A::VALUE2
    }
}
#[doc = "Field `S0M` writer - Source selector 0 set configuration"]
pub type S0M_W<'a, REG> = crate::FieldWriter<'a, REG, 2, S0M_A>;
impl<'a, REG> S0M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S0M_A::VALUE1)
    }
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S0M_A::VALUE2)
    }
}
#[doc = "Field `C0M` reader - Source selector 0 clear configuration"]
pub type C0M_R = crate::FieldReader<C0M_A>;
#[doc = "Source selector 0 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0M_A {
    #[doc = "0: Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    VALUE2 = 1,
}
impl From<C0M_A> for u8 {
    #[inline(always)]
    fn from(variant: C0M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0M_A {
    type Ux = u8;
}
impl C0M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0M_A> {
        match self.bits {
            0 => Some(C0M_A::VALUE1),
            1 => Some(C0M_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0M_A::VALUE1
    }
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0M_A::VALUE2
    }
}
#[doc = "Field `C0M` writer - Source selector 0 clear configuration"]
pub type C0M_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C0M_A>;
impl<'a, REG> C0M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0M_A::VALUE1)
    }
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0M_A::VALUE2)
    }
}
#[doc = "Field `S0ES` reader - Source selector 0 set edge configuration"]
pub type S0ES_R = crate::FieldReader<S0ES_A>;
#[doc = "Source selector 0 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0ES_A {
    #[doc = "0: Generation of the set signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Set signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<S0ES_A> for u8 {
    #[inline(always)]
    fn from(variant: S0ES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0ES_A {
    type Ux = u8;
}
impl S0ES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0ES_A {
        match self.bits {
            0 => S0ES_A::VALUE1,
            1 => S0ES_A::VALUE2,
            2 => S0ES_A::VALUE3,
            3 => S0ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0ES_A::VALUE1
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0ES_A::VALUE2
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == S0ES_A::VALUE3
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == S0ES_A::VALUE4
    }
}
#[doc = "Field `S0ES` writer - Source selector 0 set edge configuration"]
pub type S0ES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, S0ES_A>;
impl<'a, REG> S0ES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S0ES_A::VALUE1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S0ES_A::VALUE2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(S0ES_A::VALUE3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(S0ES_A::VALUE4)
    }
}
#[doc = "Field `C0ES` reader - Source selector 0 clear edge configuration"]
pub type C0ES_R = crate::FieldReader<C0ES_A>;
#[doc = "Source selector 0 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0ES_A {
    #[doc = "0: Generation of the clear signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Clear signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<C0ES_A> for u8 {
    #[inline(always)]
    fn from(variant: C0ES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0ES_A {
    type Ux = u8;
}
impl C0ES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0ES_A {
        match self.bits {
            0 => C0ES_A::VALUE1,
            1 => C0ES_A::VALUE2,
            2 => C0ES_A::VALUE3,
            3 => C0ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0ES_A::VALUE1
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0ES_A::VALUE2
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0ES_A::VALUE3
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C0ES_A::VALUE4
    }
}
#[doc = "Field `C0ES` writer - Source selector 0 clear edge configuration"]
pub type C0ES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C0ES_A>;
impl<'a, REG> C0ES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0ES_A::VALUE1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0ES_A::VALUE2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C0ES_A::VALUE3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C0ES_A::VALUE4)
    }
}
#[doc = "Field `C1SS` reader - Source selector 1 comparator set configuration"]
pub type C1SS_R = crate::FieldReader<C1SS_A>;
#[doc = "Source selector 1 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1SS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as set source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as set source for the output latch"]
    VALUE3 = 2,
}
impl From<C1SS_A> for u8 {
    #[inline(always)]
    fn from(variant: C1SS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1SS_A {
    type Ux = u8;
}
impl C1SS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1SS_A> {
        match self.bits {
            0 => Some(C1SS_A::VALUE1),
            1 => Some(C1SS_A::VALUE2),
            2 => Some(C1SS_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1SS_A::VALUE1
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1SS_A::VALUE2
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1SS_A::VALUE3
    }
}
#[doc = "Field `C1SS` writer - Source selector 1 comparator set configuration"]
pub type C1SS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, C1SS_A>;
impl<'a, REG> C1SS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1SS_A::VALUE1)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1SS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C1SS_A::VALUE3)
    }
}
#[doc = "Field `C1CS` reader - Source selector 1 comparator clear configuration"]
pub type C1CS_R = crate::FieldReader<C1CS_A>;
#[doc = "Source selector 1 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1CS_A {
    #[doc = "0: CMP output of CSG0 unit can be used as clear source for the output latch"]
    VALUE1 = 0,
    #[doc = "1: CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as clear source for the output latch"]
    VALUE3 = 2,
}
impl From<C1CS_A> for u8 {
    #[inline(always)]
    fn from(variant: C1CS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1CS_A {
    type Ux = u8;
}
impl C1CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1CS_A> {
        match self.bits {
            0 => Some(C1CS_A::VALUE1),
            1 => Some(C1CS_A::VALUE2),
            2 => Some(C1CS_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1CS_A::VALUE1
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1CS_A::VALUE2
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1CS_A::VALUE3
    }
}
#[doc = "Field `C1CS` writer - Source selector 1 comparator clear configuration"]
pub type C1CS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, C1CS_A>;
impl<'a, REG> C1CS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1CS_A::VALUE1)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1CS_A::VALUE2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C1CS_A::VALUE3)
    }
}
#[doc = "Field `S1M` reader - Source selector 1 set configuration"]
pub type S1M_R = crate::FieldReader<S1M_A>;
#[doc = "Source selector 1 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1M_A {
    #[doc = "0: Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    VALUE2 = 1,
}
impl From<S1M_A> for u8 {
    #[inline(always)]
    fn from(variant: S1M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1M_A {
    type Ux = u8;
}
impl S1M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1M_A> {
        match self.bits {
            0 => Some(S1M_A::VALUE1),
            1 => Some(S1M_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1M_A::VALUE1
    }
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1M_A::VALUE2
    }
}
#[doc = "Field `S1M` writer - Source selector 1 set configuration"]
pub type S1M_W<'a, REG> = crate::FieldWriter<'a, REG, 2, S1M_A>;
impl<'a, REG> S1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S1M_A::VALUE1)
    }
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S1M_A::VALUE2)
    }
}
#[doc = "Field `C1M` reader - Source selector 1 clear configuration"]
pub type C1M_R = crate::FieldReader<C1M_A>;
#[doc = "Source selector 1 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1M_A {
    #[doc = "0: Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    VALUE1 = 0,
    #[doc = "1: Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    VALUE2 = 1,
}
impl From<C1M_A> for u8 {
    #[inline(always)]
    fn from(variant: C1M_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1M_A {
    type Ux = u8;
}
impl C1M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1M_A> {
        match self.bits {
            0 => Some(C1M_A::VALUE1),
            1 => Some(C1M_A::VALUE2),
            _ => None,
        }
    }
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1M_A::VALUE1
    }
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1M_A::VALUE2
    }
}
#[doc = "Field `C1M` writer - Source selector 1 clear configuration"]
pub type C1M_W<'a, REG> = crate::FieldWriter<'a, REG, 2, C1M_A>;
impl<'a, REG> C1M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1M_A::VALUE1)
    }
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1M_A::VALUE2)
    }
}
#[doc = "Field `S1ES` reader - Source selector 1 set edge configuration"]
pub type S1ES_R = crate::FieldReader<S1ES_A>;
#[doc = "Source selector 1 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1ES_A {
    #[doc = "0: Generation of the set signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Set signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Set signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Set signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<S1ES_A> for u8 {
    #[inline(always)]
    fn from(variant: S1ES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1ES_A {
    type Ux = u8;
}
impl S1ES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1ES_A {
        match self.bits {
            0 => S1ES_A::VALUE1,
            1 => S1ES_A::VALUE2,
            2 => S1ES_A::VALUE3,
            3 => S1ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1ES_A::VALUE1
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1ES_A::VALUE2
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == S1ES_A::VALUE3
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == S1ES_A::VALUE4
    }
}
#[doc = "Field `S1ES` writer - Source selector 1 set edge configuration"]
pub type S1ES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, S1ES_A>;
impl<'a, REG> S1ES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S1ES_A::VALUE1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S1ES_A::VALUE2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(S1ES_A::VALUE3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(S1ES_A::VALUE4)
    }
}
#[doc = "Field `C1ES` reader - Source selector 1 clear edge configuration"]
pub type C1ES_R = crate::FieldReader<C1ES_A>;
#[doc = "Source selector 1 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1ES_A {
    #[doc = "0: Generation of the clear signal is disabled"]
    VALUE1 = 0,
    #[doc = "1: Clear signal is generated on a LOW to HIGH transition of the selected input"]
    VALUE2 = 1,
    #[doc = "2: Clear signal is generated on a HIGH to LOW transition of the selected input"]
    VALUE3 = 2,
    #[doc = "3: Clear signal is generated on both transitions of the selected input"]
    VALUE4 = 3,
}
impl From<C1ES_A> for u8 {
    #[inline(always)]
    fn from(variant: C1ES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1ES_A {
    type Ux = u8;
}
impl C1ES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1ES_A {
        match self.bits {
            0 => C1ES_A::VALUE1,
            1 => C1ES_A::VALUE2,
            2 => C1ES_A::VALUE3,
            3 => C1ES_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1ES_A::VALUE1
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1ES_A::VALUE2
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1ES_A::VALUE3
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C1ES_A::VALUE4
    }
}
#[doc = "Field `C1ES` writer - Source selector 1 clear edge configuration"]
pub type C1ES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C1ES_A>;
impl<'a, REG> C1ES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1ES_A::VALUE1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1ES_A::VALUE2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C1ES_A::VALUE3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C1ES_A::VALUE4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    pub fn c0ss(&self) -> C0SS_R {
        C0SS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    pub fn c0cs(&self) -> C0CS_R {
        C0CS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    pub fn s0m(&self) -> S0M_R {
        S0M_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    pub fn c0m(&self) -> C0M_R {
        C0M_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    pub fn s0es(&self) -> S0ES_R {
        S0ES_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    pub fn c0es(&self) -> C0ES_R {
        C0ES_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    pub fn c1ss(&self) -> C1SS_R {
        C1SS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    pub fn c1cs(&self) -> C1CS_R {
        C1CS_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    pub fn s1m(&self) -> S1M_R {
        S1M_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    pub fn c1m(&self) -> C1M_R {
        C1M_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    pub fn s1es(&self) -> S1ES_R {
        S1ES_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    pub fn c1es(&self) -> C1ES_R {
        C1ES_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0ss(&mut self) -> C0SS_W<GSEL_SPEC> {
        C0SS_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0cs(&mut self) -> C0CS_W<GSEL_SPEC> {
        C0CS_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s0m(&mut self) -> S0M_W<GSEL_SPEC> {
        S0M_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0m(&mut self) -> C0M_W<GSEL_SPEC> {
        C0M_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s0es(&mut self) -> S0ES_W<GSEL_SPEC> {
        S0ES_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0es(&mut self) -> C0ES_W<GSEL_SPEC> {
        C0ES_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1ss(&mut self) -> C1SS_W<GSEL_SPEC> {
        C1SS_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1cs(&mut self) -> C1CS_W<GSEL_SPEC> {
        C1CS_W::new(self, 19)
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s1m(&mut self) -> S1M_W<GSEL_SPEC> {
        S1M_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1m(&mut self) -> C1M_W<GSEL_SPEC> {
        C1M_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s1es(&mut self) -> S1ES_W<GSEL_SPEC> {
        S1ES_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1es(&mut self) -> C1ES_W<GSEL_SPEC> {
        C1ES_W::new(self, 28)
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
#[doc = "HRC global control selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GSEL_SPEC;
impl crate::RegisterSpec for GSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsel::R`](R) reader structure"]
impl crate::Readable for GSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gsel::W`](W) writer structure"]
impl crate::Writable for GSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GSEL to value 0"]
impl crate::Resettable for GSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
