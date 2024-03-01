#[doc = "Register `HINTSET` writer"]
pub type W = crate::W<HintsetSpec>;
#[doc = "Internally Controlled Hibernate Sequence Request Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibnint {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Hardware controlled hibernate sequence request active"]
    Value2 = 1,
}
impl From<Hibnint> for bool {
    #[inline(always)]
    fn from(variant: Hibnint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNINT` writer - Internally Controlled Hibernate Sequence Request Set"]
pub type HibnintW<'a, REG> = crate::BitWriter<'a, REG, Hibnint>;
impl<'a, REG> HibnintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibnint::Value1)
    }
    #[doc = "Hardware controlled hibernate sequence request active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hibnint::Value2)
    }
}
#[doc = "VDDC Generation off on EVR Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vcoreoff {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: VDDC off to EVR set"]
    Value2 = 1,
}
impl From<Vcoreoff> for bool {
    #[inline(always)]
    fn from(variant: Vcoreoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOREOFF` writer - VDDC Generation off on EVR Set"]
pub type VcoreoffW<'a, REG> = crate::BitWriter<'a, REG, Vcoreoff>;
impl<'a, REG> VcoreoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vcoreoff::Value1)
    }
    #[doc = "VDDC off to EVR set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vcoreoff::Value2)
    }
}
#[doc = "VDDP Supply Switch of Flash Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashoff {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Switch off VDDP supply of Flash"]
    Value2 = 1,
}
impl From<Flashoff> for bool {
    #[inline(always)]
    fn from(variant: Flashoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHOFF` writer - VDDP Supply Switch of Flash Set"]
pub type FlashoffW<'a, REG> = crate::BitWriter<'a, REG, Flashoff>;
impl<'a, REG> FlashoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashoff::Value1)
    }
    #[doc = "Switch off VDDP supply of Flash"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashoff::Value2)
    }
}
#[doc = "Flash Power Down Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashpd {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Flash power down mode request set"]
    Value2 = 1,
}
impl From<Flashpd> for bool {
    #[inline(always)]
    fn from(variant: Flashpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHPD` writer - Flash Power Down Set"]
pub type FlashpdW<'a, REG> = crate::BitWriter<'a, REG, Flashpd>;
impl<'a, REG> FlashpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashpd::Value1)
    }
    #[doc = "Flash power down mode request set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashpd::Value2)
    }
}
#[doc = "PORST Pull-up OFF Direct Control Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poffd {
    #[doc = "0: No effect"]
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
#[doc = "Field `POFFD` writer - PORST Pull-up OFF Direct Control Set"]
pub type PoffdW<'a, REG> = crate::BitWriter<'a, REG, Poffd>;
impl<'a, REG> PoffdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Poffd::Value1)
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Poffd::Value2)
    }
}
#[doc = "Field `PPODEL` writer - Delay on PORTS Pull-up Switching OFF on Hibernate Request Set"]
pub type PpodelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "PORST Pull-up OFF in Hibernate Mode Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poffh {
    #[doc = "0: No effect"]
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
#[doc = "Field `POFFH` writer - PORST Pull-up OFF in Hibernate Mode Set"]
pub type PoffhW<'a, REG> = crate::BitWriter<'a, REG, Poffh>;
impl<'a, REG> PoffhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Poffh::Value1)
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Poffh::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn hibnint(&mut self) -> HibnintW<HintsetSpec> {
        HibnintW::new(self, 0)
    }
    #[doc = "Bit 1 - VDDC Generation off on EVR Set"]
    #[inline(always)]
    #[must_use]
    pub fn vcoreoff(&mut self) -> VcoreoffW<HintsetSpec> {
        VcoreoffW::new(self, 1)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Set"]
    #[inline(always)]
    #[must_use]
    pub fn flashoff(&mut self) -> FlashoffW<HintsetSpec> {
        FlashoffW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Power Down Set"]
    #[inline(always)]
    #[must_use]
    pub fn flashpd(&mut self) -> FlashpdW<HintsetSpec> {
        FlashpdW::new(self, 3)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Set"]
    #[inline(always)]
    #[must_use]
    pub fn poffd(&mut self) -> PoffdW<HintsetSpec> {
        PoffdW::new(self, 4)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn ppodel(&mut self) -> PpodelW<HintsetSpec> {
        PpodelW::new(self, 16)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Set"]
    #[inline(always)]
    #[must_use]
    pub fn poffh(&mut self) -> PoffhW<HintsetSpec> {
        PoffhW::new(self, 20)
    }
}
#[doc = "Hibernate Internal Control Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hintset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HintsetSpec;
impl crate::RegisterSpec for HintsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hintset::W`](W) writer structure"]
impl crate::Writable for HintsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HINTSET to value 0"]
impl crate::Resettable for HintsetSpec {
    const RESET_VALUE: u32 = 0;
}
