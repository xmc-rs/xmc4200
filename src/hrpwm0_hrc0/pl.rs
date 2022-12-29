#[doc = "Register `PL` reader"]
pub struct R(crate::R<PL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PL` writer"]
pub struct W(crate::W<PL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSL0` reader - HRPWMx.OUTy0 passive level"]
pub type PSL0_R = crate::BitReader<PSL0_A>;
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
impl PSL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL0_A {
        match self.bits {
            false => PSL0_A::VALUE1,
            true => PSL0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL0_A::VALUE2
    }
}
#[doc = "Field `PSL0` writer - HRPWMx.OUTy0 passive level"]
pub type PSL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PL_SPEC, PSL0_A, O>;
impl<'a, const O: u8> PSL0_W<'a, O> {
    #[doc = "HRPWMx.OUTy0 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL0_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy0 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PSL0_A::VALUE2)
    }
}
#[doc = "Field `PSL1` reader - HRPWMx.OUTy1 passive level"]
pub type PSL1_R = crate::BitReader<PSL1_A>;
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
impl PSL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSL1_A {
        match self.bits {
            false => PSL1_A::VALUE1,
            true => PSL1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PSL1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PSL1_A::VALUE2
    }
}
#[doc = "Field `PSL1` writer - HRPWMx.OUTy1 passive level"]
pub type PSL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PL_SPEC, PSL1_A, O>;
impl<'a, const O: u8> PSL1_W<'a, O> {
    #[doc = "HRPWMx.OUTy1 output passive level is set to LOW"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PSL1_A::VALUE1)
    }
    #[doc = "HRPWMx.OUTy1 output passive level is set to HIGH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
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
    pub fn psl0(&mut self) -> PSL0_W<0> {
        PSL0_W::new(self)
    }
    #[doc = "Bit 1 - HRPWMx.OUTy1 passive level"]
    #[inline(always)]
    #[must_use]
    pub fn psl1(&mut self) -> PSL1_W<1> {
        PSL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRC output passive level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl](index.html) module"]
pub struct PL_SPEC;
impl crate::RegisterSpec for PL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pl::R](R) reader structure"]
impl crate::Readable for PL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pl::W](W) writer structure"]
impl crate::Writable for PL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PL to value 0"]
impl crate::Resettable for PL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
