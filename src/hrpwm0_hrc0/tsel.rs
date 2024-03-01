#[doc = "Register `TSEL` reader"]
pub type R = crate::R<TselSpec>;
#[doc = "Register `TSEL` writer"]
pub type W = crate::W<TselSpec>;
#[doc = "Source Selector 0 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsel0 {
    #[doc = "0: Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    Value1 = 0,
    #[doc = "1: Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    Value2 = 1,
    #[doc = "2: Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    Value3 = 2,
    #[doc = "3: Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    Value4 = 3,
}
impl From<Tsel0> for u8 {
    #[inline(always)]
    fn from(variant: Tsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsel0 {
    type Ux = u8;
}
#[doc = "Field `TSEL0` reader - Source Selector 0 Timer connection"]
pub type Tsel0R = crate::FieldReader<Tsel0>;
impl Tsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsel0> {
        match self.bits {
            0 => Some(Tsel0::Value1),
            1 => Some(Tsel0::Value2),
            2 => Some(Tsel0::Value3),
            3 => Some(Tsel0::Value4),
            _ => None,
        }
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsel0::Value1
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsel0::Value2
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Tsel0::Value3
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Tsel0::Value4
    }
}
#[doc = "Field `TSEL0` writer - Source Selector 0 Timer connection"]
pub type Tsel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsel0>;
impl<'a, REG> Tsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel0::Value1)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel0::Value2)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel0::Value3)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel0::Value4)
    }
}
#[doc = "Source Selector 1 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsel1 {
    #[doc = "0: Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    Value1 = 0,
    #[doc = "1: Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    Value2 = 1,
    #[doc = "2: Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    Value3 = 2,
    #[doc = "3: Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    Value4 = 3,
}
impl From<Tsel1> for u8 {
    #[inline(always)]
    fn from(variant: Tsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsel1 {
    type Ux = u8;
}
#[doc = "Field `TSEL1` reader - Source Selector 1 Timer connection"]
pub type Tsel1R = crate::FieldReader<Tsel1>;
impl Tsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsel1> {
        match self.bits {
            0 => Some(Tsel1::Value1),
            1 => Some(Tsel1::Value2),
            2 => Some(Tsel1::Value3),
            3 => Some(Tsel1::Value4),
            _ => None,
        }
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Tsel1::Value1
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Tsel1::Value2
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Tsel1::Value3
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Tsel1::Value4
    }
}
#[doc = "Field `TSEL1` writer - Source Selector 1 Timer connection"]
pub type Tsel1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Tsel1>;
impl<'a, REG> Tsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::Value1)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::Value2)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::Value3)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Tsel1::Value4)
    }
}
#[doc = "Source selector 0 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ts0e {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    Value1 = 0,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    Value2 = 1,
}
impl From<Ts0e> for bool {
    #[inline(always)]
    fn from(variant: Ts0e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS0E` reader - Source selector 0 TRAP enable"]
pub type Ts0eR = crate::BitReader<Ts0e>;
impl Ts0eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts0e {
        match self.bits {
            false => Ts0e::Value1,
            true => Ts0e::Value2,
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ts0e::Value1
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ts0e::Value2
    }
}
#[doc = "Field `TS0E` writer - Source selector 0 TRAP enable"]
pub type Ts0eW<'a, REG> = crate::BitWriter<'a, REG, Ts0e>;
impl<'a, REG> Ts0eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts0e::Value1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ts0e::Value2)
    }
}
#[doc = "Source selector 1 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ts1e {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    Value1 = 0,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    Value2 = 1,
}
impl From<Ts1e> for bool {
    #[inline(always)]
    fn from(variant: Ts1e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1E` reader - Source selector 1 TRAP enable"]
pub type Ts1eR = crate::BitReader<Ts1e>;
impl Ts1eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts1e {
        match self.bits {
            false => Ts1e::Value1,
            true => Ts1e::Value2,
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ts1e::Value1
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ts1e::Value2
    }
}
#[doc = "Field `TS1E` writer - Source selector 1 TRAP enable"]
pub type Ts1eW<'a, REG> = crate::BitWriter<'a, REG, Ts1e>;
impl<'a, REG> Ts1eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1e::Value1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ts1e::Value2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    pub fn tsel0(&self) -> Tsel0R {
        Tsel0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    pub fn tsel1(&self) -> Tsel1R {
        Tsel1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    pub fn ts0e(&self) -> Ts0eR {
        Ts0eR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    pub fn ts1e(&self) -> Ts1eR {
        Ts1eR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel0(&mut self) -> Tsel0W<TselSpec> {
        Tsel0W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> Tsel1W<TselSpec> {
        Tsel1W::new(self, 3)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts0e(&mut self) -> Ts0eW<TselSpec> {
        Ts0eW::new(self, 16)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts1e(&mut self) -> Ts1eW<TselSpec> {
        Ts1eW::new(self, 17)
    }
}
#[doc = "HRC timer selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TselSpec;
impl crate::RegisterSpec for TselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsel::R`](R) reader structure"]
impl crate::Readable for TselSpec {}
#[doc = "`write(|w| ..)` method takes [`tsel::W`](W) writer structure"]
impl crate::Writable for TselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSEL to value 0"]
impl crate::Resettable for TselSpec {
    const RESET_VALUE: u32 = 0;
}
