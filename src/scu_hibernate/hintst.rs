#[doc = "Register `HINTST` reader"]
pub struct R(crate::R<HINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Internally Controlled Hibernate Sequence Request State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `HIBNINT` reader - Internally Controlled Hibernate Sequence Request State"]
pub struct HIBNINT_R(crate::FieldReader<bool, HIBNINT_A>);
impl HIBNINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIBNINT_R(crate::FieldReader::new(bits))
    }
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
        **self == HIBNINT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HIBNINT_A::VALUE2
    }
}
impl core::ops::Deref for HIBNINT_R {
    type Target = crate::FieldReader<bool, HIBNINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "VDDP Supply Switch of Flash State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FLASHOFF` reader - VDDP Supply Switch of Flash State"]
pub struct FLASHOFF_R(crate::FieldReader<bool, FLASHOFF_A>);
impl FLASHOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHOFF_R(crate::FieldReader::new(bits))
    }
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
        **self == FLASHOFF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FLASHOFF_A::VALUE2
    }
}
impl core::ops::Deref for FLASHOFF_R {
    type Target = crate::FieldReader<bool, FLASHOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash Power Down State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `FLASHPD` reader - Flash Power Down State"]
pub struct FLASHPD_R(crate::FieldReader<bool, FLASHPD_A>);
impl FLASHPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHPD_R(crate::FieldReader::new(bits))
    }
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
        **self == FLASHPD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == FLASHPD_A::VALUE2
    }
}
impl core::ops::Deref for FLASHPD_R {
    type Target = crate::FieldReader<bool, FLASHPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PORST Pull-up OFF Direct Control State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `POFFD` reader - PORST Pull-up OFF Direct Control State"]
pub struct POFFD_R(crate::FieldReader<bool, POFFD_A>);
impl POFFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        POFFD_R(crate::FieldReader::new(bits))
    }
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
        **self == POFFD_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == POFFD_A::VALUE2
    }
}
impl core::ops::Deref for POFFD_R {
    type Target = crate::FieldReader<bool, POFFD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPODEL` reader - Delay on PORTS Pull-up Switching OFF on Hibernate Request"]
pub struct PPODEL_R(crate::FieldReader<u8, u8>);
impl PPODEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPODEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPODEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PORST Pull-up OFF in Hibernate Mode State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `POFFH` reader - PORST Pull-up OFF in Hibernate Mode State"]
pub struct POFFH_R(crate::FieldReader<bool, POFFH_A>);
impl POFFH_R {
    pub(crate) fn new(bits: bool) -> Self {
        POFFH_R(crate::FieldReader::new(bits))
    }
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
        **self == POFFH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == POFFH_A::VALUE2
    }
}
impl core::ops::Deref for POFFH_R {
    type Target = crate::FieldReader<bool, POFFH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Hibernate Internal Control State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hintst](index.html) module"]
pub struct HINTST_SPEC;
impl crate::RegisterSpec for HINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hintst::R](R) reader structure"]
impl crate::Readable for HINTST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HINTST to value 0"]
impl crate::Resettable for HINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
