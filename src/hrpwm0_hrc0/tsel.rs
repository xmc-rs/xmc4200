#[doc = "Register `TSEL` reader"]
pub struct R(crate::R<TSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSEL` writer"]
pub struct W(crate::W<TSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEL_SPEC>;
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
impl From<crate::W<TSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSEL0` reader - Source Selector 0 Timer connection"]
pub type TSEL0_R = crate::FieldReader<u8, TSEL0_A>;
#[doc = "Source Selector 0 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL0_A {
    #[doc = "0: Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1 = 0,
    #[doc = "1: Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2 = 1,
    #[doc = "2: Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3 = 2,
    #[doc = "3: Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4 = 3,
}
impl From<TSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL0_A) -> Self {
        variant as _
    }
}
impl TSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEL0_A> {
        match self.bits {
            0 => Some(TSEL0_A::VALUE1),
            1 => Some(TSEL0_A::VALUE2),
            2 => Some(TSEL0_A::VALUE3),
            3 => Some(TSEL0_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSEL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSEL0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSEL0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSEL0_A::VALUE4
    }
}
#[doc = "Field `TSEL0` writer - Source Selector 0 Timer connection"]
pub type TSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSEL_SPEC, u8, TSEL0_A, 3, O>;
impl<'a, const O: u8> TSEL0_W<'a, O> {
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE1)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE2)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE3)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSEL0_A::VALUE4)
    }
}
#[doc = "Field `TSEL1` reader - Source Selector 1 Timer connection"]
pub type TSEL1_R = crate::FieldReader<u8, TSEL1_A>;
#[doc = "Source Selector 1 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1 = 0,
    #[doc = "1: Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2 = 1,
    #[doc = "2: Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3 = 2,
    #[doc = "3: Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4 = 3,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEL1_A> {
        match self.bits {
            0 => Some(TSEL1_A::VALUE1),
            1 => Some(TSEL1_A::VALUE2),
            2 => Some(TSEL1_A::VALUE3),
            3 => Some(TSEL1_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSEL1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSEL1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSEL1_A::VALUE4
    }
}
#[doc = "Field `TSEL1` writer - Source Selector 1 Timer connection"]
pub type TSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSEL_SPEC, u8, TSEL1_A, 3, O>;
impl<'a, const O: u8> TSEL1_W<'a, O> {
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE1)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE2)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE3)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TSEL1_A::VALUE4)
    }
}
#[doc = "Field `TS0E` reader - Source selector 0 TRAP enable"]
pub type TS0E_R = crate::BitReader<TS0E_A>;
#[doc = "Source selector 0 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS0E_A {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    VALUE1 = 0,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    VALUE2 = 1,
}
impl From<TS0E_A> for bool {
    #[inline(always)]
    fn from(variant: TS0E_A) -> Self {
        variant as u8 != 0
    }
}
impl TS0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS0E_A {
        match self.bits {
            false => TS0E_A::VALUE1,
            true => TS0E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS0E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS0E_A::VALUE2
    }
}
#[doc = "Field `TS0E` writer - Source selector 0 TRAP enable"]
pub type TS0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSEL_SPEC, TS0E_A, O>;
impl<'a, const O: u8> TS0E_W<'a, O> {
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TS0E_A::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TS0E_A::VALUE2)
    }
}
#[doc = "Field `TS1E` reader - Source selector 1 TRAP enable"]
pub type TS1E_R = crate::BitReader<TS1E_A>;
#[doc = "Source selector 1 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1E_A {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    VALUE1 = 0,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    VALUE2 = 1,
}
impl From<TS1E_A> for bool {
    #[inline(always)]
    fn from(variant: TS1E_A) -> Self {
        variant as u8 != 0
    }
}
impl TS1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TS1E_A {
        match self.bits {
            false => TS1E_A::VALUE1,
            true => TS1E_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS1E_A::VALUE2
    }
}
#[doc = "Field `TS1E` writer - Source selector 1 TRAP enable"]
pub type TS1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSEL_SPEC, TS1E_A, O>;
impl<'a, const O: u8> TS1E_W<'a, O> {
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TS1E_A::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TS1E_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    pub fn tsel0(&self) -> TSEL0_R {
        TSEL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    pub fn ts0e(&self) -> TS0E_R {
        TS0E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    pub fn ts1e(&self) -> TS1E_R {
        TS1E_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel0(&mut self) -> TSEL0_W<0> {
        TSEL0_W::new(self)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<3> {
        TSEL1_W::new(self)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts0e(&mut self) -> TS0E_W<16> {
        TS0E_W::new(self)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts1e(&mut self) -> TS1E_W<17> {
        TS1E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC timer selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsel](index.html) module"]
pub struct TSEL_SPEC;
impl crate::RegisterSpec for TSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsel::R](R) reader structure"]
impl crate::Readable for TSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsel::W](W) writer structure"]
impl crate::Writable for TSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSEL to value 0"]
impl crate::Resettable for TSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
