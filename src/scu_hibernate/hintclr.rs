#[doc = "Register `HINTCLR` writer"]
pub type W = crate::W<HINTCLR_SPEC>;
#[doc = "Internally Controlled Hibernate Sequence Request Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBNINT_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Hibernate bit clear"]
    VALUE2 = 1,
}
impl From<HIBNINT_A> for bool {
    #[inline(always)]
    fn from(variant: HIBNINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNINT` writer - Internally Controlled Hibernate Sequence Request Clear"]
pub type HIBNINT_W<'a, REG> = crate::BitWriter<'a, REG, HIBNINT_A>;
impl<'a, REG> HIBNINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBNINT_A::VALUE1)
    }
    #[doc = "Hibernate bit clear"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBNINT_A::VALUE2)
    }
}
#[doc = "VDDP Supply Switch of Flash Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHOFF_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Switch on VDDP supply of Flash"]
    VALUE2 = 1,
}
impl From<FLASHOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHOFF` writer - VDDP Supply Switch of Flash Clear"]
pub type FLASHOFF_W<'a, REG> = crate::BitWriter<'a, REG, FLASHOFF_A>;
impl<'a, REG> FLASHOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHOFF_A::VALUE1)
    }
    #[doc = "Switch on VDDP supply of Flash"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHOFF_A::VALUE2)
    }
}
#[doc = "Flash Power Down Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHPD_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Flash power down mode leave request"]
    VALUE2 = 1,
}
impl From<FLASHPD_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHPD` writer - Flash Power Down Clear"]
pub type FLASHPD_W<'a, REG> = crate::BitWriter<'a, REG, FLASHPD_A>;
impl<'a, REG> FLASHPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHPD_A::VALUE1)
    }
    #[doc = "Flash power down mode leave request"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHPD_A::VALUE2)
    }
}
#[doc = "PORST Pull-up OFF Direct Control Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFD_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up on"]
    VALUE2 = 1,
}
impl From<POFFD_A> for bool {
    #[inline(always)]
    fn from(variant: POFFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFD` writer - PORST Pull-up OFF Direct Control Clear"]
pub type POFFD_W<'a, REG> = crate::BitWriter<'a, REG, POFFD_A>;
impl<'a, REG> POFFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POFFD_A::VALUE1)
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POFFD_A::VALUE2)
    }
}
#[doc = "Field `PPODEL` writer - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
pub type PPODEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "PORST Pull-up OFF in Hibernate Mode Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFH_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up on"]
    VALUE2 = 1,
}
impl From<POFFH_A> for bool {
    #[inline(always)]
    fn from(variant: POFFH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFH` writer - PORST Pull-up OFF in Hibernate Mode Clear"]
pub type POFFH_W<'a, REG> = crate::BitWriter<'a, REG, POFFH_A>;
impl<'a, REG> POFFH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POFFH_A::VALUE1)
    }
    #[doc = "Pull-up on"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POFFH_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hibnint(&mut self) -> HIBNINT_W<HINTCLR_SPEC> {
        HIBNINT_W::new(self, 0)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flashoff(&mut self) -> FLASHOFF_W<HINTCLR_SPEC> {
        FLASHOFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Power Down Clear"]
    #[inline(always)]
    #[must_use]
    pub fn flashpd(&mut self) -> FLASHPD_W<HINTCLR_SPEC> {
        FLASHPD_W::new(self, 3)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Clear"]
    #[inline(always)]
    #[must_use]
    pub fn poffd(&mut self) -> POFFD_W<HINTCLR_SPEC> {
        POFFD_W::new(self, 4)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ppodel(&mut self) -> PPODEL_W<HINTCLR_SPEC> {
        PPODEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Clear"]
    #[inline(always)]
    #[must_use]
    pub fn poffh(&mut self) -> POFFH_W<HINTCLR_SPEC> {
        POFFH_W::new(self, 20)
    }
}
#[doc = "Hibernate Internal Control Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HINTCLR_SPEC;
impl crate::RegisterSpec for HINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hintclr::W`](W) writer structure"]
impl crate::Writable for HINTCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HINTCLR to value 0"]
impl crate::Resettable for HINTCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
