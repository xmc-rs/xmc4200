#[doc = "Register `MCHKCON` reader"]
pub type R = crate::R<MchkconSpec>;
#[doc = "Register `MCHKCON` writer"]
pub type W = crate::W<MchkconSpec>;
#[doc = "Select Memory Check for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selps {
    #[doc = "0: Not selected"]
    Value1 = 0,
    #[doc = "1: Selected"]
    Value2 = 1,
}
impl From<Selps> for bool {
    #[inline(always)]
    fn from(variant: Selps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELPS` reader - Select Memory Check for PSRAM"]
pub type SelpsR = crate::BitReader<Selps>;
impl SelpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selps {
        match self.bits {
            false => Selps::Value1,
            true => Selps::Value2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selps::Value1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Selps::Value2
    }
}
#[doc = "Field `SELPS` writer - Select Memory Check for PSRAM"]
pub type SelpsW<'a, REG> = crate::BitWriter<'a, REG, Selps>;
impl<'a, REG> SelpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selps::Value1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selps::Value2)
    }
}
#[doc = "Select Memory Check for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selds1 {
    #[doc = "0: Not selected"]
    Value1 = 0,
    #[doc = "1: Selected"]
    Value2 = 1,
}
impl From<Selds1> for bool {
    #[inline(always)]
    fn from(variant: Selds1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELDS1` reader - Select Memory Check for DSRAM1"]
pub type Selds1R = crate::BitReader<Selds1>;
impl Selds1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selds1 {
        match self.bits {
            false => Selds1::Value1,
            true => Selds1::Value2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selds1::Value1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Selds1::Value2
    }
}
#[doc = "Field `SELDS1` writer - Select Memory Check for DSRAM1"]
pub type Selds1W<'a, REG> = crate::BitWriter<'a, REG, Selds1>;
impl<'a, REG> Selds1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selds1::Value1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selds1::Value2)
    }
}
#[doc = "Select Memory Check for USIC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic0dra {
    #[doc = "0: Not selected"]
    Value1 = 0,
    #[doc = "1: Selected"]
    Value2 = 1,
}
impl From<Usic0dra> for bool {
    #[inline(always)]
    fn from(variant: Usic0dra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0DRA` reader - Select Memory Check for USIC0"]
pub type Usic0draR = crate::BitReader<Usic0dra>;
impl Usic0draR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic0dra {
        match self.bits {
            false => Usic0dra::Value1,
            true => Usic0dra::Value2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usic0dra::Value1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usic0dra::Value2
    }
}
#[doc = "Field `USIC0DRA` writer - Select Memory Check for USIC0"]
pub type Usic0draW<'a, REG> = crate::BitWriter<'a, REG, Usic0dra>;
impl<'a, REG> Usic0draW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0dra::Value1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usic0dra::Value2)
    }
}
#[doc = "Select Memory Check for USIC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usic1dra {
    #[doc = "0: Not selected"]
    Value1 = 0,
    #[doc = "1: Selected"]
    Value2 = 1,
}
impl From<Usic1dra> for bool {
    #[inline(always)]
    fn from(variant: Usic1dra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1DRA` reader - Select Memory Check for USIC1"]
pub type Usic1draR = crate::BitReader<Usic1dra>;
impl Usic1draR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usic1dra {
        match self.bits {
            false => Usic1dra::Value1,
            true => Usic1dra::Value2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Usic1dra::Value1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Usic1dra::Value2
    }
}
#[doc = "Field `USIC1DRA` writer - Select Memory Check for USIC1"]
pub type Usic1draW<'a, REG> = crate::BitWriter<'a, REG, Usic1dra>;
impl<'a, REG> Usic1draW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1dra::Value1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Usic1dra::Value2)
    }
}
#[doc = "Select Memory Check for MultiCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcandra {
    #[doc = "0: Not selected"]
    Value1 = 0,
    #[doc = "1: Selected"]
    Value2 = 1,
}
impl From<Mcandra> for bool {
    #[inline(always)]
    fn from(variant: Mcandra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANDRA` reader - Select Memory Check for MultiCAN"]
pub type McandraR = crate::BitReader<Mcandra>;
impl McandraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcandra {
        match self.bits {
            false => Mcandra::Value1,
            true => Mcandra::Value2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mcandra::Value1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mcandra::Value2
    }
}
#[doc = "Field `MCANDRA` writer - Select Memory Check for MultiCAN"]
pub type McandraW<'a, REG> = crate::BitWriter<'a, REG, Mcandra>;
impl<'a, REG> McandraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mcandra::Value1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mcandra::Value2)
    }
}
#[doc = "Select Memory Check for PMU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pprfdra {
    #[doc = "0: Not selected"]
    Value1 = 0,
    #[doc = "1: Selected"]
    Value2 = 1,
}
impl From<Pprfdra> for bool {
    #[inline(always)]
    fn from(variant: Pprfdra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPRFDRA` reader - Select Memory Check for PMU"]
pub type PprfdraR = crate::BitReader<Pprfdra>;
impl PprfdraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pprfdra {
        match self.bits {
            false => Pprfdra::Value1,
            true => Pprfdra::Value2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Pprfdra::Value1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Pprfdra::Value2
    }
}
#[doc = "Field `PPRFDRA` writer - Select Memory Check for PMU"]
pub type PprfdraW<'a, REG> = crate::BitWriter<'a, REG, Pprfdra>;
impl<'a, REG> PprfdraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Pprfdra::Value1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Pprfdra::Value2)
    }
}
#[doc = "Select Memory Check for USB SRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selusb {
    #[doc = "0: Not selected"]
    Value1 = 0,
    #[doc = "1: Selected"]
    Value2 = 1,
}
impl From<Selusb> for bool {
    #[inline(always)]
    fn from(variant: Selusb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELUSB` reader - Select Memory Check for USB SRAM"]
pub type SelusbR = crate::BitReader<Selusb>;
impl SelusbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Selusb {
        match self.bits {
            false => Selusb::Value1,
            true => Selusb::Value2,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Selusb::Value1
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Selusb::Value2
    }
}
#[doc = "Field `SELUSB` writer - Select Memory Check for USB SRAM"]
pub type SelusbW<'a, REG> = crate::BitWriter<'a, REG, Selusb>;
impl<'a, REG> SelusbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Selusb::Value1)
    }
    #[doc = "Selected"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Selusb::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    pub fn selps(&self) -> SelpsR {
        SelpsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    pub fn selds1(&self) -> Selds1R {
        Selds1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    pub fn usic0dra(&self) -> Usic0draR {
        Usic0draR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    pub fn usic1dra(&self) -> Usic1draR {
        Usic1draR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    pub fn mcandra(&self) -> McandraR {
        McandraR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    pub fn pprfdra(&self) -> PprfdraR {
        PprfdraR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    pub fn selusb(&self) -> SelusbR {
        SelusbR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Memory Check for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selps(&mut self) -> SelpsW<MchkconSpec> {
        SelpsW::new(self, 0)
    }
    #[doc = "Bit 1 - Select Memory Check for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn selds1(&mut self) -> Selds1W<MchkconSpec> {
        Selds1W::new(self, 1)
    }
    #[doc = "Bit 8 - Select Memory Check for USIC0"]
    #[inline(always)]
    #[must_use]
    pub fn usic0dra(&mut self) -> Usic0draW<MchkconSpec> {
        Usic0draW::new(self, 8)
    }
    #[doc = "Bit 9 - Select Memory Check for USIC1"]
    #[inline(always)]
    #[must_use]
    pub fn usic1dra(&mut self) -> Usic1draW<MchkconSpec> {
        Usic1draW::new(self, 9)
    }
    #[doc = "Bit 12 - Select Memory Check for MultiCAN"]
    #[inline(always)]
    #[must_use]
    pub fn mcandra(&mut self) -> McandraW<MchkconSpec> {
        McandraW::new(self, 12)
    }
    #[doc = "Bit 13 - Select Memory Check for PMU"]
    #[inline(always)]
    #[must_use]
    pub fn pprfdra(&mut self) -> PprfdraW<MchkconSpec> {
        PprfdraW::new(self, 13)
    }
    #[doc = "Bit 16 - Select Memory Check for USB SRAM"]
    #[inline(always)]
    #[must_use]
    pub fn selusb(&mut self) -> SelusbW<MchkconSpec> {
        SelusbW::new(self, 16)
    }
}
#[doc = "Memory Checking Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mchkcon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mchkcon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MchkconSpec;
impl crate::RegisterSpec for MchkconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mchkcon::R`](R) reader structure"]
impl crate::Readable for MchkconSpec {}
#[doc = "`write(|w| ..)` method takes [`mchkcon::W`](W) writer structure"]
impl crate::Writable for MchkconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCHKCON to value 0"]
impl crate::Resettable for MchkconSpec {
    const RESET_VALUE: u32 = 0;
}
