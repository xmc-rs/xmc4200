#[doc = "Register `HRCSTSG` reader"]
pub type R = crate::R<HrcstsgSpec>;
#[doc = "HRC0 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H0ste {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value2 = 1,
}
impl From<H0ste> for bool {
    #[inline(always)]
    fn from(variant: H0ste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H0STE` reader - HRC0 high resolution values shadow transfer status"]
pub type H0steR = crate::BitReader<H0ste>;
impl H0steR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H0ste {
        match self.bits {
            false => H0ste::Value1,
            true => H0ste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H0ste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H0ste::Value2
    }
}
#[doc = "HRC0 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H0dste {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value2 = 1,
}
impl From<H0dste> for bool {
    #[inline(always)]
    fn from(variant: H0dste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H0DSTE` reader - HRC0 dead time value shadow transfer status"]
pub type H0dsteR = crate::BitReader<H0dste>;
impl H0dsteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H0dste {
        match self.bits {
            false => H0dste::Value1,
            true => H0dste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H0dste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H0dste::Value2
    }
}
#[doc = "HRC1 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H1ste {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value2 = 1,
}
impl From<H1ste> for bool {
    #[inline(always)]
    fn from(variant: H1ste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H1STE` reader - HRC1 high resolution values shadow transfer status"]
pub type H1steR = crate::BitReader<H1ste>;
impl H1steR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H1ste {
        match self.bits {
            false => H1ste::Value1,
            true => H1ste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H1ste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H1ste::Value2
    }
}
#[doc = "HRC1 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H1dste {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value2 = 1,
}
impl From<H1dste> for bool {
    #[inline(always)]
    fn from(variant: H1dste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H1DSTE` reader - HRC1 dead time value shadow transfer status"]
pub type H1dsteR = crate::BitReader<H1dste>;
impl H1dsteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H1dste {
        match self.bits {
            false => H1dste::Value1,
            true => H1dste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H1dste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H1dste::Value2
    }
}
#[doc = "HRC2 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H2ste {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value2 = 1,
}
impl From<H2ste> for bool {
    #[inline(always)]
    fn from(variant: H2ste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H2STE` reader - HRC2 high resolution values shadow transfer status"]
pub type H2steR = crate::BitReader<H2ste>;
impl H2steR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H2ste {
        match self.bits {
            false => H2ste::Value1,
            true => H2ste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H2ste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H2ste::Value2
    }
}
#[doc = "HRC2 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H2dste {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value2 = 1,
}
impl From<H2dste> for bool {
    #[inline(always)]
    fn from(variant: H2dste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H2DSTE` reader - HRC2 dead time value shadow transfer status"]
pub type H2dsteR = crate::BitReader<H2dste>;
impl H2dsteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H2dste {
        match self.bits {
            false => H2dste::Value1,
            true => H2dste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H2dste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H2dste::Value2
    }
}
#[doc = "HRC3 high resolution values shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H3ste {
    #[doc = "0: No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    Value2 = 1,
}
impl From<H3ste> for bool {
    #[inline(always)]
    fn from(variant: H3ste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H3STE` reader - HRC3 high resolution values shadow transfer status"]
pub type H3steR = crate::BitReader<H3ste>;
impl H3steR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H3ste {
        match self.bits {
            false => H3ste::Value1,
            true => H3ste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H3ste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyCR1This register holds the value for the rising edge high resolution signal placement. the update of this value should be done via the associated shadow register, . and HRCyCR2This register holds the value for the falling edge high resolution signal placement. the update of this value should be done via the associated shadow register, . values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H3ste::Value2
    }
}
#[doc = "HRC3 dead time value shadow transfer status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum H3dste {
    #[doc = "0: No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value1 = 0,
    #[doc = "1: Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    Value2 = 1,
}
impl From<H3dste> for bool {
    #[inline(always)]
    fn from(variant: H3dste) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `H3DSTE` reader - HRC3 dead time value shadow transfer status"]
pub type H3dsteR = crate::BitReader<H3dste>;
impl H3dsteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> H3dste {
        match self.bits {
            false => H3dste::Value1,
            true => H3dste::Value2,
        }
    }
    #[doc = "No shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == H3dste::Value1
    }
    #[doc = "Shadow transfer pending for HRCyDCRThis register holds the dead time value that is going to be inserted whenever a rising transition on the output latch is sensed. and HRCyDCFThis register holds the dead time value that is going to be inserted whenever a falling transition on the output latch is sensed. values"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H3dste::Value2
    }
}
impl R {
    #[doc = "Bit 0 - HRC0 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h0ste(&self) -> H0steR {
        H0steR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HRC0 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h0dste(&self) -> H0dsteR {
        H0dsteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - HRC1 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h1ste(&self) -> H1steR {
        H1steR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HRC1 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h1dste(&self) -> H1dsteR {
        H1dsteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - HRC2 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h2ste(&self) -> H2steR {
        H2steR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HRC2 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h2dste(&self) -> H2dsteR {
        H2dsteR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - HRC3 high resolution values shadow transfer status"]
    #[inline(always)]
    pub fn h3ste(&self) -> H3steR {
        H3steR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HRC3 dead time value shadow transfer status"]
    #[inline(always)]
    pub fn h3dste(&self) -> H3dsteR {
        H3dsteR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Global HRC shadow transfer status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrcstsg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrcstsgSpec;
impl crate::RegisterSpec for HrcstsgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrcstsg::R`](R) reader structure"]
impl crate::Readable for HrcstsgSpec {}
#[doc = "`reset()` method sets HRCSTSG to value 0"]
impl crate::Resettable for HrcstsgSpec {
    const RESET_VALUE: u32 = 0;
}
