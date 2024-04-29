#[doc = "Register `TSEL` reader"]
pub type R = crate::R<TSEL_SPEC>;
#[doc = "Register `TSEL` writer"]
pub type W = crate::W<TSEL_SPEC>;
#[doc = "Source Selector 0 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL0_A {
    #[doc = "0: Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1 = 0,
    #[doc = "1: Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2 = 1,
    #[doc = "2: Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3 = 2,
    #[doc = "3: Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4 = 3,
}
impl From<TSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL0_A {
    type Ux = u8;
}
impl crate::IsEnum for TSEL0_A {}
#[doc = "Field `TSEL0` reader - Source Selector 0 Timer connection"]
pub type TSEL0_R = crate::FieldReader<TSEL0_A>;
impl TSEL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL0_A> {
        match self.bits {
            0 => Some(TSEL0_A::VALUE1),
            1 => Some(TSEL0_A::VALUE2),
            2 => Some(TSEL0_A::VALUE3),
            3 => Some(TSEL0_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSEL0_A::VALUE1
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSEL0_A::VALUE2
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSEL0_A::VALUE3
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSEL0_A::VALUE4
    }
}
#[doc = "Field `TSEL0` writer - Source Selector 0 Timer connection"]
pub type TSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSEL0_A>;
impl<'a, REG> TSEL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL0_A::VALUE1)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL0_A::VALUE2)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL0_A::VALUE3)
    }
    #[doc = "Source Selector 0 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL0_A::VALUE4)
    }
}
#[doc = "Source Selector 1 Timer connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1_A {
    #[doc = "0: Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    VALUE1 = 0,
    #[doc = "1: Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    VALUE2 = 1,
    #[doc = "2: Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    VALUE3 = 2,
    #[doc = "3: Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    VALUE4 = 3,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TSEL1_A {
    type Ux = u8;
}
impl crate::IsEnum for TSEL1_A {}
#[doc = "Field `TSEL1` reader - Source Selector 1 Timer connection"]
pub type TSEL1_R = crate::FieldReader<TSEL1_A>;
impl TSEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TSEL1_A> {
        match self.bits {
            0 => Some(TSEL1_A::VALUE1),
            1 => Some(TSEL1_A::VALUE2),
            2 => Some(TSEL1_A::VALUE3),
            3 => Some(TSEL1_A::VALUE4),
            _ => None,
        }
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TSEL1_A::VALUE1
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TSEL1_A::VALUE2
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == TSEL1_A::VALUE3
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == TSEL1_A::VALUE4
    }
}
#[doc = "Field `TSEL1` writer - Source Selector 1 Timer connection"]
pub type TSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TSEL1_A>;
impl<'a, REG> TSEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 0 (CCST0 can be used)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::VALUE1)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 1 (CCST1 can be used)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::VALUE2)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 2 (CCST2 can be used)"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::VALUE3)
    }
    #[doc = "Source Selector 1 is connected to Capture/Compare Unit Timer 3 (CCST3 can be used)"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(TSEL1_A::VALUE4)
    }
}
#[doc = "Source selector 0 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS0E_A {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    VALUE1 = 0,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    VALUE2 = 1,
}
impl From<TS0E_A> for bool {
    #[inline(always)]
    fn from(variant: TS0E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS0E` reader - Source selector 0 TRAP enable"]
pub type TS0E_R = crate::BitReader<TS0E_A>;
impl TS0E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS0E_A {
        match self.bits {
            false => TS0E_A::VALUE1,
            true => TS0E_A::VALUE2,
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS0E_A::VALUE1
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS0E_A::VALUE2
    }
}
#[doc = "Field `TS0E` writer - Source selector 0 TRAP enable"]
pub type TS0E_W<'a, REG> = crate::BitWriter<'a, REG, TS0E_A>;
impl<'a, REG> TS0E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TS0E_A::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 0 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TS0E_A::VALUE2)
    }
}
#[doc = "Source selector 1 TRAP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TS1E_A {
    #[doc = "0: TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    VALUE1 = 0,
    #[doc = "1: TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    VALUE2 = 1,
}
impl From<TS1E_A> for bool {
    #[inline(always)]
    fn from(variant: TS1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1E` reader - Source selector 1 TRAP enable"]
pub type TS1E_R = crate::BitReader<TS1E_A>;
impl TS1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TS1E_A {
        match self.bits {
            false => TS1E_A::VALUE1,
            true => TS1E_A::VALUE2,
        }
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TS1E_A::VALUE1
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == TS1E_A::VALUE2
    }
}
#[doc = "Field `TS1E` writer - Source selector 1 TRAP enable"]
pub type TS1E_W<'a, REG> = crate::BitWriter<'a, REG, TS1E_A>;
impl<'a, REG> TS1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(TS1E_A::VALUE1)
    }
    #[doc = "TRAP signal generated from the Timer connected to Source Selector 1 is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(TS1E_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    pub fn tsel0(&self) -> TSEL0_R {
        TSEL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    pub fn ts0e(&self) -> TS0E_R {
        TS0E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    pub fn ts1e(&self) -> TS1E_R {
        TS1E_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Source Selector 0 Timer connection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel0(&mut self) -> TSEL0_W<TSEL_SPEC> {
        TSEL0_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Source Selector 1 Timer connection"]
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<TSEL_SPEC> {
        TSEL1_W::new(self, 3)
    }
    #[doc = "Bit 16 - Source selector 0 TRAP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts0e(&mut self) -> TS0E_W<TSEL_SPEC> {
        TS0E_W::new(self, 16)
    }
    #[doc = "Bit 17 - Source selector 1 TRAP enable"]
    #[inline(always)]
    #[must_use]
    pub fn ts1e(&mut self) -> TS1E_W<TSEL_SPEC> {
        TS1E_W::new(self, 17)
    }
}
#[doc = "HRC timer selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSEL_SPEC;
impl crate::RegisterSpec for TSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsel::R`](R) reader structure"]
impl crate::Readable for TSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsel::W`](W) writer structure"]
impl crate::Writable for TSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSEL to value 0"]
impl crate::Resettable for TSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
