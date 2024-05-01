#[doc = "Register `HINTCLR` writer"]
pub type W = crate::W<HintclrSpec>;
#[doc = "Internally Controlled Hibernate Sequence Request Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibnint {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Hibernate bit clear"]
    Value2 = 1,
}
impl From<Hibnint> for bool {
    #[inline(always)]
    fn from(variant: Hibnint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNINT` writer - Internally Controlled Hibernate Sequence Request Clear"]
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
    #[doc = "Hibernate bit clear"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Hibnint::Value2)
    }
}
#[doc = "VDDP Supply Switch of Flash Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashoff {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Switch on VDDP supply of Flash"]
    Value2 = 1,
}
impl From<Flashoff> for bool {
    #[inline(always)]
    fn from(variant: Flashoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHOFF` writer - VDDP Supply Switch of Flash Clear"]
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
    #[doc = "Switch on VDDP supply of Flash"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashoff::Value2)
    }
}
#[doc = "Flash Power Down Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashpd {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Flash power down mode leave request"]
    Value2 = 1,
}
impl From<Flashpd> for bool {
    #[inline(always)]
    fn from(variant: Flashpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHPD` writer - Flash Power Down Clear"]
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
    #[doc = "Flash power down mode leave request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashpd::Value2)
    }
}
#[doc = "PORST Pull-up OFF Direct Control Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poffd {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Pull-up on"]
    Value2 = 1,
}
impl From<Poffd> for bool {
    #[inline(always)]
    fn from(variant: Poffd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFD` writer - PORST Pull-up OFF Direct Control Clear"]
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
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Poffd::Value2)
    }
}
#[doc = "Field `PPODEL` writer - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
pub type PpodelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "PORST Pull-up OFF in Hibernate Mode Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poffh {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Pull-up on"]
    Value2 = 1,
}
impl From<Poffh> for bool {
    #[inline(always)]
    fn from(variant: Poffh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFH` writer - PORST Pull-up OFF in Hibernate Mode Clear"]
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
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Poffh::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hibnint(&mut self) -> HibnintW<HintclrSpec> {
        HibnintW::new(self, 0)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flashoff(&mut self) -> FlashoffW<HintclrSpec> {
        FlashoffW::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Power Down Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flashpd(&mut self) -> FlashpdW<HintclrSpec> {
        FlashpdW::new(self, 3)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn poffd(&mut self) -> PoffdW<HintclrSpec> {
        PoffdW::new(self, 4)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ppodel(&mut self) -> PpodelW<HintclrSpec> {
        PpodelW::new(self, 16)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Clear"]
    #[inline(always)]
    #[must_use]
    pub fn poffh(&mut self) -> PoffhW<HintclrSpec> {
        PoffhW::new(self, 20)
    }
}
#[doc = "Hibernate Internal Control Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HintclrSpec;
impl crate::RegisterSpec for HintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hintclr::W`](W) writer structure"]
impl crate::Writable for HintclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HINTCLR to value 0"]
impl crate::Resettable for HintclrSpec {
    const RESET_VALUE: u32 = 0;
}
