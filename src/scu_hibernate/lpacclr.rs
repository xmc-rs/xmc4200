#[doc = "Register `LPACCLR` writer"]
pub type W = crate::W<LPACCLR_SPEC>;
#[doc = "Trigger VBAT Single Compare Operation Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATSCMP_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the sticky bit"]
    VALUE2 = 1,
}
impl From<VBATSCMP_A> for bool {
    #[inline(always)]
    fn from(variant: VBATSCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATSCMP` writer - Trigger VBAT Single Compare Operation Clear"]
pub type VBATSCMP_W<'a, REG> = crate::BitWriter<'a, REG, VBATSCMP_A>;
impl<'a, REG> VBATSCMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATSCMP_A::VALUE1)
    }
    #[doc = "Clear the sticky bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATSCMP_A::VALUE2)
    }
}
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0SCMP_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Clear the sticky bit"]
    VALUE2 = 1,
}
impl From<AHIBIO0SCMP_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0SCMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0SCMP` writer - Trigger HIB_IO_0 Input Single Compare Operation Clear"]
pub type AHIBIO0SCMP_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0SCMP_A>;
impl<'a, REG> AHIBIO0SCMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0SCMP_A::VALUE1)
    }
    #[doc = "Clear the sticky bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0SCMP_A::VALUE2)
    }
}
#[doc = "VBAT Compare Operation Initial Value Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATVAL_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Below programmed threshold"]
    VALUE2 = 1,
}
impl From<VBATVAL_A> for bool {
    #[inline(always)]
    fn from(variant: VBATVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATVAL` writer - VBAT Compare Operation Initial Value Clear"]
pub type VBATVAL_W<'a, REG> = crate::BitWriter<'a, REG, VBATVAL_A>;
impl<'a, REG> VBATVAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VBATVAL_A::VALUE1)
    }
    #[doc = "Below programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VBATVAL_A::VALUE2)
    }
}
#[doc = "HIB_IO_0 Input Compare Initial Value Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHIBIO0VAL_A {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Below programmed threshold"]
    VALUE2 = 1,
}
impl From<AHIBIO0VAL_A> for bool {
    #[inline(always)]
    fn from(variant: AHIBIO0VAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0VAL` writer - HIB_IO_0 Input Compare Initial Value Clear"]
pub type AHIBIO0VAL_W<'a, REG> = crate::BitWriter<'a, REG, AHIBIO0VAL_A>;
impl<'a, REG> AHIBIO0VAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0VAL_A::VALUE1)
    }
    #[doc = "Below programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHIBIO0VAL_A::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatscmp(&mut self) -> VBATSCMP_W<LPACCLR_SPEC> {
        VBATSCMP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0scmp(&mut self) -> AHIBIO0SCMP_W<LPACCLR_SPEC> {
        AHIBIO0SCMP_W::new(self, 1)
    }
    #[doc = "Bit 16 - VBAT Compare Operation Initial Value Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatval(&mut self) -> VBATVAL_W<LPACCLR_SPEC> {
        VBATVAL_W::new(self, 16)
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Initial Value Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0val(&mut self) -> AHIBIO0VAL_W<LPACCLR_SPEC> {
        AHIBIO0VAL_W::new(self, 17)
    }
}
#[doc = "LPAC Control Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpacclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPACCLR_SPEC;
impl crate::RegisterSpec for LPACCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lpacclr::W`](W) writer structure"]
impl crate::Writable for LPACCLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPACCLR to value 0"]
impl crate::Resettable for LPACCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
