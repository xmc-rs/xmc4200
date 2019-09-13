#[doc = "Reader of register LPACST"]
pub type R = crate::R<u32, super::LPACST>;
#[doc = "Trigger VBAT Single Compare Operation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATSCMP_A {
    #[doc = "0: Ready to start new compare operation"]
    VALUE1,
    #[doc = "1: Compare operation completed"]
    VALUE2,
}
impl From<VBATSCMP_A> for bool {
    #[inline(always)]
    fn from(variant: VBATSCMP_A) -> Self {
        match variant {
            VBATSCMP_A::VALUE1 => false,
            VBATSCMP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VBATSCMP`"]
pub type VBATSCMP_R = crate::R<bool, VBATSCMP_A>;
impl VBATSCMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATSCMP_A {
        match self.bits {
            false => VBATSCMP_A::VALUE1,
            true => VBATSCMP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATSCMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATSCMP_A::VALUE2
    }
}
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0SCMP_A {
    #[doc = "0: Ready to start new compare operation"]
    VALUE1,
    #[doc = "1: Compare operation completed"]
    VALUE2,
}
impl From<AHIBIO0SCMP_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0SCMP_A) -> Self {
        match variant {
            AHIBIO0SCMP_A::VALUE1 => false,
            AHIBIO0SCMP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AHIBIO0SCMP`"]
pub type AHIBIO0SCMP_R = crate::R<bool, AHIBIO0SCMP_A>;
impl AHIBIO0SCMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHIBIO0SCMP_A {
        match self.bits {
            false => AHIBIO0SCMP_A::VALUE1,
            true => AHIBIO0SCMP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0SCMP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0SCMP_A::VALUE2
    }
}
#[doc = "VBAT Compare Operation Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBATVAL_A {
    #[doc = "0: Below programmed threshold"]
    VALUE1,
    #[doc = "1: Above programmed threshold"]
    VALUE2,
}
impl From<VBATVAL_A> for bool {
    #[inline(always)]
    fn from(variant: VBATVAL_A) -> Self {
        match variant {
            VBATVAL_A::VALUE1 => false,
            VBATVAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `VBATVAL`"]
pub type VBATVAL_R = crate::R<bool, VBATVAL_A>;
impl VBATVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATVAL_A {
        match self.bits {
            false => VBATVAL_A::VALUE1,
            true => VBATVAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VBATVAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VBATVAL_A::VALUE2
    }
}
#[doc = "HIB_IO_0 Input Compare Operation Result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHIBIO0VAL_A {
    #[doc = "0: Below programmed threshold"]
    VALUE1,
    #[doc = "1: Above programmed threshold"]
    VALUE2,
}
impl From<AHIBIO0VAL_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0VAL_A) -> Self {
        match variant {
            AHIBIO0VAL_A::VALUE1 => false,
            AHIBIO0VAL_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `AHIBIO0VAL`"]
pub type AHIBIO0VAL_R = crate::R<bool, AHIBIO0VAL_A>;
impl AHIBIO0VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHIBIO0VAL_A {
        match self.bits {
            false => AHIBIO0VAL_A::VALUE1,
            true => AHIBIO0VAL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHIBIO0VAL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHIBIO0VAL_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Status"]
    #[inline(always)]
    pub fn vbatscmp(&self) -> VBATSCMP_R {
        VBATSCMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Status"]
    #[inline(always)]
    pub fn ahibio0scmp(&self) -> AHIBIO0SCMP_R {
        AHIBIO0SCMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - VBAT Compare Operation Result"]
    #[inline(always)]
    pub fn vbatval(&self) -> VBATVAL_R {
        VBATVAL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Operation Result"]
    #[inline(always)]
    pub fn ahibio0val(&self) -> AHIBIO0VAL_R {
        AHIBIO0VAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
