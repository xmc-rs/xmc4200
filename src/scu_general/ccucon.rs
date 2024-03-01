#[doc = "Register `CCUCON` reader"]
pub type R = crate::R<CcuconSpec>;
#[doc = "Register `CCUCON` writer"]
pub type W = crate::W<CcuconSpec>;
#[doc = "Global Start Control CCU40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gsc40 {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Gsc40> for bool {
    #[inline(always)]
    fn from(variant: Gsc40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC40` reader - Global Start Control CCU40"]
pub type Gsc40R = crate::BitReader<Gsc40>;
impl Gsc40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gsc40 {
        match self.bits {
            false => Gsc40::Value1,
            true => Gsc40::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gsc40::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gsc40::Value2
    }
}
#[doc = "Field `GSC40` writer - Global Start Control CCU40"]
pub type Gsc40W<'a, REG> = crate::BitWriter<'a, REG, Gsc40>;
impl<'a, REG> Gsc40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gsc40::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gsc40::Value2)
    }
}
#[doc = "Global Start Control CCU41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gsc41 {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Gsc41> for bool {
    #[inline(always)]
    fn from(variant: Gsc41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC41` reader - Global Start Control CCU41"]
pub type Gsc41R = crate::BitReader<Gsc41>;
impl Gsc41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gsc41 {
        match self.bits {
            false => Gsc41::Value1,
            true => Gsc41::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gsc41::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gsc41::Value2
    }
}
#[doc = "Field `GSC41` writer - Global Start Control CCU41"]
pub type Gsc41W<'a, REG> = crate::BitWriter<'a, REG, Gsc41>;
impl<'a, REG> Gsc41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gsc41::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gsc41::Value2)
    }
}
#[doc = "Global Start Control CCU80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gsc80 {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Gsc80> for bool {
    #[inline(always)]
    fn from(variant: Gsc80) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC80` reader - Global Start Control CCU80"]
pub type Gsc80R = crate::BitReader<Gsc80>;
impl Gsc80R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gsc80 {
        match self.bits {
            false => Gsc80::Value1,
            true => Gsc80::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gsc80::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gsc80::Value2
    }
}
#[doc = "Field `GSC80` writer - Global Start Control CCU80"]
pub type Gsc80W<'a, REG> = crate::BitWriter<'a, REG, Gsc80>;
impl<'a, REG> Gsc80W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gsc80::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gsc80::Value2)
    }
}
#[doc = "Global Start Control HRPWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gshr0 {
    #[doc = "0: Disable"]
    Value1 = 0,
    #[doc = "1: Enable"]
    Value2 = 1,
}
impl From<Gshr0> for bool {
    #[inline(always)]
    fn from(variant: Gshr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSHR0` reader - Global Start Control HRPWM0"]
pub type Gshr0R = crate::BitReader<Gshr0>;
impl Gshr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gshr0 {
        match self.bits {
            false => Gshr0::Value1,
            true => Gshr0::Value2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Gshr0::Value1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Gshr0::Value2
    }
}
#[doc = "Field `GSHR0` writer - Global Start Control HRPWM0"]
pub type Gshr0W<'a, REG> = crate::BitWriter<'a, REG, Gshr0>;
impl<'a, REG> Gshr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Gshr0::Value1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Gshr0::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> Gsc40R {
        Gsc40R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> Gsc41R {
        Gsc41R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> Gsc80R {
        Gsc80R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - Global Start Control HRPWM0"]
    #[inline(always)]
    pub fn gshr0(&self) -> Gshr0R {
        Gshr0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    #[must_use]
    pub fn gsc40(&mut self) -> Gsc40W<CcuconSpec> {
        Gsc40W::new(self, 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    #[must_use]
    pub fn gsc41(&mut self) -> Gsc41W<CcuconSpec> {
        Gsc41W::new(self, 1)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    #[must_use]
    pub fn gsc80(&mut self) -> Gsc80W<CcuconSpec> {
        Gsc80W::new(self, 8)
    }
    #[doc = "Bit 24 - Global Start Control HRPWM0"]
    #[inline(always)]
    #[must_use]
    pub fn gshr0(&mut self) -> Gshr0W<CcuconSpec> {
        Gshr0W::new(self, 24)
    }
}
#[doc = "CCU Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccucon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccucon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcuconSpec;
impl crate::RegisterSpec for CcuconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccucon::R`](R) reader structure"]
impl crate::Readable for CcuconSpec {}
#[doc = "`write(|w| ..)` method takes [`ccucon::W`](W) writer structure"]
impl crate::Writable for CcuconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCUCON to value 0"]
impl crate::Resettable for CcuconSpec {
    const RESET_VALUE: u32 = 0;
}
