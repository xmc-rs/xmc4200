#[doc = "Register `CGATCLR0` writer"]
pub type W = crate::W<CGATCLR0_SPEC>;
#[doc = "VADC Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VADC_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<VADC_AW> for bool {
    #[inline(always)]
    fn from(variant: VADC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VADC` writer - VADC Gating Clear"]
pub type VADC_W<'a, REG> = crate::BitWriter<'a, REG, VADC_AW>;
impl<'a, REG> VADC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(VADC_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(VADC_AW::VALUE2)
    }
}
#[doc = "CCU40 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU40_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<CCU40_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU40_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU40` writer - CCU40 Gating Clear"]
pub type CCU40_W<'a, REG> = crate::BitWriter<'a, REG, CCU40_AW>;
impl<'a, REG> CCU40_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU40_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU40_AW::VALUE2)
    }
}
#[doc = "CCU41 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU41_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<CCU41_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU41_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU41` writer - CCU41 Gating Clear"]
pub type CCU41_W<'a, REG> = crate::BitWriter<'a, REG, CCU41_AW>;
impl<'a, REG> CCU41_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU41_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU41_AW::VALUE2)
    }
}
#[doc = "CCU80 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCU80_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<CCU80_AW> for bool {
    #[inline(always)]
    fn from(variant: CCU80_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCU80` writer - CCU80 Gating Clear"]
pub type CCU80_W<'a, REG> = crate::BitWriter<'a, REG, CCU80_AW>;
impl<'a, REG> CCU80_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CCU80_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CCU80_AW::VALUE2)
    }
}
#[doc = "POSIF0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POSIF0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<POSIF0_AW> for bool {
    #[inline(always)]
    fn from(variant: POSIF0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POSIF0` writer - POSIF0 Gating Clear"]
pub type POSIF0_W<'a, REG> = crate::BitWriter<'a, REG, POSIF0_AW>;
impl<'a, REG> POSIF0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(POSIF0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(POSIF0_AW::VALUE2)
    }
}
#[doc = "USIC0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<USIC0_AW> for bool {
    #[inline(always)]
    fn from(variant: USIC0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC0` writer - USIC0 Gating Clear"]
pub type USIC0_W<'a, REG> = crate::BitWriter<'a, REG, USIC0_AW>;
impl<'a, REG> USIC0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(USIC0_AW::VALUE2)
    }
}
#[doc = "ERU1 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERU1_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<ERU1_AW> for bool {
    #[inline(always)]
    fn from(variant: ERU1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERU1` writer - ERU1 Gating Clear"]
pub type ERU1_W<'a, REG> = crate::BitWriter<'a, REG, ERU1_AW>;
impl<'a, REG> ERU1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ERU1_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ERU1_AW::VALUE2)
    }
}
#[doc = "HRPWM0 Gating Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRPWM0_AW {
    #[doc = "0: No effect"]
    VALUE1 = 0,
    #[doc = "1: Disable gating"]
    VALUE2 = 1,
}
impl From<HRPWM0_AW> for bool {
    #[inline(always)]
    fn from(variant: HRPWM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HRPWM0` writer - HRPWM0 Gating Clear"]
pub type HRPWM0_W<'a, REG> = crate::BitWriter<'a, REG, HRPWM0_AW>;
impl<'a, REG> HRPWM0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HRPWM0_AW::VALUE1)
    }
    #[doc = "Disable gating"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HRPWM0_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bit 0 - VADC Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vadc(&mut self) -> VADC_W<CGATCLR0_SPEC> {
        VADC_W::new(self, 0)
    }
    #[doc = "Bit 2 - CCU40 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu40(&mut self) -> CCU40_W<CGATCLR0_SPEC> {
        CCU40_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCU41 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu41(&mut self) -> CCU41_W<CGATCLR0_SPEC> {
        CCU41_W::new(self, 3)
    }
    #[doc = "Bit 7 - CCU80 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ccu80(&mut self) -> CCU80_W<CGATCLR0_SPEC> {
        CCU80_W::new(self, 7)
    }
    #[doc = "Bit 9 - POSIF0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn posif0(&mut self) -> POSIF0_W<CGATCLR0_SPEC> {
        POSIF0_W::new(self, 9)
    }
    #[doc = "Bit 11 - USIC0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn usic0(&mut self) -> USIC0_W<CGATCLR0_SPEC> {
        USIC0_W::new(self, 11)
    }
    #[doc = "Bit 16 - ERU1 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eru1(&mut self) -> ERU1_W<CGATCLR0_SPEC> {
        ERU1_W::new(self, 16)
    }
    #[doc = "Bit 23 - HRPWM0 Gating Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hrpwm0(&mut self) -> HRPWM0_W<CGATCLR0_SPEC> {
        HRPWM0_W::new(self, 23)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral 0 Clock Gating Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cgatclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CGATCLR0_SPEC;
impl crate::RegisterSpec for CGATCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cgatclr0::W`](W) writer structure"]
impl crate::Writable for CGATCLR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CGATCLR0 to value 0"]
impl crate::Resettable for CGATCLR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
