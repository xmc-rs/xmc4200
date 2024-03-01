#[doc = "Register `HINTST` reader"]
pub type R = crate::R<HintstSpec>;
#[doc = "Internally Controlled Hibernate Sequence Request State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibnint {
    #[doc = "0: Hibernate not entered"]
    Value1 = 0,
    #[doc = "1: Hibernate entered"]
    Value2 = 1,
}
impl From<Hibnint> for bool {
    #[inline(always)]
    fn from(variant: Hibnint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNINT` reader - Internally Controlled Hibernate Sequence Request State"]
pub type HibnintR = crate::BitReader<Hibnint>;
impl HibnintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibnint {
        match self.bits {
            false => Hibnint::Value1,
            true => Hibnint::Value2,
        }
    }
    #[doc = "Hibernate not entered"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Hibnint::Value1
    }
    #[doc = "Hibernate entered"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Hibnint::Value2
    }
}
#[doc = "VDDP Supply Switch of Flash State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashoff {
    #[doc = "0: VDDP supply of Flash switched on"]
    Value1 = 0,
    #[doc = "1: VDDP supply of Flash switched off"]
    Value2 = 1,
}
impl From<Flashoff> for bool {
    #[inline(always)]
    fn from(variant: Flashoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHOFF` reader - VDDP Supply Switch of Flash State"]
pub type FlashoffR = crate::BitReader<Flashoff>;
impl FlashoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashoff {
        match self.bits {
            false => Flashoff::Value1,
            true => Flashoff::Value2,
        }
    }
    #[doc = "VDDP supply of Flash switched on"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Flashoff::Value1
    }
    #[doc = "VDDP supply of Flash switched off"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Flashoff::Value2
    }
}
#[doc = "Flash Power Down State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashpd {
    #[doc = "0: Normal mode"]
    Value1 = 0,
    #[doc = "1: Power down mode effectively entered"]
    Value2 = 1,
}
impl From<Flashpd> for bool {
    #[inline(always)]
    fn from(variant: Flashpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHPD` reader - Flash Power Down State"]
pub type FlashpdR = crate::BitReader<Flashpd>;
impl FlashpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashpd {
        match self.bits {
            false => Flashpd::Value1,
            true => Flashpd::Value2,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Flashpd::Value1
    }
    #[doc = "Power down mode effectively entered"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Flashpd::Value2
    }
}
#[doc = "PORST Pull-up OFF Direct Control State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poffd {
    #[doc = "0: Pull-up on"]
    Value1 = 0,
    #[doc = "1: Pull-up off"]
    Value2 = 1,
}
impl From<Poffd> for bool {
    #[inline(always)]
    fn from(variant: Poffd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFD` reader - PORST Pull-up OFF Direct Control State"]
pub type PoffdR = crate::BitReader<Poffd>;
impl PoffdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Poffd {
        match self.bits {
            false => Poffd::Value1,
            true => Poffd::Value2,
        }
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Poffd::Value1
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Poffd::Value2
    }
}
#[doc = "Field `PPODEL` reader - Delay on PORTS Pull-up Switching OFF on Hibernate Request"]
pub type PpodelR = crate::FieldReader;
#[doc = "PORST Pull-up OFF in Hibernate Mode State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poffh {
    #[doc = "0: Pull-up on"]
    Value1 = 0,
    #[doc = "1: Pull-up off"]
    Value2 = 1,
}
impl From<Poffh> for bool {
    #[inline(always)]
    fn from(variant: Poffh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFH` reader - PORST Pull-up OFF in Hibernate Mode State"]
pub type PoffhR = crate::BitReader<Poffh>;
impl PoffhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Poffh {
        match self.bits {
            false => Poffh::Value1,
            true => Poffh::Value2,
        }
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Poffh::Value1
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Poffh::Value2
    }
}
impl R {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request State"]
    #[inline(always)]
    pub fn hibnint(&self) -> HibnintR {
        HibnintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash State"]
    #[inline(always)]
    pub fn flashoff(&self) -> FlashoffR {
        FlashoffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash Power Down State"]
    #[inline(always)]
    pub fn flashpd(&self) -> FlashpdR {
        FlashpdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control State"]
    #[inline(always)]
    pub fn poffd(&self) -> PoffdR {
        PoffdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request"]
    #[inline(always)]
    pub fn ppodel(&self) -> PpodelR {
        PpodelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode State"]
    #[inline(always)]
    pub fn poffh(&self) -> PoffhR {
        PoffhR::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Hibernate Internal Control State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hintst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HintstSpec;
impl crate::RegisterSpec for HintstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hintst::R`](R) reader structure"]
impl crate::Readable for HintstSpec {}
#[doc = "`reset()` method sets HINTST to value 0"]
impl crate::Resettable for HintstSpec {
    const RESET_VALUE: u32 = 0;
}
