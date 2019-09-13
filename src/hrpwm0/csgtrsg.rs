#[doc = "Reader of register CSGTRSG"]
pub type R = crate::R<u32, super::CSGTRSG>;
#[doc = "DAC0 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D0STE_A {
    #[doc = "0: Shadow transfer has been performed."]
    VALUE1,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    VALUE2,
}
impl From<D0STE_A> for bool {
    #[inline(always)]
    fn from(variant: D0STE_A) -> Self {
        match variant {
            D0STE_A::VALUE1 => false,
            D0STE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `D0STE`"]
pub type D0STE_R = crate::R<bool, D0STE_A>;
impl D0STE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D0STE_A {
        match self.bits {
            false => D0STE_A::VALUE1,
            true => D0STE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D0STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D0STE_A::VALUE2
    }
}
#[doc = "CMP0 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW0ST_A {
    #[doc = "0: Inverting input connected to HRPWMx.C0I\\[A\\]"]
    VALUE1,
    #[doc = "1: Inverting input connected to HRPWMx.C0I\\[B\\]"]
    VALUE2,
}
impl From<SW0ST_A> for bool {
    #[inline(always)]
    fn from(variant: SW0ST_A) -> Self {
        match variant {
            SW0ST_A::VALUE1 => false,
            SW0ST_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SW0ST`"]
pub type SW0ST_R = crate::R<bool, SW0ST_A>;
impl SW0ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW0ST_A {
        match self.bits {
            false => SW0ST_A::VALUE1,
            true => SW0ST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW0ST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW0ST_A::VALUE2
    }
}
#[doc = "DAC1 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D1STE_A {
    #[doc = "0: Shadow transfer has been performed."]
    VALUE1,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    VALUE2,
}
impl From<D1STE_A> for bool {
    #[inline(always)]
    fn from(variant: D1STE_A) -> Self {
        match variant {
            D1STE_A::VALUE1 => false,
            D1STE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `D1STE`"]
pub type D1STE_R = crate::R<bool, D1STE_A>;
impl D1STE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D1STE_A {
        match self.bits {
            false => D1STE_A::VALUE1,
            true => D1STE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D1STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D1STE_A::VALUE2
    }
}
#[doc = "CMP1 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW1ST_A {
    #[doc = "0: Inverting input connected to HRPWMx.C1I\\[A\\]"]
    VALUE1,
    #[doc = "1: Inverting input connected to HRPWMx.C1I\\[B\\]"]
    VALUE2,
}
impl From<SW1ST_A> for bool {
    #[inline(always)]
    fn from(variant: SW1ST_A) -> Self {
        match variant {
            SW1ST_A::VALUE1 => false,
            SW1ST_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SW1ST`"]
pub type SW1ST_R = crate::R<bool, SW1ST_A>;
impl SW1ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW1ST_A {
        match self.bits {
            false => SW1ST_A::VALUE1,
            true => SW1ST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW1ST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW1ST_A::VALUE2
    }
}
#[doc = "DAC2 shadow transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D2STE_A {
    #[doc = "0: Shadow transfer has been performed."]
    VALUE1,
    #[doc = "1: Shadow transfer has been requested but is still pending completion."]
    VALUE2,
}
impl From<D2STE_A> for bool {
    #[inline(always)]
    fn from(variant: D2STE_A) -> Self {
        match variant {
            D2STE_A::VALUE1 => false,
            D2STE_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `D2STE`"]
pub type D2STE_R = crate::R<bool, D2STE_A>;
impl D2STE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D2STE_A {
        match self.bits {
            false => D2STE_A::VALUE1,
            true => D2STE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == D2STE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == D2STE_A::VALUE2
    }
}
#[doc = "CMP2 inverting input connection status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW2ST_A {
    #[doc = "0: Inverting input connected to HRPWMx.C2I\\[A\\]"]
    VALUE1,
    #[doc = "1: Inverting input connected to HRPWMx.C2I\\[B\\]"]
    VALUE2,
}
impl From<SW2ST_A> for bool {
    #[inline(always)]
    fn from(variant: SW2ST_A) -> Self {
        match variant {
            SW2ST_A::VALUE1 => false,
            SW2ST_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SW2ST`"]
pub type SW2ST_R = crate::R<bool, SW2ST_A>;
impl SW2ST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW2ST_A {
        match self.bits {
            false => SW2ST_A::VALUE1,
            true => SW2ST_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SW2ST_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SW2ST_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - DAC0 shadow transfer enable"]
    #[inline(always)]
    pub fn d0ste(&self) -> D0STE_R {
        D0STE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMP0 inverting input connection status"]
    #[inline(always)]
    pub fn sw0st(&self) -> SW0ST_R {
        SW0ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DAC1 shadow transfer enable"]
    #[inline(always)]
    pub fn d1ste(&self) -> D1STE_R {
        D1STE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CMP1 inverting input connection status"]
    #[inline(always)]
    pub fn sw1st(&self) -> SW1ST_R {
        SW1ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAC2 shadow transfer enable"]
    #[inline(always)]
    pub fn d2ste(&self) -> D2STE_R {
        D2STE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CMP2 inverting input connection status"]
    #[inline(always)]
    pub fn sw2st(&self) -> SW2ST_R {
        SW2ST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
