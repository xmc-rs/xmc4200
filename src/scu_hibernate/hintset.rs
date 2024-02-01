#[doc = "Register `HINTSET` writer"]
pub type W = crate::W<HINTSET_SPEC>;
#[doc = "Internally Controlled Hibernate Sequence Request Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIBNINT_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Hardware controlled hibernate sequence request active"]
    VALUE2 = 1,
}
impl From<HIBNINT_AW> for bool {
    #[inline(always)]
    fn from(variant: HIBNINT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBNINT` writer - Internally Controlled Hibernate Sequence Request Set"]
pub type HIBNINT_W<'a, REG> = crate::BitWriter<'a, REG, HIBNINT_AW>;
impl<'a, REG> HIBNINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HIBNINT_AW::VALUE1)
    }
    #[doc = "Hardware controlled hibernate sequence request active"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HIBNINT_AW::VALUE2)
    }
}
#[doc = "VDDC Generation off on EVR Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOREOFF_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: VDDC off to EVR set"]
    VALUE2 = 1,
}
impl From<VCOREOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: VCOREOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOREOFF` writer - VDDC Generation off on EVR Set"]
pub type VCOREOFF_W<'a, REG> = crate::BitWriter<'a, REG, VCOREOFF_AW>;
impl<'a, REG> VCOREOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VCOREOFF_AW::VALUE1)
    }
    #[doc = "VDDC off to EVR set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VCOREOFF_AW::VALUE2)
    }
}
#[doc = "VDDP Supply Switch of Flash Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHOFF_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Switch off VDDP supply of Flash"]
    VALUE2 = 1,
}
impl From<FLASHOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHOFF` writer - VDDP Supply Switch of Flash Set"]
pub type FLASHOFF_W<'a, REG> = crate::BitWriter<'a, REG, FLASHOFF_AW>;
impl<'a, REG> FLASHOFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHOFF_AW::VALUE1)
    }
    #[doc = "Switch off VDDP supply of Flash"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHOFF_AW::VALUE2)
    }
}
#[doc = "Flash Power Down Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHPD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Flash power down mode request set"]
    VALUE2 = 1,
}
impl From<FLASHPD_AW> for bool {
    #[inline(always)]
    fn from(variant: FLASHPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHPD` writer - Flash Power Down Set"]
pub type FLASHPD_W<'a, REG> = crate::BitWriter<'a, REG, FLASHPD_AW>;
impl<'a, REG> FLASHPD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHPD_AW::VALUE1)
    }
    #[doc = "Flash power down mode request set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(FLASHPD_AW::VALUE2)
    }
}
#[doc = "PORST Pull-up OFF Direct Control Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFD_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up off"]
    VALUE2 = 1,
}
impl From<POFFD_AW> for bool {
    #[inline(always)]
    fn from(variant: POFFD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFD` writer - PORST Pull-up OFF Direct Control Set"]
pub type POFFD_W<'a, REG> = crate::BitWriter<'a, REG, POFFD_AW>;
impl<'a, REG> POFFD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POFFD_AW::VALUE1)
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POFFD_AW::VALUE2)
    }
}
#[doc = "Field `PPODEL` writer - Delay on PORTS Pull-up Switching OFF on Hibernate Request Set"]
pub type PPODEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "PORST Pull-up OFF in Hibernate Mode Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POFFH_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Pull-up off"]
    VALUE2 = 1,
}
impl From<POFFH_AW> for bool {
    #[inline(always)]
    fn from(variant: POFFH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POFFH` writer - PORST Pull-up OFF in Hibernate Mode Set"]
pub type POFFH_W<'a, REG> = crate::BitWriter<'a, REG, POFFH_AW>;
impl<'a, REG> POFFH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POFFH_AW::VALUE1)
    }
    #[doc = "Pull-up off"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POFFH_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Internally Controlled Hibernate Sequence Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn hibnint(&mut self) -> HIBNINT_W<HINTSET_SPEC> {
        HIBNINT_W::new(self, 0)
    }
    #[doc = "Bit 1 - VDDC Generation off on EVR Set"]
    #[inline(always)]
    #[must_use]
    pub fn vcoreoff(&mut self) -> VCOREOFF_W<HINTSET_SPEC> {
        VCOREOFF_W::new(self, 1)
    }
    #[doc = "Bit 2 - VDDP Supply Switch of Flash Set"]
    #[inline(always)]
    #[must_use]
    pub fn flashoff(&mut self) -> FLASHOFF_W<HINTSET_SPEC> {
        FLASHOFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Flash Power Down Set"]
    #[inline(always)]
    #[must_use]
    pub fn flashpd(&mut self) -> FLASHPD_W<HINTSET_SPEC> {
        FLASHPD_W::new(self, 3)
    }
    #[doc = "Bit 4 - PORST Pull-up OFF Direct Control Set"]
    #[inline(always)]
    #[must_use]
    pub fn poffd(&mut self) -> POFFD_W<HINTSET_SPEC> {
        POFFD_W::new(self, 4)
    }
    #[doc = "Bits 16:17 - Delay on PORTS Pull-up Switching OFF on Hibernate Request Set"]
    #[inline(always)]
    #[must_use]
    pub fn ppodel(&mut self) -> PPODEL_W<HINTSET_SPEC> {
        PPODEL_W::new(self, 16)
    }
    #[doc = "Bit 20 - PORST Pull-up OFF in Hibernate Mode Set"]
    #[inline(always)]
    #[must_use]
    pub fn poffh(&mut self) -> POFFH_W<HINTSET_SPEC> {
        POFFH_W::new(self, 20)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hibernate Internal Control Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hintset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HINTSET_SPEC;
impl crate::RegisterSpec for HINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hintset::W`](W) writer structure"]
impl crate::Writable for HINTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HINTSET to value 0"]
impl crate::Resettable for HINTSET_SPEC {
    const RESET_VALUE: u32 = 0;
}
