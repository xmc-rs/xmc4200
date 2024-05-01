#[doc = "Register `LPACCLR` writer"]
pub type W = crate::W<LpacclrSpec>;
#[doc = "Trigger VBAT Single Compare Operation Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatscmp {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the sticky bit"]
    Value2 = 1,
}
impl From<Vbatscmp> for bool {
    #[inline(always)]
    fn from(variant: Vbatscmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATSCMP` writer - Trigger VBAT Single Compare Operation Clear"]
pub type VbatscmpW<'a, REG> = crate::BitWriter<'a, REG, Vbatscmp>;
impl<'a, REG> VbatscmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatscmp::Value1)
    }
    #[doc = "Clear the sticky bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatscmp::Value2)
    }
}
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0scmp {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Clear the sticky bit"]
    Value2 = 1,
}
impl From<Ahibio0scmp> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0scmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0SCMP` writer - Trigger HIB_IO_0 Input Single Compare Operation Clear"]
pub type Ahibio0scmpW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0scmp>;
impl<'a, REG> Ahibio0scmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0scmp::Value1)
    }
    #[doc = "Clear the sticky bit"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0scmp::Value2)
    }
}
#[doc = "VBAT Compare Operation Initial Value Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatval {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Below programmed threshold"]
    Value2 = 1,
}
impl From<Vbatval> for bool {
    #[inline(always)]
    fn from(variant: Vbatval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATVAL` writer - VBAT Compare Operation Initial Value Clear"]
pub type VbatvalW<'a, REG> = crate::BitWriter<'a, REG, Vbatval>;
impl<'a, REG> VbatvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatval::Value1)
    }
    #[doc = "Below programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatval::Value2)
    }
}
#[doc = "HIB_IO_0 Input Compare Initial Value Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0val {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Below programmed threshold"]
    Value2 = 1,
}
impl From<Ahibio0val> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0VAL` writer - HIB_IO_0 Input Compare Initial Value Clear"]
pub type Ahibio0valW<'a, REG> = crate::BitWriter<'a, REG, Ahibio0val>;
impl<'a, REG> Ahibio0valW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0val::Value1)
    }
    #[doc = "Below programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0val::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatscmp(&mut self) -> VbatscmpW<LpacclrSpec> {
        VbatscmpW::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0scmp(&mut self) -> Ahibio0scmpW<LpacclrSpec> {
        Ahibio0scmpW::new(self, 1)
    }
    #[doc = "Bit 16 - VBAT Compare Operation Initial Value Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbatval(&mut self) -> VbatvalW<LpacclrSpec> {
        VbatvalW::new(self, 16)
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Initial Value Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0val(&mut self) -> Ahibio0valW<LpacclrSpec> {
        Ahibio0valW::new(self, 17)
    }
}
#[doc = "LPAC Control Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpacclrSpec;
impl crate::RegisterSpec for LpacclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lpacclr::W`](W) writer structure"]
impl crate::Writable for LpacclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPACCLR to value 0"]
impl crate::Resettable for LpacclrSpec {
    const RESET_VALUE: u32 = 0;
}
