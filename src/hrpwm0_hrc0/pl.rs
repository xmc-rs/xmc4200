#[doc = "Register `PL` reader"]
pub type R = crate::R<PL_SPEC>;
#[doc = "Register `PL` writer"]
pub type W = crate::W<PL_SPEC>;
#[doc = "HRPWMx.OUTy0 passive level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL0_A {
    #[doc = "0: HRPWMx.OUTy0 output passive level is set to LOW"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.OUTy0 output passive level is set to HIGH"]
    VALUE2 = 1,
}
impl From<PSL0_A> for bool {
    #[inline(always)]
    fn from(variant: PSL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL0` reader - HRPWMx.OUTy0 passive level"]
pub type PSL0_R = crate::BitReader<PSL0_A>;
impl PSL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSL0_A {
        match self.bits {
            false => PSL0_A::VALUE1,
            true => PSL0_A::VALUE2,
        }
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL0_A::VALUE1
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL0_A::VALUE2
    }
}
#[doc = "Field `PSL0` writer - HRPWMx.OUTy0 passive level"]
pub type PSL0_W<'a, REG> = crate::BitWriter<'a, REG, PSL0_A>;
impl<'a, REG> PSL0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL0_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL0_A::VALUE2)
    }
}
#[doc = "HRPWMx.OUTy1 passive level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSL1_A {
    #[doc = "0: HRPWMx.OUTy1 output passive level is set to LOW"]
    VALUE1 = 0,
    #[doc = "1: HRPWMx.OUTy1 output passive level is set to HIGH"]
    VALUE2 = 1,
}
impl From<PSL1_A> for bool {
    #[inline(always)]
    fn from(variant: PSL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSL1` reader - HRPWMx.OUTy1 passive level"]
pub type PSL1_R = crate::BitReader<PSL1_A>;
impl PSL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PSL1_A {
        match self.bits {
            false => PSL1_A::VALUE1,
            true => PSL1_A::VALUE2,
        }
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL1_A::VALUE1
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL1_A::VALUE2
    }
}
#[doc = "Field `PSL1` writer - HRPWMx.OUTy1 passive level"]
pub type PSL1_W<'a, REG> = crate::BitWriter<'a, REG, PSL1_A>;
impl<'a, REG> PSL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PSL1_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PSL1_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline(always)]
    pub fn psl0(&self) -> PSL0_R {
        PSL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline(always)]
    pub fn psl1(&self) -> PSL1_R {
        PSL1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HRPWMx.OUTy0 passive level"]
    #[inline(always)]
    #[must_use]
    pub fn psl0(&mut self) -> PSL0_W<PL_SPEC> {
        PSL0_W::new(self, 0)
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline(always)]
    #[must_use]
    pub fn psl1(&mut self) -> PSL1_W<PL_SPEC> {
        PSL1_W::new(self, 1)
    }
}
#[doc = "HRC output passive level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PL_SPEC;
impl crate::RegisterSpec for PL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pl::R`](R) reader structure"]
impl crate::Readable for PL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pl::W`](W) writer structure"]
impl crate::Writable for PL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PL to value 0"]
impl crate::Resettable for PL_SPEC {
    const RESET_VALUE: u32 = 0;
}
