#[doc = "Register `LPACSET` writer"]
pub type W = crate::W<LpacsetSpec>;
#[doc = "Trigger VBAT Single Compare Operation Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatscmp {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Start compare operation"]
    Value2 = 1,
}
impl From<Vbatscmp> for bool {
    #[inline(always)]
    fn from(variant: Vbatscmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATSCMP` writer - Trigger VBAT Single Compare Operation Set"]
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
    #[doc = "Start compare operation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatscmp::Value2)
    }
}
#[doc = "Trigger HIB_IO_0 Input Single Compare Operation Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0scmp {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Start compare operation"]
    Value2 = 1,
}
impl From<Ahibio0scmp> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0scmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0SCMP` writer - Trigger HIB_IO_0 Input Single Compare Operation Set"]
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
    #[doc = "Start compare operation"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0scmp::Value2)
    }
}
#[doc = "VBAT Compare Operation Initial Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbatval {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Above programmed threshold"]
    Value2 = 1,
}
impl From<Vbatval> for bool {
    #[inline(always)]
    fn from(variant: Vbatval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATVAL` writer - VBAT Compare Operation Initial Value Set"]
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
    #[doc = "Above programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Vbatval::Value2)
    }
}
#[doc = "HIB_IO_0 Input Compare Initial Value Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahibio0val {
    #[doc = "0: No effect"]
    Value1 = 0,
    #[doc = "1: Above programmed threshold"]
    Value2 = 1,
}
impl From<Ahibio0val> for bool {
    #[inline(always)]
    fn from(variant: Ahibio0val) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHIBIO0VAL` writer - HIB_IO_0 Input Compare Initial Value Set"]
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
    #[doc = "Above programmed threshold"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahibio0val::Value2)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger VBAT Single Compare Operation Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbatscmp(&mut self) -> VbatscmpW<LpacsetSpec> {
        VbatscmpW::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger HIB_IO_0 Input Single Compare Operation Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0scmp(&mut self) -> Ahibio0scmpW<LpacsetSpec> {
        Ahibio0scmpW::new(self, 1)
    }
    #[doc = "Bit 16 - VBAT Compare Operation Initial Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbatval(&mut self) -> VbatvalW<LpacsetSpec> {
        VbatvalW::new(self, 16)
    }
    #[doc = "Bit 17 - HIB_IO_0 Input Compare Initial Value Set"]
    #[inline(always)]
    #[must_use]
    pub fn ahibio0val(&mut self) -> Ahibio0valW<LpacsetSpec> {
        Ahibio0valW::new(self, 17)
    }
}
#[doc = "LPAC Control Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpacset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpacsetSpec;
impl crate::RegisterSpec for LpacsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lpacset::W`](W) writer structure"]
impl crate::Writable for LpacsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPACSET to value 0"]
impl crate::Resettable for LpacsetSpec {
    const RESET_VALUE: u32 = 0;
}
