#[doc = "Register `HINTST` reader"]
pub type R = crate::R<HINTST_SPEC>;
#[doc = "Field `HIBNINT` reader - Internally Controlled Hibernate Sequence Request State"]
pub type HIBNINT_R = crate::BitReader<HIBNINT_A>;
#[doc = "Internally Controlled Hibernate Sequence Request State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBNINT_A {
    #[doc = "0: Hibernate not entered"]
    VALUE1 = 0,
    #[doc = "1: Hibernate entered"]
    VALUE2 = 1,
}
impl From<HIBNINT_A> for bool {
    #[inline(always)]
    fn from(variant: HIBNINT_A) -> Self {
        variant as u8 != 0
    }
}
impl HIBNINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HIBNINT_A {
        match self.bits {
            false => HIBNINT_A::VALUE1,
            true => HIBNINT_A::VALUE2,
        }
    }
    #[doc = "Hibernate not entered"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HIBNINT_A::VALUE1
    }
    #[doc = "Hibernate entered"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HIBNINT_A::VALUE2
    }
}
#[doc = "Field `FLASHOFF` reader - VDDP Supply Switch of Flash State"]
pub type FLASHOFF_R = crate::BitReader<FLASHOFF_A>;
#[doc = "VDDP Supply Switch of Flash State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHOFF_A {
    #[doc = "0: VDDP supply of Flash switched on"]
    VALUE1 = 0,
    #[doc = "1: VDDP supply of Flash switched off"]
    VALUE2 = 1,
}
impl From<FLASHOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASHOFF_A {
        match self.bits {
            false => FLASHOFF_A::VALUE1,
            true => FLASHOFF_A::VALUE2,
        }
    }
    #[doc = "VDDP supply of Flash switched on"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FLASHOFF_A::VALUE1
    }
    #[doc = "VDDP supply of Flash switched off"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FLASHOFF_A::VALUE2
    }
}
#[doc = "Field `FLASHPD` reader - Flash Power Down State"]
pub type FLASHPD_R = crate::BitReader<FLASHPD_A>;
#[doc = "Flash Power Down State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHPD_A {
    #[doc = "0: Normal mode"]
    VALUE1 = 0,
    #[doc = "1: Power down mode effectively entered"]
    VALUE2 = 1,
}
impl From<FLASHPD_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHPD_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASHPD_A {
        match self.bits {
            false => FLASHPD_A::VALUE1,
            true => FLASHPD_A::VALUE2,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FLASHPD_A::VALUE1
    }
    #[doc = "Power down mode effectively entered"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FLASHPD_A::VALUE2
    }
}
#[doc = "Field `POFFD` reader - PORST Pull-up OFF Direct Control State"]
pub type POFFD_R = crate::BitReader<POFFD_A>;
#[doc = "PORST Pull-up OFF Direct Control State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFD_A {
    #[doc = "0: Pull-up on"]
    VALUE1 = 0,
    #[doc = "1: Pull-up off"]
    VALUE2 = 1,
}
impl From<POFFD_A> for bool {
    #[inline(always)]
    fn from(variant: POFFD_A) -> Self {
        variant as u8 != 0
    }
}
impl POFFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POFFD_A {
        match self.bits {
            false => POFFD_A::VALUE1,
            true => POFFD_A::VALUE2,
        }
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POFFD_A::VALUE1
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POFFD_A::VALUE2
    }
}
#[doc = "Field `PPODEL` reader - Delay on PORTS Pull-up Switching OFF on Hibernate Request"]
pub type PPODEL_R = crate::FieldReader;
#[doc = "Field `POFFH` reader - PORST Pull-up OFF in Hibernate Mode State"]
pub type POFFH_R = crate::BitReader<POFFH_A>;
#[doc = "PORST Pull-up OFF in Hibernate Mode State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFH_A {
    #[doc = "0: Pull-up on"]
    VALUE1 = 0,
    #[doc = "1: Pull-up off"]
    VALUE2 = 1,
}
impl From<POFFH_A> for bool {
    #[inline(always)]
    fn from(variant: POFFH_A) -> Self {
        variant as u8 != 0
    }
}
impl POFFH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POFFH_A {
        match self.bits {
            false => POFFH_A::VALUE1,
            true => POFFH_A::VALUE2,
        }
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == POFFH_A::VALUE1
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == POFFH_A::VALUE2
    }
}
impl R {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request State"]
    #[inline(always)]
    pub fn hibnint(&self) -> HIBNINT_R {
        HIBNINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash State"]
    #[inline(always)]
    pub fn flashoff(&self) -> FLASHOFF_R {
        FLASHOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash Power Down State"]
    #[inline(always)]
    pub fn flashpd(&self) -> FLASHPD_R {
        FLASHPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control State"]
    #[inline(always)]
    pub fn poffd(&self) -> POFFD_R {
        POFFD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request"]
    #[inline(always)]
    pub fn ppodel(&self) -> PPODEL_R {
        PPODEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode State"]
    #[inline(always)]
    pub fn poffh(&self) -> POFFH_R {
        POFFH_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Hibernate Internal Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hintst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HINTST_SPEC;
impl crate::RegisterSpec for HINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hintst::R`](R) reader structure"]
impl crate::Readable for HINTST_SPEC {}
#[doc = "`reset()` method sets HINTST to value 0"]
impl crate::Resettable for HINTST_SPEC {
    const RESET_VALUE: u32 = 0;
}
