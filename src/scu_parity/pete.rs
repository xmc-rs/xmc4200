#[doc = "Register `PETE` reader"]
pub type R = crate::R<PeteSpec>;
#[doc = "Register `PETE` writer"]
pub type W = crate::W<PeteSpec>;
#[doc = "Parity Error Trap Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteps {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteps> for bool {
    #[inline(always)]
    fn from(variant: Peteps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPS` reader - Parity Error Trap Enable for PSRAM"]
pub type PetepsR = crate::BitReader<Peteps>;
impl PetepsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteps {
        match self.bits {
            false => Peteps::Value1,
            true => Peteps::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteps::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteps::Value2
    }
}
#[doc = "Field `PETEPS` writer - Parity Error Trap Enable for PSRAM"]
pub type PetepsW<'a, REG> = crate::BitWriter<'a, REG, Peteps>;
impl<'a, REG> PetepsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteps::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteps::Value2)
    }
}
#[doc = "Parity Error Trap Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteds1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteds1> for bool {
    #[inline(always)]
    fn from(variant: Peteds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEDS1` reader - Parity Error Trap Enable for DSRAM1"]
pub type Peteds1R = crate::BitReader<Peteds1>;
impl Peteds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteds1 {
        match self.bits {
            false => Peteds1::Value1,
            true => Peteds1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteds1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteds1::Value2
    }
}
#[doc = "Field `PETEDS1` writer - Parity Error Trap Enable for DSRAM1"]
pub type Peteds1W<'a, REG> = crate::BitWriter<'a, REG, Peteds1>;
impl<'a, REG> Peteds1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteds1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteds1::Value2)
    }
}
#[doc = "Parity Error Trap Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteu0 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteu0> for bool {
    #[inline(always)]
    fn from(variant: Peteu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU0` reader - Parity Error Trap Enable for USIC0 Memory"]
pub type Peteu0R = crate::BitReader<Peteu0>;
impl Peteu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteu0 {
        match self.bits {
            false => Peteu0::Value1,
            true => Peteu0::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteu0::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteu0::Value2
    }
}
#[doc = "Field `PETEU0` writer - Parity Error Trap Enable for USIC0 Memory"]
pub type Peteu0W<'a, REG> = crate::BitWriter<'a, REG, Peteu0>;
impl<'a, REG> Peteu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu0::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu0::Value2)
    }
}
#[doc = "Parity Error Trap Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteu1 {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteu1> for bool {
    #[inline(always)]
    fn from(variant: Peteu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEU1` reader - Parity Error Trap Enable for USIC1 Memory"]
pub type Peteu1R = crate::BitReader<Peteu1>;
impl Peteu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteu1 {
        match self.bits {
            false => Peteu1::Value1,
            true => Peteu1::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteu1::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteu1::Value2
    }
}
#[doc = "Field `PETEU1` writer - Parity Error Trap Enable for USIC1 Memory"]
pub type Peteu1W<'a, REG> = crate::BitWriter<'a, REG, Peteu1>;
impl<'a, REG> Peteu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu1::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteu1::Value2)
    }
}
#[doc = "Parity Error Trap Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Petemc {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Petemc> for bool {
    #[inline(always)]
    fn from(variant: Petemc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEMC` reader - Parity Error Trap Enable for MultiCAN Memory"]
pub type PetemcR = crate::BitReader<Petemc>;
impl PetemcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Petemc {
        match self.bits {
            false => Petemc::Value1,
            true => Petemc::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Petemc::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Petemc::Value2
    }
}
#[doc = "Field `PETEMC` writer - Parity Error Trap Enable for MultiCAN Memory"]
pub type PetemcW<'a, REG> = crate::BitWriter<'a, REG, Petemc>;
impl<'a, REG> PetemcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Petemc::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Petemc::Value2)
    }
}
#[doc = "Parity Error Trap Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Petepprf {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Petepprf> for bool {
    #[inline(always)]
    fn from(variant: Petepprf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEPPRF` reader - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PetepprfR = crate::BitReader<Petepprf>;
impl PetepprfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Petepprf {
        match self.bits {
            false => Petepprf::Value1,
            true => Petepprf::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Petepprf::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Petepprf::Value2
    }
}
#[doc = "Field `PETEPPRF` writer - Parity Error Trap Enable for PMU Prefetch Memory"]
pub type PetepprfW<'a, REG> = crate::BitWriter<'a, REG, Petepprf>;
impl<'a, REG> PetepprfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Petepprf::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Petepprf::Value2)
    }
}
#[doc = "Parity Error Trap Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peteusb {
    #[doc = "0: Disabled"]
    Value1 = 0,
    #[doc = "1: Enabled"]
    Value2 = 1,
}
impl From<Peteusb> for bool {
    #[inline(always)]
    fn from(variant: Peteusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PETEUSB` reader - Parity Error Trap Enable for USB Memory"]
pub type PeteusbR = crate::BitReader<Peteusb>;
impl PeteusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peteusb {
        match self.bits {
            false => Peteusb::Value1,
            true => Peteusb::Value2,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Peteusb::Value1
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Peteusb::Value2
    }
}
#[doc = "Field `PETEUSB` writer - Parity Error Trap Enable for USB Memory"]
pub type PeteusbW<'a, REG> = crate::BitWriter<'a, REG, Peteusb>;
impl<'a, REG> PeteusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Peteusb::Value1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Peteusb::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    pub fn peteps(&self) -> PetepsR {
        PetepsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    pub fn peteds1(&self) -> Peteds1R {
        Peteds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peteu0(&self) -> Peteu0R {
        Peteu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peteu1(&self) -> Peteu1R {
        Peteu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn petemc(&self) -> PetemcR {
        PetemcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn petepprf(&self) -> PetepprfR {
        PetepprfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    pub fn peteusb(&self) -> PeteusbR {
        PeteusbR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Trap Enable for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn peteps(&mut self) -> PetepsW<PeteSpec> {
        PetepsW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Error Trap Enable for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn peteds1(&mut self) -> Peteds1W<PeteSpec> {
        Peteds1W::new(self, 1)
    }
    #[doc = "Bit 8 - Parity Error Trap Enable for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu0(&mut self) -> Peteu0W<PeteSpec> {
        Peteu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity Error Trap Enable for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteu1(&mut self) -> Peteu1W<PeteSpec> {
        Peteu1W::new(self, 9)
    }
    #[doc = "Bit 12 - Parity Error Trap Enable for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petemc(&mut self) -> PetemcW<PeteSpec> {
        PetemcW::new(self, 12)
    }
    #[doc = "Bit 13 - Parity Error Trap Enable for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn petepprf(&mut self) -> PetepprfW<PeteSpec> {
        PetepprfW::new(self, 13)
    }
    #[doc = "Bit 16 - Parity Error Trap Enable for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peteusb(&mut self) -> PeteusbW<PeteSpec> {
        PeteusbW::new(self, 16)
    }
}
#[doc = "Parity Error Trap Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pete::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pete::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeteSpec;
impl crate::RegisterSpec for PeteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pete::R`](R) reader structure"]
impl crate::Readable for PeteSpec {}
#[doc = "`write(|w| ..)` method takes [`pete::W`](W) writer structure"]
impl crate::Writable for PeteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PETE to value 0"]
impl crate::Resettable for PeteSpec {
    const RESET_VALUE: u32 = 0;
}
