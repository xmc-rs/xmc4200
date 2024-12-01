#[doc = "Register `CCUCON` reader"]
pub type R = crate::R<CCUCON_SPEC>;
#[doc = "Register `CCUCON` writer"]
pub type W = crate::W<CCUCON_SPEC>;
#[doc = "Global Start Control CCU40\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC40_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC40_A> for bool {
    #[inline(always)]
    fn from(variant: GSC40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC40` reader - Global Start Control CCU40"]
pub type GSC40_R = crate::BitReader<GSC40_A>;
impl GSC40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC40_A {
        match self.bits {
            false => GSC40_A::VALUE1,
            true => GSC40_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC40_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC40_A::VALUE2
    }
}
#[doc = "Field `GSC40` writer - Global Start Control CCU40"]
pub type GSC40_W<'a, REG> = crate::BitWriter<'a, REG, GSC40_A>;
impl<'a, REG> GSC40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC40_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC40_A::VALUE2)
    }
}
#[doc = "Global Start Control CCU41\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC41_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC41_A> for bool {
    #[inline(always)]
    fn from(variant: GSC41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC41` reader - Global Start Control CCU41"]
pub type GSC41_R = crate::BitReader<GSC41_A>;
impl GSC41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC41_A {
        match self.bits {
            false => GSC41_A::VALUE1,
            true => GSC41_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC41_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC41_A::VALUE2
    }
}
#[doc = "Field `GSC41` writer - Global Start Control CCU41"]
pub type GSC41_W<'a, REG> = crate::BitWriter<'a, REG, GSC41_A>;
impl<'a, REG> GSC41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC41_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC41_A::VALUE2)
    }
}
#[doc = "Global Start Control CCU80\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSC80_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSC80_A> for bool {
    #[inline(always)]
    fn from(variant: GSC80_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSC80` reader - Global Start Control CCU80"]
pub type GSC80_R = crate::BitReader<GSC80_A>;
impl GSC80_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSC80_A {
        match self.bits {
            false => GSC80_A::VALUE1,
            true => GSC80_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSC80_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSC80_A::VALUE2
    }
}
#[doc = "Field `GSC80` writer - Global Start Control CCU80"]
pub type GSC80_W<'a, REG> = crate::BitWriter<'a, REG, GSC80_A>;
impl<'a, REG> GSC80_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSC80_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSC80_A::VALUE2)
    }
}
#[doc = "Global Start Control HRPWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSHR0_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<GSHR0_A> for bool {
    #[inline(always)]
    fn from(variant: GSHR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GSHR0` reader - Global Start Control HRPWM0"]
pub type GSHR0_R = crate::BitReader<GSHR0_A>;
impl GSHR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GSHR0_A {
        match self.bits {
            false => GSHR0_A::VALUE1,
            true => GSHR0_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GSHR0_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GSHR0_A::VALUE2
    }
}
#[doc = "Field `GSHR0` writer - Global Start Control HRPWM0"]
pub type GSHR0_W<'a, REG> = crate::BitWriter<'a, REG, GSHR0_A>;
impl<'a, REG> GSHR0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GSHR0_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GSHR0_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&self) -> GSC40_R {
        GSC40_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&self) -> GSC41_R {
        GSC41_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&self) -> GSC80_R {
        GSC80_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 24 - Global Start Control HRPWM0"]
    #[inline(always)]
    pub fn gshr0(&self) -> GSHR0_R {
        GSHR0_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Start Control CCU40"]
    #[inline(always)]
    pub fn gsc40(&mut self) -> GSC40_W<CCUCON_SPEC> {
        GSC40_W::new(self, 0)
    }
    #[doc = "Bit 1 - Global Start Control CCU41"]
    #[inline(always)]
    pub fn gsc41(&mut self) -> GSC41_W<CCUCON_SPEC> {
        GSC41_W::new(self, 1)
    }
    #[doc = "Bit 8 - Global Start Control CCU80"]
    #[inline(always)]
    pub fn gsc80(&mut self) -> GSC80_W<CCUCON_SPEC> {
        GSC80_W::new(self, 8)
    }
    #[doc = "Bit 24 - Global Start Control HRPWM0"]
    #[inline(always)]
    pub fn gshr0(&mut self) -> GSHR0_W<CCUCON_SPEC> {
        GSHR0_W::new(self, 24)
    }
}
#[doc = "CCU Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccucon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccucon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCUCON_SPEC;
impl crate::RegisterSpec for CCUCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccucon::R`](R) reader structure"]
impl crate::Readable for CCUCON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccucon::W`](W) writer structure"]
impl crate::Writable for CCUCON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCUCON to value 0"]
impl crate::Resettable for CCUCON_SPEC {
    const RESET_VALUE: u32 = 0;
}
