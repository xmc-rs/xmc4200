#[doc = "Register `HRCSTSG` reader"]
pub struct R(crate::R<HRCSTSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRCSTSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRCSTSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRCSTSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "HRC0 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H0STE_A {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE2 = 1,
}
impl From<H0STE_A> for bool {
    #[inline(always)]
    fn from(variant: H0STE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H0STE` reader - HRC0 high resolution values shadow transfer status"]
pub struct H0STE_R(crate::FieldReader<bool, H0STE_A>);
impl H0STE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H0STE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H0STE_A {
        match self.bits {
            false => H0STE_A::VALUE1,
            true => H0STE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H0STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H0STE_A::VALUE2
    }
}
impl core::ops::Deref for H0STE_R {
    type Target = crate::FieldReader<bool, H0STE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRC0 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H0DSTE_A {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE2 = 1,
}
impl From<H0DSTE_A> for bool {
    #[inline(always)]
    fn from(variant: H0DSTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H0DSTE` reader - HRC0 dead time value shadow transfer status"]
pub struct H0DSTE_R(crate::FieldReader<bool, H0DSTE_A>);
impl H0DSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H0DSTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H0DSTE_A {
        match self.bits {
            false => H0DSTE_A::VALUE1,
            true => H0DSTE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H0DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H0DSTE_A::VALUE2
    }
}
impl core::ops::Deref for H0DSTE_R {
    type Target = crate::FieldReader<bool, H0DSTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRC1 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H1STE_A {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE2 = 1,
}
impl From<H1STE_A> for bool {
    #[inline(always)]
    fn from(variant: H1STE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H1STE` reader - HRC1 high resolution values shadow transfer status"]
pub struct H1STE_R(crate::FieldReader<bool, H1STE_A>);
impl H1STE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H1STE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H1STE_A {
        match self.bits {
            false => H1STE_A::VALUE1,
            true => H1STE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H1STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H1STE_A::VALUE2
    }
}
impl core::ops::Deref for H1STE_R {
    type Target = crate::FieldReader<bool, H1STE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRC1 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H1DSTE_A {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE2 = 1,
}
impl From<H1DSTE_A> for bool {
    #[inline(always)]
    fn from(variant: H1DSTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H1DSTE` reader - HRC1 dead time value shadow transfer status"]
pub struct H1DSTE_R(crate::FieldReader<bool, H1DSTE_A>);
impl H1DSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H1DSTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H1DSTE_A {
        match self.bits {
            false => H1DSTE_A::VALUE1,
            true => H1DSTE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H1DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H1DSTE_A::VALUE2
    }
}
impl core::ops::Deref for H1DSTE_R {
    type Target = crate::FieldReader<bool, H1DSTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRC2 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H2STE_A {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE2 = 1,
}
impl From<H2STE_A> for bool {
    #[inline(always)]
    fn from(variant: H2STE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H2STE` reader - HRC2 high resolution values shadow transfer status"]
pub struct H2STE_R(crate::FieldReader<bool, H2STE_A>);
impl H2STE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H2STE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H2STE_A {
        match self.bits {
            false => H2STE_A::VALUE1,
            true => H2STE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H2STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H2STE_A::VALUE2
    }
}
impl core::ops::Deref for H2STE_R {
    type Target = crate::FieldReader<bool, H2STE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRC2 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H2DSTE_A {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE2 = 1,
}
impl From<H2DSTE_A> for bool {
    #[inline(always)]
    fn from(variant: H2DSTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H2DSTE` reader - HRC2 dead time value shadow transfer status"]
pub struct H2DSTE_R(crate::FieldReader<bool, H2DSTE_A>);
impl H2DSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H2DSTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H2DSTE_A {
        match self.bits {
            false => H2DSTE_A::VALUE1,
            true => H2DSTE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H2DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H2DSTE_A::VALUE2
    }
}
impl core::ops::Deref for H2DSTE_R {
    type Target = crate::FieldReader<bool, H2DSTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRC3 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H3STE_A {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    VALUE2 = 1,
}
impl From<H3STE_A> for bool {
    #[inline(always)]
    fn from(variant: H3STE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H3STE` reader - HRC3 high resolution values shadow transfer status"]
pub struct H3STE_R(crate::FieldReader<bool, H3STE_A>);
impl H3STE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H3STE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H3STE_A {
        match self.bits {
            false => H3STE_A::VALUE1,
            true => H3STE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H3STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H3STE_A::VALUE2
    }
}
impl core::ops::Deref for H3STE_R {
    type Target = crate::FieldReader<bool, H3STE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "HRC3 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum H3DSTE_A {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    VALUE2 = 1,
}
impl From<H3DSTE_A> for bool {
    #[inline(always)]
    fn from(variant: H3DSTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H3DSTE` reader - HRC3 dead time value shadow transfer status"]
pub struct H3DSTE_R(crate::FieldReader<bool, H3DSTE_A>);
impl H3DSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        H3DSTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> H3DSTE_A {
        match self.bits {
            false => H3DSTE_A::VALUE1,
            true => H3DSTE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == H3DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == H3DSTE_A::VALUE2
    }
}
impl core::ops::Deref for H3DSTE_R {
    type Target = crate::FieldReader<bool, H3DSTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h0ste(&self) -> H0STE_R {
        H0STE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h0dste(&self) -> H0DSTE_R {
        H0DSTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h1ste(&self) -> H1STE_R {
        H1STE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRC1 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h1dste(&self) -> H1DSTE_R {
        H1DSTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h2ste(&self) -> H2STE_R {
        H2STE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HRC2 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h2dste(&self) -> H2DSTE_R {
        H2DSTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h3ste(&self) -> H3STE_R {
        H3STE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HRC3 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h3dste(&self) -> H3DSTE_R {
        H3DSTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Global HRC shadow transfer status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrcstsg](index.html) module"]
pub struct HRCSTSG_SPEC;
impl crate::RegisterSpec for HRCSTSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrcstsg::R](R) reader structure"]
impl crate::Readable for HRCSTSG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HRCSTSG to value 0"]
impl crate::Resettable for HRCSTSG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
