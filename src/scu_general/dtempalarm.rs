#[doc = "Reader of register DTEMPALARM"]
pub type R = crate::R<u32, super::DTEMPALARM>;
#[doc = "Lower Limit Underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFL_A {
    #[doc = "0: No temperature underflow was detected"]
    VALUE1,
    #[doc = "1: A temperature underflow was detected"]
    VALUE2,
}
impl From<UNDERFL_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFL_A) -> Self {
        match variant {
            UNDERFL_A::VALUE1 => false,
            UNDERFL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `UNDERFL`"]
pub type UNDERFL_R = crate::R<bool, UNDERFL_A>;
impl UNDERFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFL_A {
        match self.bits {
            false => UNDERFL_A::VALUE1,
            true => UNDERFL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UNDERFL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UNDERFL_A::VALUE2
    }
}
#[doc = "Upper Limit Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFL_A {
    #[doc = "0: No temperature overflow was detected"]
    VALUE1,
    #[doc = "1: A temperature overflow was detected"]
    VALUE2,
}
impl From<OVERFL_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFL_A) -> Self {
        match variant {
            OVERFL_A::VALUE1 => false,
            OVERFL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `OVERFL`"]
pub type OVERFL_R = crate::R<bool, OVERFL_A>;
impl OVERFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFL_A {
        match self.bits {
            false => OVERFL_A::VALUE1,
            true => OVERFL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == OVERFL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == OVERFL_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Lower Limit Underflow"]
    #[inline(always)]
    pub fn underfl(&self) -> UNDERFL_R {
        UNDERFL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Upper Limit Overflow"]
    #[inline(always)]
    pub fn overfl(&self) -> OVERFL_R {
        OVERFL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
