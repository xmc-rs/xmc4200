#[doc = "Register `GSEL` reader"]
pub type R = crate::R<GselSpec>;
#[doc = "Register `GSEL` writer"]
pub type W = crate::W<GselSpec>;
#[doc = "Source selector 0 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0ss {
    #[doc = "0: CMP output of CSG0 unit can be used as set source for the output latch"]
    Value1 = 0,
    #[doc = "1: CMP output of CSG1 unit can be used as set source for the output latch"]
    Value2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as set source for the output latch"]
    Value3 = 2,
}
impl From<C0ss> for u8 {
    #[inline(always)]
    fn from(variant: C0ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0ss {
    type Ux = u8;
}
#[doc = "Field `C0SS` reader - Source selector 0 comparator set configuration"]
pub type C0ssR = crate::FieldReader<C0ss>;
impl C0ssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0ss> {
        match self.bits {
            0 => Some(C0ss::Value1),
            1 => Some(C0ss::Value2),
            2 => Some(C0ss::Value3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0ss::Value1
    }
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0ss::Value2
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0ss::Value3
    }
}
#[doc = "Field `C0SS` writer - Source selector 0 comparator set configuration"]
pub type C0ssW<'a, REG> = crate::FieldWriter<'a, REG, 3, C0ss>;
impl<'a, REG> C0ssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0ss::Value1)
    }
    #[doc = "CMP output of CSG1 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0ss::Value2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C0ss::Value3)
    }
}
#[doc = "Source selector 0 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0cs {
    #[doc = "0: CMP output of CSG0 unit can be used as clear source for the output latch"]
    Value1 = 0,
    #[doc = "1: CMP output of CSG1 unit can be used as clear source for the output latch"]
    Value2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as clear source for the output latch"]
    Value3 = 2,
}
impl From<C0cs> for u8 {
    #[inline(always)]
    fn from(variant: C0cs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0cs {
    type Ux = u8;
}
#[doc = "Field `C0CS` reader - Source selector 0 comparator clear configuration"]
pub type C0csR = crate::FieldReader<C0cs>;
impl C0csR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0cs> {
        match self.bits {
            0 => Some(C0cs::Value1),
            1 => Some(C0cs::Value2),
            2 => Some(C0cs::Value3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0cs::Value1
    }
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0cs::Value2
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0cs::Value3
    }
}
#[doc = "Field `C0CS` writer - Source selector 0 comparator clear configuration"]
pub type C0csW<'a, REG> = crate::FieldWriter<'a, REG, 3, C0cs>;
impl<'a, REG> C0csW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0cs::Value1)
    }
    #[doc = "CMP output of CSG1 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0cs::Value2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C0cs::Value3)
    }
}
#[doc = "Source selector 0 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0m {
    #[doc = "0: Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    Value1 = 0,
    #[doc = "1: Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    Value2 = 1,
}
impl From<S0m> for u8 {
    #[inline(always)]
    fn from(variant: S0m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0m {
    type Ux = u8;
}
#[doc = "Field `S0M` reader - Source selector 0 set configuration"]
pub type S0mR = crate::FieldReader<S0m>;
impl S0mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0m> {
        match self.bits {
            0 => Some(S0m::Value1),
            1 => Some(S0m::Value2),
            _ => None,
        }
    }
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0m::Value1
    }
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0m::Value2
    }
}
#[doc = "Field `S0M` writer - Source selector 0 set configuration"]
pub type S0mW<'a, REG> = crate::FieldWriter<'a, REG, 2, S0m>;
impl<'a, REG> S0mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S0m::Value1)
    }
    #[doc = "Set from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0SS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S0m::Value2)
    }
}
#[doc = "Source selector 0 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0m {
    #[doc = "0: Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    Value1 = 0,
    #[doc = "1: Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    Value2 = 1,
}
impl From<C0m> for u8 {
    #[inline(always)]
    fn from(variant: C0m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0m {
    type Ux = u8;
}
#[doc = "Field `C0M` reader - Source selector 0 clear configuration"]
pub type C0mR = crate::FieldReader<C0m>;
impl C0mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C0m> {
        match self.bits {
            0 => Some(C0m::Value1),
            1 => Some(C0m::Value2),
            _ => None,
        }
    }
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0m::Value1
    }
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0m::Value2
    }
}
#[doc = "Field `C0M` writer - Source selector 0 clear configuration"]
pub type C0mW<'a, REG> = crate::FieldWriter<'a, REG, 2, C0m>;
impl<'a, REG> C0mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clear from source selector 0 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0m::Value1)
    }
    #[doc = "Clear from source selector 0 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C0CS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0m::Value2)
    }
}
#[doc = "Source selector 0 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0es {
    #[doc = "0: Generation of the set signal is disabled"]
    Value1 = 0,
    #[doc = "1: Set signal is generated on a LOW to HIGH transition of the selected input"]
    Value2 = 1,
    #[doc = "2: Set signal is generated on a HIGH to LOW transition of the selected input"]
    Value3 = 2,
    #[doc = "3: Set signal is generated on both transitions of the selected input"]
    Value4 = 3,
}
impl From<S0es> for u8 {
    #[inline(always)]
    fn from(variant: S0es) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0es {
    type Ux = u8;
}
#[doc = "Field `S0ES` reader - Source selector 0 set edge configuration"]
pub type S0esR = crate::FieldReader<S0es>;
impl S0esR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S0es {
        match self.bits {
            0 => S0es::Value1,
            1 => S0es::Value2,
            2 => S0es::Value3,
            3 => S0es::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S0es::Value1
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S0es::Value2
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == S0es::Value3
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == S0es::Value4
    }
}
#[doc = "Field `S0ES` writer - Source selector 0 set edge configuration"]
pub type S0esW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, S0es>;
impl<'a, REG> S0esW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S0es::Value1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S0es::Value2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(S0es::Value3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(S0es::Value4)
    }
}
#[doc = "Source selector 0 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C0es {
    #[doc = "0: Generation of the clear signal is disabled"]
    Value1 = 0,
    #[doc = "1: Clear signal is generated on a LOW to HIGH transition of the selected input"]
    Value2 = 1,
    #[doc = "2: Clear signal is generated on a HIGH to LOW transition of the selected input"]
    Value3 = 2,
    #[doc = "3: Clear signal is generated on both transitions of the selected input"]
    Value4 = 3,
}
impl From<C0es> for u8 {
    #[inline(always)]
    fn from(variant: C0es) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C0es {
    type Ux = u8;
}
#[doc = "Field `C0ES` reader - Source selector 0 clear edge configuration"]
pub type C0esR = crate::FieldReader<C0es>;
impl C0esR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0es {
        match self.bits {
            0 => C0es::Value1,
            1 => C0es::Value2,
            2 => C0es::Value3,
            3 => C0es::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C0es::Value1
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C0es::Value2
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C0es::Value3
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C0es::Value4
    }
}
#[doc = "Field `C0ES` writer - Source selector 0 clear edge configuration"]
pub type C0esW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C0es>;
impl<'a, REG> C0esW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C0es::Value1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C0es::Value2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C0es::Value3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C0es::Value4)
    }
}
#[doc = "Source selector 1 comparator set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1ss {
    #[doc = "0: CMP output of CSG0 unit can be used as set source for the output latch"]
    Value1 = 0,
    #[doc = "1: CMP output of CSG2 unit can be used as set source for the output latch"]
    Value2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as set source for the output latch"]
    Value3 = 2,
}
impl From<C1ss> for u8 {
    #[inline(always)]
    fn from(variant: C1ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1ss {
    type Ux = u8;
}
#[doc = "Field `C1SS` reader - Source selector 1 comparator set configuration"]
pub type C1ssR = crate::FieldReader<C1ss>;
impl C1ssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1ss> {
        match self.bits {
            0 => Some(C1ss::Value1),
            1 => Some(C1ss::Value2),
            2 => Some(C1ss::Value3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1ss::Value1
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1ss::Value2
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1ss::Value3
    }
}
#[doc = "Field `C1SS` writer - Source selector 1 comparator set configuration"]
pub type C1ssW<'a, REG> = crate::FieldWriter<'a, REG, 3, C1ss>;
impl<'a, REG> C1ssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1ss::Value1)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1ss::Value2)
    }
    #[doc = "CMP output of CSG2 unit can be used as set source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C1ss::Value3)
    }
}
#[doc = "Source selector 1 comparator clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1cs {
    #[doc = "0: CMP output of CSG0 unit can be used as clear source for the output latch"]
    Value1 = 0,
    #[doc = "1: CMP output of CSG2 unit can be used as clear source for the output latch"]
    Value2 = 1,
    #[doc = "2: CMP output of CSG2 unit can be used as clear source for the output latch"]
    Value3 = 2,
}
impl From<C1cs> for u8 {
    #[inline(always)]
    fn from(variant: C1cs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1cs {
    type Ux = u8;
}
#[doc = "Field `C1CS` reader - Source selector 1 comparator clear configuration"]
pub type C1csR = crate::FieldReader<C1cs>;
impl C1csR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1cs> {
        match self.bits {
            0 => Some(C1cs::Value1),
            1 => Some(C1cs::Value2),
            2 => Some(C1cs::Value3),
            _ => None,
        }
    }
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1cs::Value1
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1cs::Value2
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1cs::Value3
    }
}
#[doc = "Field `C1CS` writer - Source selector 1 comparator clear configuration"]
pub type C1csW<'a, REG> = crate::FieldWriter<'a, REG, 3, C1cs>;
impl<'a, REG> C1csW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CMP output of CSG0 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1cs::Value1)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1cs::Value2)
    }
    #[doc = "CMP output of CSG2 unit can be used as clear source for the output latch"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C1cs::Value3)
    }
}
#[doc = "Source selector 1 set configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1m {
    #[doc = "0: Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    Value1 = 0,
    #[doc = "1: Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    Value2 = 1,
}
impl From<S1m> for u8 {
    #[inline(always)]
    fn from(variant: S1m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1m {
    type Ux = u8;
}
#[doc = "Field `S1M` reader - Source selector 1 set configuration"]
pub type S1mR = crate::FieldReader<S1m>;
impl S1mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1m> {
        match self.bits {
            0 => Some(S1m::Value1),
            1 => Some(S1m::Value2),
            _ => None,
        }
    }
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1m::Value1
    }
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1m::Value2
    }
}
#[doc = "Field `S1M` writer - Source selector 1 set configuration"]
pub type S1mW<'a, REG> = crate::FieldWriter<'a, REG, 2, S1m>;
impl<'a, REG> S1mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S1m::Value1)
    }
    #[doc = "Set from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1SS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S1m::Value2)
    }
}
#[doc = "Source selector 1 clear configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1m {
    #[doc = "0: Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    Value1 = 0,
    #[doc = "1: Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    Value2 = 1,
}
impl From<C1m> for u8 {
    #[inline(always)]
    fn from(variant: C1m) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1m {
    type Ux = u8;
}
#[doc = "Field `C1M` reader - Source selector 1 clear configuration"]
pub type C1mR = crate::FieldReader<C1m>;
impl C1mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1m> {
        match self.bits {
            0 => Some(C1m::Value1),
            1 => Some(C1m::Value2),
            _ => None,
        }
    }
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1m::Value1
    }
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1m::Value2
    }
}
#[doc = "Field `C1M` writer - Source selector 1 clear configuration"]
pub type C1mW<'a, REG> = crate::FieldWriter<'a, REG, 2, C1m>;
impl<'a, REG> C1mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clear from source selector 1 is controlled via the Capture/Compare Unit timer, CCSTy signal"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1m::Value1)
    }
    #[doc = "Clear from source selector 1 is controlled via the CMP output from the CSGy unit. Which unit is being used is configured via the C1CS field."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1m::Value2)
    }
}
#[doc = "Source selector 1 set edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1es {
    #[doc = "0: Generation of the set signal is disabled"]
    Value1 = 0,
    #[doc = "1: Set signal is generated on a LOW to HIGH transition of the selected input"]
    Value2 = 1,
    #[doc = "2: Set signal is generated on a HIGH to LOW transition of the selected input"]
    Value3 = 2,
    #[doc = "3: Set signal is generated on both transitions of the selected input"]
    Value4 = 3,
}
impl From<S1es> for u8 {
    #[inline(always)]
    fn from(variant: S1es) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1es {
    type Ux = u8;
}
#[doc = "Field `S1ES` reader - Source selector 1 set edge configuration"]
pub type S1esR = crate::FieldReader<S1es>;
impl S1esR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> S1es {
        match self.bits {
            0 => S1es::Value1,
            1 => S1es::Value2,
            2 => S1es::Value3,
            3 => S1es::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == S1es::Value1
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == S1es::Value2
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == S1es::Value3
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == S1es::Value4
    }
}
#[doc = "Field `S1ES` writer - Source selector 1 set edge configuration"]
pub type S1esW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, S1es>;
impl<'a, REG> S1esW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the set signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(S1es::Value1)
    }
    #[doc = "Set signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(S1es::Value2)
    }
    #[doc = "Set signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(S1es::Value3)
    }
    #[doc = "Set signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(S1es::Value4)
    }
}
#[doc = "Source selector 1 clear edge configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum C1es {
    #[doc = "0: Generation of the clear signal is disabled"]
    Value1 = 0,
    #[doc = "1: Clear signal is generated on a LOW to HIGH transition of the selected input"]
    Value2 = 1,
    #[doc = "2: Clear signal is generated on a HIGH to LOW transition of the selected input"]
    Value3 = 2,
    #[doc = "3: Clear signal is generated on both transitions of the selected input"]
    Value4 = 3,
}
impl From<C1es> for u8 {
    #[inline(always)]
    fn from(variant: C1es) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for C1es {
    type Ux = u8;
}
#[doc = "Field `C1ES` reader - Source selector 1 clear edge configuration"]
pub type C1esR = crate::FieldReader<C1es>;
impl C1esR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1es {
        match self.bits {
            0 => C1es::Value1,
            1 => C1es::Value2,
            2 => C1es::Value3,
            3 => C1es::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == C1es::Value1
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == C1es::Value2
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == C1es::Value3
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == C1es::Value4
    }
}
#[doc = "Field `C1ES` writer - Source selector 1 clear edge configuration"]
pub type C1esW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, C1es>;
impl<'a, REG> C1esW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generation of the clear signal is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(C1es::Value1)
    }
    #[doc = "Clear signal is generated on a LOW to HIGH transition of the selected input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(C1es::Value2)
    }
    #[doc = "Clear signal is generated on a HIGH to LOW transition of the selected input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(C1es::Value3)
    }
    #[doc = "Clear signal is generated on both transitions of the selected input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(C1es::Value4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    pub fn c0ss(&self) -> C0ssR {
        C0ssR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    pub fn c0cs(&self) -> C0csR {
        C0csR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    pub fn s0m(&self) -> S0mR {
        S0mR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    pub fn c0m(&self) -> C0mR {
        C0mR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    pub fn s0es(&self) -> S0esR {
        S0esR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    pub fn c0es(&self) -> C0esR {
        C0esR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    pub fn c1ss(&self) -> C1ssR {
        C1ssR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    pub fn c1cs(&self) -> C1csR {
        C1csR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    pub fn s1m(&self) -> S1mR {
        S1mR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    pub fn c1m(&self) -> C1mR {
        C1mR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    pub fn s1es(&self) -> S1esR {
        S1esR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    pub fn c1es(&self) -> C1esR {
        C1esR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source selector 0 comparator set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0ss(&mut self) -> C0ssW<GselSpec> {
        C0ssW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Source selector 0 comparator clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0cs(&mut self) -> C0csW<GselSpec> {
        C0csW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Source selector 0 set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s0m(&mut self) -> S0mW<GselSpec> {
        S0mW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Source selector 0 clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0m(&mut self) -> C0mW<GselSpec> {
        C0mW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Source selector 0 set edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s0es(&mut self) -> S0esW<GselSpec> {
        S0esW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Source selector 0 clear edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c0es(&mut self) -> C0esW<GselSpec> {
        C0esW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Source selector 1 comparator set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1ss(&mut self) -> C1ssW<GselSpec> {
        C1ssW::new(self, 16)
    }
    #[doc = "Bits 19:21 - Source selector 1 comparator clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1cs(&mut self) -> C1csW<GselSpec> {
        C1csW::new(self, 19)
    }
    #[doc = "Bits 22:23 - Source selector 1 set configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s1m(&mut self) -> S1mW<GselSpec> {
        S1mW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Source selector 1 clear configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1m(&mut self) -> C1mW<GselSpec> {
        C1mW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Source selector 1 set edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn s1es(&mut self) -> S1esW<GselSpec> {
        S1esW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Source selector 1 clear edge configuration"]
    #[inline(always)]
    #[must_use]
    pub fn c1es(&mut self) -> C1esW<GselSpec> {
        C1esW::new(self, 28)
    }
}
#[doc = "HRC global control selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GselSpec;
impl crate::RegisterSpec for GselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsel::R`](R) reader structure"]
impl crate::Readable for GselSpec {}
#[doc = "`write(|w| ..)` method takes [`gsel::W`](W) writer structure"]
impl crate::Writable for GselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GSEL to value 0"]
impl crate::Resettable for GselSpec {
    const RESET_VALUE: u32 = 0;
}
