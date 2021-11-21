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
#[doc = "Source Selector 0 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TSEL0` reader - Source Selector 0 Timer connection"]
pub struct TSEL0_R(crate::FieldReader<u8, TSEL0_A>);
impl TSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TSEL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSEL0_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TSEL0_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TSEL0_A::VALUE4
    }
}
impl core::ops::Deref for TSEL0_R {
    type Target = crate::FieldReader<u8, TSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL0` writer - Source Selector 0 Timer connection"]
pub struct TSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Source Selector 1 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TSEL1` reader - Source Selector 1 Timer connection"]
pub struct TSEL1_R(crate::FieldReader<u8, TSEL1_A>);
impl TSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TSEL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TSEL1_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TSEL1_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TSEL1_A::VALUE4
    }
}
impl core::ops::Deref for TSEL1_R {
    type Target = crate::FieldReader<u8, TSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEL1` writer - Source Selector 1 Timer connection"]
pub struct TSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Source selector 0 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TS0E` reader - Source selector 0 TRAP enable"]
pub struct TS0E_R(crate::FieldReader<bool, TS0E_A>);
impl TS0E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS0E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TS0E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TS0E_A::VALUE2
    }
}
impl core::ops::Deref for TS0E_R {
    type Target = crate::FieldReader<bool, TS0E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS0E` writer - Source selector 0 TRAP enable"]
pub struct TS0E_W<'a> {
    w: &'a mut W,
}
impl<'a> TS0E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS0E_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
#[doc = "Source selector 1 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TS1E` reader - Source selector 1 TRAP enable"]
pub struct TS1E_R(crate::FieldReader<bool, TS1E_A>);
impl TS1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS1E_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == TS1E_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TS1E_A::VALUE2
    }
}
impl core::ops::Deref for TS1E_R {
    type Target = crate::FieldReader<bool, TS1E_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS1E` writer - Source selector 1 TRAP enable"]
pub struct TS1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TS1E_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    pub fn tsel0(&self) -> TSEL0_R {
        TSEL0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    pub fn ts0e(&self) -> TS0E_R {
        TS0E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    pub fn ts1e(&self) -> TS1E_R {
        TS1E_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    pub fn tsel0(&mut self) -> TSEL0_W {
        TSEL0_W { w: self }
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    pub fn tsel1(&mut self) -> TSEL1_W {
        TSEL1_W { w: self }
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    pub fn ts0e(&mut self) -> TS0E_W {
        TS0E_W { w: self }
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    pub fn ts1e(&mut self) -> TS1E_W {
        TS1E_W { w: self }
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
}
#[doc = "`reset()` method sets TSEL to value 0"]
impl crate::Resettable for TSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
