#[doc = "Register `LPACCONF` reader"]
pub struct R(crate::R<LPACCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPACCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPACCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPACCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPACCONF` writer"]
pub struct W(crate::W<LPACCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPACCONF_SPEC>;
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
impl From<crate::W<LPACCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPACCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Compare Enable for Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPEN_A {
    #[doc = "0: Comparator permanently in power down"]
    VALUE1 = 0,
    #[doc = "1: Comparator activated for VBAT input"]
    VALUE2 = 1,
    #[doc = "2: Comparator activated for HIB_IO_0 input"]
    VALUE3 = 2,
    #[doc = "4: Comparator activated for HIB_IO_1 input"]
    VALUE4 = 4,
}
impl From<CMPEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPEN` reader - Compare Enable for Input Selection"]
pub struct CMPEN_R(crate::FieldReader<u8, CMPEN_A>);
impl CMPEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPEN_A> {
        match self.bits {
            0 => Some(CMPEN_A::VALUE1),
            1 => Some(CMPEN_A::VALUE2),
            2 => Some(CMPEN_A::VALUE3),
            4 => Some(CMPEN_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CMPEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CMPEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == CMPEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == CMPEN_A::VALUE4
    }
}
impl core::ops::Deref for CMPEN_R {
    type Target = crate::FieldReader<u8, CMPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPEN` writer - Compare Enable for Input Selection"]
pub struct CMPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Comparator permanently in power down"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE1)
    }
    #[doc = "Comparator activated for VBAT input"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE2)
    }
    #[doc = "Comparator activated for HIB_IO_0 input"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE3)
    }
    #[doc = "Comparator activated for HIB_IO_1 input"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(CMPEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Analog Compare Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    #[doc = "0: Sub-second interval counter"]
    VALUE1 = 0,
    #[doc = "1: RTC alarm event"]
    VALUE2 = 1,
    #[doc = "2: RTC periodic event"]
    VALUE3 = 2,
    #[doc = "3: On digital WKUP input positive edge event"]
    VALUE4 = 3,
    #[doc = "5: On digital WKUP input negative edge event"]
    VALUE5 = 5,
    #[doc = "6: Continuous measurement"]
    VALUE6 = 6,
    #[doc = "7: Single-shot on software request"]
    VALUE7 = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGSEL` reader - Analog Compare Trigger Select"]
pub struct TRIGSEL_R(crate::FieldReader<u8, TRIGSEL_A>);
impl TRIGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSEL_A> {
        match self.bits {
            0 => Some(TRIGSEL_A::VALUE1),
            1 => Some(TRIGSEL_A::VALUE2),
            2 => Some(TRIGSEL_A::VALUE3),
            3 => Some(TRIGSEL_A::VALUE4),
            5 => Some(TRIGSEL_A::VALUE5),
            6 => Some(TRIGSEL_A::VALUE6),
            7 => Some(TRIGSEL_A::VALUE7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == TRIGSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == TRIGSEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == TRIGSEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == TRIGSEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == TRIGSEL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        **self == TRIGSEL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        **self == TRIGSEL_A::VALUE7
    }
}
impl core::ops::Deref for TRIGSEL_R {
    type Target = crate::FieldReader<u8, TRIGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSEL` writer - Analog Compare Trigger Select"]
pub struct TRIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sub-second interval counter"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE1)
    }
    #[doc = "RTC alarm event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE2)
    }
    #[doc = "RTC periodic event"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE3)
    }
    #[doc = "On digital WKUP input positive edge event"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE4)
    }
    #[doc = "On digital WKUP input negative edge event"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE5)
    }
    #[doc = "Continuous measurement"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE6)
    }
    #[doc = "Single-shot on software request"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(TRIGSEL_A::VALUE7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `CONVDEL` reader - Conversion Delay"]
pub struct CONVDEL_R(crate::FieldReader<bool, bool>);
impl CONVDEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONVDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONVDEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONVDEL` writer - Conversion Delay"]
pub struct CONVDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVDEL_W<'a> {
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
#[doc = "Field `INTERVCNT` reader - Sub-second Interval Counter"]
pub struct INTERVCNT_R(crate::FieldReader<u16, u16>);
impl INTERVCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        INTERVCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERVCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTERVCNT` writer - Sub-second Interval Counter"]
pub struct INTERVCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERVCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `SETTLECNT` reader - LPAC Settle Time Counter"]
pub struct SETTLECNT_R(crate::FieldReader<u8, u8>);
impl SETTLECNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SETTLECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETTLECNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETTLECNT` writer - LPAC Settle Time Counter"]
pub struct SETTLECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTLECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    pub fn convdel(&self) -> CONVDEL_R {
        CONVDEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    pub fn intervcnt(&self) -> INTERVCNT_R {
        INTERVCNT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    pub fn settlecnt(&self) -> SETTLECNT_R {
        SETTLECNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Compare Enable for Input Selection"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W {
        CMPEN_W { w: self }
    }
    #[doc = "Bits 4:6 - Analog Compare Trigger Select"]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TRIGSEL_W {
        TRIGSEL_W { w: self }
    }
    #[doc = "Bit 12 - Conversion Delay"]
    #[inline(always)]
    pub fn convdel(&mut self) -> CONVDEL_W {
        CONVDEL_W { w: self }
    }
    #[doc = "Bits 16:27 - Sub-second Interval Counter"]
    #[inline(always)]
    pub fn intervcnt(&mut self) -> INTERVCNT_W {
        INTERVCNT_W { w: self }
    }
    #[doc = "Bits 28:31 - LPAC Settle Time Counter"]
    #[inline(always)]
    pub fn settlecnt(&mut self) -> SETTLECNT_W {
        SETTLECNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Wake-up Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpacconf](index.html) module"]
pub struct LPACCONF_SPEC;
impl crate::RegisterSpec for LPACCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpacconf::R](R) reader structure"]
impl crate::Readable for LPACCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpacconf::W](W) writer structure"]
impl crate::Writable for LPACCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPACCONF to value 0x7000_0000"]
impl crate::Resettable for LPACCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7000_0000
    }
}
