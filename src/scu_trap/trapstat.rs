#[doc = "Reader of register TRAPSTAT"]
pub type R = crate::R<u32, super::TRAPSTAT>;
#[doc = "System OSC WDT Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOSCWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<SOSCWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: SOSCWDGT_A) -> Self {
        match variant {
            SOSCWDGT_A::VALUE1 => false,
            SOSCWDGT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SOSCWDGT`"]
pub type SOSCWDGT_R = crate::R<bool, SOSCWDGT_A>;
impl SOSCWDGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCWDGT_A {
        match self.bits {
            false => SOSCWDGT_A::VALUE1,
            true => SOSCWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SOSCWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SOSCWDGT_A::VALUE2
    }
}
#[doc = "System VCO Lock Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<SVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: SVCOLCKT_A) -> Self {
        match variant {
            SVCOLCKT_A::VALUE1 => false,
            SVCOLCKT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `SVCOLCKT`"]
pub type SVCOLCKT_R = crate::R<bool, SVCOLCKT_A>;
impl SVCOLCKT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCOLCKT_A {
        match self.bits {
            false => SVCOLCKT_A::VALUE1,
            true => SVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SVCOLCKT_A::VALUE2
    }
}
#[doc = "USB VCO Lock Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVCOLCKT_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<UVCOLCKT_A> for bool {
    #[inline(always)]
    fn from(variant: UVCOLCKT_A) -> Self {
        match variant {
            UVCOLCKT_A::VALUE1 => false,
            UVCOLCKT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `UVCOLCKT`"]
pub type UVCOLCKT_R = crate::R<bool, UVCOLCKT_A>;
impl UVCOLCKT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVCOLCKT_A {
        match self.bits {
            false => UVCOLCKT_A::VALUE1,
            true => UVCOLCKT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == UVCOLCKT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == UVCOLCKT_A::VALUE2
    }
}
#[doc = "Parity Error Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PET_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<PET_A> for bool {
    #[inline(always)]
    fn from(variant: PET_A) -> Self {
        match variant {
            PET_A::VALUE1 => false,
            PET_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `PET`"]
pub type PET_R = crate::R<bool, PET_A>;
impl PET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PET_A {
        match self.bits {
            false => PET_A::VALUE1,
            true => PET_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PET_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PET_A::VALUE2
    }
}
#[doc = "Brown Out Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRWNT_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<BRWNT_A> for bool {
    #[inline(always)]
    fn from(variant: BRWNT_A) -> Self {
        match variant {
            BRWNT_A::VALUE1 => false,
            BRWNT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BRWNT`"]
pub type BRWNT_R = crate::R<bool, BRWNT_A>;
impl BRWNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRWNT_A {
        match self.bits {
            false => BRWNT_A::VALUE1,
            true => BRWNT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BRWNT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BRWNT_A::VALUE2
    }
}
#[doc = "OSC_ULP WDG Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULPWDGT_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<ULPWDGT_A> for bool {
    #[inline(always)]
    fn from(variant: ULPWDGT_A) -> Self {
        match variant {
            ULPWDGT_A::VALUE1 => false,
            ULPWDGT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `ULPWDGT`"]
pub type ULPWDGT_R = crate::R<bool, ULPWDGT_A>;
impl ULPWDGT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULPWDGT_A {
        match self.bits {
            false => ULPWDGT_A::VALUE1,
            true => ULPWDGT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ULPWDGT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ULPWDGT_A::VALUE2
    }
}
#[doc = "Peripheral Bridge 0 Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR0T_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<BWERR0T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR0T_A) -> Self {
        match variant {
            BWERR0T_A::VALUE1 => false,
            BWERR0T_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BWERR0T`"]
pub type BWERR0T_R = crate::R<bool, BWERR0T_A>;
impl BWERR0T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR0T_A {
        match self.bits {
            false => BWERR0T_A::VALUE1,
            true => BWERR0T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWERR0T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWERR0T_A::VALUE2
    }
}
#[doc = "Peripheral Bridge 1 Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWERR1T_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<BWERR1T_A> for bool {
    #[inline(always)]
    fn from(variant: BWERR1T_A) -> Self {
        match variant {
            BWERR1T_A::VALUE1 => false,
            BWERR1T_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `BWERR1T`"]
pub type BWERR1T_R = crate::R<bool, BWERR1T_A>;
impl BWERR1T_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWERR1T_A {
        match self.bits {
            false => BWERR1T_A::VALUE1,
            true => BWERR1T_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BWERR1T_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BWERR1T_A::VALUE2
    }
}
#[doc = "Die Temperature Too High Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPHIT_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<TEMPHIT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPHIT_A) -> Self {
        match variant {
            TEMPHIT_A::VALUE1 => false,
            TEMPHIT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TEMPHIT`"]
pub type TEMPHIT_R = crate::R<bool, TEMPHIT_A>;
impl TEMPHIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPHIT_A {
        match self.bits {
            false => TEMPHIT_A::VALUE1,
            true => TEMPHIT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TEMPHIT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TEMPHIT_A::VALUE2
    }
}
#[doc = "Die Temperature Too Low Trap Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPLOT_A {
    #[doc = "0: No pending trap request"]
    VALUE1,
    #[doc = "1: Pending trap request"]
    VALUE2,
}
impl From<TEMPLOT_A> for bool {
    #[inline(always)]
    fn from(variant: TEMPLOT_A) -> Self {
        match variant {
            TEMPLOT_A::VALUE1 => false,
            TEMPLOT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `TEMPLOT`"]
pub type TEMPLOT_R = crate::R<bool, TEMPLOT_A>;
impl TEMPLOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMPLOT_A {
        match self.bits {
            false => TEMPLOT_A::VALUE1,
            true => TEMPLOT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TEMPLOT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TEMPLOT_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - System OSC WDT Trap Status"]
    #[inline(always)]
    pub fn soscwdgt(&self) -> SOSCWDGT_R {
        SOSCWDGT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - System VCO Lock Trap Status"]
    #[inline(always)]
    pub fn svcolckt(&self) -> SVCOLCKT_R {
        SVCOLCKT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB VCO Lock Trap Status"]
    #[inline(always)]
    pub fn uvcolckt(&self) -> UVCOLCKT_R {
        UVCOLCKT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error Trap Status"]
    #[inline(always)]
    pub fn pet(&self) -> PET_R {
        PET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Brown Out Trap Status"]
    #[inline(always)]
    pub fn brwnt(&self) -> BRWNT_R {
        BRWNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OSC_ULP WDG Trap Status"]
    #[inline(always)]
    pub fn ulpwdgt(&self) -> ULPWDGT_R {
        ULPWDGT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Peripheral Bridge 0 Trap Status"]
    #[inline(always)]
    pub fn bwerr0t(&self) -> BWERR0T_R {
        BWERR0T_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Bridge 1 Trap Status"]
    #[inline(always)]
    pub fn bwerr1t(&self) -> BWERR1T_R {
        BWERR1T_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Die Temperature Too High Trap Status"]
    #[inline(always)]
    pub fn temphit(&self) -> TEMPHIT_R {
        TEMPHIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Die Temperature Too Low Trap Status"]
    #[inline(always)]
    pub fn templot(&self) -> TEMPLOT_R {
        TEMPLOT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
