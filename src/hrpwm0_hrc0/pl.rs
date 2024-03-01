#[doc = "Register `PL` reader"]
pub type R = crate::R<PlSpec>;
#[doc = "Register `PL` writer"]
pub type W = crate::W<PlSpec>;
#[doc = "HRPWMx.OUTy0 passive level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl0 {
    #[doc = "0: HRPWMx.OUTy0 output passive level is set to LOW"]
    Value1 = 0,
    #[doc = "1: HRPWMx.OUTy0 output passive level is set to HIGH"]
    Value2 = 1,
}
impl From<Psl0> for bool {
    #[inline(always)]
    fn from(variant: Psl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL0` reader - HRPWMx.OUTy0 passive level"]
pub type Psl0R = crate::BitReader<Psl0>;
impl Psl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl0 {
        match self.bits {
            false => Psl0::Value1,
            true => Psl0::Value2,
        }
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl0::Value1
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl0::Value2
    }
}
#[doc = "Field `PSL0` writer - HRPWMx.OUTy0 passive level"]
pub type Psl0W<'a, REG> = crate::BitWriter<'a, REG, Psl0>;
impl<'a, REG> Psl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl0::Value1)
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl0::Value2)
    }
}
#[doc = "HRPWMx.OUTy1 passive level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Psl1 {
    #[doc = "0: HRPWMx.OUTy1 output passive level is set to LOW"]
    Value1 = 0,
    #[doc = "1: HRPWMx.OUTy1 output passive level is set to HIGH"]
    Value2 = 1,
}
impl From<Psl1> for bool {
    #[inline(always)]
    fn from(variant: Psl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL1` reader - HRPWMx.OUTy1 passive level"]
pub type Psl1R = crate::BitReader<Psl1>;
impl Psl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psl1 {
        match self.bits {
            false => Psl1::Value1,
            true => Psl1::Value2,
        }
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Psl1::Value1
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Psl1::Value2
    }
}
#[doc = "Field `PSL1` writer - HRPWMx.OUTy1 passive level"]
pub type Psl1W<'a, REG> = crate::BitWriter<'a, REG, Psl1>;
impl<'a, REG> Psl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Psl1::Value1)
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Psl1::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline(always)]
    pub fn psl0(&self) -> Psl0R {
        Psl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline(always)]
    pub fn psl1(&self) -> Psl1R {
        Psl1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline(always)]
    #[must_use]
    pub fn psl0(&mut self) -> Psl0W<PlSpec> {
        Psl0W::new(self, 0)
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline(always)]
    #[must_use]
    pub fn psl1(&mut self) -> Psl1W<PlSpec> {
        Psl1W::new(self, 1)
    }
}
#[doc = "HRC output passive level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlSpec;
impl crate::RegisterSpec for PlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pl::R`](R) reader structure"]
impl crate::Readable for PlSpec {}
#[doc = "`write(|w| ..)` method takes [`pl::W`](W) writer structure"]
impl crate::Writable for PlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PL to value 0"]
impl crate::Resettable for PlSpec {
    const RESET_VALUE: u32 = 0;
}
