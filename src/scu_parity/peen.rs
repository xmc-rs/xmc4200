#[doc = "Register `PEEN` reader"]
pub struct R(crate::R<PEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEEN` writer"]
pub struct W(crate::W<PEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEEN_SPEC>;
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
impl From<crate::W<PEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEENPS` reader - Parity Error Enable for PSRAM"]
pub type PEENPS_R = crate::BitReader<PEENPS_A>;
#[doc = "Parity Error Enable for PSRAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENPS_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENPS_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPS_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENPS_A {
        match self.bits {
            false => PEENPS_A::VALUE1,
            true => PEENPS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENPS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENPS_A::VALUE2
    }
}
#[doc = "Field `PEENPS` writer - Parity Error Enable for PSRAM"]
pub type PEENPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEEN_SPEC, PEENPS_A, O>;
impl<'a, const O: u8> PEENPS_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENPS_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENPS_A::VALUE2)
    }
}
#[doc = "Field `PEENDS1` reader - Parity Error Enable for DSRAM1"]
pub type PEENDS1_R = crate::BitReader<PEENDS1_A>;
#[doc = "Parity Error Enable for DSRAM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENDS1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENDS1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENDS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENDS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENDS1_A {
        match self.bits {
            false => PEENDS1_A::VALUE1,
            true => PEENDS1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENDS1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENDS1_A::VALUE2
    }
}
#[doc = "Field `PEENDS1` writer - Parity Error Enable for DSRAM1"]
pub type PEENDS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEEN_SPEC, PEENDS1_A, O>;
impl<'a, const O: u8> PEENDS1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENDS1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENDS1_A::VALUE2)
    }
}
#[doc = "Field `PEENU0` reader - Parity Error Enable for USIC0 Memory"]
pub type PEENU0_R = crate::BitReader<PEENU0_A>;
#[doc = "Parity Error Enable for USIC0 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENU0_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENU0_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU0_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENU0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENU0_A {
        match self.bits {
            false => PEENU0_A::VALUE1,
            true => PEENU0_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENU0_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENU0_A::VALUE2
    }
}
#[doc = "Field `PEENU0` writer - Parity Error Enable for USIC0 Memory"]
pub type PEENU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEEN_SPEC, PEENU0_A, O>;
impl<'a, const O: u8> PEENU0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENU0_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENU0_A::VALUE2)
    }
}
#[doc = "Field `PEENU1` reader - Parity Error Enable for USIC1 Memory"]
pub type PEENU1_R = crate::BitReader<PEENU1_A>;
#[doc = "Parity Error Enable for USIC1 Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENU1_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENU1_A> for bool {
    #[inline(always)]
    fn from(variant: PEENU1_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENU1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENU1_A {
        match self.bits {
            false => PEENU1_A::VALUE1,
            true => PEENU1_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENU1_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENU1_A::VALUE2
    }
}
#[doc = "Field `PEENU1` writer - Parity Error Enable for USIC1 Memory"]
pub type PEENU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEEN_SPEC, PEENU1_A, O>;
impl<'a, const O: u8> PEENU1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENU1_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENU1_A::VALUE2)
    }
}
#[doc = "Field `PEENMC` reader - Parity Error Enable for MultiCAN Memory"]
pub type PEENMC_R = crate::BitReader<PEENMC_A>;
#[doc = "Parity Error Enable for MultiCAN Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENMC_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENMC_A> for bool {
    #[inline(always)]
    fn from(variant: PEENMC_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENMC_A {
        match self.bits {
            false => PEENMC_A::VALUE1,
            true => PEENMC_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENMC_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENMC_A::VALUE2
    }
}
#[doc = "Field `PEENMC` writer - Parity Error Enable for MultiCAN Memory"]
pub type PEENMC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEEN_SPEC, PEENMC_A, O>;
impl<'a, const O: u8> PEENMC_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENMC_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENMC_A::VALUE2)
    }
}
#[doc = "Field `PEENPPRF` reader - Parity Error Enable for PMU Prefetch Memory"]
pub type PEENPPRF_R = crate::BitReader<PEENPPRF_A>;
#[doc = "Parity Error Enable for PMU Prefetch Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENPPRF_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENPPRF_A> for bool {
    #[inline(always)]
    fn from(variant: PEENPPRF_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENPPRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENPPRF_A {
        match self.bits {
            false => PEENPPRF_A::VALUE1,
            true => PEENPPRF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENPPRF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENPPRF_A::VALUE2
    }
}
#[doc = "Field `PEENPPRF` writer - Parity Error Enable for PMU Prefetch Memory"]
pub type PEENPPRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEEN_SPEC, PEENPPRF_A, O>;
impl<'a, const O: u8> PEENPPRF_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENPPRF_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENPPRF_A::VALUE2)
    }
}
#[doc = "Field `PEENUSB` reader - Parity Error Enable for USB Memory"]
pub type PEENUSB_R = crate::BitReader<PEENUSB_A>;
#[doc = "Parity Error Enable for USB Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEENUSB_A {
    #[doc = "0: Disabled"]
    VALUE1 = 0,
    #[doc = "1: Enabled"]
    VALUE2 = 1,
}
impl From<PEENUSB_A> for bool {
    #[inline(always)]
    fn from(variant: PEENUSB_A) -> Self {
        variant as u8 != 0
    }
}
impl PEENUSB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEENUSB_A {
        match self.bits {
            false => PEENUSB_A::VALUE1,
            true => PEENUSB_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PEENUSB_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PEENUSB_A::VALUE2
    }
}
#[doc = "Field `PEENUSB` writer - Parity Error Enable for USB Memory"]
pub type PEENUSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PEEN_SPEC, PEENUSB_A, O>;
impl<'a, const O: u8> PEENUSB_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEENUSB_A::VALUE1)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEENUSB_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    pub fn peenps(&self) -> PEENPS_R {
        PEENPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    pub fn peends1(&self) -> PEENDS1_R {
        PEENDS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    pub fn peenu0(&self) -> PEENU0_R {
        PEENU0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    pub fn peenu1(&self) -> PEENU1_R {
        PEENU1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    pub fn peenmc(&self) -> PEENMC_R {
        PEENMC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    pub fn peenpprf(&self) -> PEENPPRF_R {
        PEENPPRF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    pub fn peenusb(&self) -> PEENUSB_R {
        PEENUSB_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Enable for PSRAM"]
    #[inline(always)]
    #[must_use]
    pub fn peenps(&mut self) -> PEENPS_W<0> {
        PEENPS_W::new(self)
    }
    #[doc = "Bit 1 - Parity Error Enable for DSRAM1"]
    #[inline(always)]
    #[must_use]
    pub fn peends1(&mut self) -> PEENDS1_W<1> {
        PEENDS1_W::new(self)
    }
    #[doc = "Bit 8 - Parity Error Enable for USIC0 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenu0(&mut self) -> PEENU0_W<8> {
        PEENU0_W::new(self)
    }
    #[doc = "Bit 9 - Parity Error Enable for USIC1 Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenu1(&mut self) -> PEENU1_W<9> {
        PEENU1_W::new(self)
    }
    #[doc = "Bit 12 - Parity Error Enable for MultiCAN Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenmc(&mut self) -> PEENMC_W<12> {
        PEENMC_W::new(self)
    }
    #[doc = "Bit 13 - Parity Error Enable for PMU Prefetch Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenpprf(&mut self) -> PEENPPRF_W<13> {
        PEENPPRF_W::new(self)
    }
    #[doc = "Bit 16 - Parity Error Enable for USB Memory"]
    #[inline(always)]
    #[must_use]
    pub fn peenusb(&mut self) -> PEENUSB_W<16> {
        PEENUSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parity Error Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peen](index.html) module"]
pub struct PEEN_SPEC;
impl crate::RegisterSpec for PEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peen::R](R) reader structure"]
impl crate::Readable for PEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peen::W](W) writer structure"]
impl crate::Writable for PEEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEEN to value 0"]
impl crate::Resettable for PEEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
