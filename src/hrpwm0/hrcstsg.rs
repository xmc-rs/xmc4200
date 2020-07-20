#[doc = "Reader of register HRCSTSG"]
pub type R = crate::R<u32, super::HRCSTSG>;
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
#[doc = "Reader of field `H0STE`"]
pub type H0STE_R = crate::R<bool, H0STE_A>;
impl H0STE_R {
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
        *self == H0STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H0STE_A::VALUE2
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
#[doc = "Reader of field `H0DSTE`"]
pub type H0DSTE_R = crate::R<bool, H0DSTE_A>;
impl H0DSTE_R {
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
        *self == H0DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H0DSTE_A::VALUE2
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
#[doc = "Reader of field `H1STE`"]
pub type H1STE_R = crate::R<bool, H1STE_A>;
impl H1STE_R {
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
        *self == H1STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H1STE_A::VALUE2
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
#[doc = "Reader of field `H1DSTE`"]
pub type H1DSTE_R = crate::R<bool, H1DSTE_A>;
impl H1DSTE_R {
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
        *self == H1DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H1DSTE_A::VALUE2
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
#[doc = "Reader of field `H2STE`"]
pub type H2STE_R = crate::R<bool, H2STE_A>;
impl H2STE_R {
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
        *self == H2STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H2STE_A::VALUE2
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
#[doc = "Reader of field `H2DSTE`"]
pub type H2DSTE_R = crate::R<bool, H2DSTE_A>;
impl H2DSTE_R {
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
        *self == H2DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H2DSTE_A::VALUE2
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
#[doc = "Reader of field `H3STE`"]
pub type H3STE_R = crate::R<bool, H3STE_A>;
impl H3STE_R {
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
        *self == H3STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H3STE_A::VALUE2
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
#[doc = "Reader of field `H3DSTE`"]
pub type H3DSTE_R = crate::R<bool, H3DSTE_A>;
impl H3DSTE_R {
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
        *self == H3DSTE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == H3DSTE_A::VALUE2
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
