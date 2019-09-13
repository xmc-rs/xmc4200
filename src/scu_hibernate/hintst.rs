#[doc = "Reader of register HINTST"]
pub type R = crate::R<u32, super::HINTST>;
#[doc = "Internally Controlled Hibernate Sequence Request State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HIBNINT_A {
    #[doc = "0: Hibernate not entered"]
    VALUE1,
    #[doc = "1: Hibernate entered"]
    VALUE2,
}
impl From<HIBNINT_A> for bool {
    #[inline(always)]
    fn from(variant: HIBNINT_A) -> Self {
        match variant {
            HIBNINT_A::VALUE1 => false,
            HIBNINT_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `HIBNINT`"]
pub type HIBNINT_R = crate::R<bool, HIBNINT_A>;
impl HIBNINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIBNINT_A {
        match self.bits {
            false => HIBNINT_A::VALUE1,
            true => HIBNINT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBNINT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBNINT_A::VALUE2
    }
}
#[doc = "VDDP Supply Switch of Flash State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHOFF_A {
    #[doc = "0: VDDP supply of Flash switched on"]
    VALUE1,
    #[doc = "1: VDDP supply of Flash switched off"]
    VALUE2,
}
impl From<FLASHOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHOFF_A) -> Self {
        match variant {
            FLASHOFF_A::VALUE1 => false,
            FLASHOFF_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FLASHOFF`"]
pub type FLASHOFF_R = crate::R<bool, FLASHOFF_A>;
impl FLASHOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHOFF_A {
        match self.bits {
            false => FLASHOFF_A::VALUE1,
            true => FLASHOFF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FLASHOFF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FLASHOFF_A::VALUE2
    }
}
#[doc = "Flash Power Down State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHPD_A {
    #[doc = "0: Normal mode"]
    VALUE1,
    #[doc = "1: Power down mode effectively entered"]
    VALUE2,
}
impl From<FLASHPD_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHPD_A) -> Self {
        match variant {
            FLASHPD_A::VALUE1 => false,
            FLASHPD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FLASHPD`"]
pub type FLASHPD_R = crate::R<bool, FLASHPD_A>;
impl FLASHPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHPD_A {
        match self.bits {
            false => FLASHPD_A::VALUE1,
            true => FLASHPD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FLASHPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FLASHPD_A::VALUE2
    }
}
#[doc = "PORST Pull-up OFF Direct Control State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFD_A {
    #[doc = "0: Pull-up on"]
    VALUE1,
    #[doc = "1: Pull-up off"]
    VALUE2,
}
impl From<POFFD_A> for bool {
    #[inline(always)]
    fn from(variant: POFFD_A) -> Self {
        match variant {
            POFFD_A::VALUE1 => false,
            POFFD_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `POFFD`"]
pub type POFFD_R = crate::R<bool, POFFD_A>;
impl POFFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POFFD_A {
        match self.bits {
            false => POFFD_A::VALUE1,
            true => POFFD_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POFFD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POFFD_A::VALUE2
    }
}
#[doc = "Reader of field `PPODEL`"]
pub type PPODEL_R = crate::R<u8, u8>;
#[doc = "PORST Pull-up OFF in Hibernate Mode State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFFH_A {
    #[doc = "0: Pull-up on"]
    VALUE1,
    #[doc = "1: Pull-up off"]
    VALUE2,
}
impl From<POFFH_A> for bool {
    #[inline(always)]
    fn from(variant: POFFH_A) -> Self {
        match variant {
            POFFH_A::VALUE1 => false,
            POFFH_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `POFFH`"]
pub type POFFH_R = crate::R<bool, POFFH_A>;
impl POFFH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POFFH_A {
        match self.bits {
            false => POFFH_A::VALUE1,
            true => POFFH_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POFFH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POFFH_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request State"]
    #[inline(always)]
    pub fn hibnint(&self) -> HIBNINT_R {
        HIBNINT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash State"]
    #[inline(always)]
    pub fn flashoff(&self) -> FLASHOFF_R {
        FLASHOFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash Power Down State"]
    #[inline(always)]
    pub fn flashpd(&self) -> FLASHPD_R {
        FLASHPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control State"]
    #[inline(always)]
    pub fn poffd(&self) -> POFFD_R {
        POFFD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request"]
    #[inline(always)]
    pub fn ppodel(&self) -> PPODEL_R {
        PPODEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode State"]
    #[inline(always)]
    pub fn poffh(&self) -> POFFH_R {
        POFFH_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
