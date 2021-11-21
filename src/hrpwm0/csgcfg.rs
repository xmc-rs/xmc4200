#[doc = "Register `CSGCFG` reader"]
pub struct R(crate::R<CSGCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCFG` writer"]
pub struct W(crate::W<CSGCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCFG_SPEC>;
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
impl From<crate::W<CSGCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CSG0 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C0PM_A {
    #[doc = "0: CSG0 unit is powered OFF"]
    VALUE1 = 0,
    #[doc = "1: CSG0 unit is set in Low Speed Mode"]
    VALUE2 = 1,
    #[doc = "3: CSG0 unit is set in High Speed Mode"]
    VALUE4 = 3,
}
impl From<C0PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C0PM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `C0PM` reader - CSG0 Power Mode"]
pub struct C0PM_R(crate::FieldReader<u8, C0PM_A>);
impl C0PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        C0PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C0PM_A> {
        match self.bits {
            0 => Some(C0PM_A::VALUE1),
            1 => Some(C0PM_A::VALUE2),
            3 => Some(C0PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == C0PM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == C0PM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == C0PM_A::VALUE4
    }
}
impl core::ops::Deref for C0PM_R {
    type Target = crate::FieldReader<u8, C0PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C0PM` writer - CSG0 Power Mode"]
pub struct C0PM_W<'a> {
    w: &'a mut W,
}
impl<'a> C0PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C0PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CSG0 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C0PM_A::VALUE1)
    }
    #[doc = "CSG0 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C0PM_A::VALUE2)
    }
    #[doc = "CSG0 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C0PM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "CSG1 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C1PM_A {
    #[doc = "0: CSG1 unit is powered OFF"]
    VALUE1 = 0,
    #[doc = "1: CSG1 unit is set in Low Speed Mode"]
    VALUE2 = 1,
    #[doc = "3: CSG1 unit is set in High Speed Mode"]
    VALUE4 = 3,
}
impl From<C1PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C1PM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `C1PM` reader - CSG1 Power Mode"]
pub struct C1PM_R(crate::FieldReader<u8, C1PM_A>);
impl C1PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        C1PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C1PM_A> {
        match self.bits {
            0 => Some(C1PM_A::VALUE1),
            1 => Some(C1PM_A::VALUE2),
            3 => Some(C1PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == C1PM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == C1PM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == C1PM_A::VALUE4
    }
}
impl core::ops::Deref for C1PM_R {
    type Target = crate::FieldReader<u8, C1PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1PM` writer - CSG1 Power Mode"]
pub struct C1PM_W<'a> {
    w: &'a mut W,
}
impl<'a> C1PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C1PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CSG1 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C1PM_A::VALUE1)
    }
    #[doc = "CSG1 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C1PM_A::VALUE2)
    }
    #[doc = "CSG1 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C1PM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "CSG2 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum C2PM_A {
    #[doc = "0: CSG2 unit is powered OFF"]
    VALUE1 = 0,
    #[doc = "1: CSG2 unit is set in Low Speed Mode"]
    VALUE2 = 1,
    #[doc = "3: CSG2 unit is set in High Speed Mode"]
    VALUE4 = 3,
}
impl From<C2PM_A> for u8 {
    #[inline(always)]
    fn from(variant: C2PM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `C2PM` reader - CSG2 Power Mode"]
pub struct C2PM_R(crate::FieldReader<u8, C2PM_A>);
impl C2PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        C2PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C2PM_A> {
        match self.bits {
            0 => Some(C2PM_A::VALUE1),
            1 => Some(C2PM_A::VALUE2),
            3 => Some(C2PM_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == C2PM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == C2PM_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == C2PM_A::VALUE4
    }
}
impl core::ops::Deref for C2PM_R {
    type Target = crate::FieldReader<u8, C2PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2PM` writer - CSG2 Power Mode"]
pub struct C2PM_W<'a> {
    w: &'a mut W,
}
impl<'a> C2PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: C2PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CSG2 unit is powered OFF"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(C2PM_A::VALUE1)
    }
    #[doc = "CSG2 unit is set in Low Speed Mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(C2PM_A::VALUE2)
    }
    #[doc = "CSG2 unit is set in High Speed Mode"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(C2PM_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `C0CD` reader - CSG0 Clock disable"]
pub struct C0CD_R(crate::FieldReader<bool, bool>);
impl C0CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        C0CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C0CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C0CD` writer - CSG0 Clock disable"]
pub struct C0CD_W<'a> {
    w: &'a mut W,
}
impl<'a> C0CD_W<'a> {
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
#[doc = "Field `C1CD` reader - CSG1 Clock disable"]
pub struct C1CD_R(crate::FieldReader<bool, bool>);
impl C1CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        C1CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C1CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C1CD` writer - CSG1 Clock disable"]
pub struct C1CD_W<'a> {
    w: &'a mut W,
}
impl<'a> C1CD_W<'a> {
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
#[doc = "Field `C2CD` reader - CSG2 Clock disable"]
pub struct C2CD_R(crate::FieldReader<bool, bool>);
impl C2CD_R {
    pub(crate) fn new(bits: bool) -> Self {
        C2CD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C2CD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C2CD` writer - CSG2 Clock disable"]
pub struct C2CD_W<'a> {
    w: &'a mut W,
}
impl<'a> C2CD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    pub fn c0pm(&self) -> C0PM_R {
        C0PM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    pub fn c1pm(&self) -> C1PM_R {
        C1PM_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    pub fn c2pm(&self) -> C2PM_R {
        C2PM_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    pub fn c0cd(&self) -> C0CD_R {
        C0CD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    pub fn c1cd(&self) -> C1CD_R {
        C1CD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    pub fn c2cd(&self) -> C2CD_R {
        C2CD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSG0 Power Mode"]
    #[inline(always)]
    pub fn c0pm(&mut self) -> C0PM_W {
        C0PM_W { w: self }
    }
    #[doc = "Bits 2:3 - CSG1 Power Mode"]
    #[inline(always)]
    pub fn c1pm(&mut self) -> C1PM_W {
        C1PM_W { w: self }
    }
    #[doc = "Bits 4:5 - CSG2 Power Mode"]
    #[inline(always)]
    pub fn c2pm(&mut self) -> C2PM_W {
        C2PM_W { w: self }
    }
    #[doc = "Bit 16 - CSG0 Clock disable"]
    #[inline(always)]
    pub fn c0cd(&mut self) -> C0CD_W {
        C0CD_W { w: self }
    }
    #[doc = "Bit 17 - CSG1 Clock disable"]
    #[inline(always)]
    pub fn c1cd(&mut self) -> C1CD_W {
        C1CD_W { w: self }
    }
    #[doc = "Bit 18 - CSG2 Clock disable"]
    #[inline(always)]
    pub fn c2cd(&mut self) -> C2CD_W {
        C2CD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CSG configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcfg](index.html) module"]
pub struct CSGCFG_SPEC;
impl crate::RegisterSpec for CSGCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcfg::R](R) reader structure"]
impl crate::Readable for CSGCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcfg::W](W) writer structure"]
impl crate::Writable for CSGCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCFG to value 0"]
impl crate::Resettable for CSGCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
