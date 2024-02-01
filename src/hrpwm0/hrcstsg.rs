#[doc = "Register `HRCSTSG` reader"]
pub type R = crate::R<HRCSTSG_SPEC>;
#[doc = "Field `H0STE` reader - HRC0 high resolution values shadow transfer status"]
pub type H0STE_R = crate::BitReader<H0STE_A>;
#[doc = "HRC0 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H0STE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H0STE_A {
        match self.bits {
            false => H0STE_A::VALUE1,
            true => H0STE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H0STE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H0STE_A::VALUE2
    }
}
#[doc = "Field `H0DSTE` reader - HRC0 dead time value shadow transfer status"]
pub type H0DSTE_R = crate::BitReader<H0DSTE_A>;
#[doc = "HRC0 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H0DSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H0DSTE_A {
        match self.bits {
            false => H0DSTE_A::VALUE1,
            true => H0DSTE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H0DSTE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H0DSTE_A::VALUE2
    }
}
#[doc = "Field `H1STE` reader - HRC1 high resolution values shadow transfer status"]
pub type H1STE_R = crate::BitReader<H1STE_A>;
#[doc = "HRC1 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H1STE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H1STE_A {
        match self.bits {
            false => H1STE_A::VALUE1,
            true => H1STE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H1STE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H1STE_A::VALUE2
    }
}
#[doc = "Field `H1DSTE` reader - HRC1 dead time value shadow transfer status"]
pub type H1DSTE_R = crate::BitReader<H1DSTE_A>;
#[doc = "HRC1 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H1DSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H1DSTE_A {
        match self.bits {
            false => H1DSTE_A::VALUE1,
            true => H1DSTE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H1DSTE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H1DSTE_A::VALUE2
    }
}
#[doc = "Field `H2STE` reader - HRC2 high resolution values shadow transfer status"]
pub type H2STE_R = crate::BitReader<H2STE_A>;
#[doc = "HRC2 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H2STE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H2STE_A {
        match self.bits {
            false => H2STE_A::VALUE1,
            true => H2STE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H2STE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H2STE_A::VALUE2
    }
}
#[doc = "Field `H2DSTE` reader - HRC2 dead time value shadow transfer status"]
pub type H2DSTE_R = crate::BitReader<H2DSTE_A>;
#[doc = "HRC2 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H2DSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H2DSTE_A {
        match self.bits {
            false => H2DSTE_A::VALUE1,
            true => H2DSTE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H2DSTE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H2DSTE_A::VALUE2
    }
}
#[doc = "Field `H3STE` reader - HRC3 high resolution values shadow transfer status"]
pub type H3STE_R = crate::BitReader<H3STE_A>;
#[doc = "HRC3 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H3STE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H3STE_A {
        match self.bits {
            false => H3STE_A::VALUE1,
            true => H3STE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H3STE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H3STE_A::VALUE2
    }
}
#[doc = "Field `H3DSTE` reader - HRC3 dead time value shadow transfer status"]
pub type H3DSTE_R = crate::BitReader<H3DSTE_A>;
#[doc = "HRC3 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl H3DSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H3DSTE_A {
        match self.bits {
            false => H3DSTE_A::VALUE1,
            true => H3DSTE_A::VALUE2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H3DSTE_A::VALUE1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H3DSTE_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h0ste(&self) -> H0STE_R {
        H0STE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h0dste(&self) -> H0DSTE_R {
        H0DSTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h1ste(&self) -> H1STE_R {
        H1STE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRC1 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h1dste(&self) -> H1DSTE_R {
        H1DSTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h2ste(&self) -> H2STE_R {
        H2STE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HRC2 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h2dste(&self) -> H2DSTE_R {
        H2DSTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h3ste(&self) -> H3STE_R {
        H3STE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HRC3 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h3dste(&self) -> H3DSTE_R {
        H3DSTE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Global HRC shadow transfer status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrcstsg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HRCSTSG_SPEC;
impl crate::RegisterSpec for HRCSTSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrcstsg::R`](R) reader structure"]
impl crate::Readable for HRCSTSG_SPEC {}
#[doc = "`reset()` method sets HRCSTSG to value 0"]
impl crate::Resettable for HRCSTSG_SPEC {
    const RESET_VALUE: u32 = 0;
}
